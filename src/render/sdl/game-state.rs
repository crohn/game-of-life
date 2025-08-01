pub struct GameState {
    pub running: bool,
    pub show_help: bool,
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
}
