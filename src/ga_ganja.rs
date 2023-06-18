use std::ops::{Add, Mul};

use itertools::Itertools;
use nalgebra::{Matrix3, Vector3};
use rand::{rngs::StdRng, seq::SliceRandom, Rng, SeedableRng};
mod pga3d;
use pga3d::{Basis, PGA3D};
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

impl IntoRerun<PGA3D> for PGA3D {
    type Output = rr::Point3D;
    fn into_rerun(self) -> Self::Output {
        rr::Point3D::new(self[11] as _, self[12] as _, self[13] as _)
    }
}

fn is_colliding(plane: &PGA3D, points: &[PGA3D]) -> bool {
    // TODO: Should this really test only the scalar part?
    points.iter().any(|p| (p & plane)[0] < 0.0)
}

fn is_edge(i: usize, j: usize) -> bool {
    let x = i ^ j; // xor both together, all differing bits will be '1'
    (x & (x - 1)) == 0 // only if x has one bit, this returns true.
}

const ATTACH_ANCHOR: PGA3D = PGA3D::point(0.0, 0.0, 1.5);
const ATTACH_IN_BODY: PGA3D = PGA3D::point(0.5, 0.5, 0.5);

struct State {
    pub world_from_local: PGA3D,
    pub velocity_in_local: PGA3D,
}

struct DState {
    pub d_world_from_local: PGA3D,
    pub d_velocity_in_local: PGA3D,
}

struct Edge {
    pub i: usize,
    pub j: usize,
}

fn dState(state: &State) -> DState {
    let world_from_local = &state.world_from_local;
    let velocity_in_local = &state.velocity_in_local;
    let d_velocity_in_local = PGA3D::from_elements(
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        velocity_in_local[Basis::e02 as _] * velocity_in_local[Basis::e12 as _]
            - velocity_in_local[Basis::e03 as _] * velocity_in_local[Basis::e31 as _],
        velocity_in_local[Basis::e03 as _] * velocity_in_local[Basis::e23 as _]
            - velocity_in_local[Basis::e01 as _] * velocity_in_local[Basis::e12 as _],
        velocity_in_local[Basis::e01 as _] * velocity_in_local[Basis::e31 as _]
            - velocity_in_local[Basis::e02 as _] * velocity_in_local[Basis::e23 as _],
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
    );
    DState {
        d_world_from_local: -0.5 * world_from_local * velocity_in_local,
        d_velocity_in_local: d_velocity_in_local,
    }
}

fn forques(state: &State) -> PGA3D {
    let world_from_local = &state.world_from_local;
    let velocity_in_local = &state.velocity_in_local;
    let gravity_vector = PGA3D::e02() * -9.81;
    let gravity = ((world_from_local.Reverse()) * gravity_vector.Dual() * world_from_local).Dual();
    let k = 12.0;
    let hooke =
        k * (((world_from_local.Reverse()) * ATTACH_ANCHOR * world_from_local) & ATTACH_IN_BODY);
    let damping = -0.25 * velocity_in_local;
    gravity // + damping
            // gravity + hooke + damping
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let recording =
        rr::RecordingStreamBuilder::new("GA physics ganja").connect(rerun::default_server_addr())?;
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
        PGA3D::point(-0.5, -0.5, -0.5),
        PGA3D::point(-0.5, -0.5, 0.5),
        PGA3D::point(-0.5, 0.5, -0.5),
        PGA3D::point(-0.5, 0.5, 0.5),
        PGA3D::point(0.5, -0.5, -0.5),
        PGA3D::point(0.5, -0.5, 0.5),
        PGA3D::point(0.5, 0.5, -0.5),
        PGA3D::point(0.5, 0.5, 0.5),
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
        world_from_local: PGA3D::new(1.0, 0),
        velocity_in_local: PGA3D::e12() + 1.3 * PGA3D::e31() + 0.5 * PGA3D::e23(),
    };

    let dt = 0.001;
    for i in 0..3000 {
        let d_state = dState(&state);
        state.world_from_local = &state.world_from_local + &d_state.d_world_from_local * dt;
        state.velocity_in_local = &state.velocity_in_local + &d_state.d_velocity_in_local * dt;
        // state.world_from_local.normalize_motor_in_place();

        let points = points_in_local
            .iter()
            .map(|p_in_local| {
                (state.world_from_local.clone() * p_in_local * state.world_from_local.Reverse())
                    .into_rerun()
            })
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
        let anchor = ATTACH_IN_BODY.into_rerun();
        let point = (&state.world_from_local * ATTACH_IN_BODY * state.world_from_local.Reverse())
            .into_rerun();
        let springs = vec![rr::Arrow3D {
            origin: [anchor.x, anchor.y, anchor.z].into(),
            vector: [point.x - anchor.x, point.y - anchor.y, point.z - anchor.z].into(),
        }];

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
        rr::MsgSender::new("world/springs")
            .with_time(
                stable_time,
                rr::Time::from_seconds_since_epoch(i as f64 * dt as f64),
            )
            .with_component(&springs)?
            .send(&recording)?;
    }

    Ok(())
}
