use std::fmt::Display;

use super::{
    cell::Cell, cell_collection::CellCollection, constants::GRID_SIZE, coords::Coord,
    format::get_index, utility::utility,
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Square {
    //The row index the square starts at
    pub row: usize,
    //The column index the square starts at
    pub col: usize,
    pub grid: [Cell; GRID_SIZE],
}

impl Square {
    pub fn new(row: usize, col: usize, grid: [Cell; GRID_SIZE]) -> Self {
        Self { row, col, grid }
    }

    pub fn from(row: usize, col: usize, grid: [Cell; GRID_SIZE]) -> Self {
        let row_offset = row - row % 3;
        let col_offset = col - col % 3;

        Square::new(row_offset, col_offset, grid)
    }

    pub fn get_cell_at(&self, coord: Coord) -> Cell {
        let c = Coord::new(self.row + coord.row, self.col + coord.col);
        self.grid[get_index(c)]
    }

    pub fn get_coord_at(&self, row: usize, col: usize) -> Coord {
        Coord::new(self.row + row, self.col + col)
    }

    pub fn is_column_in_square(&self, col: usize) -> bool {
        col >= self.col && col < self.col + 3
    }

    pub fn is_row_in_square(&self, row: usize) -> bool {
        row >= self.row && row < self.row + 3
    }

    pub fn is_coord_in_square(&self, coord: Coord) -> bool {
        self.is_row_in_square(coord.row) && self.is_column_in_square(coord.col)
    }

    pub fn iter_square_coords() -> impl Iterator<Item = Coord> {
        static COORDS: [Coord; 9] = [
            Coord { row: 0, col: 0 },
            Coord { row: 0, col: 3 },
            Coord { row: 0, col: 6 },
            Coord { row: 3, col: 0 },
            Coord { row: 3, col: 3 },
            Coord { row: 3, col: 6 },
            Coord { row: 6, col: 0 },
            Coord { row: 6, col: 3 },
            Coord { row: 6, col: 6 },
        ];

        COORDS.iter().map(move |x| *x)
    }
}

impl CellCollection for Square {
    fn get_cell(&self, index: usize) -> Cell {
        let coord = self.get_coord(index);
        self.grid[get_index(coord)]
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
    fn test_square_iter_coords() {
        use crate::grid::constants::{GRID_HEIGHT_RANGE, GRID_WIDTH_RANGE};

        let grid = general_tests::filled_sudoku();

        for row in GRID_HEIGHT_RANGE {
            for col in GRID_WIDTH_RANGE {
                let square = grid.get_square(row, col);

                for c in square.iter_coords() {
                    assert!(square.is_coord_in_square(c));

                    //is coord within the square
                    assert!(c.row >= square.row && c.row < square.row + 3);
                    assert!(c.col >= square.col && c.col < square.col + 3);
                }
            }
        }
    }
}
