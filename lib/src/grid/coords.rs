use std::fmt::{self, Display, Formatter};

use crate::grid::constants::GRID_SIZE;

use super::constants::GRID_WIDTH;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Coord {
    index: usize,
}

impl Coord {
    /// Creates a new coord
    pub const fn new(row: usize, col: usize) -> Self {
        let index = super::format::get_index_from(row, col);
        Coord::from_index(index)
    }

    /// Creates a new coord from an index
    pub const fn from_index(index: usize) -> Self {
        debug_assert!(index < GRID_SIZE, "Index out of bounds");
        Coord { index: index }
    }

    /// Returns the index of the coord
    pub const fn get_row(self) -> usize {
        self.index / GRID_WIDTH
    }

    /// Returns the index of the coord
    pub const fn get_col(self) -> usize {
        self.index % GRID_WIDTH
    }

    /// Returns the index of the coord
    pub const fn get_index(self) -> usize {
        self.index
    }

    /// Returns the row and column of the coord
    pub const fn get_row_col(&self) -> (usize, usize) {
        (self.get_row(), self.get_col())
    }

    /// Returns a new coord with the row and column offset by the given amount
    pub const fn offset_row(self, offset: usize) -> Self {
        Coord::new(self.get_row() + offset, self.get_col())
    }

    /// Returns a new coord with the row and column offset by the given amount
    pub const fn offset_col(self, offset: usize) -> Self {
        Coord::new(self.get_row(), self.get_col() + offset)
    }

    /// Returns a new coord with the row and column offset by the given amount
    pub const fn offset(self, row_offset: usize, col_offset: usize) -> Self {
        Coord::new(self.get_row() + row_offset, self.get_col() + col_offset)
    }
}

impl Display for Coord {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "[{}, {}]", self.get_row(), self.get_col())
    }
}

#[cfg(test)]
mod test {
    use crate::grid::constants::GRID_HEIGHT_RANGE;

    use super::*;

    #[test]
    fn test_coord() {
        let coord = Coord::new(1, 2);
        assert_eq!(coord.get_row(), 1);
        assert_eq!(coord.get_col(), 2);
    }

    #[test]
    fn test_coord_from_index() {
        for row in GRID_HEIGHT_RANGE {
            for col in GRID_HEIGHT_RANGE {
                let index = Coord::new(row, col).get_index();
                let coord = Coord::from_index(index);
                assert_eq!(coord.get_row(), row);
                assert_eq!(coord.get_col(), col);
            }
        }
    }
}
