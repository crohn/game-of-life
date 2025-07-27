use std::time::Duration;

use game_of_life::core::{Cell, Coords, State};
use game_of_life::render::ascii;

const COLS: u32 = 80;
const ROWS: u32 = 25;

fn main() {
    let mut state = State::new(COLS, ROWS);

    state.set_cell(Coords { x: 40, y: 39 }, Cell::Alive);
    state.set_cell(Coords { x: 40, y: 40 }, Cell::Alive);
    state.set_cell(Coords { x: 40, y: 41 }, Cell::Alive);
    state.set_cell(Coords { x: 39, y: 40 }, Cell::Alive);
    state.set_cell(Coords { x: 41, y: 41 }, Cell::Alive);

    let mut frame = ascii::Frame::new(COLS, ROWS);

    loop {
        ascii::render_ascii(&mut frame, &state);
        ascii::draw_ascii(&frame);
        state.next();
        std::thread::sleep(Duration::from_millis(50));
    }
}
