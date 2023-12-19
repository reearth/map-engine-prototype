use bevy_app::PostUpdate;
use bevy_ecs::{
    entity::Entity,
    query::Changed,
    removal_detection::RemovedComponents,
    system::{Query, ResMut, Resource},
    world::World,
};

use super::Object;

#[derive(Debug, Clone)]
pub struct EventPlugin;

impl bevy_app::Plugin for EventPlugin {
    fn build(&self, app: &mut bevy_app::App) {
        app.insert_resource(Events::default())
            .add_systems(PostUpdate, collect_events);
    }
}

#[derive(Debug, Clone)]
pub struct ObjectEvent<'a> {
    pub event_type: ObjectEventType,
    pub ind: u32,
    pub gen: u32,
    pub object: Option<&'a Object>,
}

#[derive(Debug, Clone)]
pub enum ObjectEventType {
    ObjectRemoved,
    ObjectChanged,
}

#[derive(Debug, Default, Resource)]
pub struct Events {
    pub updated: Vec<Entity>,
    pub removed: Vec<Entity>,
}

impl Events {
    pub fn clear(&mut self) {
        self.updated.clear();
        self.removed.clear();
    }

    pub fn event_objects<'a>(&self, world: &'a World) -> Vec<ObjectEvent<'a>> {
        let mut events = Vec::new();
        for e in self.updated.iter() {
            events.push(ObjectEvent {
                event_type: ObjectEventType::ObjectChanged,
                ind: e.index(),
                gen: e.generation(),
                object: world.get::<Object>(*e),
            });
        }
        for e in self.removed.iter() {
            events.push(ObjectEvent {
                event_type: ObjectEventType::ObjectRemoved,
                ind: e.index(),
                gen: e.generation(),
                object: world.get::<Object>(*e),
            });
        }
        events
    }
}

pub fn collect_events(
    mut events: ResMut<Events>,
    mut removed: RemovedComponents<Object>,
    mut changed: Query<Entity, Changed<Object>>,
) {
    events.clear();

    for e in removed.read() {
        events.removed.push(e);
    }
    for e in changed.iter_mut() {
        events.updated.push(e);
    }
}

pub fn read_events(app: &mut bevy_app::App) -> Vec<ObjectEvent> {
    let ev = if let Some(ev) = app.world.get_resource::<Events>() {
        ev
    } else {
        return Vec::new();
    };

    ev.event_objects(&app.world)
}
