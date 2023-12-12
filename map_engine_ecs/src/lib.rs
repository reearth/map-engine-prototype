mod app;
mod event;

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

    pub fn trigger_event(&mut self, ev: Event) {
        event::trigger_event(&mut self.app, self.win, ev);
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}
