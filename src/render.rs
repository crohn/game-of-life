use crate::{base64, state::State};
use std::{io::Write, str};

const CELL_BYTES: usize = 3;

pub struct Frame<'a> {
    state: &'a State,
}

impl<'a> Frame<'a> {
    pub fn to_ascii(&self) -> String {
        let mut cursor: usize = 0;

        let &State { cols, rows, .. } = self.state;
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
    pub fn to_rgb(&self, scale: usize) -> String {
        let bytes = self.state.to_rgb();
        let bytes_row_len = self.state.cols * CELL_BYTES;

        let mut buf = vec![0u8; bytes.len() * scale * scale];
        let mut buf_cur = 0;

        for row in bytes.chunks_exact(bytes_row_len) {
            let row_cur = buf_cur;

            for cell in row.chunks_exact(CELL_BYTES) {
                buf[buf_cur..buf_cur + CELL_BYTES].copy_from_slice(cell);

                for i in 1..scale {
                    buf.copy_within(buf_cur..buf_cur + CELL_BYTES, buf_cur + CELL_BYTES * i);
                }

                buf_cur += CELL_BYTES * scale;
            }

            for i in 0..scale - 1 {
                buf.copy_within(row_cur..buf_cur, buf_cur + (buf_cur - row_cur) * i);
            }

            buf_cur += (buf_cur - row_cur) * (scale - 1);
        }

        base64::encode(&buf)
    }
}

impl<'a> From<&'a State> for Frame<'a> {
    fn from(state: &'a State) -> Self {
        Frame { state }
    }
}

pub fn render_kitty(frame: &Frame, scale: usize) -> Result<(), std::io::Error> {
    let &State {
        cols,
        rows,
        iteration,
        ..
    } = frame.state;

    let id = "123";
    let depth: u8 = 24;
    let width = cols * scale;
    let height = rows * scale;
    let payload = frame.to_rgb(scale);
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
