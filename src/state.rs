use crate::{cell::Cell, coords::Coords};

const RGB_BYTES: usize = 3;

pub struct State {
    pub(crate) iteration: usize,
    pub(crate) cols: usize,
    pub(crate) rows: usize,
    pub(crate) board: Vec<Cell>,
}

impl State {
    pub fn new(cols: usize, rows: usize) -> Self {
        State {
            iteration: 0,
            cols,
            rows,
            board: vec![Cell::Dead; cols * rows],
        }
    }

    fn count_alive_neighbors(&self, coords: Coords) -> usize {
        let mut count: usize = 0;
        for x in coords.x - 1..=coords.x + 1 {
            for y in coords.y - 1..=coords.y + 1 {
                if coords.x == x && coords.y == y {
                    continue;
                }
                let index = Coords::new(x, y).to_index(self.cols, self.rows);
                if self.board[index] == Cell::Alive {
                    count += 1;
                }
            }
        }
        count
    }

    pub fn next(&mut self) {
        let mut next = self.board.clone();

        for (i, cell) in self.board.iter().enumerate() {
            let coords = Coords::from_index(i, self.cols);
            let alive_neighbors = self.count_alive_neighbors(coords);
            next[i] = match cell {
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
        self.board = next;
    }

    pub fn set_cell(&mut self, coords: Coords, cell: Cell) {
        let index = coords.to_index(self.cols, self.rows);
        self.board[index] = cell;
    }

    pub fn to_ascii(&self) -> Vec<u8> {
        self.board.iter().map(|cell| cell.to_ascii()).collect()
    }

    pub fn to_rgb(&self) -> Vec<u8> {
        let mut buf = vec![0u8; self.board.len() * RGB_BYTES];
        let mut i = 0;
        for cell in &self.board {
            let (r, g, b) = cell.to_rgb();
            buf[i..i + 3].copy_from_slice(&[r, g, b]);
            i += 3;
        }
        buf
    }
}
