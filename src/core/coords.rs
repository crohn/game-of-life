//! Simple abstraction of 2D system coordinates.
//!
//! Naming the dimensions {x, y} results in a more readable API than using
//! indices of a tuple (x, y).

#[derive(Debug, Eq, Hash, PartialEq)]
pub struct Coords {
    pub x: i32,
    pub y: i32,
}

impl Coords {
    pub fn rescale(&self, scale: u32) -> Self {
        Coords {
            x: self.x / scale as i32,
            y: self.y / scale as i32,
        }
    }

    pub fn rescale_mut(&mut self, scale: u32) {
        self.x /= scale as i32;
        self.y /= scale as i32;
    }
}

// Allow `(i32, i32) -> Coords` conversion.
//
// Sometimes `Coords { x: 0, y: 1 }` can be verbose to write, so when the
// intent is clear, using arguments like `(0, 1)` can come handy, eg.
//
// ```txt
// selection.toggle((0, 0))
// ```
//
// The implementation of `From` trait opens several possibilites. For example,
// we get a free implementation of `.into()` for `(i32, i32)` tuple, that allows
// to write generic functions using the `Into<Coords>` trait bound, which
// implies that the same function can be invoked with either `(i32, i32)` or
// `Coords { x, y }` arguments.
//
// ```txt
// fn toggle<C: Into<Coords>(coords: C) { }
//
// selection.toggle((0, 0))
// selection.toggle(Coords { x: 0, y: 0 })
// ```
impl From<(i32, i32)> for Coords {
    fn from(tuple: (i32, i32)) -> Self {
        Coords {
            x: tuple.0,
            y: tuple.1,
        }
    }
}
