use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Coord {
    pub row: usize,
    pub col: usize,
}

impl Coord {
    /// Creates a new coord
    pub fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }

    /// Creates a new coord from an index
    pub fn from_index(index: usize) -> Self {
        super::format::to_row_col(index)
    }

    /// Returns the index of the coord
    pub fn get_row(self) -> usize {
        self.row
    }

    /// Returns the index of the coord
    pub fn get_col(self) -> usize {
        self.col
    }

    /// Returns the index of the coord
    pub fn get_index(self) -> usize {
        super::format::get_index(self)
    }

    /// Returns the row and column of the coord
    pub fn get_row_col(&self) -> (usize, usize) {
        (self.row, self.col)
    }
}

impl Display for Coord {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "[{}, {}]", self.row, self.col)
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
