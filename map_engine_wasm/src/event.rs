use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Events {
    pub camera_transform_updated: Option<Transform>,
    pub object_transform_updated: Vec<ObjectEvent<Transform>>,
    pub object_removed: Vec<ObjectEvent>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ObjectEvent<T = ()> {
    pub ind: u32,
    pub gen: u32,
    pub object: Option<T>,
}

#[derive(Debug, Clone, Serialize)]
pub struct Transform {
    translation: Vec<f32>,
    quaternion: Vec<f32>,
    scale: Vec<f32>,
}

impl<'a> From<map_engine_ecs::Events<'a>> for Events {
    fn from(ev: map_engine_ecs::Events) -> Self {
        Self {
            camera_transform_updated: ev.camera_transform_updated.map(|ev| (*ev).into()),
            object_transform_updated: ev
                .object_transform_updated
                .into_iter()
                .map(|ev| ev.into())
                .collect(),
            object_removed: ev.object_removed.into_iter().map(|ev| ev.into()).collect(),
        }
    }
}

impl<'a, T, K: Into<T> + Clone> From<map_engine_ecs::ComponentEvent<'a, K>> for ObjectEvent<T> {
    fn from(ev: map_engine_ecs::ComponentEvent<'a, K>) -> Self {
        Self {
            ind: ev.ind,
            gen: ev.gen,
            object: ev.comp.map(|o| o.clone().into()),
        }
    }
}

impl From<map_engine_ecs::EntityEvent> for ObjectEvent {
    fn from(ev: map_engine_ecs::EntityEvent) -> Self {
        Self {
            ind: ev.ind,
            gen: ev.gen,
            object: None,
        }
    }
}

impl From<map_engine_ecs::Transform> for Transform {
    fn from(t: map_engine_ecs::Transform) -> Self {
        Self {
            translation: vec![t.translation.x, t.translation.y, t.translation.z],
            quaternion: vec![t.rotation.x, t.rotation.y, t.rotation.z, t.rotation.w],
            scale: vec![t.scale.x, t.scale.y, t.scale.z],
        }
    }
}
