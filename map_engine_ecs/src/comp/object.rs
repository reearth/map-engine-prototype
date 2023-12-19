use bevy_ecs::component::Component;

use super::Transform;

#[derive(Component, Debug, Clone)]
pub struct Object {
    pub transform: Transform,
}
