use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use bevy_log::prelude::*;
use bevy_time::TimePlugin;

pub struct Plugin;

impl bevy_app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        // bevy plugins
        app.add_plugins(bevy_log::LogPlugin::default());
        app.add_plugins(TimePlugin);

        // custom plugins
        app.add_plugins(super::input::InputPlugin);
        app.add_plugins(super::event::EventPlugin);
        app.add_plugins(super::object::ObjectPlugin);
        app.add_plugins(super::camera::CameraPlugin);

        // custom systems
        app.add_systems(Startup, startup);
        app.add_systems(Update, update);

        // example
        app.add_systems(Update, log_keyboard_events);
    }
}

fn startup() {
    // TODO
}

fn update() {
    // TODO
}

fn log_keyboard_events(
    mut keyboard_input_events: EventReader<bevy_input::keyboard::KeyboardInput>,
) {
    for event in keyboard_input_events.read() {
        info!("keyboard event: {:?}", event);
    }
}
