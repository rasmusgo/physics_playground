mod conversions;
mod fixed_angle_constraint;
mod inertia_map;
mod logging;
mod spherical_constraint;

use std::ops::{Add, Mul};

use fixed_angle_constraint::CompliantFixedAngleConstraint;
use geometric_algebra::*;
use inertia_map::InertiaMap;
use physics_playground::start_puffin_server;
use rand::{rngs::StdRng, seq::SliceRandom, Rng, SeedableRng};
use rerun::external::glam::{self, vec3, Quat, Vec3};

use crate::{
    conversions::ToGlam, fixed_angle_constraint::resolve_compliant_fixed_angle_constraints,
    logging::log_with_rerun,
};
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

const ORIGIN: ppga3d::Point = ppga3d::Point::new(1.0, 0.0, 0.0, 0.0);

fn main() -> Result<(), Box<dyn std::error::Error>> {
    start_puffin_server();

    let dt = 0.001;
    let inertia_map = InertiaMap::new(0.1, vec3(0.1, 0.1, 0.1));
    let offset_x = 0.1;
    let positional_compliance = 0.0001001;
    let angular_compliance = 0.001;
    let stiction_factor = 0.25; // Maximum tangential correction per correction along normal.

    let recording = rerun::RecordingStreamBuilder::new("XPBD rigid bodies").connect()?;

    let nx = 10;
    let ny = 10;
    let nz = 10;
    let points = grid(
        vec3(-1.0 + offset_x, -1.0, 2.0),
        vec3(1.0 + offset_x, 1.0, 2.0 + (nz as f32 - 1.0) * 0.2),
        nx,
        ny,
        nz,
    )
    .collect::<Vec<_>>();
    let mut compliant_fixed_angle_constraints = create_constraints(
        &points,
        nx,
        ny,
        nz,
        positional_compliance,
        angular_compliance,
    );
    let mut rng = StdRng::seed_from_u64(1188553);
    compliant_fixed_angle_constraints.shuffle(&mut rng);
    let compliant_fixed_angle_constraints = compliant_fixed_angle_constraints;

    let colors = points
        .iter()
        .map(|_| {
            rerun::Color::from_rgb(
                rng.gen::<u8>() / 2 + 64,
                rng.gen::<u8>() / 2 + 64,
                rng.gen::<u8>() / 2 + 64,
            )
        })
        .collect::<Vec<_>>();
    recording.log_timeless("world", &rerun::ViewCoordinates::RIGHT_HAND_Z_UP)?;

    let boxes = vec![[0.1, 0.1, 0.1,]; points.len()];

    let inner_r = 1.0;
    let inner_r2 = inner_r * inner_r;
    let outer_r = 5.0;
    let outer_r2 = outer_r * outer_r;
    recording.log_timeless(
        "world/collider",
        &rerun::Points3D::new([[0.0, 0.0, 0.0]])
            .with_colors([rerun::Color::from_rgb(100, 100, 100)])
            .with_radii([inner_r]),
    )?;

    let gravity_vector = ppga3d::Line::new(0.0, 0.0, -9.82, 0.0, 0.0, 0.0);
    let mut motors_curr = points
        .iter()
        .map(|&p| ppga3d::Motor::new(1., 0., 0., 0., 0., -0.5 * p.x, -0.5 * p.y, -0.5 * p.z))
        .collect::<Vec<_>>();

    let mut rate_in_body = vec![ppga3d::Line::zero(); points.len()];
    let mut active_collisions = Vec::<Option<Contact>>::new();
    active_collisions.resize_with(points.len(), Default::default);
    for i in 0..5000 {
        puffin::GlobalProfiler::lock().new_frame();
        puffin::profile_scope!("physics step");
        let time = i as f32 * dt;

        // Apply external forces
        {
            puffin::profile_scope!("Apply external forces");
            for (b, m) in rate_in_body.iter_mut().zip(motors_curr.iter()) {
                let gravity =
                    inertia_map.rate_to_momentum(m.reversal().transformation(gravity_vector));
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
        resolve_collisions(
            &mut motors_next,
            &mut active_collisions,
            inner_r2,
            inner_r,
            stiction_factor,
            outer_r2,
            outer_r,
        );

        // Resolve constraints
        resolve_compliant_fixed_angle_constraints(
            &compliant_fixed_angle_constraints,
            &mut motors_next,
            &inertia_map,
            dt,
        );

        // Update velocities (including angular velocity)
        {
            puffin::profile_scope!("Update velocities");
            rate_in_body = motors_next
                .iter()
                .zip(&motors_curr)
                .map(|(next, curr)| {
                    Into::<ppga3d::Line>::into(next.reversal().geometric_product(*next - *curr))
                        .scale(-2.0 / dt)
                })
                .collect::<Vec<_>>();
        }

        motors_curr = motors_next;

        log_with_rerun(
            &motors_curr,
            &active_collisions,
            time,
            &colors,
            &boxes,
            &recording,
        )?;
    }
    Ok(())
}

fn resolve_collisions(
    motors_next: &mut [ppga3d::Motor],
    active_collisions: &mut [Option<Contact>],
    inner_r2: f32,
    inner_r: f32,
    stiction_factor: f32,
    outer_r2: f32,
    outer_r: f32,
) {
    puffin::profile_function!();
    for (m, c) in motors_next.iter_mut().zip(active_collisions) {
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
}
