#[derive(Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

impl Cell {
    pub fn to_ascii(&self) -> u8 {
        match self {
            Cell::Alive => b'@',
            Cell::Dead => b'.',
        }
    }

    pub fn next(&self, alive_neighbors: usize) -> Self {
        match (self, alive_neighbors) {
            (Cell::Alive, 2 | 3) => Cell::Alive,
            (Cell::Alive, _) => Cell::Dead,
            (Cell::Dead, 3) => Cell::Alive,
            (Cell::Dead, _) => Cell::Dead,
        }
    }
}
