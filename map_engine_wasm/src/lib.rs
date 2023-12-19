mod utils;

use std::{
    collections::HashMap,
    sync::{Mutex, OnceLock},
};

use map_engine_ecs::App;
use wasm_bindgen::prelude::*;
mod event;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen(getter_with_clone)]
pub struct Core {
    pub id: String,
}

#[wasm_bindgen]
impl Core {
    #[wasm_bindgen(constructor)]
    pub fn new(id: String) -> Self {
        Self { id }
    }

    pub fn start(&self) {
        init(self.id.clone());
    }

    pub fn update(&self) {
        update(self.id.clone());
    }

    #[wasm_bindgen(js_name = readEvents)]
    pub fn read_events(&self) -> Result<JsValue, serde_wasm_bindgen::Error> {
        read_events(self.id.clone())
    }
}

#[wasm_bindgen(start)]
pub fn start() {
    utils::set_panic_hook();
    log("init map_engine_wasm");
}

pub fn init(id: String) {
    app(id, |a| {
        // debug
        a.trigger_event(map_engine_ecs::Event::Keyboard(
            map_engine_ecs::KeyboardEvent {
                scan_code: 0,
                key_code: Some(map_engine_ecs::KeyCode::A),
                state: map_engine_ecs::ButtonState::Pressed,
            },
        ));
    });
}

pub fn update(id: String) {
    app(id, |a| a.update());
}

pub fn read_events(id: String) -> Result<JsValue, serde_wasm_bindgen::Error> {
    app(id, |a| {
        let mut events = a.read_events();
        let mut js_events = Vec::new();

        while let Some(ev) = events.pop() {
            js_events.push(event::ObjectEvent::from(ev));
        }

        serde_wasm_bindgen::to_value(&js_events)
    })
}

fn app<T>(id: String, f: impl FnOnce(&mut App) -> T) -> T {
    static APP: OnceLock<Mutex<HashMap<String, Mutex<App>>>> = OnceLock::new();
    let mut map = APP
        .get_or_init(|| Mutex::new(HashMap::new()))
        .lock()
        .unwrap();

    let app = map
        .entry(id.to_string())
        .or_insert_with(|| Mutex::new(App::new()))
        .get_mut()
        .unwrap();

    f(app)
}
