use core::panic;
use std::error::Error;

use crate::grid::{
    cell::Cell, cell_collection::CellCollection, column::Column, coords::Coord, grid::Grid,
    mark::Mark, queries::count_determine_value, row::Row, slice::Slice,
};

pub fn validate_grid(grid: &Grid) -> Result<(), Box<dyn Error>> {
    for index in grid.iter() {
        let coord = grid.get_coord(index);
        let cell = grid.get_cell_at(coord);
        validate_cell(cell, coord)?;
    }

    for r in grid.iter_rows() {
        validate_area(grid, r)?;
    }
    for c in grid.iter_columns() {
        validate_area(grid, c)?;
    }
    for s in grid.iter_squares() {
        validate_area(grid, s)?;
    }

    Ok(())
}

pub fn is_valid(grid: &Grid) -> bool {
    for index in grid.iter() {
        let coord = grid.get_coord(index);
        let cell = grid.get_cell_at(coord);
        if is_valid_cell(cell) == false {
            return false;
        }
    }

    for r in grid.iter_rows() {
        if is_valid_area(grid, r) == false {
            return false;
        }
    }
    for c in grid.iter_columns() {
        if is_valid_area(grid, c) == false {
            return false;
        }
    }
    for s in grid.iter_squares() {
        if is_valid_area(grid, s) == false {
            return false;
        }
    }

    true
}

pub fn validate_placement(grid: &Grid, coord: Coord) -> Result<(), Box<dyn Error>> {
    //println!("{}", utility::ascii_grid(&grid));

    let cell = grid.get_cell_at(coord);
    if !cell.is_determined() {
        panic!("Cell at {} is not determined", coord);
    }
    let v = cell.get_value();

    // Row should only be 1
    let determined = count_determine_value(grid, Row::new(coord.get_row()), v);
    if determined != 1 {
        panic!("Row {} should only be 1", coord.get_row());
    }

    // Column should only be 1
    let determined = count_determine_value(grid, Column::new(coord.get_col()), v);
    if determined != 1 {
        panic!("Column {} should only be 1", coord.get_col());
    }

    // Square should only be 1
    let determined = count_determine_value(grid, grid.get_square_at(coord), v);
    if determined != 1 {
        panic!("Square should only be 1");
    }

    Ok(())
}

pub fn validate_cell(cell: &Cell, coord: Coord) -> Result<(), Box<dyn Error>> {
    let possible = cell.iter_possible().count();
    if let Some(v) = cell.value() {
        if v > 9 || v < 1 {
            let msg = format!("Invalid value {} at {}", v, coord);
            return Err(msg)?;
        }
        return Ok(());
    }

    return match possible {
        0 => {
            let msg = format!("No possible values at {}", coord);
            Err(msg)?
        }
        _ => Ok(()),
    };
}

pub fn is_valid_cell(cell: &Cell) -> bool {
    let possible = cell.iter_possible().count();
    if let Some(v) = cell.value() {
        if v > 9 || v < 1 {
            return false;
        }
        return true;
    }

    return match possible {
        0 => false,
        _ => true,
    };
}

pub fn validate_area<T: CellCollection>(grid: &Grid, area: T) -> Result<(), Box<dyn Error>> {
    let first: Coord = area.get_coord(0);
    let last = area.get_coord(area.max() - 1);
    let slice = Slice::from(grid, &area);

    for mark in Mark::iter() {
        let determined = slice.count_determined_value(mark.to_value());
        let possible = slice.any_possible(mark);

        // If value is placed more than once, Then it's invalid
        if determined > 1 {
            let msg = format!("More than one {} in area, from {} to {}", mark, first, last);
            return Err(msg)?;

        // If value is placed then it can't be possible
        } else if determined == 1 && possible {
            let msg = format!(
                "Determined {} with possible values, from {} to {}",
                mark, first, last
            );
            return Err(msg)?;

        // If value is not placed then it must be possible somewhere
        } else if determined == 0 && !possible {
            let msg = format!(
                "No possible values for {}, from {} to {}",
                mark, first, last
            );
            return Err(msg)?;
        }
    }

    Ok(())
}

pub fn is_valid_area<T: CellCollection>(grid: &Grid, area: T) -> bool {
    let slice = Slice::from(grid, &area);

    for mark in Mark::iter() {
        let determined = slice.count_determined_value(mark.to_value());
        let possible = slice.any_possible(mark);

        // If value is placed more than once, Then it's invalid
        match determined {
            // If value is not placed then it must be possible somewhere
            0 => {
                if !possible {
                    return false;
                }
            }
            // If value is placed then it can't be possible
            1 => {
                if possible {
                    return false;
                }
            }
            _ => return false,
        }
    }

    true
}

#[cfg(test)]
mod test {
    use crate::grid::utility::utility::parse_from_ascii;

    #[test]
    pub fn test_specific_case() {
        let grid = parse_from_ascii(
            "6 . . | . . . | 7 . .
             . . 2 | 5 . . | 4 . .
             1 . . | 8 . . | 5 . .
             ------|-------|------
             . . . | . . . | . . .
             . . . | . . . | 9 . .
             . . . | . . . | . . .
             ------|-------|------
             . . . | . . . | . . .
             . . . | . 5 . | . . .
             . . . | . . . | . . .",
        );

        assert!(super::is_valid(&grid));
    }
}
