use std::f32::consts::{PI, TAU};
use std::ops::{Add, Mul};

use geometric_algebra::simd::Simd32x3;
use geometric_algebra::*;
use itertools::Itertools;
use rand::{rngs::StdRng, seq::SliceRandom, Rng, SeedableRng};
use rerun::components::{Box3D, Quaternion, Transform3D};
use rerun::transform::{Rotation3D, Transform3DRepr, TranslationRotationScale3D};
use rerun::{
    components::{ColorRGBA, Point3D, Radius, Vec3D, ViewCoordinates},
    coordinates::{Handedness, SignedAxis3},
    demo_util::lerp,
    external::glam::{self, vec3, Quat, Vec3, Vec4},
    time::{Time, TimeType, Timeline},
    MsgSender, RecordingStreamBuilder,
};

pub struct CompliantFixedAngleConstraint {
    pub node_a: usize,
    pub node_b: usize,
    pub point_in_a: Vec3,
    pub point_in_b: Vec3,
    pub b_in_a: Quat,
    pub positional_compliance: f32,
    pub angular_compliance: f32,
}

enum ContactState {
    New,
    Sticking,
    Sliding,
}

struct Contact {
    point: Vec3,
    state: ContactState,
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

fn create_constraints(
    points: &[Vec3],
    nx: usize,
    ny: usize,
    nz: usize,
    positional_compliance: f32,
    angular_compliance: f32,
) -> Vec<CompliantFixedAngleConstraint> {
    let mut constraints = Vec::<CompliantFixedAngleConstraint>::with_capacity(
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
                let m = (*p1 + *p2) * 0.5;
                constraints.push(CompliantFixedAngleConstraint {
                    node_a: ip1,
                    node_b: ip2,
                    point_in_a: m - *p1,
                    point_in_b: m - *p2,
                    b_in_a: Quat::IDENTITY,
                    positional_compliance,
                    angular_compliance,
                });
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
                let m = (*p1 + *p2) * 0.5;
                constraints.push(CompliantFixedAngleConstraint {
                    node_a: ip1,
                    node_b: ip2,
                    point_in_a: m - *p1,
                    point_in_b: m - *p2,
                    b_in_a: Quat::IDENTITY,
                    positional_compliance,
                    angular_compliance,
                });
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
                let m = (*p1 + *p2) * 0.5;
                constraints.push(CompliantFixedAngleConstraint {
                    node_a: ip1,
                    node_b: ip2,
                    point_in_a: m - *p1,
                    point_in_b: m - *p2,
                    b_in_a: Quat::IDENTITY,
                    positional_compliance,
                    angular_compliance,
                });
            }
        }
    }
    // for c in &constraints {
    //     let p1 = points[c.node_a];
    //     let p2 = points[c.node_b];
    //     assert_ne!(p1, p2);
    //     let delta = p2 - p1;
    //     assert!(delta.length() > 0.2 - 0.0001);
    //     assert!(delta.length() < 0.2 + 0.0001);
    // }
    constraints
}

struct InertiaMap {
    pub mass: Simd32x3,
    pub inertia: Simd32x3,
}

impl InertiaMap {
    fn new(mass: f32, inertia: Vec3) -> Self {
        Self {
            mass: Simd32x3::from([mass, mass, mass]),
            inertia: Simd32x3::from(inertia.to_array()),
        }
    }

    fn rate_to_momentum(&self, line: ppga3d::Line) -> ppga3d::Line {
        ppga3d::Line::from_groups(line.group1() * self.inertia, line.group0() * self.mass)
    }

    fn momentum_to_rate(&self, line: ppga3d::Line) -> ppga3d::Line {
        ppga3d::Line::from_groups(line.group1() / self.mass, line.group0() / self.inertia)
    }
}

trait ToGlam {
    type Output;
    fn to_glam(&self) -> Self::Output;
}

