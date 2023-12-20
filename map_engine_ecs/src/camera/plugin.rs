use super::super::event::EventStore;
use super::CameraBundle;
use crate::Transform;

use super::CameraMerker;
use bevy_app::{PostUpdate, Startup, Update};
use bevy_ecs::{
    entity::Entity,
    query::Changed,
    system::{Commands, Query, ResMut},
};
use bevy_math::Vec3;

#[derive(Debug)]
pub struct CameraPlugin;

impl bevy_app::Plugin for CameraPlugin {
    fn build(&self, app: &mut bevy_app::App) {
        app.add_systems(Startup, startup)
            .add_systems(Update, super::system::example)
            .add_systems(PostUpdate, commit);
    }
}

fn startup(mut commands: Commands) {
    let translation = Vec3::new(-1000.0, 0.0, 0.0);
    commands.spawn(CameraBundle {
        marker: CameraMerker,
        transform: Transform::from_translation(translation),
    });
}

fn commit(
    mut events: ResMut<EventStore>,
    mut query: Query<(Entity, &CameraMerker), Changed<Transform>>,
) {
    let (e, _) = query.single_mut();
    events.camera_transform_updated = Some(e);
}
