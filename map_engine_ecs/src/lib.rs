mod app;
mod comp;
mod event;

pub use comp::{Object, ObjectEvent, ObjectEventType, Transform};

pub use event::*;

pub struct App {
    app: bevy_app::App,
    win: bevy_ecs::entity::Entity,
}

impl App {
    pub fn new() -> Self {
        let mut app = bevy_app::App::new();

        app.add_plugins(app::Plugin);

        let win = app.world.spawn_empty().id();

        Self { app, win }
    }

    pub fn update(&mut self) {
        self.app.update();
    }

    pub fn trigger_event(&mut self, ev: event::Event) {
        event::trigger_event(&mut self.app, self.win, ev);
    }

    pub fn read_events(&mut self) -> Vec<ObjectEvent> {
        comp::read_events(&mut self.app)
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}