impl ToGlam for ppga3d::Point {
    type Output = glam::Vec3;
    fn to_glam(&self) -> glam::Vec3 {
        let g = self.group0();
        glam::Vec3::new(g[1], g[2], g[3]) / g[0]
    }
}
const ORIGIN: ppga3d::Point = ppga3d::Point::new(1.0, 0.0, 0.0, 0.0);

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let dt = 0.001;
    let inertia_map = InertiaMap::new(0.1, vec3(0.1, 0.1, 0.1));
    let offset_x = 0.1;
    let positional_compliance = 0.0001001;
    let angular_compliance = 0.01;
    let stiction_factor = 0.25; // Maximum tangential correction per correction along normal.

    let recording =
        RecordingStreamBuilder::new("XPBD rigid bodies").connect(rerun::default_server_addr())?;

    let stable_time = Timeline::new("stable_time", TimeType::Time);

    let nx = 10;
    let ny = 10;
    let nz = 10;
    let mut points_curr = grid(
        vec3(-1.0 + offset_x, -1.0, 2.0),
        vec3(1.0 + offset_x, 1.0, 2.0 + (nz as f32 - 1.0) * 0.2),
        nx,
        ny,
        nz,
    )
    .collect::<Vec<_>>();
    let mut compliant_fixed_angle_constraints = create_constraints(
        &points_curr,
        nx,
        ny,
        nz,
        positional_compliance,
        angular_compliance,
    );
    let mut rng = StdRng::seed_from_u64(1188553);
    compliant_fixed_angle_constraints.shuffle(&mut rng);
    let compliant_fixed_angle_constraints = compliant_fixed_angle_constraints;

    let colors = points_curr
        .iter()
        .map(|_| {
            ColorRGBA::from_rgb(
                rng.gen::<u8>() / 2 + 64,
                rng.gen::<u8>() / 2 + 64,
                rng.gen::<u8>() / 2 + 64,
            )
        })
        .collect::<Vec<_>>();
    let radius = Radius(0.025);
    MsgSender::new("world")
        .with_timeless(true)
        .with_splat(ViewCoordinates::from_up_and_handedness(
            SignedAxis3::POSITIVE_Z,
            Handedness::Right,
        ))?
        .send(&recording)?;

    let boxes = vec![
        Box3D {
            x: 0.1,
            y: 0.1,
            z: 0.1,
        };
        points_curr.len()
    ];

    let inner_r = 1.0;
    let inner_r2 = inner_r * inner_r;
    let outer_r = 5.0;
    let outer_r2 = outer_r * outer_r;
    MsgSender::new("world/collider")
        .with_timeless(true)
        .with_component(&[Point3D::ZERO])?
        .with_splat(ColorRGBA::from_rgb(100, 100, 100))?
        .with_splat(Radius(inner_r))?
        .send(&recording)?;

    let gravity_vector = ppga3d::Line::new(0.0, 0.0, -9.82, 0.0, 0.0, 0.0);
    let mut motors_curr = points_curr
        .iter()
        .map(|&p| ppga3d::Motor::new(1., 0., 0., 0., 0., -0.5 * p.x, -0.5 * p.y, -0.5 * p.z))
        .collect::<Vec<_>>();

    let mut rate_in_body = vec![ppga3d::Line::zero(); points_curr.len()];
    let mut active_collisions = Vec::<Option<Contact>>::new();
    active_collisions.resize_with(points_curr.len(), Default::default);
    for i in 0..5000 {
        let time = i as f32 * dt;

        // Update velocities
        for (b, m) in rate_in_body.iter_mut().zip(motors_curr.iter()) {
            let gravity = inertia_map.rate_to_momentum(m.reversal().transformation(gravity_vector));
            let forques = gravity;
            let momentum = inertia_map.rate_to_momentum(*b);
            let d_b = inertia_map.momentum_to_rate(
                forques
                    - Into::<ppga3d::Line>::into(
                        momentum.geometric_product(*b) - b.geometric_product(momentum),
                    )
                    .scale(0.5),
            );
            *b += d_b.scale(dt);
        }

        // Predict new positions (including orientation)
        let mut motors_next = motors_curr
            .iter()
            .zip(rate_in_body.iter())
            .map(|(m, b)| *m + m.geometric_product(*b).scale(-0.5 * dt))
            .collect::<Vec<_>>();

        // Resolve collisions
        for (m, c) in motors_next.iter_mut().zip(&mut active_collisions) {
            let mut p = m.transformation(ORIGIN).to_glam();
            let d2 = p.length_squared();
            if d2 < inner_r2 {
                let length = p.length();
                p *= inner_r / length;
                let stiction_d = (inner_r - length) * stiction_factor;
                let stiction_d2 = stiction_d * stiction_d;
                if let Some(Contact {
                    point: contact_point,
                    state: contact_state,
                }) = c
                {
                    if p.distance_squared(*contact_point) > stiction_d2 {
                        let delta = p - *contact_point;
                        p -= delta * (stiction_d * delta.length_recip());
                        p *= inner_r / p.length();
                        *contact_point = p;
                        *contact_state = ContactState::Sliding;
                    } else {
                        p = *contact_point;
                        *contact_state = ContactState::Sticking;
                    }
                } else {
                    *c = Some(Contact {
                        point: p,
                        state: ContactState::New,
                    });
                }
            } else if d2 > outer_r2 {
                let length = p.length();
                p *= outer_r / length;
                let stiction_d = (length - outer_r) * stiction_factor;
                let stiction_d2 = stiction_d * stiction_d;
                if let Some(Contact {
                    point: contact_point,
                    state: contact_state,
                }) = c
                {
                    if p.distance_squared(*contact_point) > stiction_d2 {
                        let delta = p - *contact_point;
                        p -= delta * (stiction_d * delta.length_recip());
                        p *= outer_r / p.length();
                        *contact_point = p;
                        *contact_state = ContactState::Sliding;
                    } else {
                        p = *contact_point;
                        *contact_state = ContactState::Sticking;
                    }
                } else {
                    *c = Some(Contact {
                        point: p,
                        state: ContactState::New,
                    });
                }
            } else {
                *c = None;
            }
            // if p.z < -r {
            //     p.z = -r;
            // }
            let t = -0.5 * (p - m.transformation(ORIGIN).to_glam());
            *m = ppga3d::Translator::new(1.0, t.x, t.y, t.z).geometric_product(*m);
        }

        // // Resolve constraints
        // for constraint in &compliant_fixed_angle_constraints {
        //     let node_a = constraint.node_a;
        //     let node_b = constraint.node_b;
        //     let r1 = orientations_next[node_a] * constraint.point_in_a;
        //     let r2 = orientations_next[node_b] * constraint.point_in_b;
        //     assert!(r1.is_finite());
        //     assert!(r2.is_finite());
        //     // w = inv_mass + p.cross(n)ᵀ * inv_inertia * p.cross(n)
        //     let r1_squares = r1 * r1;
        //     let w1 = vec3(
        //         inv_mass + inv_inertia * (r1_squares.y + r1_squares.z),
        //         inv_mass + inv_inertia * (r1_squares.z + r1_squares.x),
        //         inv_mass + inv_inertia * (r1_squares.x + r1_squares.y),
        //     );
        //     let r2_squares = r2 * r2;
        //     let w2 = vec3(
        //         inv_mass + inv_inertia * (r2_squares.y + r2_squares.z),
        //         inv_mass + inv_inertia * (r2_squares.z + r2_squares.x),
        //         inv_mass + inv_inertia * (r2_squares.x + r2_squares.y),
        //     );
        //     let p1 = points_next[node_a] + r1;
        //     let p2 = points_next[node_b] + r2;
        //     let c = p1 - p2;
        //     let correction = -c / (w1 + w2 + constraint.positional_compliance / (dt * dt));
        //     points_next[node_a] += correction * inv_mass;
        //     points_next[node_b] -= correction * inv_mass;
        //     // q1 <- q1 + 0.5 * (p1.cross(correction) * q1)
        //     let q1 = &mut orientations_next[node_a];
        //     let omega = r1.cross(correction);
        //     *q1 = Quat::from_vec4(
        //         Vec4::from(*q1)
        //             + 0.5 * inv_inertia * Vec4::from(Quat::from_vec4(omega.extend(0.0)) * *q1),
        //     )
        //     .normalize();
        //     // q2 <- q2 - 0.5 * (p1.cross(correction) * q2)
        //     let q2 = &mut orientations_next[node_b];
        //     let omega = r2.cross(correction);
        //     *q2 = Quat::from_vec4(
        //         Vec4::from(*q2)
        //             - 0.5 * inv_inertia * Vec4::from(Quat::from_vec4(omega.extend(0.0)) * *q2),
        //     )
        //     .normalize();
        // }
        // // for constraint in &compliant_fixed_angle_constraints {
        // //     let node_a = constraint.node_a;
        // //     let node_b = constraint.node_b;
        // //     let stage_from_a = orientations_next[node_a];
        // //     let stage_from_b = orientations_next[node_b];
        // //     let stage_from_wanted_b = stage_from_a * constraint.b_in_a;
        // //     let delta = stage_from_b * stage_from_wanted_b.inverse();
        // //     let (axis, mut angle) = delta.to_axis_angle();
        // //     if angle > PI {
        // //         angle -= TAU;
        // //     }
        // //     let (s, c) =
        // //         (angle * 0.5 / (2.0 + constraint.angular_compliance / (dt * dt))).sin_cos();
        // //     let v = axis * s;
        // //     let delta1 = Quat::from_xyzw(v.x, v.y, v.z, c).normalize();
        // //     let delta2 = Quat::from_xyzw(-v.x, -v.y, -v.z, c).normalize();
        // //     orientations_next[node_a] = delta1 * orientations_next[node_a];
        // //     orientations_next[node_b] = delta2 * orientations_next[node_b];
        // // }
        // for constraint in &compliant_fixed_angle_constraints {
        //     let node_a = constraint.node_a;
        //     let node_b = constraint.node_b;
        //     let stage_from_a = orientations_next[node_a];
        //     let stage_from_b = orientations_next[node_b];
        //     let stage_from_wanted_b = stage_from_a * constraint.b_in_a;
        //     let delta = stage_from_b * stage_from_wanted_b.inverse();
        //     let (axis, mut angle) = delta.to_axis_angle();
        //     if angle > PI {
        //         angle -= TAU;
        //     }
        //     let w1 = inv_inertia;
        //     let w2 = inv_inertia;
        //     let lambda = -angle / (w1 + w2 + constraint.angular_compliance / (dt * dt));
        //     let delta1 = Quat::from_vec4((inv_inertia * axis * lambda).extend(0.0));
        //     let delta2 = Quat::from_vec4((inv_inertia * axis * lambda).extend(0.0));
        //     orientations_next[node_a] = Quat::from_vec4(
        //         Vec4::from(orientations_next[node_a])
        //             + 0.5 * Vec4::from(delta1 /* * orientations_next[node_a]*/),
        //     )
        //     .normalize();
        //     orientations_next[node_b] = Quat::from_vec4(
        //         Vec4::from(orientations_next[node_b])
        //             - 0.5 * Vec4::from(delta2 /* * orientations_next[node_b]*/),
        //     )
        //     .normalize();
        // }

        // Update velocities (including angular velocity)
        rate_in_body = motors_next
            .iter()
            .zip(&motors_curr)
            .map(|(next, curr)| {
                Into::<ppga3d::Line>::into(next.reversal().geometric_product(*next - *curr))
                    .scale(-2.0 / dt)
            })
            .collect::<Vec<_>>();

        motors_curr = motors_next;

        let positions = motors_curr
            .iter()
            .map(|m| m.transformation(ORIGIN).to_glam().into())
            .collect::<Vec<Vec3D>>();
        let rotations = motors_curr
            .iter()
            .map(|m| {
                let g = m.group0();
                Quaternion::new(g[1], g[2], g[3], g[0])
            })
            .collect::<Vec<Quaternion>>();
        let collisions = active_collisions
            .iter()
            .filter_map(|op| op.as_ref())
            .map(|p| p.point.into())
            .collect::<Vec<Point3D>>();
        let collision_colors = active_collisions
            .iter()
            .filter_map(|op| op.as_ref())
            .map(|p| match p.state {
                ContactState::New => ColorRGBA::from_rgb(255, 0, 255),
                ContactState::Sticking => ColorRGBA::from_rgb(255, 0, 0),
                ContactState::Sliding => ColorRGBA::from_rgb(255, 255, 0),
            })
            .collect::<Vec<ColorRGBA>>();
        MsgSender::new("world/boxes")
            .with_time(stable_time, Time::from_seconds_since_epoch(time as _))
            .with_component(&positions)?
            .with_component(&rotations)?
            .with_component(&colors)?
            // .with_splat(radius)?
            .with_component(&boxes)?
            .send(&recording)?;
        // for i in 0..points_curr.len() {
        //     MsgSender::new(format!("world/boxes/{}", i))
        //         .with_time(stable_time, Time::from_seconds_since_epoch(time as _))
        //         .with_splat(transformations[i])?
        //         .with_splat(colors[i])?
        //         // .with_splat(radius)?
        //         .with_splat(Box3D {
        //             x: 0.1,
        //             y: 0.1,
        //             z: 0.1,
        //         })?
        //         .send(&recording)?;
        // }
        MsgSender::new("world/collisions")
            .with_time(stable_time, Time::from_seconds_since_epoch(time as _))
            .with_component(&collisions)?
            .with_component(&collision_colors)?
            .with_splat(Radius(0.03))?
            .send(&recording)?;
    }
    Ok(())
}

