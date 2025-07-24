use game_of_life::{
    cell::Cell,
    coords::Coords,
    render::{self, Frame},
    state::State,
};

fn main() -> Result<(), std::io::Error> {
    let cols = 80;
    let rows = 25;
    let mut state = State::new(cols, rows);
    let mut frame = Frame::new(cols, rows, 10);

    state.set_cell(Coords::new(4, 5), Cell::Alive);
    state.set_cell(Coords::new(4, 6), Cell::Alive);
    state.set_cell(Coords::new(4, 7), Cell::Alive);
    state.set_cell(Coords::new(3, 6), Cell::Alive);
    state.set_cell(Coords::new(5, 7), Cell::Alive);

    loop {
        render::render_kitty(&mut frame, &state)?;
        state.next();
    }
}
