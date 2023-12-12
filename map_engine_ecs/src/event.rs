use bevy_app::App;
use bevy_ecs::entity::Entity;

pub use bevy_input::{keyboard::KeyCode, ButtonState};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Event {
    Keyboard(KeyboardEvent),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KeyboardEvent {
    pub scan_code: u32,
    pub key_code: Option<KeyCode>,
    pub state: ButtonState,
}

impl KeyboardEvent {
    pub fn into_event(self, win: Entity) -> bevy_input::keyboard::KeyboardInput {
        bevy_input::keyboard::KeyboardInput {
            scan_code: self.scan_code,
            key_code: self.key_code,
            state: self.state,
            window: win,
        }
    }
}

pub fn trigger_event(app: &mut App, win: Entity, ev: Event) {
    match ev {
        Event::Keyboard(ev) => app.world.send_event(ev.into_event(win)),
    }
}
