use geometric_algebra::*;
use rerun::external::glam::{Quat, Vec3};

use crate::{conversions::ToPpgaTranslator, inertia_map::InertiaMap};
pub struct CompliantFixedAngleConstraint {
    pub node_a: usize,
    pub node_b: usize,
    pub point_in_a: Vec3,
    pub point_in_b: Vec3,
    pub b_in_a: Quat,
    pub positional_compliance: f32,
    pub angular_compliance: f32,
}

#[inline(never)]
pub fn resolve_compliant_fixed_angle_constraints(
    compliant_fixed_angle_constraints: &[CompliantFixedAngleConstraint],
    motors_next: &mut [ppga3d::Motor],
    inertia_map: &InertiaMap,
    dt: f32,
) {
    puffin::profile_function!();
    for constraint in compliant_fixed_angle_constraints {
        let node_a = constraint.node_a;
        let node_b = constraint.node_b;
        let world_from_a = motors_next[node_a];
        let world_from_b = motors_next[node_b];
        let a_from_anchor_a = constraint.point_in_a.to_ppga_translator();
        let b_from_anchor_b = constraint.point_in_b.to_ppga_translator();
        let world_from_anchor_a = world_from_a.geometric_product(a_from_anchor_a);
        let world_from_anchor_b = world_from_b.geometric_product(b_from_anchor_b);
        let anchor_a_from_anchor_b = world_from_anchor_a
            .reversal()
            .geometric_product(world_from_anchor_b);
        let meet_line_in_anchor_a = anchor_a_from_anchor_b.ln();
        let join_line_in_anchor_a = meet_line_in_anchor_a.dual();
        let c: f32 = (
            join_line_in_anchor_a[0] * join_line_in_anchor_a[0]
                + join_line_in_anchor_a[1] * join_line_in_anchor_a[1]
                + join_line_in_anchor_a[2] * join_line_in_anchor_a[2]
                + join_line_in_anchor_a[3] * join_line_in_anchor_a[3]
                + join_line_in_anchor_a[4] * join_line_in_anchor_a[4]
                + join_line_in_anchor_a[5] * join_line_in_anchor_a[5]
            //
        )
        .sqrt();
        if c == 0.0 {
            continue;
        }
        let n_in_anchor_a = join_line_in_anchor_a.scale(1.0 / c);
        let b_from_anchor_a = b_from_anchor_b.geometric_product(anchor_a_from_anchor_b.reversal());
        let a_from_anchor_a = ppga3d::Motor::one().geometric_product(a_from_anchor_a);
        let n_in_a = a_from_anchor_a.transformation(n_in_anchor_a);
        let n_in_b = b_from_anchor_a.transformation(n_in_anchor_a);
        let s_in_a = inertia_map.momentum_to_rate(n_in_a);
        let s_in_b = inertia_map.momentum_to_rate(n_in_b);
        let w1: f32 = s_in_a.regressive_product(n_in_a).into();
        let w2: f32 = s_in_b.regressive_product(n_in_b).into();
        let correction = c / (w1 + w2 + constraint.angular_compliance / (dt * dt));
        let u_in_a = s_in_a.scale(correction);
        let u_in_b = s_in_b.scale(-correction);
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
    use rerun::external::glam::{self, vec3};

    use crate::logging::log_with_rerun;

    use super::*;

    #[test]
    fn test_resolve_compliant_fixed_angle_constraints_1() {
        let mut motors_next = [ppga3d::Motor::one(), ppga3d::Motor::one()];
        let inertia_map = InertiaMap::new(1.0, vec3(0.1, 0.1, 0.1));
        let dt = 0.001;
        let compliant_fixed_angle_constraints = [CompliantFixedAngleConstraint {
            node_a: 0,
            node_b: 1,
            point_in_a: glam::Vec3::new(0.1, 0.0, 0.0),
            point_in_b: glam::Vec3::new(-0.1, 0.0, 0.0),
            b_in_a: glam::Quat::IDENTITY,
            positional_compliance: 0.0,
            angular_compliance: 0.0,
        }];

        test_resolve_compliant_fixed_angle_constraints(
            &mut motors_next,
            dt,
            &compliant_fixed_angle_constraints,
            inertia_map,
        );
    }

    #[test]
    fn test_resolve_compliant_fixed_angle_constraints_2() {
        let mut motors_next = [ppga3d::Motor::one(), ppga3d::Motor::one()];
        let inertia_map = InertiaMap::new(1.0, vec3(0.1, 0.1, 0.1));
        let dt = 0.001;
        let compliant_fixed_angle_constraints = [CompliantFixedAngleConstraint {
            node_a: 0,
            node_b: 1,
            point_in_a: glam::Vec3::new(0.1, 0.0, 0.05),
            point_in_b: glam::Vec3::new(-0.1, 0.0, 0.05),
            b_in_a: glam::Quat::IDENTITY,
            positional_compliance: 0.0,
            angular_compliance: 0.0,
        }];

        test_resolve_compliant_fixed_angle_constraints(
            &mut motors_next,
            dt,
            &compliant_fixed_angle_constraints,
            inertia_map,
        );
    }

    fn test_resolve_compliant_fixed_angle_constraints(
        motors_next: &mut [ppga3d::Motor],
        dt: f32,
        compliant_fixed_angle_constraints: &[CompliantFixedAngleConstraint],
        inertia_map: InertiaMap,
    ) {
        let recording = rerun::RecordingStreamBuilder::new(
            "XPBD test_resolve_compliant_fixed_angle_constraints",
        )
        .connect()
        .unwrap();
        let mut rng = StdRng::seed_from_u64(5);

        let colors = motors_next
            .iter()
            .map(|_| {
                rerun::Color::from_rgb(
                    rng.gen::<u8>() / 2 + 64,
                    rng.gen::<u8>() / 2 + 64,
                    rng.gen::<u8>() / 2 + 64,
                )
            })
            .collect::<Vec<_>>();

        let boxes = vec![[0.1, 0.1, 0.1,]; motors_next.len()];

        log_with_rerun(&motors_next, &[], 0.0, &colors, &boxes, &recording).unwrap();

        for i in 1..=10 {
            let time = i as f32 * dt;

            resolve_compliant_fixed_angle_constraints(
                &compliant_fixed_angle_constraints,
                motors_next,
                &inertia_map,
                dt,
            );

            log_with_rerun(&motors_next, &[], time, &colors, &boxes, &recording).unwrap();
        }
    }
}
