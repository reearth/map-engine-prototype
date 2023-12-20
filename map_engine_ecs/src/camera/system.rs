use bevy_ecs::system::{Query, Res};
use bevy_math::Vec3;
use bevy_time::Time;

use crate::Transform;

use super::CameraMerker;

pub fn example(time: Res<Time>, mut query: Query<(&mut Transform, &CameraMerker)>) {
    let (mut transform, _) = query.single_mut();

    let radian = time.elapsed_seconds() % 30.0 / 30.0 * std::f32::consts::PI * 2.0;
    let radius = transform.translation.length();
    let x = radian.cos() * radius;
    let z = radian.sin() * radius;

    transform.translation.x = x;
    transform.translation.z = z;
    transform.look_at(Vec3::ZERO, Vec3::Y);
}
