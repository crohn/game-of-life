/// All the surrounding cells are considered neighbors: horizontal (2), vertical (2) and diagonal (4).
const CELL_NEIGHBORS: usize = 8;

/// Simple abstraction of 2D system coordinates.
///
/// Naming the dimensions { x, y } results in a more readable API in ambiguous contexts where `(i32,
/// i32)` can be too generic.
#[derive(Debug, PartialEq)]
pub struct Coords {
    pub x: i32,
    pub y: i32,
}

/// Simple 2D space abstraction.
///
/// This abstraction enables for a better separation of concerns between [World], focussed on
/// game's logic, and [Grid], responsible for coordinates manipulation.
#[derive(Debug)]
pub struct Grid {
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

        for dy in -1..=1 {
            for dx in -1..=1 {
                if dx == 0 && dy == 0 { continue; }

                let neighbor = self.coords_to_index(&Coords {
                   x: curr.x + dx, y: curr.y + dy,
                });
                neighbors.push(neighbor)
            }
        }

        neighbors
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grid_maps_coords_to_vec_index_wrapping_beyond_bounds() {
        let grid = Grid::new(3, 3);

        let cases = [
            (Coords { x:  0, y:  0 }, 0),
            (Coords { x:  1, y:  0 }, 1),
            (Coords { x:  2, y:  0 }, 2),
            (Coords { x:  0, y:  1 }, 3),
            (Coords { x:  1, y:  1 }, 4),
            (Coords { x:  2, y:  1 }, 5),
            (Coords { x:  0, y:  2 }, 6),
            (Coords { x:  1, y:  2 }, 7),
            (Coords { x:  2, y:  2 }, 8),
        ];

        for (coords, expected_index) in cases {
            assert_eq!(grid.coords_to_index(&coords), expected_index);
        }
    }

    ///         grid  
    ///         ----- 
    /// (3,3)-> * * * 
    ///         * * * 
    ///         * * * <-(-1,-1)
    ///
    #[test]
    fn grid_coords_mapping_wraps() {
        let grid = Grid::new(3, 3);
        let cases = [
            (Coords { x:  3, y:  3 }, 0),
            (Coords { x: -1, y: -1 }, 8),
        ];

        for (coords, expected_index) in cases {
            assert_eq!(grid.coords_to_index(&coords), expected_index);
        }
    }

    #[test]
    fn grid_maps_index_to_coords() {
        let grid = Grid::new(3, 3);
        let cases = [
            (0, Coords { x: 0, y: 0 }),
            (1, Coords { x: 1, y: 0 }),
            (2, Coords { x: 2, y: 0 }),
            (3, Coords { x: 0, y: 1 }),
            (4, Coords { x: 1, y: 1 }),
            (5, Coords { x: 2, y: 1 }),
            (6, Coords { x: 0, y: 2 }),
            (7, Coords { x: 1, y: 2 }),
            (8, Coords { x: 2, y: 2 }),
        ];

        for (index, expected_coords) in cases {
            assert_eq!(grid.index_to_coords(index), expected_coords);
        }
    }

    #[test]
    fn grid_index_mapping_wraps() {
        let grid = Grid::new(3, 3);
        let cases = [
            ( 9, Coords { x: 0, y: 3 }, 0),
            (10, Coords { x: 1, y: 3 }, 1),
            (11, Coords { x: 2, y: 3 }, 2),
            (12, Coords { x: 0, y: 4 }, 3),
            (13, Coords { x: 1, y: 4 }, 4),
            (14, Coords { x: 2, y: 4 }, 5),
        ];

        for (index, expected_coords, wrapped_index) in cases {
            let coords = grid.index_to_coords(index);

            assert_eq!(coords, expected_coords);
            assert_eq!(grid.coords_to_index(&coords), wrapped_index);
        }
    }

    ///     grid            neighbors(0)
    ///     ----            ------------
    ///    c 6 * 4     (1   2   3  4  6  7  8  9)
    ///    8 9 * 7 => [15, 12, 13, 3, 1, 7, 4, 5]
    ///    * * * *
    ///    2 3 * 1
    #[test]
    fn grid_computes_neighbors_by_index() {
        let grid = Grid::new(4, 4);
        let cases = [
            (0, vec![15, 12, 13, 3, 1, 7, 4, 5]),
            (1, vec![12, 13, 14, 0, 2, 4, 5, 6]),
            (2, vec![13, 14, 15, 1, 3, 5, 6, 7]),
            (3, vec![14, 15, 12, 2, 0, 6, 7, 4]),
            (4, vec![3, 0, 1, 7, 5, 11, 8, 9]),
            (5, vec![0, 1, 2, 4, 6, 8, 9, 10]),
            (6, vec![1, 2, 3, 5, 7, 9, 10, 11]),
            (7, vec![2, 3, 0, 6, 4, 10, 11, 8]),
            (8, vec![7, 4, 5, 11, 9, 15, 12, 13]),
            (9, vec![4, 5, 6, 8, 10, 12, 13, 14]),
            (10, vec![5, 6, 7, 9, 11, 13, 14, 15]),
            (11, vec![6, 7, 4, 10, 8, 14, 15, 12]),
            (12, vec![11, 8, 9, 15, 13, 3, 0, 1]),
            (13, vec![8, 9, 10, 12, 14, 0, 1, 2]),
            (14, vec![9, 10, 11, 13, 15, 1, 2, 3]),
            (15, vec![10, 11, 8, 14, 12, 2, 3, 0]),
        ];

        for (index, expected_neighbors) in cases {
            assert_eq!(grid.calc_neighbors(index), expected_neighbors);
        }

    }
}

