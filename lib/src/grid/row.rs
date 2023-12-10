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
    pub fn new(row: usize) -> Self {
        Self { row }
    }

    pub fn get_square(&self, index: usize) -> Square {
        Square::from(self.row, index)
    }

    pub fn row_index(&self) -> usize {
        self.row
    }

    pub fn iter_row() -> impl Iterator<Item = Row> {
        (0..GRID_HEIGHT).map(|i| Row::new(i))
    }
}

impl CellCollection for Row {
    fn get_coord(&self, col: usize) -> Coord {
        Coord::new(self.row, col)
    }

    fn iter(&self) -> std::ops::Range<usize> {
        0..GRID_HEIGHT
    }

    fn max(&self) -> usize {
        GRID_HEIGHT
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
    use crate::{
        grid::{cell_collection::CellCollection, constants::GRID_HEIGHT_RANGE},
        test::util::general_tests,
    };

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
        let grid = general_tests::filled_sudoku();

        for row_index in GRID_HEIGHT_RANGE {
            let row = grid.get_row(row_index);

            for c in row.iter() {
                let coord = row.get_coord(c);
                assert_eq!(coord.get_row(), row_index, "coord: {:?}", coord);
            }
        }
    }

    #[test]
    fn test_row_iter_cells() {
        let grid = general_tests::filled_sudoku();

        for row_index in GRID_HEIGHT_RANGE {
            let row = grid.get_row(row_index);

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
