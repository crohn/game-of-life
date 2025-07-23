use std::{thread::sleep, time::Duration};

use game_of_life::{
    cell::Cell,
    coords::Coords,
    render::{self, Frame},
    state::State,
};

fn main() -> Result<(), std::io::Error> {
    let mut state = State::new(80, 25);

    // state.set_cell(Coords::new(5, 3), Cell::Alive);
    // state.set_cell(Coords::new(6, 4), Cell::Alive);
    // state.set_cell(Coords::new(4, 5), Cell::Alive);
    // state.set_cell(Coords::new(5, 5), Cell::Alive);
    // state.set_cell(Coords::new(6, 5), Cell::Alive);
    state.set_cell(Coords::new(4, 5), Cell::Alive);
    state.set_cell(Coords::new(4, 6), Cell::Alive);
    state.set_cell(Coords::new(4, 7), Cell::Alive);
    state.set_cell(Coords::new(3, 6), Cell::Alive);
    state.set_cell(Coords::new(5, 7), Cell::Alive);

    // state.set_cell(Coords::new(30, 30), Cell::Alive);
    // state.set_cell(Coords::new(31, 30), Cell::Alive);
    // state.set_cell(Coords::new(32, 30), Cell::Alive);
    // state.set_cell(Coords::new(32, 29), Cell::Alive);
    // state.set_cell(Coords::new(31, 28), Cell::Alive);

    // loop {
    //     let frame = Frame::new(&mut state, 10);
    //     println!("\x1b[3J\x1b[H\x1b[2J");
    //     println!("{}", frame.to_ascii());
    //     state.next();
    //     sleep(Duration::from_millis(100));
    // }

    loop {
        let mut frame = Frame::new(&mut state, 10);
        render::render_kitty(&mut frame)?;
        state.next();
    }
}
