use crate::{base64, state::State};
use std::{io::Write, str};

const CELL_BYTES: usize = 3;

pub struct Frame<'a> {
    buffer: Vec<u8>,
    cols: usize,
    scale: usize,
    state: &'a mut State,
}

impl<'a> Frame<'a> {
    pub fn new(state: &'a mut State, scale: usize) -> Self {
        let cols = state.cols;
        let rows = state.rows;
        let buffer = vec![0u8; cols * rows * CELL_BYTES * scale * scale];

        Frame {
            buffer,
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

    //
    // 0 1 2   3 4 5      0 1 2 3 4 5   6 7 8 9 a b
    //                 => c d e f 0 1   2 3 4 5 6 7
    // 6 7 8   9 a b
    //                    8 9 a b c d   e f 0 1 2 3
    //                    4 5 6 7 8 9   a b c d e f
    //
    pub fn to_rgb(&mut self) -> String {
        let bytes = &mut self.state.to_rgb();
        let bytes_row_len = self.cols * CELL_BYTES;

        let mut buf_cur = 0;

        for row in bytes.chunks_exact(bytes_row_len) {
            let row_cur = buf_cur;

            for cell in row.chunks_exact(CELL_BYTES) {
                self.buffer[buf_cur..buf_cur + CELL_BYTES].copy_from_slice(cell);

                for i in 1..self.scale {
                    self.buffer
                        .copy_within(buf_cur..buf_cur + CELL_BYTES, buf_cur + CELL_BYTES * i);
                }

                buf_cur += CELL_BYTES * self.scale;
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
