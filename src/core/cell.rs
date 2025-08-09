//! In Conway's Game of Life a cell can be dead or alive. In a square grid, each
//! cell has 8 neighbors. Every iteration depends on the previous state of the
//! board and each cell evaluates the count of its living neighbors:
//!
//! - A living cell that has exactly 2 or 3 living neighbors stays alive,
//! otherwise it becomes dead.
//! - A dead cell that has exactly 3 living neighbors becomes alive, otherwise
//! it stays dead.

#[derive(Clone, Copy)]
pub enum Cell {
    Dead,
    Alive,
}

impl Cell {
    pub fn as_value(&self) -> u8 {
        match self {
            Cell::Dead => 0,
            Cell::Alive => 1,
        }
    }

    pub fn next(&self, alive_neighbors: u8) -> Self {
        match (self, alive_neighbors) {
            (Cell::Alive, 2 | 3) => Cell::Alive,
            (Cell::Alive, _) => Cell::Dead,
            (Cell::Dead, 3) => Cell::Alive,
            (Cell::Dead, _) => Cell::Dead,
        }
    }

    /// Flip cell state, `Dead -> Alive`, `Alive -> Dead`.
    pub fn toggle(&mut self) {
        *self = match self {
            Cell::Alive => Cell::Dead,
            Cell::Dead => Cell::Alive,
        }
    }
}
