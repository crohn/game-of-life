//! Cell selection allows players to interact with the board. It allows for cell
//! inspection and multi-cell state toggle.
//!
//! Cell selection offers the following features:
//!
//! - When clicking on a cell with Left mouse button, we either create a new
//! selection or move the existing selection to the target coordinates.
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
    ///   1. calculate center
    ///   2. normalize coords to center
    ///   3. apply rotation
    ///      clockwise (x',y') = (y, -x)
    ///      counterclockwise (x',y') = (-y, x)
    ///   4. move back to center
    fn rotate(&mut self, clockwise: bool) {
        if let Some(center) = self.calc_center() {
            self.coords = self
                .coords
                .drain()
                .into_iter()
                .map(|coords| {
                    let x = coords.x - center.x;
                    let y = coords.y - center.y;

                    let (x_rot, y_rot) = if clockwise { (-y, x) } else { (y, -x) };

                    Coords {
                        x: x_rot + center.x,
                        y: y_rot + center.y,
                    }
                })
                .collect()
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
    /// This computation is essential for re-centering and rotation.
    ///
    /// The center of a selection is calculated collecting x min/max and y
    /// min/max boundaries and calculating their middle points.
    fn calc_center(&self) -> Option<Coords> {
        self.coords
            .iter()
            .fold(None, |bounds, coords| match bounds {
                None => Some((coords.x, coords.x, coords.y, coords.y)),
                Some((x_min, x_max, y_min, y_max)) => Some((
                    coords.x.min(x_min),
                    coords.x.max(x_max),
                    coords.y.min(y_min),
                    coords.y.max(y_max),
                )),
            })
            .map(|(x_min, x_max, y_min, y_max)| Coords {
                x: (x_min + x_max) / 2,
                y: (y_min + y_max) / 2,
            })
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
