pub mod utility {

    use crate::grid::{
        cell::Cell,
        constants::{GRID_HEIGHT_RANGE, GRID_WIDTH_RANGE},
        coords::Coord,
        grid::Grid,
        square::Square,
    };

    pub fn ascii_grid(grid: &Grid) -> String {
        let mut result = String::new();

        for row in GRID_WIDTH_RANGE {
            for col in GRID_HEIGHT_RANGE {
                let coord = Coord::new(row, col);
                let cell = grid.get_cell_at(coord);
                let value = cell.value;

                if cell.is_determined() {
                    result.push_str(&format!("{} ", value));
                } else {
                    result.push_str(". ");
                }

                if col == 2 || col == 5 {
                    result.push_str("| ");
                }
            }
            result.push_str("\n");

            if row == 2 || row == 5 {
                result.push_str("------|-------|------\n");
            }
        }

        result
    }

    pub fn ascii_square(square: &Square) -> String {
        let mut result = String::new();

        // 3x3 square
        for row in 0..3 {
            for col in 0..3 {
                let coord = Coord::new(row, col);
                let cell = square.get_cell_at(coord);
                let value = cell.value;

                if cell.is_determined() {
                    result.push_str(&format!("{} ", value));
                } else {
                    result.push_str(". ");
                }
            }
            result.push_str("\n");
        }

        result
    }

    pub fn parse_from_ascii(ascii: &str) -> Grid {
        let mut grid = Grid::new();
        let mut index = 0;

        for line in ascii.lines() {
            for char in line.chars() {
                if char == '.' || char == '0' {
                    let c = Cell::new();
                    grid.set_cell(index, &c);
                    index += 1;
                } else if let Some(value) = char.to_digit(10) {
                    let c = Cell::new_with_value(value as u8);
                    grid.set_cell(index, &c);
                    index += 1;
                }
            }
        }

        grid
    }
}