#[test]
fn test_angular_constraint() {
    let quats = [
        [
            -0.00000024719682,
            0.00000024719682,
            -0.00000000000006110627,
            1.0,
        ],
        [0.0, 0.00000019102302, 0.0, 1.0],
        [0.0, -0.00000020222608, 0.0, 1.0],
        [0.0, -0.00000024719685, 0.0, 1.0],
        [0.0, 0.00000012359845, 0.0, 1.0],
        [
            0.00000010108715,
            -0.00000006742458,
            -0.00000000000011666013,
            1.0,
        ],
        [
            -0.00000024719682,
            -0.00000013485193,
            0.00000000000009444124,
            1.0,
        ],
        [
            0.00000012359841,
            0.00000012359841,
            -0.000000000000015276568,
            1.0,
        ],
    ]
    .iter()
    .map(|q| Quat::from_array(*q))
    .collect::<Vec<_>>();
    for (a, b) in quats.iter().cartesian_product(&quats) {
        let delta = *a * b.inverse();
        let (axis, mut angle) = delta.to_axis_angle();
        if angle > PI {
            angle -= TAU;
        }
        assert!(angle.abs() < 0.00001);
        assert!(axis.length() < 1.00001);
        assert!(axis.length() > 1.0 - 0.00001);
    }
}

// #[test]
// fn line_scalar_mul() {
//     let line = ppga3d::Line::new(1., 2., 3., 4., 5., 6.);
//     let scalar = ppga3d::Scalar::new(2.);
//     line * scalar;
// }
