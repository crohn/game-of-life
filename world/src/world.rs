use std::mem;

use crate::grid::{Coords, Grid};

const CELL_DEAD: u8 = 0;
const CELL_ALIVE: u8 = 1;

/// During the simulation, each cell in [World]'s board is either in [Cell::Alive] or [Cell::Dead]
/// state.
///
/// By following the B/S (Birth/Survival) rules, cells can change state. Birth rules are applied to
/// [Cell::Dead], while Survival rules are applied to [Cell::Alive].
///
/// In the standard game (B3/S23), a dead cell becomes alive (Birth) if it has exactly 3 alive
/// neighbors, while a living cell Survives if it has exactly 2 or 3 neighbors.
#[derive(Debug, PartialEq)]
pub enum Cell {
    Alive,
    Dead,
}

impl From<u8> for Cell {
    fn from(value: u8) -> Self {
        match value {
            0 => Cell::Dead,
            1 => Cell::Alive,
            _ => unreachable!(),
        }
    }
}

impl From<Cell> for u8 {
    fn from(value: Cell) -> Self {
        match value {
            Cell::Dead => 0,
            Cell::Alive => 1,
        }
    }
}

/// Represents the state and rules of a Conway's Game of Life simulation.
///
/// This struct holds the grid of cells and the logic required to advance the simulation from one
/// generation to the next.
pub struct World {
    curr: Vec<u8>,
    next: Vec<u8>,
    neighbors: Vec<Vec<usize>>,
    grid: Grid,
}

impl World {
    /// Creates an empty `World`.
    ///
    /// The world is initialized with all cells in [Cell::Dead] state.
    ///
    /// For every cell, it also pre-computes all the neighbors' indices, to speed up the
    /// simulation.
    pub fn new(cols: u32, rows: u32) -> Self {
        let capacity = (cols * rows) as usize;
        let curr = vec![CELL_DEAD; capacity];
        let next = curr.clone();

        let grid = Grid::new(cols, rows);
        let neighbors = (0..capacity)
            .map(|i| grid.calc_neighbors(i)).collect();

        World { curr, next, neighbors, grid }
    }

    /// Advances the simulation by one generation.
    ///
    /// The advancement is performed by applying game's rules to every cell based on its neighbors,
    /// updating the board to its next state.
    pub fn next(&mut self) {
        for (i, &cell) in self.curr.iter().enumerate() {
            let alive_neighs = self.count_alive_neighs(i);
            // Apply B/S rules to advance current cell's state.
            self.next[i] = match (cell, alive_neighs) {
                (CELL_ALIVE, 2 | 3) => CELL_ALIVE, // S23
                (CELL_ALIVE, _)     => CELL_DEAD,
                (CELL_DEAD, 3)      => CELL_ALIVE, // B3
                (CELL_DEAD, _)      => CELL_DEAD,
                _ => unreachable!(),
            }
        }
        mem::swap(&mut self.curr, &mut self.next);
    }

    /// Updates current board's cell value.
    ///
    /// Internally, [Coords] is converted to [usize], and [Cell] is converted to [u8], because the
    /// board is a `Vec<u8>`.
    pub fn set_cell(&mut self, coords: &Coords, cell: Cell) {
        let index = self.grid.coords_to_index(coords);
        self.curr[index] = cell.into();
    }

    pub fn get_cell(&self, coords: &Coords) -> Cell {
        let index = self.grid.coords_to_index(coords);
        self.curr[index].into()
    }

