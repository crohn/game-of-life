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
}
