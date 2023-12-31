pub mod utility {

    use crate::grid::{
        cell::Cell,
        cell_collection::CellCollection,
        constants::{GRID_HEIGHT_RANGE, GRID_SIZE, GRID_WIDTH_RANGE},
        coords::Coord,
        grid::Grid,
        square::Square,
    };

    /// Returns a string representation of a grid
    pub fn ascii_grid(grid: &Grid) -> String {
        let mut result = String::new();

        for row in GRID_WIDTH_RANGE {
            for col in GRID_HEIGHT_RANGE {
                let coord = Coord::new(row, col);
                let cell = grid.get_cell_at(coord);

                if cell.is_determined() {
                    let value = cell.get_value();
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

    /// Returns a hex string representation of a grid, each 4 characters representing a cell
    pub fn hex_grid(grid: &Grid) -> String {
        let mut s = String::with_capacity(GRID_SIZE * 4);

        for i in grid.iter() {
            let cell = grid.get_cell(i).get_value();
            s.push_str(&format!("{:04x}", cell));
        }

        s
    }

    /// Returns a hex string representation of a grid but only on the value, each 1 character representing a cell
    pub fn hex_value_grid(grid: &Grid) -> String {
        let mut s = String::with_capacity(GRID_SIZE);

        for i in grid.iter() {
            let cell = grid.get_cell(i).only_determined().get_value() as u8;
            let c = '0' as u8 + cell;
            s.push(c as char);
        }

        s
    }

    /// Returns a grid from a hex string
    pub fn grid_from_hex(hex: &str) -> Grid {
        let mut grid = Grid::new();
        let max = (hex.len() / 4).min(GRID_SIZE);

        for i in 0..max {
            let value = u16::from_str_radix(&hex[i * 4..i * 4 + 4], 16).unwrap();
            grid.set_cell(i, &Cell::new_with_value(value));
        }

        grid
    }

    /// Returns a grid from a hex string
    pub fn grid_from_hex_value(hex: &str) -> Grid {
        let mut grid = Grid::new();

        for (index, c) in hex.chars().enumerate() {
            let value = c as u16 - '0' as u16;
            grid.set_cell(index, &Cell::new_with_value(value));
        }

        grid
    }

    /// Returns a string representation of a square
    pub fn ascii_square(grid: &Grid, square: &Square) -> String {
        let mut result = String::new();

        for row in 0..3 {
            for col in 0..3 {
                let coord = square.get_coord_at(row, col);
                let cell = grid.get_cell_at(coord);

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
                    index += 1;
                } else if let Some(value) = char.to_digit(10) {
                    grid.place_value(index, value as u16);
                    index += 1;
                }
            }
        }

        grid
    }

    /// Returns a grid with all cells set to the given value
    pub fn filled_sudoku() -> Grid {
        parse_from_ascii(
            r#"4 3 5 | 2 6 9 | 7 8 1
               6 8 2 | 5 7 1 | 4 9 3
               1 9 7 | 8 3 4 | 5 6 2
               ------|-------|------
               8 2 6 | 1 9 5 | 3 4 7
               3 7 4 | 6 8 2 | 9 1 5
               9 5 1 | 7 4 3 | 6 2 8
               ------|-------|------
               5 1 9 | 3 2 6 | 8 7 4
               2 4 8 | 9 5 7 | 1 3 6
               7 6 3 | 4 1 8 | 2 5 9"#,
        )
    }

    /// Returns a string representation of a grid in a digit format
    pub fn to_digits(grid: &Grid) -> String {
        let mut chars = ['0'; GRID_SIZE];

        for i in grid.iter() {
            let cell = grid.get_cell(i);

            if let Some(v) = cell.value() {
                chars[i] = std::char::from_digit(v as u32, 10).unwrap();
            } else {
                chars[i] = '0';
            }
        }

        chars.iter().collect()
    }

    /// Returns a grid from a digit format
    pub fn from_digit(digits: &str) -> Grid {
        let mut grid = Grid::new();

        for (i, c) in digits.chars().enumerate() {
            if c != '0' {
                grid.place_value(i, c.to_digit(10).unwrap() as u16);
            }
        }

        grid
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        grid::utility::utility::{
            ascii_grid, filled_sudoku, grid_from_hex, hex_grid, parse_from_ascii,
        },
        test::util::general_tests::remove_cells_amount,
    };

    use super::utility;

    #[test]
    fn test_filled_sudoku() {
        let grid = filled_sudoku();

        assert_eq!(grid.get_cell(0).get_value(), 4);
        assert_eq!(grid.get_cell(80).get_value(), 9);
    }

    #[test]
    fn test_hex_grid() {
        let grid = &mut filled_sudoku();
        let hex = hex_grid(&grid);
        let grid2 = &grid_from_hex(&hex);

        assert_eq!(grid, grid2);

        remove_cells_amount(grid, 16);
        let hex = hex_grid(&grid);
        let grid2 = &grid_from_hex(&hex);

        assert_eq!(grid, grid2);
    }

    #[test]
    fn test_ascii_grid() {
        let grid = &mut filled_sudoku();
        let ascii = ascii_grid(&grid);
        let grid2 = &mut parse_from_ascii(&ascii);

        assert_eq!(grid, grid2);
    }

    #[test]
    fn test_specific_parse_test() {
        let grid = &mut utility::parse_from_ascii(
            r#"4 3 5 | 2 6 9 | . . .
               6 8 2 | . 7 . | 4 9 3
               1 9 7 | 8 3 4 | 5 . .
               ------|-------|------
               8 2 6 | . 9 . | 3 4 7
               3 7 4 | 6 8 2 | 9 1 5
               9 5 1 | 7 4 3 | 6 . .
               ------|-------|------
               5 1 9 | 3 2 6 | . . 4
               2 4 8 | 9 5 7 | . . .
               7 6 3 | 4 1 8 | 2 5 9"#,
        );

        println!("{}", grid);

        assert_eq!(grid.get_cell(0).get_value(), 4);
        assert_eq!(grid.get_cell(80).get_value(), 9);

        assert_eq!(grid.get_cell(6).is_determined(), false);
    }
}
