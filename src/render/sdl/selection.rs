//! Cell selection allows players to interact with the board. It allows for cell
//! inspection and multi-cell state toggle.
//!
//! Cell selection offers the following features:
//!
//! - When clicking on a cell with Left mouse button, if a selection exists, we
//! move it to the target coordinates.
//!
//! - When Shift+clicking on a cell with Left mouse button, the existing
//! selection is modified to include/remove the target cell, depending on
//! whether the target cell was already included or not.
//!
//! - When pressing directional arrows or `h,j,k,l` keys, the existing selection
//! is moved up/right/down/left by one position, according to the pressed key.
//!
//! - When pressing `r,R` keys, the existing selection is rotated 90
//! clockwise/counterclockwise respectively.
//!
//! - When pressing `x` key, the existing selection is cleared.
//!
//! - When pressing `Space` key, the selected cells are toggled, meaning that
//! alive cells become dead and viceversa.

use std::{collections::HashSet, iter::Iterator};

use crate::core::Coords;

struct Bounds {
    x_min: i32,
    x_max: i32,
    y_min: i32,
    y_max: i32,
}

/// Calculate x and y bounds of a selection.
///
/// The bounds are used for those operations that require manipulating the whole
/// selection, like re-center and rotate.
#[rustfmt::skip]
fn calc_bounds(selection: &HashSet<Coords>) -> Option<Bounds> {
    selection
        .iter()
        .fold(None, |bounds, &Coords { x, y }| match bounds {
            None => Some(Bounds { x_min: x, x_max: x, y_min: y, y_max: y, }),
            Some(bounds) => Some(Bounds {
                x_min: x.min(bounds.x_min),
                x_max: x.max(bounds.x_max),
                y_min: y.min(bounds.y_min),
                y_max: y.max(bounds.y_max),
            }),
        })
}

#[derive(Default, Debug)]
pub struct Selection {
    coords: HashSet<Coords>,
}

impl Selection {
    /// Clear the existing selection.
    pub fn clear(&mut self) {
        self.coords.clear();
    }

    /// Returns `true` when the existing selection contains the (x,y) coordinates.
    pub fn contains<C: Into<Coords>>(&self, coords: C) -> bool {
        self.coords.contains(&coords.into())
    }

    /// Returns `true` when the existing selection is empty.
    pub fn is_empty(&self) -> bool {
        self.coords.is_empty()
    }

    /// Returns an iterator over existing selection coordinates. Element ordering
    /// ing is never guaranteed because the underlying data structure is a set.
    pub fn iter(&self) -> impl Iterator<Item = &Coords> {
        self.coords.iter()
    }

    /// Returns the number of selected cells.
    pub fn len(&self) -> usize {
        self.coords.len()
    }

    /// Move every selected coords by (dx,dy).
    ///
    /// _Example: move existing selection by `(1,0)`:_
    /// ```txt
    ///   (0,0) |       |      |
    ///         | (1,1) |      |
    ///         | (1,2) |      |
    /// ```
    ///
    /// after move
    /// ```txt
    ///   (1,0) |       |      |
    ///         | (2,1) |      |
    ///         | (2,2) |      |
    /// ```
    pub fn move_by<C: Into<Coords>>(&mut self, delta: C) {
        let delta = delta.into();
        self.coords = self
            .coords
            .drain()
            .into_iter()
            .map(|mut coords| {
                coords.x += delta.x;
                coords.y += delta.y;
                coords
            })
            .collect()
    }

    /// Re-center the existing selection (if any) to (x',y') coordinates.
    ///
    /// _Example: move existing selection centered at `(1,1)` to `(4,1)`:_
    /// ```txt
    ///   (0,0) |       |      |
    ///         | (1,1) |      |
    ///         | (1,2) |      |
    /// ```
    ///
    /// after re-center
    /// ```txt
    ///   (3,0) |       |      |
    ///         | (4,1) |      |
    ///         | (4,2) |      |
    /// ```
    pub fn recenter_at<C: Into<Coords>>(&mut self, coords: C) {
        let new_center: Coords = coords.into();
        // Center is None when selection is empty
        if let Some(center) = self.calc_center() {
            self.move_by((new_center.x - center.x, new_center.y - center.y));
        }
    }

