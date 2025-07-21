use crate::state::State;
use std::str;

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
}

impl<'a> From<&'a State> for Frame<'a> {
    fn from(state: &'a State) -> Self {
        Frame { state }
    }
}
