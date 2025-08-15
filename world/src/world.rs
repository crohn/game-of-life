use std::mem;

/// During the simulation, each cell in [World]'s board is either in [Cell::Alive] or [Cell::Dead]
/// state.
///
/// By following the B/S (Birth/Survival) rules, cells can change state. Birth rules are applied to
/// [Cell::Dead], while Survival rules are applied to [Cell::Alive].
///
/// In the standard game (B3/S23), a dead cell becomes alive (Birth) if it has exactly 3 alive
/// neighbors, while a living cell Survives if it has exactly 2 or 3 neighbors.
#[derive(Clone)]
enum Cell {
    Alive,
    Dead,
}

impl Cell {
    /// Converts [Cell] into [u8] numerical value.
    ///
    /// The mapping `alive -> 1`, `dead -> 0` is convenient for neighbor count computation.
    pub fn as_value(&self) -> u8 {
        match self {
            Cell::Alive => 1,
            Cell::Dead => 0,
        }
    }

    /// Applies B/S rules to advance the state of current cell.
    pub fn next(&self, alive_count: u8) -> Self {
        match (self, alive_count) {
            (Cell::Alive, 2 | 3) => Cell::Alive, // S23
            (Cell::Alive, _) => Cell::Dead,
            (Cell::Dead, 3) => Cell::Alive,      // B3
            (Cell::Dead, _) => Cell::Dead,
        }
    }
}

/// Simple abstraction of 2D system coordinates.
///
/// Naming the dimensions { x, y } results in a more readable API in ambiguous contexts where `(i32,
/// i32)` can be too generic.
struct Coords {
    x: i32,
    y: i32,
}

/// Simple 2D space abstraction.
///
/// This abstraction enables for a better separation of concerns between [World], focussed on
/// game's logic, and [Grid], responsible for coordinates manipulation.
struct Grid {
    cols: u32,
    rows: u32,
}

impl Grid {
    pub fn new(cols: u32, rows: u32) -> Self {
        Grid { cols, rows }
    }

    /// Converts a [Coords] into a vector index ([usize]).
    ///
    /// The conversion wraps both `x` and `y` coordinates when they are past grid's bounds on both
    /// left and right. This means that, in an 10x10 grid:
    ///
    ///   - ( 0,  0) is top-left corner     -> index: 0  
    ///   - ( 9,  9) is bottom-right corner -> index: 99 
    ///   - (-1, -1) is bottom-right corner -> index: 99 (last index again)
    ///   - (10, 10) is top-left corner     -> index: 0  (first index again)
    pub fn coords_to_index(&self, coords: &Coords) -> usize {
        let x = coords.x.rem_euclid(self.cols as i32) as usize;
        let y = coords.y.rem_euclid(self.rows as i32) as usize;
        self.cols as usize * y + x
    }

    /// Converts a vector index ([usize]) to [Coords].
    ///
    /// Based on the number of grid's columns, indices are wrapped into next rows.
    pub fn index_to_coords(&self, index: usize) -> Coords {
        let x = (index as u32 % self.cols) as i32;
        let y = (index as u32 / self.cols) as i32;
        Coords { x, y }
    }

    /// Returns a vector containing current cell's neighbors indices.
    ///
    /// The neighbors are calculated following the steps:
    ///
    ///   1. convert index to 2D coordinates (x_curr, y_curr);
    ///   2. iterate the 3x3 square around (x_curr, y_curr);
    ///   3. skip the current cell;
    ///   4. for each neighbor (x', y'), convert their 2D coords to index.
    ///
    /// ```txt
    /// ASCII representation
    /// +--------------------------+----------------------+--------------------------+
    /// | (x_curr - 1, y_curr - 1) | (x_curr, y_curr - 1) | (x_curr + 1, y_curr - 1) |
    /// +--------------------------+----------------------+--------------------------+
    /// | (x_curr - 1, y_curr    ) | (x_curr, y_curr    ) | (x_curr + 1, y_curr    ) |
    /// +--------------------------+----------------------+--------------------------+
    /// | (x_curr - 1, y_curr + 1) | (x_curr, y_curr + 1) | (x_curr + 1, y_curr + 1) |
    /// +--------------------------+----------------------+--------------------------+
    /// ```
    pub fn calc_neighbors(&self, index: usize) -> Vec<usize> {
        let mut neighbors = Vec::with_capacity(CELL_NEIGHBORS);

        let curr = self.index_to_coords(index);

        for y in curr.y - 1..=curr.y + 1 {
            for x in curr.x - 1..=curr.x + 1 {
                if x == curr.x && y == curr.y { continue; }

                let neighbor = self.coords_to_index(&Coords { x, y });
                neighbors.push(neighbor);
            }
        }

        neighbors
    }
}

/// All the surrounding cells are considered neighbors: horizontal (2), vertical (2) and diagonal (4).
const CELL_NEIGHBORS: usize = 8;

/// Represents the state and rules of a Conway's Game of Life simulation.
///
/// This struct holds the grid of cells and the logic required to advance the simulation from one
/// generation to the next.
pub struct World {
    curr: Vec<Cell>,
    next: Vec<Cell>,
    neighbors: Vec<Vec<usize>>,
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
        let curr = vec![Cell::Dead; capacity];
        let next = curr.clone();

        let grid = Grid::new(cols, rows);
        let neighbors = (0..capacity)
            .map(|i| grid.calc_neighbors(i)).collect();

        World { curr, next, neighbors }
    }

    /// Advances the simulation by one generation.
    ///
    /// The advancement is performed by applying game's rules to every cell based on its neighbors,
    /// updating the board to its next state.
    pub fn next(&mut self) {
        for (i, cell) in self.curr.iter().enumerate() {
            let alive_neighs = self.count_alive_neighs(i);
            self.next[i] = cell.next(alive_neighs);
        }
        mem::swap(&mut self.curr, &mut self.next);
    }

    /// Returns the amount of [Cell::Alive] among neighbors.
    ///
    /// This computation is fast because [World] pre-computes neighbors indices upon creation.
    fn count_alive_neighs(&self, index: usize) -> u8 {
        self.neighbors[index].iter()
            .fold(0, |count, &neigh| count + self.curr[neigh].as_value())
    }
}

