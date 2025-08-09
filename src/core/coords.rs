//!
//!

#[derive(Debug, Eq, Hash, PartialEq)]
pub struct Coords {
    pub x: i32,
    pub y: i32,
}

impl From<(i32, i32)> for Coords {
    fn from(tuple: (i32, i32)) -> Self {
        Coords {
            x: tuple.0,
            y: tuple.1,
        }
    }
}
