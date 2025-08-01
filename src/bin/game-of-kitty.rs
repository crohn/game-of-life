use game_of_life::{
    core::{Cell, Config, Coords, State},
    render::kitty,
};

const COLS: u32 = 80;
const ROWS: u32 = 25;

fn main() -> Result<(), std::io::Error> {
    let config = Config {
        cols: COLS,
        rows: ROWS,
    };

    let mut state = State::new(&config);

    state.set_cell(Coords { x: 40, y: 39 }, Cell::Alive);
    state.set_cell(Coords { x: 40, y: 40 }, Cell::Alive);
    state.set_cell(Coords { x: 40, y: 41 }, Cell::Alive);
    state.set_cell(Coords { x: 39, y: 40 }, Cell::Alive);
    state.set_cell(Coords { x: 41, y: 41 }, Cell::Alive);

    let mut frame = kitty::Frame::new(COLS, ROWS, 10);

    loop {
        kitty::render_kitty(&mut frame, &state);
        kitty::draw_kitty(&frame)?;
        state.next();
    }
}
