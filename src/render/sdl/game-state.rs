pub struct GameState {
    pub(crate) running: bool,
    pub(crate) show_grid: bool,
    pub(crate) show_help: bool,
    pub(crate) command: Option<String>,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            running: false,
            show_grid: true,
            show_help: false,
            command: None,
        }
    }
}

impl GameState {
    pub fn help(&mut self) {
        self.show_help = !self.show_help;
    }

    pub fn pause(&mut self) {
        self.running = false;
    }

    pub fn toggle(&mut self) {
        if !self.show_help {
            self.running = !self.running;
        }
    }

    pub fn toggle_grid(&mut self) {
        self.show_grid = !self.show_grid;
    }
}
