use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct ObjectEvent {
    pub ind: u32,
    pub gen: u32,
    pub event_type: ObjectEventType,
    pub object: Option<Object>,
}

#[derive(Debug, Clone, Serialize)]
pub enum ObjectEventType {
    ObjectRemoved,
    ObjectChanged,
}

#[derive(Debug, Clone, Serialize)]
pub struct Object {
    pub transform: Transform,
}

#[derive(Debug, Clone, Serialize)]
pub struct Transform {
    translation: Vec<f32>,
    quaternion: Vec<f32>,
    scale: Vec<f32>,
}

impl<'a> From<map_engine_ecs::ObjectEvent<'a>> for ObjectEvent {
    fn from(ev: map_engine_ecs::ObjectEvent) -> Self {
        Self {
            ind: ev.ind,
            gen: ev.gen,
            event_type: ev.event_type.into(),
            object: ev.object.map(|o| o.clone().into()),
        }
    }
}

impl From<map_engine_ecs::ObjectEventType> for ObjectEventType {
    fn from(ev: map_engine_ecs::ObjectEventType) -> Self {
        match ev {
            map_engine_ecs::ObjectEventType::ObjectRemoved => Self::ObjectRemoved,
            map_engine_ecs::ObjectEventType::ObjectChanged => Self::ObjectChanged,
        }
    }
}

impl From<map_engine_ecs::Object> for Object {
    fn from(o: map_engine_ecs::Object) -> Self {
        Self {
            transform: o.transform.into(),
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
