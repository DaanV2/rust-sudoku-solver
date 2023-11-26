use std::fmt::Display;

use super::{
    cell::Cell, cell_collection::CellCollection, constants::GRID_SIZE, coords::Coord,
    format::get_index, utility::utility,
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Square {
    /// The row index the square starts at
    pub row: usize,
    /// The column index the square starts at
    pub col: usize,
    /// The grid of cells
    pub grid: [Cell; GRID_SIZE],
}

impl Square {
    /// Creates a new square from the given row and column
    pub fn new(row: usize, col: usize, grid: [Cell; GRID_SIZE]) -> Self {
        Self { row, col, grid }
    }

    /// Creates a new square from the given row and column
    pub fn from(row: usize, col: usize, grid: [Cell; GRID_SIZE]) -> Self {
        let row_offset = row - row % 3;
        let col_offset = col - col % 3;

        Square::new(row_offset, col_offset, grid)
    }

    /// Gets the cell at the given index
    pub fn get_cell_at(&self, coord: Coord) -> Cell {
        let c = coord.offset(self.row, self.col);
        self.grid[get_index(c)]
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
    pub fn iter_square_coords() -> impl Iterator<Item = Coord> {
        static COORDS: [Coord; 9] = [
            Coord::new(0, 0),
            Coord::new(0, 3),
            Coord::new(0, 6),
            Coord::new(3, 0),
            Coord::new(3, 3),
            Coord::new(3, 6),
            Coord::new(6, 0),
            Coord::new(6, 3),
            Coord::new(6, 6),
        ];

        COORDS.iter().map(move |x| *x)
    }
}

impl CellCollection for Square {
    fn get_cell(&self, index: usize) -> Cell {
        let coord = self.get_coord(index);
        self.grid[coord.get_index()]
    }

    fn get_coord(&self, index: usize) -> Coord {
        let row = self.row + index / 3;
        let col = self.col + index % 3;
        Coord::new(row, col)
    }
}

impl Default for Square {
    fn default() -> Self {
        Self {
            row: 0,
            col: 0,
            grid: [Cell::default(); GRID_SIZE],
        }
    }
}

impl Display for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", utility::ascii_square(self))
    }
}

#[cfg(test)]
mod test {
    use crate::{grid::cell_collection::CellCollection, test::util::general_tests};

    #[test]
    fn test_iter_square_coords() {
        for (i, coord) in super::Square::iter_square_coords().enumerate() {
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
