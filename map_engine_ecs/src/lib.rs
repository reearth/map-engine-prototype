mod app;

pub struct App {
    app: bevy_app::App,
}

impl App {
    pub fn new() -> Self {
        let mut app = bevy_app::App::new();

        app.add_plugins(bevy_log::LogPlugin::default());
        app.add_plugins(app::Plugin);

        Self { app }
    }

    pub fn update(&mut self) {
        self.app.update();
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}
