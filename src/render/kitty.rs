use std::io::Write;

use crate::{
    base64,
    core::{Cell, State},
};

const ALIVE: u8 = 0xff;
const DEAD: u8 = 0x00;
const KITTY_COLOR_DEPTH: u8 = 24;
const KITTY_IMG_ID: u8 = 246;
const KITTY_QUIET: u8 = 2;
const RGB_BYTES: u8 = 3;

pub struct Frame {
    buffer: Vec<u8>,
    chunk_alive: Vec<u8>,
    generation: u32,
    height: u32,
    scale: u8,
    width: u32,
}

impl Frame {
    pub fn new(cols: u32, rows: u32, scale: u8) -> Self {
        Frame {
            buffer: vec![
                DEAD;
                (cols * rows * RGB_BYTES as u32 * scale as u32 * scale as u32) as usize
            ],
            // pre-allocate the horizontal chunk of subsequent ALIVE pixels so
            // it can be easily copied into the buffer, whenever a living cell
            // is encountered.
            chunk_alive: vec![ALIVE; (RGB_BYTES * scale) as usize],
            generation: 0,
            scale,
            width: cols * scale as u32,
            height: rows * scale as u32,
        }
    }
}

pub fn render_kitty(frame: &mut Frame, state: &State) {
    frame.buffer.fill(DEAD);

    let vert_scale_iterations = (frame.scale - 1) as usize;
    let chunk_len = frame.chunk_alive.len();
    let mut buf_cur = 0;

    for row in state.curr.chunks_exact(state.cols as usize) {
        let row_cur = buf_cur;

        for cell in row {
            // Skip dead cells, because the background is set to dead. This
            // potentially saves a bunch of writes.
            if matches!(cell, Cell::Alive) {
                frame.buffer[buf_cur..buf_cur + chunk_len].copy_from_slice(&frame.chunk_alive);
            }

            buf_cur += chunk_len;
        }

        for i in 0..vert_scale_iterations {
            frame
                .buffer
                .copy_within(row_cur..buf_cur, buf_cur + (buf_cur - row_cur) * i);
        }

        // advance cursor by a whole row
        buf_cur += (buf_cur - row_cur) * (vert_scale_iterations);
    }

    frame.generation = state.generation;
}

pub fn draw_kitty(frame: &Frame) -> Result<(), std::io::Error> {
    let payload = base64::encode(&frame.buffer);
    let action = if frame.generation == 0 { "T" } else { "t" };

    print!(
        "\x1b_Ga={},f={},s={},v={},i={},q={};{}\x1b\\",
        action, KITTY_COLOR_DEPTH, frame.width, frame.height, KITTY_IMG_ID, KITTY_QUIET, payload
    );

    if frame.generation % 30 == 0 {
        std::io::stdout().flush()?;
    }

    Ok(())
}
