use std::mem;

use crate::{cell::Cell, coords::Coords};

pub struct State {
    pub(crate) iteration: usize,
    pub(crate) cols: usize,
    pub(crate) rows: usize,
    pub(crate) board: Vec<Cell>,
    neighbors: Vec<Vec<usize>>,
    next: Vec<Cell>,
}

impl State {
    pub fn new(cols: usize, rows: usize) -> Self {
        let mut neighbors = vec![vec![]; cols * rows];

        for index in 0..cols * rows {
            let coords = Coords::from_index(index, cols);
            for x in coords.x - 1..=coords.x + 1 {
                for y in coords.y - 1..=coords.y + 1 {
                    if coords.x == x && coords.y == y {
                        continue;
                    }
                    neighbors[index].push(Coords::new(x, y).to_index(cols, rows));
                }
            }
        }

        State {
            iteration: 0,
            cols,
            rows,
            board: vec![Cell::Dead; cols * rows],
            neighbors,
            next: vec![Cell::Dead; cols * rows],
        }
    }

    fn count_alive_neighbors(&self, index: usize) -> usize {
        let mut count: usize = 0;
        for &neighbor in &self.neighbors[index] {
            if self.board[neighbor] == Cell::Alive {
                count += 1;
            }
        }
        count
    }

    pub fn next(&mut self) {
        for (i, cell) in self.board.iter().enumerate() {
            let alive_neighbors = self.count_alive_neighbors(i);
            self.next[i] = match cell {
                Cell::Alive => {
                    if alive_neighbors < 2 || alive_neighbors > 3 {
                        Cell::Dead
                    } else {
                        Cell::Alive
                    }
                }
                Cell::Dead => {
                    if alive_neighbors == 3 {
                        Cell::Alive
                    } else {
                        Cell::Dead
                    }
                }
            };
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
