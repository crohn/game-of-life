#[derive(Eq, Hash, PartialEq)]
pub struct Coords {
    pub x: i32,
    pub y: i32,
}

impl Coords {
    /// Wrapper method around [coords_to_index].
    pub fn to_index(&self, cols: u32, rows: u32) -> usize {
        coords_to_index(self.x, self.y, cols, rows)
    }
}

/// Converts Vector index into 2D coordinates (x, y).
pub fn coords_from_index(index: usize, cols: u32) -> Coords {
    let x = (index % cols as usize) as i32;
    let y = (index / cols as usize) as i32;
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
