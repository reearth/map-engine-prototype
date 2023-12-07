mod utils;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, map_engine_wasm!");
}

#[wasm_bindgen(start)]
pub fn start() {
    utils::set_panic_hook();

    log(&format!(
        "Hello world!!!!!! {}",
        &map_engine_core::add(2, 3)
    ));

    // Create a new empty World to hold our Entities and Components
    let mut world = World::new();

    // Spawn an entity with Position and Velocity components
    world.spawn((Position { x: 0.0, y: 0.0 }, Velocity { x: 1.0, y: 0.0 }));

    // Create a new Schedule, which defines an execution strategy for Systems
    let mut schedule = Schedule::default();

    // Add our system to the schedule
    schedule.add_systems(movement);

    // Run the schedule once. If your app has a "loop", you would run this once per loop
    schedule.run(&mut world);
}

use bevy_ecs::prelude::*;

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

// This system moves each entity with a Position and Velocity component
fn movement(mut query: Query<(&mut Position, &Velocity)>) {
    for (mut position, velocity) in &mut query {
        position.x += velocity.x;
        position.y += velocity.y;

        log(&format!(
            "movement - position ({}, {})",
            &position.x, &position.y
        ));
    }
}
