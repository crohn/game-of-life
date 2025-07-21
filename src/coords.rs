fn wrap(z: i32, bound: usize) -> usize {
    if z >= 0 {
        z as usize % bound
    } else {
        bound - (z.abs() as usize % bound)
    }
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

    pub fn to_index(&self, cols: usize, rows: usize) -> usize {
        let y = wrap(self.y, rows);
        let x = wrap(self.x, cols);

        cols * y + x
    }
}
