use std::collections::HashSet;

use crate::core::{Coords, State};

pub struct GameState {
    pub(crate) command: Option<String>,
    pub(crate) running: bool,
    pub(crate) cursor: HashSet<Coords>,
    pub(crate) show_grid: bool,
    pub(crate) show_help: bool,
    pub(crate) sim_period_ms: u64,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            command: None,
            cursor: HashSet::new(),
            running: false,
            show_grid: true,
            show_help: false,
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

    pub fn toggle_running(&mut self) {
        // if !self.show_help {
        self.running = !self.running;
        // }
    }

    pub fn toggle_grid(&mut self) {
        self.show_grid = !self.show_grid;
    }

    pub fn hide_cursor(&mut self) {
        self.cursor.clear();
    }

    // Move existing cursor by (x,y) offset. Negative final coordinates are
    // wrapped.
    pub fn move_cursor(&mut self, x: i32, y: i32, state: &State) {
        self.cursor = self
            .cursor
            .drain()
            .map(|mut coords| {
                coords.x += x;
                coords.y += y;
                state.wrap_coords(&mut coords);
                coords
            })
            .collect();
    }

    pub fn add_cursor(&mut self, x: i32, y: i32, state: &State) {
        let coords = state.create_coords(x, y);

        if let None = self.cursor.get(&coords) {
            self.cursor.insert(coords);
        } else {
            self.cursor.remove(&coords);
        }
    }
}
