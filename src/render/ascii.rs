use core::str;

use crate::core::{Cell, State};

const ALIVE: u8 = b'@';
const DEAD: u8 = b'.';

pub struct Frame {
    buffer: Vec<u8>,
    generation: u32,
}

impl Frame {
    pub fn new(cols: u32, rows: u32) -> Self {
        Frame {
            // add `rows` because the buffer must contain linefeed characters for each row
            buffer: vec![DEAD; (cols * rows + rows) as usize],
            generation: 0,
        }
    }
}

fn cell_to_ascii(cell: &Cell) -> u8 {
    match cell {
        Cell::Alive => ALIVE,
        Cell::Dead => DEAD,
    }
}

pub fn draw_ascii(frame: &Frame) {
    let Frame { buffer, generation } = frame;
    println!("\x1b[3J\x1b[H\x1b[2J"); // clear terminal
    let frame = str::from_utf8(buffer).expect("");
    println!("{frame}");
    println!("Generation: {generation}");
}

pub fn render_ascii(frame: &mut Frame, state: &State) {
    let cols = state.cols as usize;
    let mut i = 0;

    for row in state.curr.chunks_exact(cols) {
        for cell in row {
            frame.buffer[i] = cell_to_ascii(cell);
            i += 1;
        }
        frame.buffer[i] = b'\n';
        i += 1;
    }

    frame.generation = state.generation;
}
