use std::fmt::{Display, Formatter};

use super::{cell_collection::CellCollection, column::Column, coords::Coord, row::Row};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Square {
    /// The row index the square starts at
    pub row: usize,
    /// The column index the square starts at
    pub col: usize,
}

impl Square {
    /// Creates a new square from the given row and column
    pub const fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }

    /// Creates a new square from the given row and column
    pub fn from(row: usize, col: usize) -> Self {
        let row_offset = row - row % 3;
        let col_offset = col - col % 3;

        Square::new(row_offset, col_offset)
    }

    pub fn get_coord_start(&self) -> Coord {
        Coord::new(self.row, self.col)
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

    /// Returns the square associated with the index (0..9)
    pub fn from_square_index(index: usize) -> Square {
        let row = index / 3;
        let col = index % 3;

        return Square::new(row * 3, col * 3);
    }

    /// Returns the index of the square (0..9)
    pub fn to_square_index(&self) -> usize {
        let row = self.row / 3;
        let col = self.col / 3;

        return row * 3 + col;
    }

    pub fn iter() -> std::ops::Range<usize> {
        (0..9).into_iter()
    }

    /// Returns an iterator over the indices of the cells in the square
    pub fn iter_coords() -> impl Iterator<Item = Coord> {
        Square::iter_squares().map(|i| i.get_coord_start())
    }

    pub fn iter_squares() -> impl Iterator<Item = Square> {
        Square::iter().map(|i| Square::from_square_index(i))
    }

    /// Iterates over all rows
    pub fn iter_rows(&self) -> impl Iterator<Item = Row> {
        let row = self.row;
        (0..3).map(move |i| Row::new(row + i))
    }

    /// Iterates over all columns
    pub fn iter_columns(&self) -> impl Iterator<Item = Column> {
        let col = self.col;
        (0..3).map(move |i| Column::new(col + i))
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

impl Display for Square {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Square({}, {})", self.row, self.col)
    }
}

#[cfg(test)]
mod test {
    use crate::{grid::cell_collection::CellCollection, test::util::general_tests};

    use super::Square;

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

    #[test]
    fn test_iter_square() {
        for sqr in Square::iter_squares() {
            assert_eq!(sqr.row % 3, 0);
            assert_eq!(sqr.col % 3, 0);
        }
    }

    #[test]
    fn test_from_index() {
        for i in 0..9 {
            let square = Square::from_square_index(i);
            assert_eq!(square.row, i / 3 * 3);
            assert_eq!(square.col, i % 3 * 3);

            let j = square.to_square_index();
            assert_eq!(i, j);
        }
    }
}
