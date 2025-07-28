use std::mem;

use super::{
    cell::Cell,
    coords::{self, Coords, coords_from_index},
};

/// Game of Life state
///
/// Includes board size (expressed in number of columns and rows), current
/// generation, board state (current and next), and pre-computed neighbor list
/// for each cell.
pub struct State {
    pub(crate) generation: u32,

    // Board
    pub(crate) cols: u32,
    pub(crate) rows: u32,
    pub(crate) curr: Vec<Cell>,
    next: Vec<Cell>,

    neighbors: Vec<Vec<usize>>,
}

impl State {
    /// State is initialized with two boards, one representing current state and
    /// another for the next one. This double buffer enables better performance,
    /// especially in conjunction with mem::swap.
    ///
    /// We also pre-compute the indices of all neighbors for each cell, in order
    /// to speedup neighbor inspection.
    pub fn new(cols: u32, rows: u32) -> Self {
        let board_capacity = (cols * rows) as usize;
        let curr = vec![Cell::Dead; board_capacity];
        let next = curr.clone();

        let neighbors = (0..board_capacity)
            .map(|i| get_neighbors_indices(i, cols, rows))
            .collect();

        State {
            generation: 0,
            cols,
            rows,
            curr,
            next,
            neighbors,
        }
    }

    /// Returns the number of alive neighbors using the pre-computed neighbors
    /// index.
    fn count_alive_neighbors(&self, index: usize) -> u8 {
        self.neighbors[index]
            .iter()
            .fold(0, |count, &neighbor| count + self.curr[neighbor].as_value())
    }

    /// Compute next generation state based on current one.
    pub fn next(&mut self) {
        for (i, &cell) in self.curr.iter().enumerate() {
            let alive_neighbors = self.count_alive_neighbors(i);
            self.next[i] = cell.next(alive_neighbors);
        }
        self.generation += 1;
        mem::swap(&mut self.curr, &mut self.next);
    }

    /// Updates current board's cell state to match provided coordinates and
    /// cell.
    pub fn set_cell(&mut self, coords: Coords, value: Cell) {
        let index = coords.to_index(self.cols, self.rows);
        self.curr[index] = value;
    }

    pub fn toggle_cell(&mut self, coords: Coords) {
        let index = coords.to_index(self.cols, self.rows);
        self.curr[index].toggle();
    }
}

/// Returns a vector containing current cell's neighbors indices.
///
/// ```
/// +-----+-----+-----+
/// | nw  |  n  | ne  |
/// +-----+-----+-----+
/// |  w  | cur |  e  |
/// +-----+-----+-----+
/// | sw  |  s  | se  |
/// +-----+-----+-----+
/// ```
///
fn get_neighbors_indices(index: usize, cols: u32, rows: u32) -> Vec<usize> {
    let coords = coords_from_index(index, cols);
    let mut indices = Vec::with_capacity(8);
    for y in coords.y - 1..=coords.y + 1 {
        for x in coords.x - 1..=coords.x + 1 {
            if coords.x == x && coords.y == y {
                continue;
            }
            indices.push(coords::coords_to_index(x, y, cols, rows))
        }
    }
    indices
}
