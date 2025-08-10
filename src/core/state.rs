use crate::core::{Config, Coords};
use std::iter::Iterator;
use std::mem;

use super::cell::Cell;

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
    pub fn new(config: &Config) -> Self {
        let cols = config.cols;
        let rows = config.rows;
        //
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

    pub fn iter(&self) -> BoardIterator {
        BoardIterator {
            index: 0,
            state: &self,
        }
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

    /// Updates current board's cell state to match provided coordinates and value.
    pub fn set_cell(&mut self, x: i32, y: i32, value: Cell) {
        let index = coords_to_index(x, y, self.cols, self.rows);
        self.curr[index] = value;
    }

    /// Flips cell state.
    pub fn toggle_cell(&mut self, coords: &Coords) {
        let index = coords_to_index(coords.x, coords.y, self.cols, self.rows);
        self.curr[index].toggle();
    }

    /// Fills the entire board with dead cells.
    pub fn clear(&mut self) {
        self.curr.fill(Cell::Dead);
    }

    pub fn create_coords(&self, x: i32, y: i32) -> Coords {
        Coords {
            x: x.rem_euclid(self.cols as i32),
            y: y.rem_euclid(self.rows as i32),
        }
    }

    pub fn wrap_coords(&self, coords: &Coords) -> Coords {
        Coords {
            x: coords.x.rem_euclid(self.cols as i32),
            y: coords.y.rem_euclid(self.rows as i32),
        }
    }
}

pub struct BoardIterator<'a> {
    index: usize,
    state: &'a State,
}

impl<'a> Iterator for BoardIterator<'a> {
    type Item = (Coords, &'a Cell);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index == self.state.curr.len() {
            None
        } else {
            let coords = coords_from_index(self.index, self.state.cols);
            let cell = &self.state.curr[self.index];
            self.index += 1;
            Some((coords, cell))
        }
    }
}

/// Returns a vector containing current cell's neighbors indices.
///
/// ```txt
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
            indices.push(coords_to_index(x, y, cols, rows))
        }
    }
    indices
}

/// Converts Vector index into 2D coordinates (x, y).
pub fn coords_from_index(index: usize, cols: u32) -> Coords {
    let x = (index as u32 % cols) as i32;
    let y = (index as u32 / cols) as i32;
    Coords { x, y }
}

/// Converts 2D coordinates (x, y) into Vector index.
///
/// The conversion is wrapping, meaning that `x` values greater than `cols` or
/// less than zero are always converted into ranges `0..=cols`. The same applies
/// to `y` and `rows`.
pub fn coords_to_index(x: i32, y: i32, cols: u32, rows: u32) -> usize {
    let x = x.rem_euclid(cols as i32) as usize;
    let y = y.rem_euclid(rows as i32) as usize;

    cols as usize * y + x
}
