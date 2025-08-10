use crate::{core::Coords, render::sdl::selection::Selection};

const SIM_PERIOD_STEP: u64 = 33;
const SIM_PERIOD_MAX: u64 = 33;
const SIM_PERIOD_MIN: u64 = 330;

pub struct GameState {
    pub(crate) command: Option<String>,
    pub(crate) running: bool,
    pub(crate) selection: Selection,
    pub(crate) show_grid: bool,
    pub(crate) show_help: bool,
    pub(crate) sim_period_ms: u64,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            command: None,
            selection: Selection::default(),
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

    // Simulation
    pub fn sim_speed_decr(&mut self) {
        self.sim_period_ms = (self.sim_period_ms - SIM_PERIOD_STEP).min(SIM_PERIOD_MIN);
    }
    pub fn sim_speed_incr(&mut self) {
        self.sim_period_ms = (self.sim_period_ms - SIM_PERIOD_STEP).max(SIM_PERIOD_MAX);
    }
    pub fn toggle_grid(&mut self) {
        self.show_grid = !self.show_grid;
    }

    // Selection
    pub fn add_to_sel<C: Into<Coords>>(&mut self, coords: C) {
        self.selection.toggle(coords);
    }
    pub fn clear_sel(&mut self) {
        self.selection.clear();
    }
    pub fn has_sel(&self) -> bool {
        !self.selection.is_empty()
    }
    pub fn iter_sel(&mut self) -> impl Iterator<Item = &Coords> {
        self.selection.iter()
    }
    pub fn mv_sel_down(&mut self, offset: i32) {
        self.selection.move_by((0, offset))
    }
    pub fn mv_sel_left(&mut self, offset: i32) {
        self.selection.move_by((-offset, 0))
    }
    pub fn mv_sel_right(&mut self, offset: i32) {
        self.selection.move_by((offset, 0))
    }
    pub fn mv_sel_up(&mut self, offset: i32) {
        self.selection.move_by((0, -offset))
    }
    pub fn recenter_sel(&mut self, coords: Coords) {
        self.selection.recenter_at(coords);
    }
    pub fn rot_sel_clockwise(&mut self) {
        self.selection.rotate_right();
    }
    pub fn rot_sel_counter(&mut self) {
        self.selection.rotate_left();
    }
}
