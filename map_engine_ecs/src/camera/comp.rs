use bevy_ecs::{bundle::Bundle, component::Component};

use crate::Transform;

#[derive(Component)]
pub struct CameraMerker;

#[derive(Bundle)]
pub struct CameraBundle {
    pub marker: CameraMerker,
    pub transform: Transform,
}
