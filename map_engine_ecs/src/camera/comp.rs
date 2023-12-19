use bevy_ecs::component::Component;

use super::Object;

#[derive(Component)]
pub struct Camera {
    pub object: Object,
}