    /// Rotate existing selection around its center by 90 degrees, either
    /// clockwise or counterclockwise.
    ///
    /// Rotation is computed as follows:
    ///   1. calculate bounds
    ///   2. normalize coords to top-left (x_min, y_min)
    ///   3. apply rotation
    ///      clockwise (x',y') = (y, -x)
    ///      counterclockwise (x',y') = (-y, x)
    ///   4. normalize coords to new top-left after rotation
    ///   5. move back to old top-left
    fn rotate(&mut self, clockwise: bool) {
        if let Some(bounds) = calc_bounds(&self.coords) {
            #[rustfmt::skip]
            let rotated: HashSet<Coords> = self
                .coords
                .iter()
                .map(|coords| Coords {     // (2) normalize top-left
                    x: coords.x - bounds.x_min,
                    y: coords.y - bounds.y_min,
                })
                .map(|normalized| Coords { // (3) rotate
                    x: if clockwise { normalized.y } else { -normalized.y },
                    y: if clockwise { -normalized.x } else { normalized.x },
                })
                .collect();

            if let Some(new_bounds) = calc_bounds(&rotated) {
                self.coords = rotated
                    .iter()
                    .map(|coords| Coords {
                        // (4,5) normalize new top-left + move back
                        x: coords.x + bounds.x_min - new_bounds.x_min,
                        y: coords.y + bounds.y_min - new_bounds.y_min,
                    })
                    .collect();
            }
        }
    }

    /// Rotate existing selection around its center by 90 degrees counterclockwise.
    pub fn rotate_left(&mut self) {
        self.rotate(false);
    }

    /// Rotate existing selection around its center by 90 degrees clockwise.
    pub fn rotate_right(&mut self) {
        self.rotate(true);
    }

    /// Adds/removes (x,y) to/from existing selection according to whether it is
    /// already included or not.
    pub fn toggle<C: Into<Coords>>(&mut self, coords: C) {
        let coords: Coords = coords.into();
        if !self.coords.remove(&coords) {
            self.coords.insert(coords);
        }
    }

    /// Compute the center of current selection.
    ///
    /// This computation is used for re-centering the selection.
    ///
    /// The center of a selection is calculated collecting x and y bounds and
    /// then calculating their middle points.
    #[rustfmt::skip]
    fn calc_center(&self) -> Option<Coords> {
        calc_bounds(&self.coords).map(
            |Bounds { x_min, x_max, y_min, y_max, }| Coords {
                x: (x_min + x_max) / 2,
                y: (y_min + y_max) / 2,
            },
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn toggles_selected_coords() {
        let mut selection = Selection::default();
        selection.toggle((0, 0));
        assert!(selection.contains((0, 0)));

        selection.toggle((0, 0));
        assert!(selection.is_empty());
    }

    #[test]
    fn moves_selection_by_delta() {
        let mut selection = Selection::default();
        selection.toggle((0, 0));
        selection.toggle((0, 1));
        selection.move_by((4, 1));
        assert_eq!(selection.len(), 2);
        assert!(selection.contains((4, 1)));
        assert!(selection.contains((4, 2)));
    }

    #[test]
    fn recenters_selection_to_specified_coords() {
        let mut selection = Selection::default();
        selection.toggle((0, 0));
        selection.recenter_at((4, 5));
        assert_eq!(selection.len(), 1);
        assert!(selection.contains((4, 5)));

        let mut selection = Selection::default();
        selection.toggle((0, 0));
        selection.toggle((1, 1));
        selection.toggle((1, 2));
        selection.recenter_at((10, 1));

        dbg!(&selection);

        assert_eq!(selection.len(), 3);
        assert!(selection.contains((10, 0)));
        assert!(selection.contains((11, 1)));
        assert!(selection.contains((11, 2)));
    }

    #[test]
    fn does_nothing_when_selection_is_empty() {
        let mut selection = Selection::default();
        assert!(selection.is_empty());

        selection.recenter_at((4, 5));
        assert!(selection.is_empty());
    }

    #[test]
    fn rotates_selection_around_center() {
        let mut selection = Selection::default();
        selection.toggle((1, 1));
        selection.toggle((2, 2));
        selection.toggle((3, 3));

        selection.rotate_right();
        assert_eq!(selection.len(), 3);
        assert!(selection.contains((3, 1)));
        assert!(selection.contains((2, 2)));
        assert!(selection.contains((1, 3)));

        selection.rotate_left();
        assert_eq!(selection.len(), 3);
        assert!(selection.contains((1, 1)));
        assert!(selection.contains((2, 2)));
        assert!(selection.contains((3, 3)));
    }
}