    /// Returns the amount of alive cells among neighbors.
    ///
    /// This computation is fast because [World] pre-computes neighbors indices upon creation.
    fn count_alive_neighs(&self, index: usize) -> u8 {
        self.neighbors[index].iter().map(|&neigh| self.curr[neigh]).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn world_init() {
        let cols = 4;
        let rows = 8;

        let world = World::new(cols, rows);

        assert_eq!(world.curr.len(), (cols * rows) as usize);
        assert!(world.curr.iter().all(|&cell| cell == 0));
        assert_eq!(world.curr, world.next);
        assert_eq!(world.neighbors.len(), (cols * rows) as usize);
    }

    #[test]
    fn world_can_mutate_cell() {
        let mut world = World::new(4, 4);
        assert_eq!(world.curr[0], 0);

        world.set_cell(&Coords { x: 4, y: 4 }, Cell::Alive);
        assert_eq!(world.curr[0], 1);
    }

    #[test]
    fn world_can_read_cell() {
        let mut world = World::new(4, 4);

        world.set_cell(&Coords { x: 4, y: 4 }, Cell::Alive);
        let value = world.get_cell(&Coords { x: 4, y: 4});
        assert_eq!(value, Cell::Alive);
    }

    //   glider
    //  -------
    //  0 0 1 0 
    //  0 0 0 1
    //  0 1 1 1
    //  0 0 0 0
    //
    #[test]
    fn world_counts_alive_neighbors() {
        let mut world = World::new(4, 4);

        world.set_cell(&Coords { x: 2, y: 0 }, Cell::Alive);
        world.set_cell(&Coords { x: 3, y: 1 }, Cell::Alive);
        world.set_cell(&Coords { x: 1, y: 2 }, Cell::Alive);
        world.set_cell(&Coords { x: 2, y: 2 }, Cell::Alive);
        world.set_cell(&Coords { x: 3, y: 2 }, Cell::Alive);

        let cases = [
            (0, 1), (1, 1), (2, 1), (3, 2),
            (4, 3), (5, 3), (6, 5), (7, 3),
            (8, 3), (9, 1), (10, 3), (11, 2),
            (12, 2), (13, 3), (14, 4), (15, 3),
        ];

        for (index, neighbor_count) in cases {
            assert_eq!(world.count_alive_neighs(index), neighbor_count);
        }
    }

    //        t0             t1            t2 
    //      -----          -----          -----
    //    0 0 0 0 0      0 0 0 0 0      0 0 0 0 0
    //    0 0 1 0 0      0 0 0 0 0      0 0 1 0 0
    //    0 0 1 0 0  =>  0 1 1 1 0  =>  0 0 1 0 0
    //    0 0 1 0 0      0 0 0 0 0      0 0 1 0 0
    //    0 0 0 0 0      0 0 0 0 0      0 0 0 0 0
    //
    #[test]
    fn it_works() {
        let mut world = World::new(5, 5);

        // blinker pattern, see ASCII diagram above
        world.set_cell(&Coords { x: 2, y: 1}, Cell::Alive);
        world.set_cell(&Coords { x: 2, y: 2}, Cell::Alive);
        world.set_cell(&Coords { x: 2, y: 3}, Cell::Alive);

        // check (2,2) coords neighborhood because of how blinker pattern behaves

        assert_eq!(world.get_cell(&Coords { x: 1, y: 1}), Cell::Dead);
        assert_eq!(world.get_cell(&Coords { x: 2, y: 1}), Cell::Alive);
        assert_eq!(world.get_cell(&Coords { x: 3, y: 1}), Cell::Dead);
        assert_eq!(world.get_cell(&Coords { x: 1, y: 2}), Cell::Dead);
        assert_eq!(world.get_cell(&Coords { x: 2, y: 2}), Cell::Alive);
        assert_eq!(world.get_cell(&Coords { x: 3, y: 2}), Cell::Dead);
        assert_eq!(world.get_cell(&Coords { x: 1, y: 3}), Cell::Dead);
        assert_eq!(world.get_cell(&Coords { x: 2, y: 3}), Cell::Alive);
        assert_eq!(world.get_cell(&Coords { x: 3, y: 3}), Cell::Dead);

        world.next();

        assert_eq!(world.get_cell(&Coords { x: 1, y: 1}), Cell::Dead);
        assert_eq!(world.get_cell(&Coords { x: 2, y: 1}), Cell::Dead);
        assert_eq!(world.get_cell(&Coords { x: 3, y: 1}), Cell::Dead);
        assert_eq!(world.get_cell(&Coords { x: 1, y: 2}), Cell::Alive);
        assert_eq!(world.get_cell(&Coords { x: 2, y: 2}), Cell::Alive);
        assert_eq!(world.get_cell(&Coords { x: 3, y: 2}), Cell::Alive);
        assert_eq!(world.get_cell(&Coords { x: 1, y: 3}), Cell::Dead);
        assert_eq!(world.get_cell(&Coords { x: 2, y: 3}), Cell::Dead);
        assert_eq!(world.get_cell(&Coords { x: 3, y: 3}), Cell::Dead);

        world.next();

        assert_eq!(world.get_cell(&Coords { x: 1, y: 1}), Cell::Dead);
        assert_eq!(world.get_cell(&Coords { x: 2, y: 1}), Cell::Alive);
        assert_eq!(world.get_cell(&Coords { x: 3, y: 1}), Cell::Dead);
        assert_eq!(world.get_cell(&Coords { x: 1, y: 2}), Cell::Dead);
        assert_eq!(world.get_cell(&Coords { x: 2, y: 2}), Cell::Alive);
        assert_eq!(world.get_cell(&Coords { x: 3, y: 2}), Cell::Dead);
        assert_eq!(world.get_cell(&Coords { x: 1, y: 3}), Cell::Dead);
        assert_eq!(world.get_cell(&Coords { x: 2, y: 3}), Cell::Alive);
        assert_eq!(world.get_cell(&Coords { x: 3, y: 3}), Cell::Dead);
    }
}

