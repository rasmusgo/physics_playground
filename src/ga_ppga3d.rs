use geometric_algebra::*;
use itertools::Itertools;
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

impl IntoRerun<ppga3d::Point> for ppga3d::Point {
    fn into_rerun(self) -> rr::Point3D {
        unsafe {
            rr::Point3D::new(
                self.group0().f32x4[1],
                self.group0().f32x4[2],
                self.group0().f32x4[3],
            )
        }
    }

    fn into_rerun_vec(self) -> rr::Vec3D {
        unsafe {
            rr::Vec3D::new(
                self.group0().f32x4[1],
                self.group0().f32x4[2],
                self.group0().f32x4[3],
            )
        }
    }
}

fn is_edge(i: usize, j: usize) -> bool {
    let x = i ^ j; // xor both together, all differing bits will be '1'
    (x & (x - 1)) == 0 // only if x has one bit, this returns true.
}

const ATTACH_ANCHOR: ppga3d::Point = ppga3d::Point::new(1.0, 0.0, 0.0, 1.5);
const ATTACH_IN_BODY: ppga3d::Point = ppga3d::Point::new(1.0, 0.5, 0.5, 0.5);

struct State {
    pub world_from_local: ppga3d::Motor,
    pub velocity_in_local: ppga3d::Line,
}

struct DState {
    pub d_world_from_local: ppga3d::Motor,
    pub d_velocity_in_local: ppga3d::Line,
}

struct Edge {
    pub i: usize,
    pub j: usize,
}

fn d_state(state: &State) -> DState {
    DState {
        d_world_from_local: state
            .world_from_local
            .geometric_product(state.velocity_in_local)
            .scale(-0.5),
        d_velocity_in_local: (forques(state)
            - Into::<ppga3d::Line>::into(
                state
                    .velocity_in_local
                    .dual()
                    .geometric_product(state.velocity_in_local)
                    - state
                        .velocity_in_local
                        .geometric_product(state.velocity_in_local.dual()),
            )
            .scale(0.5))
        .dual(),
    }
}

fn forques(state: &State) -> ppga3d::Line {
    let gravity_vector = ppga3d::Line::new(0.0, 0.0, -9.81, 0.0, 0.0, 0.0);
    let gravity = state
        .world_from_local
        .reversal()
        .transformation(gravity_vector)
        .dual();
    let k = -10.0;
    let hooke = state
        .world_from_local
        .reversal()
        .transformation(ATTACH_ANCHOR)
        .regressive_product(ATTACH_IN_BODY)
        .scale(k);
    let damping = state.velocity_in_local.scale(-0.25).dual();
    // gravity
    // gravity + damping
    // damping
    gravity + hooke + damping
    // hooke + damping
    // ppga3d::Line::new(0.0, 0.0, 0.0, 0.0, 0.0, 0.0)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let recording = rr::RecordingStreamBuilder::new("GA physics ppga3d")
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
        ppga3d::Point::new(1.0, -0.5, -0.5, -0.5),
        ppga3d::Point::new(1.0, -0.5, -0.5, 0.5),
        ppga3d::Point::new(1.0, -0.5, 0.5, -0.5),
        ppga3d::Point::new(1.0, -0.5, 0.5, 0.5),
        ppga3d::Point::new(1.0, 0.5, -0.5, -0.5),
        ppga3d::Point::new(1.0, 0.5, -0.5, 0.5),
        ppga3d::Point::new(1.0, 0.5, 0.5, -0.5),
        ppga3d::Point::new(1.0, 0.5, 0.5, 0.5),
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
        world_from_local: ppga3d::Motor::one(),
        velocity_in_local: ppga3d::Line::new(
            0.0, // rx
            0.0, // ry
            0.0, // rz
            0.0, // tx
            0.0, // ty
            0.0, // tz
        ),
        // velocity_in_local: ppga3d::Motor::new(
        //     0.0,  //
        //     1.0,  // rx
        //     0.0,  // ry
        //     0.0,  // rz
        //     0.0,  //
        //     0.0,  // tx
        //     0.0,  // ty
        //     10.0, // tz
        // ),
    };

    let dt = 0.001;
    for i in 0..3000 {
        let d_state = d_state(&state);
        state.world_from_local += d_state.d_world_from_local.scale(dt);
        state.velocity_in_local += d_state.d_velocity_in_local.scale(dt);

        let points = points_in_local
            .iter()
            .map(|p_in_local| {
                state
                    .world_from_local
                    .transformation(*p_in_local)
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
        let point = state
            .world_from_local
            .transformation(ATTACH_IN_BODY)
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

        let lines_in_local = edges
            .iter()
            .map(|e| rr::Arrow3D {
                origin: points_in_local[e.i].into_rerun_vec(),
                vector: (points_in_local[e.j] - points_in_local[e.i]).into_rerun_vec(),
            })
            .collect::<Vec<rr::Arrow3D>>();
        let anchor_in_local = state
            .world_from_local
            .reversal()
            .transformation(ATTACH_ANCHOR);
        let springs_in_local = vec![rr::Arrow3D {
            origin: anchor_in_local.into_rerun_vec(),
            vector: (ATTACH_IN_BODY - anchor_in_local).into_rerun_vec(),
        }];
        rr::MsgSender::new("local/points")
            .with_time(
                stable_time,
                rr::Time::from_seconds_since_epoch(i as f64 * dt as f64),
            )
            .with_component(&[anchor_in_local.into_rerun()])?
            .with_splat(radius)?
            .send(&recording)?;
        rr::MsgSender::new("local/lines")
            .with_time(
                stable_time,
                rr::Time::from_seconds_since_epoch(i as f64 * dt as f64),
            )
            .with_component(&lines_in_local)?
            .send(&recording)?;
        rr::MsgSender::new("local/springs")
            .with_time(
                stable_time,
                rr::Time::from_seconds_since_epoch(i as f64 * dt as f64),
            )
            .with_component(&springs_in_local)?
            .send(&recording)?;
    }

    Ok(())
}
