use std::ops::{Add, Mul};

use g3;
use itertools::Itertools;
use nalgebra::{Matrix3, Vector3};
use rand::{rngs::StdRng, seq::SliceRandom, Rng, SeedableRng};
mod rr {
    pub use rerun::{
        components::{Arrow3D, ColorRGBA, Point3D, Radius, Vec3D, ViewCoordinates},
        coordinates::{Handedness, SignedAxis3},
        external::glam::{self, vec3, Vec3},
        time::{Time, TimeType, Timeline},
        MsgSender, RecordingStreamBuilder,
    };
}

trait IntoRerun<T> {
    type Output;
    fn into_rerun(self) -> Self::Output;
}

impl IntoRerun<g3::Point> for g3::Point {
    type Output = rr::Point3D;
    fn into_rerun(self) -> Self::Output {
        unsafe { rr::Point3D::new(self.x(), self.y(), self.z()) }
    }
}

fn is_colliding(plane: &g3::Plane, points: &[g3::Point]) -> bool {
    // TODO: Should this really test only the scalar part?
    points.iter().any(|p| (*p & *plane).scalar() < 0.0)
}

fn is_edge(i: usize, j: usize) -> bool {
    let x = i ^ j; // xor both together, all differing bits will be '1'
    (x & (x - 1)) == 0 // only if x has one bit, this returns true.
}

const ATTACH_ANCHOR: g3::Point = g3::point(0.0, 0.0, 1.5);
const ATTACH_IN_BODY: g3::Point = g3::point(0.5, 0.5, 0.5);

struct State {
    pub m: g3::Motor,
    pub b: g3::Line,
}

struct DState {
    pub d_m: g3::Motor,
    pub d_b: g3::Line,
}

struct Edge {
    pub i: usize,
    pub j: usize,
}

fn dState(state: &State) -> DState {
    DState {
        d_m: -0.5 * state.m * state.b,
        d_b: -0.5 * ((state.b.dual() * state.b - state.b * state.b.dual()).dual()),
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let recording =
        rr::RecordingStreamBuilder::new("GA physics g3").connect(rerun::default_server_addr())?;
    let stable_time = rr::Timeline::new("stable_time", rr::TimeType::Time);
    rr::MsgSender::new("world")
        .with_timeless(true)
        .with_splat(rr::ViewCoordinates::from_up_and_handedness(
            rr::SignedAxis3::POSITIVE_Z,
            rr::Handedness::Right,
        ))?
        .send(&recording)?;

    let radius = rr::Radius(0.025);

    let points_in_local = [
        g3::point(-0.5, -0.5, -0.5),
        g3::point(-0.5, -0.5, 0.5),
        g3::point(-0.5, 0.5, -0.5),
        g3::point(-0.5, 0.5, 0.5),
        g3::point(0.5, -0.5, -0.5),
        g3::point(0.5, -0.5, 0.5),
        g3::point(0.5, 0.5, -0.5),
        g3::point(0.5, 0.5, 0.5),
    ];

    let edges: Vec<Edge> = (0..points_in_local.len())
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
        m: g3::Motor::one(),
        b: g3::Line::new(0.0, 0.0, 0.0, 1.0, 1.3, 0.5),
    };
    let dt = 0.001;
    for i in 0..3000 {
        let d_state = dState(&state);
        state.m += dt * d_state.d_m;
        state.b += dt * d_state.d_b;

        let points = points_in_local
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
