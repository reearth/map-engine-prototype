use bevy_ecs::{component::Component, entity::Entity, world::World};

use crate::Transform;

#[derive(Debug, Default)]
pub struct Events<'a> {
    pub camera_transform_updated: Option<&'a Transform>,
    pub object_transform_updated: Vec<ComponentEvent<'a, Transform>>,
    pub object_removed: Vec<EntityEvent>,
}

#[derive(Debug)]
pub struct ComponentEvent<'a, T = ()> {
    pub ind: u32,
    pub gen: u32,
    pub comp: Option<&'a T>,
}

#[derive(Debug)]
pub struct EntityEvent {
    pub ind: u32,
    pub gen: u32,
}

impl From<Entity> for EntityEvent {
    fn from(e: Entity) -> Self {
        Self {
            ind: e.index(),
            gen: e.generation(),
        }
    }
}

impl<'a, T: Component> ComponentEvent<'a, T> {
    pub fn new(e: Entity, world: &'a World) -> Self {
        Self {
            ind: e.index(),
            gen: e.generation(),
            comp: world.get::<T>(e),
        }
    }
}
