use std::fmt::Display;

use super::{
    cell_collection::CellCollection, constants::GRID_HEIGHT, coords::Coord, square::Square,
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Row {
    //The row index
    row: usize,
}

impl Row {
    pub const fn new(row: usize) -> Self {
        Self { row }
    }

    pub fn get_square(&self, index: usize) -> Square {
        Square::from(self.row, index)
    }

    pub fn row_index(&self) -> usize {
        self.row
    }

    pub fn iter_row() -> impl Iterator<Item = Row> {
        (0..Row::max()).map(|i| Row::new(i))
    }

    pub fn max() -> usize {
        GRID_HEIGHT
    }
}

impl From<usize> for Row {
    fn from(row: usize) -> Self {
        Self::new(row)
    }
}

impl CellCollection for Row {
    fn get_coord(&self, col: usize) -> Coord {
        Coord::new(self.row, col)
    }

    fn iter(&self) -> std::ops::Range<usize> {
        0..Row::max()
    }

    fn max(&self) -> usize {
        Row::max()
    }
}

impl Display for Row {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[Row {}]", self.row)
    }
}

#[cfg(test)]
mod test {
    use super::Row;
    use crate::grid::{cell_collection::CellCollection, constants::GRID_HEIGHT_RANGE};

    #[test]
    fn test_coords() {
        for row_index in GRID_HEIGHT_RANGE {
            let row = Row::new(row_index);

            for col_index in GRID_HEIGHT_RANGE {
                let coord = row.get_coord(col_index);
                assert_eq!(coord.get_row(), row_index);
                assert_eq!(coord.get_col(), col_index);
            }
        }
    }

    #[test]
    fn test_row_iter_coords() {
        for row_index in GRID_HEIGHT_RANGE {
            let row = Row::new(row_index);

            for c in row.iter() {
                let coord = row.get_coord(c);
                assert_eq!(coord.get_row(), row_index, "coord: {:?}", coord);
            }
        }
    }

    #[test]
    fn test_row_iter_cells() {
        for row_index in GRID_HEIGHT_RANGE {
            let row = Row::new(row_index);

            for index in row.iter() {
                let coord = row.get_coord(index);
                assert_eq!(
                    coord.get_row(),
                    row_index,
                    "index: {}, coord: {:?}",
                    index,
                    coord
                );
            }
        }
    }
}
