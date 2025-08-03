pub struct GameState {
    pub running: bool,
    pub show_grid: bool,
    pub show_help: bool,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            running: false,
            show_grid: true,
            show_help: false,
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
