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

   let mut world = World::new();
    world.register::<Position>();
    world.register::<Velocity>();

    world.create_entity().with(Position { x: 4.0, y: 7.0 }).build();

    let mut hello_world = HelloWorld;
    hello_world.run_now(&world);
    world.maintain();    
}

use specs::{Builder, Component, ReadStorage, System, VecStorage, World, WorldExt, RunNow};

#[derive(Debug)]
struct Position {
    x: f32,
    y: f32,
}

impl Component for Position {
    type Storage = VecStorage<Self>;
}

#[derive(Debug)]
struct Velocity {
    x: f32,
    y: f32,
}

impl Component for Velocity {
    type Storage = VecStorage<Self>;
}

struct HelloWorld;

impl<'a> System<'a> for HelloWorld {
    type SystemData = ReadStorage<'a, Position>;

    fn run(&mut self, position: Self::SystemData) {
        use specs::Join;

        for position in position.join() {
            log(&format!("Hello, {:?}", &position));
        }
    }
}
