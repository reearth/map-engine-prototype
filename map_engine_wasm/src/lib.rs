mod utils;

use std::sync::{Mutex, OnceLock};

use map_engine_ecs::App;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen(start)]
pub fn start() {
    utils::set_panic_hook();
    log("init map_engine_wasm");

    // init app
    let _ = app();
}

#[wasm_bindgen]
pub fn update() {
    app().lock().unwrap().update();
}

fn app() -> &'static Mutex<App> {
    static APP: OnceLock<Mutex<App>> = OnceLock::new();
    APP.get_or_init(|| Mutex::new(App::new()))
}
