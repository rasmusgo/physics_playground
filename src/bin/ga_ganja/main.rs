use itertools::Itertools;
mod pga3d;
use pga3d::PGA3D;
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
    fn into_rerun(self) -> rr::Point3D;
    fn into_rerun_vec(self) -> rr::Vec3D;
}

impl IntoRerun<PGA3D> for PGA3D {
    fn into_rerun(self) -> rr::Point3D {
        rr::Point3D::new(self[11] as _, self[12] as _, self[13] as _)
    }
    fn into_rerun_vec(self) -> rr::Vec3D {
        rr::Vec3D::new(self[11] as _, self[12] as _, self[13] as _)
    }
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

fn d_state(state: &State) -> DState {
    let world_from_local = &state.world_from_local;
    let velocity_in_local = &state.velocity_in_local;
    DState {
        d_world_from_local: -0.5 * world_from_local * velocity_in_local,
        d_velocity_in_local: (forques(state)
            - 0.5
                * (velocity_in_local.Dual() * velocity_in_local
                    - velocity_in_local * velocity_in_local.Dual()))
        .Dual(),
    }
}

fn forques(state: &State) -> PGA3D {
    let world_from_local = &state.world_from_local;
    let velocity_in_local = &state.velocity_in_local;
    let gravity_vector = PGA3D::e01() * -9.81;
    let local_from_world = &world_from_local.Reverse();
    let gravity = (local_from_world * gravity_vector * world_from_local).Dual();
    let k = 12.0;
    let hooke = k * ((local_from_world * ATTACH_ANCHOR * world_from_local) & ATTACH_IN_BODY);
    let damping = -0.25 * !velocity_in_local;
    gravity + hooke + damping
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let recording = rr::RecordingStreamBuilder::new("GA physics ganja")
        .connect(rerun::default_server_addr(), rerun::default_flush_timeout())?;
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
        let d_state = d_state(&state);
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
        let anchor = ATTACH_ANCHOR.into_rerun();
        let point = (&state.world_from_local * ATTACH_IN_BODY * state.world_from_local.Reverse())
            .into_rerun();
        let springs = vec![rr::Arrow3D {
            origin: [anchor.x, anchor.y, anchor.z].into(),
            vector: [point.x - anchor.x, point.y - anchor.y, point.z - anchor.z].into(),
        }];

        rr::MsgSender::new("world/points")
            .with_time(
                stable_time,
                rr::Time::from_seconds_since_epoch(i as f64 * dt),
            )
            .with_component(&points)?
            .with_splat(radius)?
            .send(&recording)?;
        rr::MsgSender::new("world/lines")
            .with_time(
                stable_time,
                rr::Time::from_seconds_since_epoch(i as f64 * dt),
            )
            .with_component(&lines)?
            .send(&recording)?;
        rr::MsgSender::new("world/springs")
            .with_time(
                stable_time,
                rr::Time::from_seconds_since_epoch(i as f64 * dt),
            )
            .with_component(&springs)?
            .send(&recording)?;
    }

    Ok(())
}
