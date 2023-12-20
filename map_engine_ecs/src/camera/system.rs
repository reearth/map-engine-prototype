use bevy_ecs::system::{Query, Res};
use bevy_time::Time;

use crate::Transform;

use super::CameraMerker;

pub fn example(time: Res<Time>, mut query: Query<(&mut Transform, &CameraMerker)>) {
    let (mut transform, _) = query.single_mut();
    transform.translation.x += time.delta_seconds();
}
