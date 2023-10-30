use std::ops::{Add, Mul};

use nalgebra::{Matrix3, Vector3};
use rand::{rngs::StdRng, seq::SliceRandom, Rng, SeedableRng};
use rerun::external::glam::{self, vec3, Vec3};

struct ShapeConstraint(Vec<usize>, Vec<Vector3<f32>>, Matrix3<f32>);

enum ContactState {
    New,
    Sticking,
    Sliding,
}

struct Contact {
    point: Vec3,
    state: ContactState,
}

/// Linear interpolator.
#[inline]
pub fn lerp<T>(a: T, b: T, t: f32) -> <<f32 as Mul<T>>::Output as std::ops::Add>::Output
where
    T: Mul<f32>,
    f32: Mul<T>,
    <T as Mul<f32>>::Output: Add<<f32 as Mul<T>>::Output>,
    <f32 as Mul<T>>::Output: Add,
{
    (1.0 - t) * a + t * b
}

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
    let recording = rerun::RecordingStreamBuilder::new("XPBD particles").connect()?;

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
    let mut shape_constraints = create_shape_constraints(&points_curr, nx, ny, nz);
    let mut rng = StdRng::seed_from_u64(1188553);
    shape_constraints.shuffle(&mut rng);

    let colors = points_curr
        .iter()
        .map(|_| {
            rerun::Color::from_rgb(
                rng.gen::<u8>() / 2 + 64,
                rng.gen::<u8>() / 2 + 64,
                rng.gen::<u8>() / 2 + 64,
            )
        })
        .collect::<Vec<_>>();
    let radius = 0.025;
    recording.log_timeless("world", &rerun::ViewCoordinates::RIGHT_HAND_Z_UP)?;

    recording.log(
        "world/points",
        &rerun::Points3D::new(points_curr.clone())
            .with_colors(colors.clone())
            .with_radii([radius]),
    )?;

    let inner_r = 1.0;
    let inner_r2 = inner_r * inner_r;
    let outer_r = 5.0;
    let outer_r2 = outer_r * outer_r;
    let stiction_factor = 0.25; // Maximum tangential correction per correction along normal.
    let shape_compliance = 0.0001; // Inverse physical stiffness
    recording.log_timeless(
        "world/collider",
        &rerun::Points3D::new([[0.0, 0.0, 0.0]])
            .with_colors([rerun::Color::from_rgb(100, 100, 100)])
            .with_radii([inner_r]),
    )?;

    let dt = 0.005;
    let acc = vec3(0.0, 0.0, -9.82);
    let mut velocities = vec![vec3(0.0, 0.0, 0.0); points_curr.len()];
    let mut active_collisions = Vec::<Option<Contact>>::new();
    active_collisions.resize_with(points_curr.len(), Default::default);
    for i in 0..3000 {
        let time = i as f32 * dt;

        // Update velocities
        for vel in &mut velocities {
            *vel += acc * dt;
        }

        // Predict new positions
        let mut points_next = points_curr
            .iter()
            .zip(velocities)
            .map(|(&curr, vel)| curr + vel * dt)
            .collect::<Vec<_>>();

        // Resolve collisions
        for (p, c) in points_next.iter_mut().zip(&mut active_collisions) {
            let d2 = p.length_squared();
            if d2 < inner_r2 {
                let length = p.length();
                *p *= inner_r / length;
                let stiction_d = (inner_r - length) * stiction_factor;
                let stiction_d2 = stiction_d * stiction_d;
                if let Some(Contact {
                    point: contact_point,
                    state: contact_state,
                }) = c
                {
                    if p.distance_squared(*contact_point) > stiction_d2 {
                        let delta = *p - *contact_point;
                        *p -= delta * (stiction_d * delta.length_recip());
                        *p *= inner_r / p.length();
                        *contact_point = *p;
                        *contact_state = ContactState::Sliding;
                    } else {
                        *p = *contact_point;
                        *contact_state = ContactState::Sticking;
                    }
                } else {
                    *c = Some(Contact {
                        point: *p,
                        state: ContactState::New,
                    });
                }
            } else if d2 > outer_r2 {
                let length = p.length();
                *p *= outer_r / length;
                let stiction_d = (length - outer_r) * stiction_factor;
                let stiction_d2 = stiction_d * stiction_d;
                if let Some(Contact {
                    point: contact_point,
                    state: contact_state,
                }) = c
                {
                    if p.distance_squared(*contact_point) > stiction_d2 {
                        let delta = *p - *contact_point;
                        *p -= delta * (stiction_d * delta.length_recip());
                        *p *= outer_r / p.length();
                        *contact_point = *p;
                        *contact_state = ContactState::Sliding;
                    } else {
                        *p = *contact_point;
                        *contact_state = ContactState::Sticking;
                    }
                } else {
                    *c = Some(Contact {
                        point: *p,
                        state: ContactState::New,
                    });
                }
            } else {
                *c = None;
            }
            // if p.z < -r {
            //     p.z = -r;
            // }
        }

        // Resolve shape matching constraints
        let shape_compliance_per_dt2 = shape_compliance / (dt * dt);
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
                let goal = Vec3::from(mean + rot * template_shape[i]);
                let delta = points_next[*ip] - goal;
                let correction = delta * (-1.0 / (1.0 + shape_compliance_per_dt2));
                points_next[*ip] += correction;
            }
        }

        // Update velocities
        velocities = points_next
            .iter()
            .zip(points_curr)
            .map(|(&next, curr)| (next - curr) / dt)
            .collect::<Vec<_>>();

        points_curr = points_next;

        let collisions = active_collisions
            .iter()
            .filter_map(|op| op.as_ref())
            .map(|p| p.point)
            .collect::<Vec<_>>();
        let collision_colors = active_collisions
            .iter()
            .filter_map(|op| op.as_ref())
            .map(|p| match p.state {
                ContactState::New => rerun::Color::from_rgb(255, 0, 255),
                ContactState::Sticking => rerun::Color::from_rgb(255, 0, 0),
                ContactState::Sliding => rerun::Color::from_rgb(255, 255, 0),
            })
            .collect::<Vec<rerun::Color>>();
        recording.set_time_seconds("stable_time", time as f64);
        recording.log(
            "world/points",
            &rerun::Points3D::new(points_curr.clone())
                .with_colors(colors.clone())
                .with_radii([radius]),
        )?;
        recording.log(
            "world/collisions",
            &rerun::Points3D::new(collisions)
                .with_colors(collision_colors)
                .with_radii([0.03]),
        )?;
    }
    Ok(())
}
