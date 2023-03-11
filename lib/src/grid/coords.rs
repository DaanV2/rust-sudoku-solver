use std::fmt::{self, Display, Formatter};

use super::format::{get_index, to_row_col};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Coord {
    pub row: usize,
    pub col: usize,
}

impl Coord {
    pub fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }

    pub fn from_index(index: usize) -> Self {
        to_row_col(index)
    }

    pub fn get_row(&self) -> usize {
        self.row
    }

    pub fn get_col(&self) -> usize {
        self.col
    }

    pub fn get_index(&self) -> usize {
        get_index(self)
    }

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
                let index = get_index(&Coord::new(row, col));
                let coord = Coord::from_index(index);
                assert_eq!(coord.get_row(), row);
                assert_eq!(coord.get_col(), col);
            }
        }
    }
}
