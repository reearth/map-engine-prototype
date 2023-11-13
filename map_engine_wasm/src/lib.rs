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

    log(&format!("Hello world?! {}", &map_engine_core::add(2, 3)));
}
