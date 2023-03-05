use std::ops::{Add, Mul};

use nalgebra::{Matrix3, Vector3};
use rand::{rngs::StdRng, seq::SliceRandom, Rng, SeedableRng};
use rerun::{
    components::{ColorRGBA, Point3D, Radius, ViewCoordinates},
    coordinates::{Handedness, SignedAxis3},
    demo_util::lerp,
    external::glam::{self, vec3, Vec3},
    time::{Time, TimeType, Timeline},
    MsgSender, Session,
};
struct DistanceConstraint(usize, usize, f32);

struct ShapeConstraint(Vec<usize>, Vec<Vector3<f32>>, Matrix3<f32>);

/// Linearly interpolates from `a` through `b` in `n` steps, returning the intermediate result at
/// each step.
#[inline]
pub fn linspace<T>(
    a: T,
    b: T,
    n: usize,
) -> impl Iterator<Item = <<f32 as Mul<T>>::Output as std::ops::Add>::Output>
where
    T: Copy + Mul<f32>,
    f32: Mul<T>,
    <T as Mul<f32>>::Output: Add<<f32 as Mul<T>>::Output>,
    <f32 as Mul<T>>::Output: Add,
{
    (0..n).map(move |t| lerp(a, b, t as f32 / (n - 1).max(1) as f32))
}

/// Given two 3D vectors `from` and `to`, linearly interpolates between them in `n` steps along
/// the three axes, returning the intermediate result at each step.
fn grid(
    from: glam::Vec3,
    to: glam::Vec3,
    nx: usize,
    ny: usize,
    nz: usize,
) -> impl Iterator<Item = glam::Vec3> {
    linspace(from.z, to.z, nz).flat_map(move |z| {
        linspace(from.y, to.y, ny)
            .flat_map(move |y| linspace(from.x, to.x, nx).map(move |x| (x, y, z).into()))
    })
}

fn create_edges(points: &[Vec3], nx: usize, ny: usize, nz: usize) -> Vec<DistanceConstraint> {
    let mut edges = Vec::<DistanceConstraint>::with_capacity(
        nx * ny * (nz - 1) + nx * (ny - 1) * nz + (nx - 1) * ny * nz,
    );
    // Edges in x direction
    for iz in 0..nz {
        for iy in 0..ny {
            for ix2 in 1..nx {
                let ix1 = ix2 - 1;
                let ip1 = iz * nx * ny + iy * nx + ix1;
                let ip2 = iz * nx * ny + iy * nx + ix2;
                let p1 = &points[ip1];
                let p2 = &points[ip2];
                edges.push(DistanceConstraint(ip1, ip2, p1.distance(*p2)));
            }
        }
    }
    // Edges in y direction
    for iz in 0..nz {
        for iy2 in 1..ny {
            let iy1 = iy2 - 1;
            for ix in 0..nx {
                let ip1 = iz * nx * ny + iy1 * nx + ix;
                let ip2 = iz * nx * ny + iy2 * nx + ix;
                let p1 = &points[ip1];
                let p2 = &points[ip2];
                edges.push(DistanceConstraint(ip1, ip2, p1.distance(*p2)));
            }
        }
    }
    // Edges in z direction
    for iz2 in 1..nz {
        let iz1 = iz2 - 1;
        for iy in 0..ny {
            for ix in 0..nx {
                let ip1 = iz1 * nx * ny + iy * nx + ix;
                let ip2 = iz2 * nx * ny + iy * nx + ix;
                let p1 = &points[ip1];
                let p2 = &points[ip2];
                edges.push(DistanceConstraint(ip1, ip2, p1.distance(*p2)));
            }
        }
    }
    // Edges in xy direction
    for iz in 0..nz {
        for iy2 in 1..ny {
            let iy1 = iy2 - 1;
            for ix2 in 1..nx {
                let ix1 = ix2 - 1;

                let ip1 = iz * nx * ny + iy1 * nx + ix1;
                let ip2 = iz * nx * ny + iy2 * nx + ix2;
                let p1 = &points[ip1];
                let p2 = &points[ip2];
                edges.push(DistanceConstraint(ip1, ip2, p1.distance(*p2)));

                let ip1 = iz * nx * ny + iy2 * nx + ix1;
                let ip2 = iz * nx * ny + iy1 * nx + ix2;
                let p1 = &points[ip1];
                let p2 = &points[ip2];
                edges.push(DistanceConstraint(ip1, ip2, p1.distance(*p2)));
            }
        }
    }
    // Edges in yz direction
    for iz2 in 1..nz {
        let iz1 = iz2 - 1;
        for iy2 in 1..ny {
            let iy1 = iy2 - 1;
            for ix in 0..nx {
                let ip1 = iz1 * nx * ny + iy1 * nx + ix;
                let ip2 = iz2 * nx * ny + iy2 * nx + ix;
                let p1 = &points[ip1];
                let p2 = &points[ip2];
                edges.push(DistanceConstraint(ip1, ip2, p1.distance(*p2)));

                let ip1 = iz2 * nx * ny + iy1 * nx + ix;
                let ip2 = iz1 * nx * ny + iy2 * nx + ix;
                let p1 = &points[ip1];
                let p2 = &points[ip2];
                edges.push(DistanceConstraint(ip1, ip2, p1.distance(*p2)));
            }
        }
    }
    // Edges in zx direction
    for iz2 in 1..nz {
        let iz1 = iz2 - 1;
        for iy in 0..ny {
            for ix2 in 1..nx {
                let ix1 = ix2 - 1;

                let ip1 = iz1 * nx * ny + iy * nx + ix1;
                let ip2 = iz2 * nx * ny + iy * nx + ix2;
                let p1 = &points[ip1];
                let p2 = &points[ip2];
                edges.push(DistanceConstraint(ip1, ip2, p1.distance(*p2)));

                let ip1 = iz1 * nx * ny + iy * nx + ix2;
                let ip2 = iz2 * nx * ny + iy * nx + ix1;
                let p1 = &points[ip1];
                let p2 = &points[ip2];
                edges.push(DistanceConstraint(ip1, ip2, p1.distance(*p2)));
            }
        }
    }
    edges
}

