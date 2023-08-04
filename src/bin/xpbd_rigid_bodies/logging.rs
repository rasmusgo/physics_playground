use geometric_algebra::*;
use rerun::{
    components::{Box3D, ColorRGBA, Point3D, Quaternion, Radius, Vec3D},
    time::{Time, Timeline},
    MsgSender,
};

use crate::{Contact, ContactState, ToGlam, ORIGIN};

pub(crate) fn log_with_rerun(
    motors_curr: &[ppga3d::Motor],
    active_collisions: &[Option<Contact>],
    stable_time: Timeline,
    time: f32,
    colors: &[ColorRGBA],
    boxes: &[Box3D],
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
            Quaternion::new(g[1], g[2], g[3], -g[0])
        })
        .collect::<Vec<Quaternion>>();
    let collisions = active_collisions
        .iter()
        .filter_map(|op| op.as_ref())
        .map(|p| p.point.into())
        .collect::<Vec<Point3D>>();
    let collision_colors = active_collisions
        .iter()
        .filter_map(|op| op.as_ref())
        .map(|p| match p.state {
            ContactState::New => ColorRGBA::from_rgb(255, 0, 255),
            ContactState::Sticking => ColorRGBA::from_rgb(255, 0, 0),
            ContactState::Sliding => ColorRGBA::from_rgb(255, 255, 0),
        })
        .collect::<Vec<ColorRGBA>>();
    MsgSender::new("world/boxes")
        .with_time(stable_time, Time::from_seconds_since_epoch(time as _))
        .with_component(&positions)?
        .with_component(&rotations)?
        .with_component(colors)?
        // .with_splat(radius)?
        .with_component(boxes)?
        .send(recording)?;
    MsgSender::new("world/collisions")
        .with_time(stable_time, Time::from_seconds_since_epoch(time as _))
        .with_component(&collisions)?
        .with_component(&collision_colors)?
        .with_splat(Radius(0.03))?
        .send(recording)?;
    Ok(())
}
