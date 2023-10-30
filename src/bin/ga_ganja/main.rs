use itertools::Itertools;
mod pga3d;
use pga3d::PGA3D;
mod rr {
    pub use rerun::external::glam::{self, vec3, Vec3};
}

trait IntoRerun<T> {
    fn into_rerun(self) -> rerun::Position3D;
    fn into_rerun_vec(self) -> rerun::Vec3D;
}

impl IntoRerun<PGA3D> for PGA3D {
    fn into_rerun(self) -> rerun::Position3D {
        rerun::Position3D::new(self[11] as _, self[12] as _, self[13] as _)
    }
    fn into_rerun_vec(self) -> rerun::Vec3D {
        rerun::Vec3D::new(self[11] as _, self[12] as _, self[13] as _)
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
    let recording = rerun::RecordingStreamBuilder::new("GA physics ganja").connect()?;
    recording.log_timeless("world", &rerun::ViewCoordinates::RIGHT_HAND_Z_UP)?;

    let radius = 0.025;

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
        let point = (&state.world_from_local * ATTACH_IN_BODY * state.world_from_local.Reverse())
            .into_rerun();
        let origin = anchor;
        let vector = rerun::Vector3D::from([
            point[0] - anchor[0],
            point[1] - anchor[1],
            point[2] - anchor[2],
        ]);

        recording.set_time_seconds("stable_time", i as f64 * dt);
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
