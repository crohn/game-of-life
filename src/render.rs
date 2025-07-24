use crate::{base64, cell::Cell, state::State};
use std::{io::Write, str};

const CELL_BYTES: usize = 3;

pub struct Frame<'a> {
    buffer: Vec<u8>,
    chunk_alive: Vec<u8>,
    cols: usize,
    scale: usize,
    state: &'a mut State,
}

impl<'a> Frame<'a> {
    pub fn new(state: &'a mut State, scale: usize) -> Self {
        let cols = state.cols;
        let rows = state.rows;

        Frame {
            buffer: vec![0x00; cols * rows * CELL_BYTES * scale * scale],
            chunk_alive: vec![0xff; CELL_BYTES * scale],
            cols,
            scale,
            state,
        }
    }

    pub fn to_ascii(&self) -> String {
        let mut cursor: usize = 0;

        let cols = self.state.cols;
        let rows = self.state.rows;

        let bytes = self.state.to_ascii();

        let frame_capacity = cols * rows + rows;
        let mut frame = String::with_capacity(frame_capacity);

        for i in (cols..=bytes.len()).step_by(cols) {
            let chunk = str::from_utf8(&bytes[cursor..i]).expect("cell bytes are UTF-8 safe");
            frame.push_str(chunk);
            frame.push('\n');
            cursor += cols;
        }

        frame
    }

    pub fn to_rgb(&mut self) -> String {
        let chunk_len = self.chunk_alive.len();
        let mut buf_cur = 0;

        for row in self.state.board.chunks_exact(self.cols) {
            let row_cur = buf_cur;

            for cell in row {
                if matches!(cell, Cell::Alive) {
                    self.buffer[buf_cur..buf_cur + chunk_len].copy_from_slice(&self.chunk_alive);
                }

                buf_cur += chunk_len;
            }

            for i in 0..self.scale - 1 {
                self.buffer
                    .copy_within(row_cur..buf_cur, buf_cur + (buf_cur - row_cur) * i);
            }

            buf_cur += (buf_cur - row_cur) * (self.scale - 1);
        }

        base64::encode(&self.buffer)
    }
}

pub fn render_kitty(frame: &mut Frame) -> Result<(), std::io::Error> {
    let &mut State {
        cols,
        rows,
        iteration,
        ..
    } = frame.state;

    let id = "123";
    let depth: u8 = 24;
    let width = cols * frame.scale;
    let height = rows * frame.scale;
    let payload = frame.to_rgb();
    let quiet: u8 = 2;

    if iteration == 0 {
        print!(
            "\x1b_Ga=T,f={},s={},v={},i={},q={};{}\x1b\\",
            depth, width, height, id, quiet, payload
        );
        print!("\x1b_Ga=a,i={},r=30,c=0\x1b\\", id);
    } else {
        print!(
            "\x1b_Ga=t,f={},s={},v={},i={},q={};{}\x1b\\",
            depth, width, height, id, quiet, payload
        );
    }

    if iteration % 30 == 0 {
        std::io::stdout().flush()?;
    }

    Ok(())
}
