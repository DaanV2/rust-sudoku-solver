use super::{
    cell_collection::CellCollection, constants::GRID_WIDTH, coords::Coord, square::Square,
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Column {
    //The column index
    col: usize,
}

impl Column {
    pub fn new(col: usize) -> Self {
        Self { col }
    }

    pub fn get_square(&self, index: usize) -> Square {
        Square::from(index, self.col)
    }

    pub fn col_index(&self) -> usize {
        self.col
    }

    pub fn iter_col() -> impl Iterator<Item = Column> {
        (0..GRID_WIDTH).map(|i| Column::new(i))
    }
}

impl CellCollection for Column {
    fn get_coord(&self, index: usize) -> Coord {
        Coord::new(index, self.col)
    }

    fn iter(&self) -> std::ops::Range<usize> {
        0..GRID_WIDTH
    }

    fn max(&self) -> usize {
        GRID_WIDTH
    }
}

#[cfg(test)]
mod test {
    use super::Column;
    use crate::{
        grid::{
            cell_collection::CellCollection,
            constants::{GRID_HEIGHT_RANGE, GRID_WIDTH_RANGE},
        },
        test::util::general_tests,
    };

    #[test]
    fn test_coords() {
        for col_index in GRID_HEIGHT_RANGE {
            let row = Column::new(col_index);

            for row_index in GRID_HEIGHT_RANGE {
                let coord = row.get_coord(row_index);
                assert_eq!(coord.get_row(), row_index);
                assert_eq!(coord.get_col(), col_index);
            }
        }
    }

    #[test]
    fn test_column_iter_coords() {
        let grid = general_tests::filled_sudoku();

        for c in GRID_WIDTH_RANGE {
            let column = grid.get_column(c);

            for index in column.iter() {
                let coord = column.get_coord(index);
                assert_eq!(coord.get_col(), c);
            }
        }
    }

    #[test]
    fn test_column_iter_cells() {
        let grid = general_tests::filled_sudoku();

        for c in GRID_WIDTH_RANGE {
            let column = grid.get_column(c);

            for index in column.iter() {
                let coord = column.get_coord(index);
                assert_eq!(coord.get_col(), c);
            }
        }
    }
}
