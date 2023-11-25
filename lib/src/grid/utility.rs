pub mod utility {

    use crate::grid::{
        cell::Cell,
        constants::{GRID_HEIGHT_RANGE, GRID_WIDTH_RANGE},
        coords::Coord,
        grid::Grid,
        square::Square,
    };

    /// Returns a string representation of a grid
    pub fn ascii_grid(grid: &Grid) -> String {
        let mut result = String::new();

        for col in GRID_HEIGHT_RANGE {
            for row in GRID_WIDTH_RANGE {
                let coord = Coord::new(row, col);
                let cell = grid.get_cell_at(coord);

                if cell.is_determined() {
                    let value = cell.get_value();
                    result.push_str(&format!("{} ", value));
                } else {
                    result.push_str(". ");
                }
                if row == 2 || row == 5 {
                    result.push_str("| ");
                }
            }
            result.push_str("\n");

            if col == 2 || col == 5 {
                result.push_str("------|-------|------\n");
            }
        }

        result
    }

    /// Returns a string representation of a square
    pub fn ascii_square(square: &Square) -> String {
        let mut result = String::new();

        // 3x3 square
        for row in 0..3 {
            for col in 0..3 {
                let coord = Coord::new(row, col);
                let cell = square.get_cell_at(coord);

                if cell.is_determined() {
                    let value = cell.get_value();
                    result.push_str(&format!("{} ", value));
                } else {
                    result.push_str(". ");
                }
            }
            result.push_str("\n");
        }

        result
    }

    /// Returns the ASCII representation of a grid
    pub fn parse_from_ascii(ascii: &str) -> Grid {
        let mut grid = Grid::new();
        let mut index = 0;

        for line in ascii.lines() {
            for char in line.chars() {
                if char == '.' || char == '0' {
                    let c = Cell::new();
                    grid.set_cell(index, c);
                    index += 1;
                } else if let Some(value) = char.to_digit(10) {
                    let c = Cell::new_with_value(value as usize);
                    grid.set_cell(index, c);
                    index += 1;
                }
            }
        }

        grid
    }

    /// Returns a grid with all cells set to the given value
    pub fn filled_sudoku() -> Grid {
        parse_from_ascii(
            "4 3 5 | 2 6 9 | 7 8 1\n\
             6 8 2 | 5 7 1 | 4 9 3\n\
             1 9 7 | 8 3 4 | 5 6 2\n\
             ------|-------|------\n\
             8 2 6 | 1 9 5 | 3 4 7\n\
             3 7 4 | 6 8 2 | 9 1 5\n\
             9 5 1 | 7 4 3 | 6 2 8\n\
             ------|-------|------\n\
             5 1 9 | 3 2 6 | 8 7 4\n\
             2 4 8 | 9 5 7 | 1 3 6\n\
             7 6 3 | 4 1 8 | 2 5 9",
        )
    }
}
