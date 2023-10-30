use geometric_algebra::*;
use itertools::Itertools;
mod rr {
    pub use rerun::external::glam::{self, vec3, Vec3};
}

trait IntoRerun<T> {
    fn into_rerun(self) -> rerun::Position3D;
    fn into_rerun_vec(self) -> rerun::Vec3D;
}

impl IntoRerun<ppga3d::Point> for ppga3d::Point {
    fn into_rerun(self) -> rerun::Position3D {
        unsafe {
            rerun::Position3D::new(
                self.group0().f32x4[1],
                self.group0().f32x4[2],
                self.group0().f32x4[3],
            )
        }
    }

    fn into_rerun_vec(self) -> rerun::Vec3D {
        unsafe {
            rerun::Vec3D::new(
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
    let recording = rerun::RecordingStreamBuilder::new("GA physics ppga3d").connect()?;
    recording.log_timeless("world", &rerun::ViewCoordinates::RIGHT_HAND_Z_UP)?;

    let radius = 0.025;

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
            .collect::<Vec<_>>();
        let (edge_origins, edge_vectors): (Vec<_>, Vec<_>) = edges
            .iter()
            .map(|e| {
                (
                    points[e.i],
                    (
                        points[e.j][0] - points[e.i][0],
                        points[e.j][1] - points[e.i][1],
                        points[e.j][2] - points[e.i][2],
                    ),
                )
            })
            .unzip();
        let anchor = ATTACH_ANCHOR.into_rerun();
        let point = state
            .world_from_local
            .transformation(ATTACH_IN_BODY)
            .into_rerun();
        let origin = anchor;
        let vector = rerun::Vector3D::from([
            point[0] - anchor[0],
            point[1] - anchor[1],
            point[2] - anchor[2],
        ]);

        recording.set_time_seconds("stable_time", i as f64 * dt as f64);
        recording.log(
            "world/points",
            &rerun::Points3D::new(points.clone()).with_radii([radius]),
        )?;
        recording.log(
            "world/lines",
            &rerun::Arrows3D::from_vectors(edge_vectors).with_origins(edge_origins),
        )?;
        recording.log(
            "world/springs",
            &rerun::Arrows3D::from_vectors([vector]).with_origins([origin]),
        )?;
    }

    Ok(())
}
