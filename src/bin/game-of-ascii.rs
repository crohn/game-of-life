use std::time::Duration;

use game_of_life::core::{Cell, Config, State};
use game_of_life::render::ascii;

const COLS: u32 = 80;
const ROWS: u32 = 25;

fn main() {
    let config = Config {
        cols: COLS,
        rows: ROWS,
    };

    let mut state = State::new(&config);

    state.set_cell(40, 39, Cell::Alive);
    state.set_cell(40, 40, Cell::Alive);
    state.set_cell(40, 41, Cell::Alive);
    state.set_cell(39, 40, Cell::Alive);
    state.set_cell(41, 41, Cell::Alive);

    let mut frame = ascii::Frame::new(COLS, ROWS);

    loop {
        ascii::render_ascii(&mut frame, &state);
        ascii::draw_ascii(&frame);
        state.next();
        std::thread::sleep(Duration::from_millis(50));
    }
}
