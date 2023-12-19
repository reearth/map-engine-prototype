use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use bevy_input::InputPlugin;
use bevy_log::prelude::*;

pub struct Plugin;

impl bevy_app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        // bevy plugins
        app.add_plugins(bevy_log::LogPlugin::default());
        app.add_plugins(InputPlugin);

        // custom plugins
        app.add_plugins(super::comp::EventPlugin);

        // custom systems
        app.add_systems(Startup, startup);
        app.add_systems(Update, movement);
        app.add_systems(Update, log_keyboard_events);
    }
}

#[derive(Component)]
struct Position {
    x: f32,
    y: f32,
}

#[derive(Component)]
struct Velocity {
    x: f32,
    y: f32,
}

fn startup(mut commands: Commands) {
    commands.spawn((
        Position { x: 0.0, y: 0.0 },
        Velocity { x: 1.0, y: 1.0 },
        super::comp::Object {
            transform: Default::default(),
        },
    ));
}

// This system moves each entity with a Position and Velocity component
fn movement(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Position, &Velocity, &mut super::comp::Object)>,
) {
    for (e, mut position, velocity, mut object) in &mut query {
        position.x += velocity.x;
        position.y += velocity.y;
        object.transform.translation.x = position.x;

        if position.x > 100.0 {
            commands.entity(e).remove::<super::comp::Object>();
            commands.entity(e).remove::<Position>();
        }
        // info!("movement - position ({}, {})", &position.x, &position.y);
    }
}

fn log_keyboard_events(
    mut keyboard_input_events: EventReader<bevy_input::keyboard::KeyboardInput>,
) {
    for event in keyboard_input_events.read() {
        info!("keyboard event: {:?}", event);
    }
}
