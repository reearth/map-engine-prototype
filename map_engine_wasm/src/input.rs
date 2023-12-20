use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Input;

// #[wasm_bindgen]
// #[derive(Debug, Clone, Copy, PartialEq)]
// pub enum Input {
//     KeyPress(KeyPress),
//     KeyRelease,
//     MousePress,
//     MouseRelease,
//     MouseMove,
//     MouseScroll,
// }

// #[wasm_bindgen]
// #[derive(Debug, Clone, PartialEq)]
// pub struct KeyPress {
//     #[wasm_bindgen(getter_with_clone)]
//     pub key_code: String,
// }

// impl From<KeyPress> for map_engine_ecs::KeyboardEvent {
//     fn from(key_press: KeyPress) -> Self {
//         Self {
//             key_code: Some(key_press.key_code),
//         }
//     }
// }
