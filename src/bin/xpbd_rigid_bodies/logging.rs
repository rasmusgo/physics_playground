use geometric_algebra::*;
use rerun::{Quaternion, Vec3D};

use crate::{Contact, ContactState, ToGlam, ORIGIN};

pub(crate) fn log_with_rerun(
    motors_curr: &[ppga3d::Motor],
    active_collisions: &[Option<Contact>],
    time: f32,
    colors: &[rerun::Color],
    boxes: &[[f32; 3]],
    recording: &rerun::RecordingStream,
) -> Result<(), Box<dyn std::error::Error>> {
    puffin::profile_function!();
    let positions = motors_curr
        .iter()
        .map(|m| m.transformation(ORIGIN).to_glam().into())
        .collect::<Vec<Vec3D>>();
    let rotations = motors_curr
        .iter()
        .map(|m| {
            let g = m.group0();
            Quaternion::from_wxyz([g[0], -g[1], -g[2], -g[3]])
        })
        .collect::<Vec<Quaternion>>();
    let collisions = active_collisions
        .iter()
        .filter_map(|op| op.as_ref())
        .map(|p| p.point.into())
        .collect::<Vec<(f32, f32, f32)>>();
    let collision_colors = active_collisions
        .iter()
        .filter_map(|op| op.as_ref())
        .map(|p| match p.state {
            ContactState::New => rerun::Color::from_rgb(255, 0, 255),
            ContactState::Sticking => rerun::Color::from_rgb(255, 0, 0),
            ContactState::Sliding => rerun::Color::from_rgb(255, 255, 0),
        })
        .collect::<Vec<_>>();
    recording.set_time_seconds("stable_time", time);
    recording.log(
        "world/boxes",
        &rerun::Boxes3D::from_half_sizes(boxes)
            .with_centers(positions)
            .with_rotations(rotations)
            .with_colors(colors.to_vec()),
    )?;
    recording.log(
        "world/collisions",
        &rerun::Points3D::new(collisions)
            .with_colors(collision_colors)
            .with_radii([0.03]),
    )?;
    Ok(())
}
