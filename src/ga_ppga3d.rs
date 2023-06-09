use std::ops::{Add, Mul};

use geometric_algebra::*;
use itertools::Itertools;
use nalgebra::{Matrix3, Vector3};
use rand::{rngs::StdRng, seq::SliceRandom, Rng, SeedableRng};
mod rr {
    pub use rerun::{
        components::{Arrow3D, ColorRGBA, Point3D, Radius, Vec3D, ViewCoordinates},
        coordinates::{Handedness, SignedAxis3},
        demo_util::lerp,
        external::glam::{self, vec3, Vec3},
        time::{Time, TimeType, Timeline},
        MsgSender, RecordingStreamBuilder,
    };
}

trait IntoRerun<T> {
    type Output;
    fn into_rerun(self) -> Self::Output;
}

impl IntoRerun<ppga3d::Point> for ppga3d::Point {
    type Output = rr::Point3D;
    fn into_rerun(self) -> Self::Output {
        unsafe {
            rr::Point3D::new(
                self.group0().f32x4[1],
                self.group0().f32x4[2],
                self.group0().f32x4[3],
            )
        }
    }
}

fn is_colliding(plane: &ppga3d::Plane, points: &[ppga3d::Point]) -> bool {
    // TODO: Should this really test only the scalar part?
    points
        .iter()
        .any(|p| (p.regressive_product(*plane)).group0() < 0.0)
}

fn is_edge(i: usize, j: usize) -> bool {
    let x = i ^ j; // xor both together, all differing bits will be '1'
    (x & (x - 1)) == 0 // only if x has one bit, this returns true.
}

struct State {
    pub m: ppga3d::Motor,
    pub b: ppga3d::Line,
}

struct DState {
    pub d_motor: ppga3d::Motor,
    pub d_branch: ppga3d::Line,
}

struct Edge {
    pub i: usize,
    pub j: usize,
}

fn dState(state: &State) -> DState {
    DState {
        d_motor: ppga3d::Scalar::new(-0.5).geometric_product(state.m.geometric_product(state.b)),
        d_branch: ppga3d::Scalar::new(-0.5)
            .geometric_product((state.b.dual() * state.b - state.b * state.b.dual()).dual()),
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let recording =
        rr::RecordingStreamBuilder::new("GA physics").connect(rerun::default_server_addr())?;
    let stable_time = rr::Timeline::new("stable_time", rr::TimeType::Time);
    rr::MsgSender::new("world")
        .with_timeless(true)
        .with_splat(rr::ViewCoordinates::from_up_and_handedness(
            rr::SignedAxis3::POSITIVE_Z,
            rr::Handedness::Right,
        ))?
        .send(&recording)?;

    let radius = rr::Radius(0.025);

    let point_in_local = [
        ppga3d::Point::new(1.0, -0.5, -0.5, -0.5),
        ppga3d::Point::new(1.0, -0.5, -0.5, 0.5),
        ppga3d::Point::new(1.0, -0.5, 0.5, -0.5),
        ppga3d::Point::new(1.0, -0.5, 0.5, 0.5),
        ppga3d::Point::new(1.0, 0.5, -0.5, -0.5),
        ppga3d::Point::new(1.0, 0.5, -0.5, 0.5),
        ppga3d::Point::new(1.0, 0.5, 0.5, -0.5),
        ppga3d::Point::new(1.0, 0.5, 0.5, 0.5),
    ];

    let edges: Vec<Edge> = (0..point_in_local.len())
        .combinations(2)
        .filter_map(|v| {
            if is_edge(v[0], v[1]) {
                Some(Edge { i: v[0], j: v[1] })
            } else {
                None
            }
        })
        .collect();

    let mut state = State {
        m: ppga3d::Motor::one(),
        b: ppga3d::Line::new(
            0.0, // e01
            0.0, // e02
            0.0, // e03
            1.0, // e23
            1.3, // -e13
            0.5, // e12
        ),
    };
    let dt = 0.001;
    for i in 0..3000 {
        let d_state = dState(&state);
        state.m += d_state.d_motor.scale(dt);
        state.b += d_state.d_branch.scale(dt);

        let points = point_in_local
            .iter()
            .map(|p_in_local| state.m.transformation(*p_in_local).into_rerun())
            .collect::<Vec<rr::Point3D>>();
        let lines = edges
            .iter()
            .map(|e| rr::Arrow3D {
                origin: [points[e.i].x, points[e.i].y, points[e.i].z].into(),
                vector: [
                    points[e.j].x - points[e.i].x,
                    points[e.j].y - points[e.i].y,
                    points[e.j].z - points[e.i].z,
                ]
                .into(),
            })
            .collect::<Vec<rr::Arrow3D>>();

        rr::MsgSender::new("world/points")
            .with_time(
                stable_time,
                rr::Time::from_seconds_since_epoch(i as f64 * dt as f64),
            )
            .with_component(&points)?
            .with_splat(radius)?
            .send(&recording)?;
        rr::MsgSender::new("world/lines")
            .with_time(
                stable_time,
                rr::Time::from_seconds_since_epoch(i as f64 * dt as f64),
            )
            .with_component(&lines)?
            .send(&recording)?;
    }

    Ok(())
}
