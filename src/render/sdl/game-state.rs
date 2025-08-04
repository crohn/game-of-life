use crate::core::Coords;

pub struct GameState {
    pub(crate) running: bool,
    pub(crate) show_grid: bool,
    pub(crate) show_help: bool,
    pub(crate) command: Option<String>,
    pub(crate) selected_cell: Option<Coords>,
    pub(crate) sim_period_ms: u64,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            running: false,
            show_grid: true,
            show_help: false,
            command: None,
            selected_cell: None,
            sim_period_ms: 33,
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

    pub fn deselect_cell(&mut self) {
        self.selected_cell = None;
    }

    pub fn select_cell(&mut self, x: i32, y: i32, cols: u32, rows: u32) {
        if let Some(coords) = &mut self.selected_cell {
            coords.x = (coords.x + x).rem_euclid(cols as i32);
            coords.y = (coords.y + y).rem_euclid(rows as i32);
        } else {
            self.selected_cell = Some(Coords { x: 0, y: 0 })
        }
    }
}