fn create_shape_constraints(
    points: &[Vec3],
    nx: usize,
    ny: usize,
    nz: usize,
) -> Vec<ShapeConstraint> {
    let mut constraints = Vec::<ShapeConstraint>::with_capacity(
        nx * ny * (nz - 1) + nx * (ny - 1) * nz + (nx - 1) * ny * nz,
    );
    // Loop over blocks of vertices
    for iz2 in 1..nz {
        let iz1 = iz2 - 1;
        for iy2 in 1..ny {
            let iy1 = iy2 - 1;
            for ix2 in 1..nx {
                let ix1 = ix2 - 1;
                let ips = [
                    iz1 * nx * ny + iy1 * nx + ix1,
                    iz1 * nx * ny + iy1 * nx + ix2,
                    iz1 * nx * ny + iy2 * nx + ix1,
                    iz1 * nx * ny + iy2 * nx + ix2,
                    iz2 * nx * ny + iy1 * nx + ix1,
                    iz2 * nx * ny + iy1 * nx + ix2,
                    iz2 * nx * ny + iy2 * nx + ix1,
                    iz2 * nx * ny + iy2 * nx + ix2,
                ];
                let mean: Vector3<f32> = ips
                    .iter()
                    .map(|&ip| Vector3::from(points[ip]))
                    .fold(Vector3::zeros(), |acc, p| acc + p)
                    / ips.len() as f32;
                let shape: Vec<Vector3<f32>> = ips
                    .iter()
                    .map(|&ip| Vector3::from(points[ip]) - mean)
                    .collect();
                let a_qq_inv = shape
                    .iter()
                    .fold(Matrix3::zeros(), |acc, q| acc + q * q.transpose())
                    .try_inverse()
                    .unwrap();
                constraints.push(ShapeConstraint(ips.to_vec(), shape, a_qq_inv));
            }
        }
    }
    constraints
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut session = Session::init("Verlet", true);
    session.connect(rerun::default_server_addr());

    let stable_time = Timeline::new("stable_time", TimeType::Time);

    let nx = 10;
    let ny = 10;
    let nz = 10;
    let mut points_curr = grid(
        vec3(-1.0, -1.0, 2.0),
        vec3(1.0, 1.0, 2.0 + (nz as f32 - 1.0) * 0.2),
        nx,
        ny,
        nz,
    )
    .collect::<Vec<_>>();
    let edges = vec![]; //create_edges(&points_curr, nx, ny, nz);
    let mut shape_constraints = create_shape_constraints(&points_curr, nx, ny, nz);
    let mut rng = StdRng::seed_from_u64(1188553);
    shape_constraints.shuffle(&mut rng);

    let points = points_curr
        .iter()
        .map(|&p| Point3D::from(p))
        .collect::<Vec<_>>();

    let colors = points
        .iter()
        .map(|_| ColorRGBA::from_rgb(rng.gen(), rng.gen(), rng.gen()))
        .collect::<Vec<_>>();
    let radius = Radius(0.025);
    MsgSender::new("world")
        .with_timeless(true)
        .with_splat(ViewCoordinates::from_up_and_handedness(
            SignedAxis3::POSITIVE_Z,
            Handedness::Right,
        ))?
        .send(&mut session)?;
    MsgSender::new("world/points")
        .with_component(&points)?
        .with_component(&colors)?
        .with_splat(radius)?
        .send(&mut session)?;

    let inner_r = 1.0;
    let inner_r2 = inner_r * inner_r;
    let outer_r = 5.0;
    let outer_r2 = outer_r * outer_r;
    let stiction_d = 0.02;
    let stiction_d2 = stiction_d * stiction_d;
    let shape_stiffness = 0.25;
    MsgSender::new("world/collider")
        .with_timeless(true)
        .with_component(&[Point3D::ZERO])?
        .with_splat(ColorRGBA::from_rgb(100, 100, 100))?
        .with_splat(Radius(inner_r))?
        .send(&mut session)?;

    let dt = 0.005;
    let acc = vec3(0.0, 0.0, -9.82);
    let mut points_prev = points_curr.clone();
    let mut active_collisions = vec![None; points_curr.len()];
    for i in 0..3000 {
        let time = i as f32 * dt;

        // Predict new positions
        let mut points_next = points_curr
            .iter()
            .zip(points_prev)
            .map(|(&curr, prev)| curr + (curr - prev) + acc * dt * dt)
            .collect::<Vec<_>>();

        // Resolve collisions
        for (p, c) in points_next.iter_mut().zip(&mut active_collisions) {
            let d2 = p.length_squared();
            if d2 < inner_r2 {
                if let Some(prev_collision) = *c {
                    if p.distance_squared(prev_collision) > stiction_d2 {
                        let delta = *p - prev_collision;
                        *p -= delta * (stiction_d * delta.length_recip());
                        *p *= inner_r / p.length();
                        *c = Some(*p);
                    } else {
                        *p = prev_collision;
                    }
                } else {
                    *p *= inner_r / d2.sqrt();
                    *c = Some(*p);
                }
            } else if d2 > outer_r2 {
                if let Some(prev_collision) = *c {
                    if p.distance_squared(prev_collision) > stiction_d2 {
                        let delta = *p - prev_collision;
                        *p -= delta * (stiction_d * delta.length_recip());
                        *p *= outer_r / p.length();
                        *c = Some(*p);
                    } else {
                        *p = prev_collision;
                    }
                } else {
                    *p *= outer_r / d2.sqrt();
                    *c = Some(*p);
                }
            } else {
                *c = None;
            }
            // if p.z < -r {
            //     p.z = -r;
            // }
        }

        // Resolve distance constraints
        for &DistanceConstraint(ip1, ip2, relaxed_length) in &edges {
            let p1 = &points_next[ip1];
            let p2 = &points_next[ip2];
            let delta = *p2 - *p1;
            let actual_length = delta.length();
            let displacement = actual_length - relaxed_length;
            let a = 1.0;
            let adjustment = delta * (displacement / actual_length * 0.5 * a);
            points_next[ip1] += adjustment;
            points_next[ip2] -= adjustment;
        }

        // Resolve shape matching constraints
        for ShapeConstraint(ips, template_shape, a_qq_inv) in &shape_constraints {
            let mean: Vector3<f32> = ips
                .iter()
                .map(|&ip| Vector3::from(points_next[ip]))
                .fold(Vector3::zeros(), |acc, p| acc + p)
                / ips.len() as f32;
            let a_pq = ips
                .iter()
                .map(|&ip| Vector3::from(points_next[ip]) - mean)
                .zip(template_shape)
                .fold(Matrix3::zeros(), |acc, (p, q)| acc + p * q.transpose());
            let mut svd = (a_pq * a_qq_inv).svd(true, true);
            svd.singular_values[0] = 1.0;
            svd.singular_values[1] = 1.0;
            svd.singular_values[2] =
                (svd.u.unwrap().determinant() * svd.v_t.unwrap().determinant()).signum();
            let rot = svd.recompose().unwrap();
            for (i, ip) in ips.iter().enumerate() {
                points_next[*ip] = lerp(
                    points_next[*ip],
                    (mean + rot * template_shape[i]).into(),
                    shape_stiffness,
                );
            }
        }

        points_prev = points_curr;
        points_curr = points_next;

        let points = points_curr
            .iter()
            .map(|&p| p.into())
            .collect::<Vec<Point3D>>();
        let collisions = active_collisions
            .iter()
            .filter_map(|&op| op)
            .map(|p| p.into())
            .collect::<Vec<Point3D>>();
        MsgSender::new("world/points")
            .with_time(stable_time, Time::from_seconds_since_epoch(time as _))
            .with_component(&points)?
            .with_component(&colors)?
            .with_splat(radius)?
            .send(&mut session)?;
        MsgSender::new("world/collisions")
            .with_time(stable_time, Time::from_seconds_since_epoch(time as _))
            .with_component(&collisions)?
            .with_splat(ColorRGBA::from_rgb(255, 0, 0))?
            .with_splat(Radius(0.03))?
            .send(&mut session)?;
    }
    Ok(())
}
