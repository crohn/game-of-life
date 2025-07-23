#[derive(Clone, PartialEq)]
pub enum Cell {
    Alive,
    Dead,
}

impl Cell {
    pub fn to_ascii(&self) -> u8 {
        match self {
            Cell::Alive => b'@',
            Cell::Dead => b'.',
        }
    }

    pub fn to_rgb(&self) -> (u8, u8, u8) {
        match self {
            Cell::Alive => (0xff, 0xff, 0xff),
            Cell::Dead => (0x00, 0x00, 0x00),
        }
    }
}
