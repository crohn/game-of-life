use game_of_life::{
    core::{Cell, Coords, State},
    render::sdl::{game::Game, sdl_context::SdlContext},
};

const COLS: u32 = 80;
const ROWS: u32 = 25;
const SCALE: u32 = 10;

fn main() -> Result<(), String> {
    let mut state = State::new(COLS, ROWS);
    let sdl_context = SdlContext::new(COLS, ROWS, SCALE)?;

    state.set_cell(Coords { x: 40, y: 39 }, Cell::Alive);
    state.set_cell(Coords { x: 40, y: 40 }, Cell::Alive);
    state.set_cell(Coords { x: 40, y: 41 }, Cell::Alive);
    state.set_cell(Coords { x: 39, y: 40 }, Cell::Alive);
    state.set_cell(Coords { x: 41, y: 41 }, Cell::Alive);

    let mut game = Game::new(sdl_context, state);
    game.run()
}
