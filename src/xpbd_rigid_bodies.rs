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

trait ToPpgaPoint {
    type Output;
    fn to_ppga_point(&self) -> Self::Output;
}

impl ToPpgaPoint for glam::Vec3 {
    type Output = ppga3d::Point;
    fn to_ppga_point(&self) -> ppga3d::Point {
        ppga3d::Point::new(1.0, self.x, self.y, self.z)
    }
}

trait ToPpgaTranslator {
    type Output;
    fn to_ppga_translator(&self) -> Self::Output;
}

impl ToPpgaTranslator for glam::Vec3 {
    type Output = ppga3d::Translator;
    fn to_ppga_translator(&self) -> ppga3d::Translator {
        ppga3d::Translator::new(1.0, self.x * -0.5, self.y * -0.5, self.z * -0.5)
    }
}

const ORIGIN: ppga3d::Point = ppga3d::Point::new(1.0, 0.0, 0.0, 0.0);

#[test]
fn glam_vec_as_translator() {
    let v = vec3(1.0, 2.0, 3.0);
    let t = v.to_ppga_translator();
    let p = t.transformation(ORIGIN);
    let p = p.to_glam();
    assert_eq!(p, v);
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let dt = 0.001;
    let inertia_map = InertiaMap::new(0.1, vec3(0.1, 0.1, 0.1));
    let offset_x = 0.1;
    let positional_compliance = 0.0001001;
    let angular_compliance = 0.001;
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
    // let mut rate_in_body = vec![ppga3d::Line::new(0.0, 0.0, 0.0, 1.0, 0.0, 0.0); points_curr.len()];
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
            .map(|(m, b)| {
                let m_next = *m + m.geometric_product(*b).scale(-0.5 * dt);
                m_next.geometric_quotient(m_next.magnitude())
            })
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

        // Resolve constraints
        // for constraint in &compliant_fixed_angle_constraints {
        //     let node_a = constraint.node_a;
        //     let node_b = constraint.node_b;
        //     let motor1 = motors_next[node_a];
        //     let motor2 = motors_next[node_b];
        //     let p1_in_world = motor1.transformation(constraint.point_in_a.to_ppga_point());
        //     let p2_in_world = motor2.transformation(constraint.point_in_b.to_ppga_point());
        //     let join_line = p1_in_world.regressive_product(p2_in_world);
        //     let c: f32 = join_line.magnitude().into();
        //     if c <= 0.0 {
        //         continue;
        //     }
        //     let n_in_world = join_line.scale(1.0 / c);
        //     let n_in_a = motor1.reversal().transformation(n_in_world);
        //     let n_in_b = motor2.reversal().transformation(n_in_world);
        //     let s_in_a = inertia_map.momentum_to_rate(n_in_a);
        //     let s_in_b = inertia_map.momentum_to_rate(n_in_b);
        //     let w1: f32 = s_in_a.regressive_product(n_in_a).into();
        //     let w2: f32 = s_in_b.regressive_product(n_in_b).into();
        //     let correction = -c / (w1 + w2 + constraint.positional_compliance / (dt * dt));
        //     let u_in_a = s_in_a.scale(-correction);
        //     let u_in_b = s_in_b.scale(correction);
        //     motors_next[node_a] -= motor1.geometric_product(u_in_a).scale(0.5);
        //     motors_next[node_b] -= motor2.geometric_product(u_in_b).scale(0.5);
        //     motors_next[node_a] =
        //         motors_next[node_a].geometric_product(motors_next[node_a].magnitude().inverse());
        //     motors_next[node_b] =
        //         motors_next[node_b].geometric_product(motors_next[node_b].magnitude().inverse());
        // }
        for constraint in &compliant_fixed_angle_constraints {
            let node_a = constraint.node_a;
            let node_b = constraint.node_b;
            let motor1 = motors_next[node_a];
            let motor2 = motors_next[node_b];
            let a_from_anchor_a = constraint.point_in_a.to_ppga_translator();
            let b_from_anchor_b = constraint.point_in_b.to_ppga_translator();
            let world_from_anchor_a = motor1.geometric_product(a_from_anchor_a);
            let world_from_anchor_b = motor2.geometric_product(b_from_anchor_b);
            let anchor_a_from_anchor_b = world_from_anchor_a
                .reversal()
                .geometric_product(world_from_anchor_b);
            let meet_line_in_anchor_a = anchor_a_from_anchor_b.ln();
            let join_line_in_anchor_a = meet_line_in_anchor_a.dual();
            let c: f32 = join_line_in_anchor_a.magnitude().into();
            if c == 0.0 {
                continue;
            }
            let n_in_anchor_a = join_line_in_anchor_a.scale(1.0 / c);
            let b_from_anchor_a =
                b_from_anchor_b.geometric_product(anchor_a_from_anchor_b.reversal());
            let a_from_anchor_a = (ppga3d::Motor::one().geometric_product(a_from_anchor_a));
            let n_in_a = a_from_anchor_a.transformation(n_in_anchor_a);
            let n_in_b = b_from_anchor_a.transformation(n_in_anchor_a);
            let s_in_a = inertia_map.momentum_to_rate(n_in_a);
            let s_in_b = inertia_map.momentum_to_rate(n_in_b);
            let w1: f32 = s_in_a.regressive_product(n_in_a).into();
            let w2: f32 = s_in_b.regressive_product(n_in_b).into();
            let correction = c / (w1 + w2 + constraint.angular_compliance / (dt * dt));
            let u_in_a = s_in_a.scale(-correction);
            let u_in_b = s_in_b.scale(correction);
            motors_next[node_a] -= motor1.geometric_product(u_in_a).scale(0.5);
            motors_next[node_b] -= motor2.geometric_product(u_in_b).scale(0.5);
            motors_next[node_a] =
                motors_next[node_a].geometric_quotient(motors_next[node_a].magnitude());
            motors_next[node_b] =
                motors_next[node_b].geometric_quotient(motors_next[node_b].magnitude());
        }
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
                Quaternion::new(g[1], g[2], g[3], -g[0])
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
