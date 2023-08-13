use geometric_algebra::*;
use rerun::external::glam::Vec3;

use crate::{conversions::ToPpgaPoint, inertia_map::InertiaMap};

pub struct CompliantSphericalConstraint {
    pub node_a: usize,
    pub node_b: usize,
    pub point_in_a: Vec3,
    pub point_in_b: Vec3,
    pub positional_compliance: f32,
}

#[inline(never)]
pub fn resolve_compliant_spherical_constraints(
    compliant_spherical_constraints: &[CompliantSphericalConstraint],
    motors_next: &mut [ppga3d::Motor],
    inertia_map: &InertiaMap,
    dt: f32,
) {
    puffin::profile_function!();
    for constraint in compliant_spherical_constraints {
        let node_a = constraint.node_a;
        let node_b = constraint.node_b;
        let world_from_a = motors_next[node_a];
        let world_from_b = motors_next[node_b];
        let a_from_b = world_from_a.reversal().geometric_product(world_from_b);
        let point_a_in_a = constraint.point_in_a.to_ppga_point();
        let point_b_in_b = constraint.point_in_b.to_ppga_point();
        let point_b_in_a = a_from_b.transformation(point_b_in_b);
        let join_line_in_a = point_b_in_a.regressive_product(point_a_in_a);
        let c: f32 = join_line_in_a.magnitude().into();
        if c == 0.0 {
            continue;
        }
        let n_in_a = join_line_in_a.scale(1.0 / c);
        let n_in_b = a_from_b.reversal().transformation(n_in_a);
        let s_in_a = inertia_map.momentum_to_rate(n_in_a);
        let s_in_b = inertia_map.momentum_to_rate(n_in_b);
        let w1: f32 = s_in_a.regressive_product(n_in_a).into();
        let w2: f32 = s_in_b.regressive_product(n_in_b).into();
        let correction = c / (w1 + w2 + constraint.positional_compliance / (dt * dt));
        let u_in_a = s_in_a.scale(0.5 * correction);
        let u_in_b = s_in_b.scale(-0.5 * correction);
        motors_next[node_a] = world_from_a
            .geometric_product(ppga3d::Translator::new(
                1.0, u_in_a[0], u_in_a[1], u_in_a[2],
            ))
            .geometric_product(ppga3d::Rotor::new(1.0, u_in_a[3], u_in_a[4], u_in_a[5]));
        motors_next[node_b] = world_from_b
            .geometric_product(ppga3d::Translator::new(
                1.0, u_in_b[0], u_in_b[1], u_in_b[2],
            ))
            .geometric_product(ppga3d::Rotor::new(1.0, u_in_b[3], u_in_b[4], u_in_b[5]));
        motors_next[node_a] =
            motors_next[node_a].geometric_quotient(motors_next[node_a].magnitude());
        motors_next[node_b] =
            motors_next[node_b].geometric_quotient(motors_next[node_b].magnitude());
    }
}

#[cfg(test)]
mod tests {
    use rand::{rngs::StdRng, Rng, SeedableRng};
    use rerun::{
        components::{Box3D, ColorRGBA},
        external::glam::{self, vec3},
        time::{TimeType, Timeline},
        RecordingStreamBuilder,
    };

    use crate::logging::log_with_rerun;

    use super::*;

    #[test]
    fn test_resolve_compliant_spherical_constraints_1() {
        let mut motors_next = [ppga3d::Motor::one(), ppga3d::Motor::one()];
        let inertia_map = InertiaMap::new(1.0, vec3(0.01, 0.01, 0.01));
        let dt = 0.001;
        let compliant_spherical_constraints = vec![CompliantSphericalConstraint {
            node_a: 0,
            node_b: 1,
            point_in_a: glam::Vec3::new(0.1, 0.0, 0.0),
            point_in_b: glam::Vec3::new(-0.1, 0.0, 0.0),
            positional_compliance: 0.0,
        }];

        test_resolve_compliant_spherical_constraints(
            &mut motors_next,
            dt,
            compliant_spherical_constraints,
            inertia_map,
        );
    }

    #[test]
    fn test_resolve_compliant_spherical_constraints_2() {
        let mut motors_next = [ppga3d::Motor::one(), ppga3d::Motor::one()];
        let inertia_map = InertiaMap::new(1.0, vec3(0.01, 0.01, 0.01));
        let dt = 0.001;
        let compliant_spherical_constraints = vec![CompliantSphericalConstraint {
            node_a: 0,
            node_b: 1,
            point_in_a: glam::Vec3::new(0.1, 0.0, 0.1),
            point_in_b: glam::Vec3::new(-0.1, 0.0, 0.1),
            positional_compliance: 0.0,
        }];

        test_resolve_compliant_spherical_constraints(
            &mut motors_next,
            dt,
            compliant_spherical_constraints,
            inertia_map,
        );
    }

    fn test_resolve_compliant_spherical_constraints(
        motors_next: &mut [ppga3d::Motor],
        dt: f32,
        compliant_spherical_constraints: Vec<CompliantSphericalConstraint>,
        inertia_map: InertiaMap,
    ) {
        let recording =
            RecordingStreamBuilder::new("XPBD test_resolve_compliant_spherical_constraints")
                .connect(rerun::default_server_addr(), rerun::default_flush_timeout())
                .unwrap();
        let stable_time = Timeline::new("stable_time", TimeType::Time);
        let mut rng = StdRng::seed_from_u64(5);

        let colors = motors_next
            .iter()
            .map(|_| {
                ColorRGBA::from_rgb(
                    rng.gen::<u8>() / 2 + 64,
                    rng.gen::<u8>() / 2 + 64,
                    rng.gen::<u8>() / 2 + 64,
                )
            })
            .collect::<Vec<_>>();

        let boxes = vec![
            Box3D {
                x: 0.1,
                y: 0.1,
                z: 0.1,
            };
            motors_next.len()
        ];

        log_with_rerun(
            &motors_next,
            &[],
            stable_time,
            0.0,
            &colors,
            &boxes,
            &recording,
        )
        .unwrap();

        for i in 1..5 {
            let time = i as f32 * dt;

            resolve_compliant_spherical_constraints(
                &compliant_spherical_constraints,
                motors_next,
                &inertia_map,
                dt,
            );

            log_with_rerun(
                &motors_next,
                &[],
                stable_time,
                time,
                &colors,
                &boxes,
                &recording,
            )
            .unwrap();
        }
    }
}
