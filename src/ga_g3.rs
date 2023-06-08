use std::ops::{Add, Mul};

use g3;
use nalgebra::{Matrix3, Vector3};
use rand::{rngs::StdRng, seq::SliceRandom, Rng, SeedableRng};
use rerun::{
    components::{ColorRGBA, Point3D, Radius, ViewCoordinates},
    coordinates::{Handedness, SignedAxis3},
    demo_util::lerp,
    external::glam::{self, vec3, Vec3},
    time::{Time, TimeType, Timeline},
    MsgSender, RecordingStreamBuilder,
};

fn is_colliding(plane: &g3::Plane, points: &[g3::Point]) -> bool {
    // TODO: Should this really test only the scalar part?
    points.iter().any(|p| (*p & *plane).scalar() < 0.0)
}

fn is_edge(i: i32, j: i32) -> bool {
    let x = i ^ j; // xor both together, all differing bits will be '1'
    (x & (x - 1)) == 0 // only if x has one bit, this returns true.
}

struct State {
    pub m: g3::Motor,
    pub b: g3::Line,
}

struct DState {
    pub d_motor: g3::Motor,
    pub d_branch: g3::Line,
}

fn dState(state: &State) -> DState {
    DState {
        d_motor: g3::Scalar::new(-0.5).geometric_product(state.m.geometric_product(state.b)),
        d_branch: g3::Scalar::new(-0.5)
            .geometric_product((state.b.dual() * state.b - state.b * state.b.dual()).dual()),
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let recording =
        RecordingStreamBuilder::new("GA physics").connect(rerun::default_server_addr())?;
    let stable_time = Timeline::new("stable_time", TimeType::Time);
    MsgSender::new("world")
        .with_timeless(true)
        .with_splat(ViewCoordinates::from_up_and_handedness(
            SignedAxis3::POSITIVE_Z,
            Handedness::Right,
        ))?
        .send(&recording)?;

    let p = [
        g3::point(-0.5, -0.5, -0.5),
        g3::point(-0.5, -0.5, 0.5),
        g3::point(-0.5, 0.5, -0.5),
        g3::point(-0.5, 0.5, 0.5),
        g3::point(0.5, -0.5, -0.5),
        g3::point(0.5, -0.5, 0.5),
        g3::point(0.5, 0.5, -0.5),
        g3::point(0.5, 0.5, 0.5),
    ];

    let mut state = State {
        motor: g3::Motor::default(),
        b: g3::Line::default(),
    };
    let dt = 0.001;
    for i in 0..3000 {
        let d_state = dState(&state);
        state.motor += dt * d_state.d_motor;
        state.b += dt * d_state.d_branch;
    }

    Ok(())
}
