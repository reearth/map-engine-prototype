use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use bevy_log::prelude::*;

pub struct Plugin;

impl bevy_app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, startup);
        app.add_systems(Update, movement);
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
    commands.spawn((Position { x: 0.0, y: 0.0 }, Velocity { x: 1.0, y: 1.0 }));
}

// This system moves each entity with a Position and Velocity component
fn movement(mut query: Query<(&mut Position, &Velocity)>) {
    for (mut position, velocity) in &mut query {
        position.x += velocity.x;
        position.y += velocity.y;

        info!("movement - position ({}, {})", &position.x, &position.y);
    }
}
