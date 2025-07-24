const CELL_NEIGHBORS: usize = 8;

fn wrap(z: i32, bound: usize) -> usize {
    if z >= 0 {
        z as usize % bound
    } else {
        bound - (z.abs() as usize % bound)
    }
}

pub fn coords_to_index(x: i32, y: i32, cols: usize, rows: usize) -> usize {
    let x = wrap(x, cols);
    let y = wrap(y, rows);
    cols * y + x
}

pub struct Coords {
    pub(crate) x: i32,
    pub(crate) y: i32,
}

impl Coords {
    pub fn new(x: i32, y: i32) -> Self {
        Coords { x, y }
    }

    pub fn from_index(index: usize, cols: usize) -> Self {
        let x = (index % cols) as i32;
        let y = (index / cols) as i32;
        Coords { x, y }
    }

    pub fn get_neighbors_indices(&self, cols: usize, rows: usize) -> Vec<usize> {
        let mut indices = Vec::with_capacity(CELL_NEIGHBORS);
        for x in self.x - 1..=self.x + 1 {
            for y in self.y - 1..=self.y + 1 {
                if self.x == x && self.y == y {
                    continue;
                }
                indices.push(coords_to_index(x, y, cols, rows))
            }
        }
        indices
    }

    pub fn to_index(&self, cols: usize, rows: usize) -> usize {
        coords_to_index(self.x, self.y, cols, rows)
    }
}
