use super::{cell_collection::CellCollection, coords::Coord};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Square {
    /// The row index the square starts at
    pub row: usize,
    /// The column index the square starts at
    pub col: usize,
}

impl Square {
    /// Creates a new square from the given row and column
    pub fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }

    /// Creates a new square from the given row and column
    pub fn from(row: usize, col: usize) -> Self {
        let row_offset = row - row % 3;
        let col_offset = col - col % 3;

        Square::new(row_offset, col_offset)
    }

    /// Gets the coord at the row and column
    pub fn get_coord_at(&self, row: usize, col: usize) -> Coord {
        Coord::new(self.row + row, self.col + col)
    }

    /// Returns true if the column is within the square
    pub fn is_column_in_square(&self, col: usize) -> bool {
        col >= self.col && col < self.col + 3
    }

    /// Returns true if the row is within the square
    pub fn is_row_in_square(&self, row: usize) -> bool {
        row >= self.row && row < self.row + 3
    }

    /// Returns true if the coord is within the square
    pub fn is_coord_in_square(&self, coord: Coord) -> bool {
        self.is_row_in_square(coord.get_row()) && self.is_column_in_square(coord.get_col())
    }

    /// Returns an iterator over the indices of the cells in the square
    pub fn get_index(&self, coord: Coord) -> usize {
        let row = coord.get_row() - self.row;
        let col = coord.get_col() - self.col;

        row * 3 + col
    }

    /// Returns an iterator over the indices of the cells in the square
    pub fn iter_coords() -> impl Iterator<Item = Coord> {
        let mut row = 0;
        let mut col = 0;

        std::iter::from_fn(move || {
            if row > 6 {
                return None;
            }

            let coord = Coord::new(row, col);

            if col >= 6 {
                col = 0;
                row += 3;
            } else {
                col += 3;
            }

            Some(coord)
        })
    }
}

impl CellCollection for Square {
    fn get_coord(&self, index: usize) -> Coord {
        let row = self.row + index / 3;
        let col = self.col + index % 3;
        Coord::new(row, col)
    }

    fn iter(&self) -> std::ops::Range<usize> {
        0..9
    }

    fn max(&self) -> usize {
        9
    }
}

impl Default for Square {
    fn default() -> Self {
        Self { row: 0, col: 0 }
    }
}

#[cfg(test)]
mod test {
    use crate::{grid::cell_collection::CellCollection, test::util::general_tests};

    #[test]
    fn test_iter_coords() {
        for (i, coord) in super::Square::iter_coords().enumerate() {
            println!("{}: {} -> {}", i, coord, coord.get_index());
        }
    }

    #[test]
    fn test_square_iter_coords() {
        use crate::grid::constants::{GRID_HEIGHT_RANGE, GRID_WIDTH_RANGE};

        let grid = general_tests::filled_sudoku();

        for row in GRID_HEIGHT_RANGE {
            for col in GRID_WIDTH_RANGE {
                let square = grid.get_square(row, col);

                for index in square.iter() {
                    let c = square.get_coord(index);
                    assert!(square.is_coord_in_square(c));

                    //is coord within the square
                    assert!(c.get_row() >= square.row && c.get_row() < square.row + 3);
                    assert!(c.get_col() >= square.col && c.get_col() < square.col + 3);
                }
            }
        }
    }
}
