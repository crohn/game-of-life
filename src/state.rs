use std::mem;

use crate::{cell::Cell, coords::Coords};

pub struct State {
    pub(crate) iteration: usize,
    pub(crate) cols: usize,
    pub(crate) rows: usize,
    pub(crate) board: Vec<Cell>,
    next: Vec<Cell>,
    neighbors: Vec<Vec<usize>>,
}

impl State {
    pub fn new(cols: usize, rows: usize) -> Self {
        let board = vec![Cell::Dead; cols * rows];
        let next = board.clone();

        let neighbors = (0..cols * rows)
            .map(|index| Coords::from_index(index, cols).get_neighbors_indices(cols, rows))
            .collect();

        State {
            board,
            cols,
            iteration: 0,
            neighbors,
            next,
            rows,
        }
    }

    fn count_alive_neighbors(&self, index: usize) -> usize {
        self.neighbors[index]
            .iter()
            .fold(0, |count, &neighbor| count + self.board[neighbor] as usize)
    }

    pub fn next(&mut self) {
        for (i, cell) in self.board.iter().enumerate() {
            let alive_neighbors = self.count_alive_neighbors(i);
            self.next[i] = cell.next(alive_neighbors);
        }
        self.iteration += 1;
        mem::swap(&mut self.board, &mut self.next);
    }

    pub fn set_cell(&mut self, coords: Coords, cell: Cell) {
        let index = coords.to_index(self.cols, self.rows);
        self.board[index] = cell;
    }

    pub fn to_ascii(&self) -> Vec<u8> {
        self.board.iter().map(|cell| cell.to_ascii()).collect()
    }
}
