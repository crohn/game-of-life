use game_of_life::{
    core::{Cell, Coords, State},
    render::sdl,
};

const COLS: u32 = 80;
const ROWS: u32 = 25;

fn main() -> Result<(), String> {
    let mut state = State::new(COLS, ROWS);

    state.set_cell(Coords { x: 40, y: 39 }, Cell::Alive);
    state.set_cell(Coords { x: 40, y: 40 }, Cell::Alive);
    state.set_cell(Coords { x: 40, y: 41 }, Cell::Alive);
    state.set_cell(Coords { x: 39, y: 40 }, Cell::Alive);
    state.set_cell(Coords { x: 41, y: 41 }, Cell::Alive);

    sdl::run(&mut state, 10)
}
