use core::panic;
use std::error::Error;

use crate::grid::{
    cell::Cell, cell_collection::CellCollection, coords::Coord, grid::Grid, mark::Mark,
};

pub fn validate_grid(grid: &Grid) -> bool {
    for index in grid.iter() {
        let coord = grid.get_coord(index);
        let cell = grid.get_cell_at(coord);
        validate_cell(cell, coord);
    }

    for r in grid.iter_rows() {
        validate_area(grid, r);
    }
    for c in grid.iter_columns() {
        validate_area(grid, c);
    }
    for s in grid.iter_squares() {
        validate_area(grid, s);
    }

    true
}

pub fn validate_placement(grid: &Grid, coord: Coord) -> Result<(), Box<dyn Error>> {
    //println!("{}", utility::ascii_grid(&grid));

    let cell = grid.get_cell_at(coord);
    if !cell.is_determined() {
        panic!("Cell at {} is not determined", coord);
    }

    // Row should only be 1
    let determined = grid
        .get_row(coord.get_row())
        .count_determined_value(grid, cell.get_value());
    if determined != 1 {
        panic!("Row {} should only be 1", coord.get_row());
    }

    // Column should only be 1
    let determined = grid
        .get_column(coord.get_col())
        .count_determined_value(grid, cell.get_value());
    if determined != 1 {
        panic!("Column {} should only be 1", coord.get_col());
    }

    // Square should only be 1
    let determined = grid
        .get_square_at(coord)
        .count_determined_value(grid, cell.get_value());
    if determined != 1 {
        panic!("Square should only be 1");
    }

    Ok(())
}

pub fn validate_cell(cell: Cell, coord: Coord) {
    let possible = cell.iter_possible().count();
    if let Some(v) = cell.value() {
        if v > 9 || v < 1 {
            panic!("Invalid value {} at {}", v, coord);
        }
        return;
    }

    if possible == 0 {
        panic!("No possible values at {}", coord);
    }
}

pub fn validate_area<T: CellCollection>(grid: &Grid, area: T) {
    for mark in Mark::iter() {
        let first = area.get_coord(0);
        let last = area.get_coord(area.max() - 1);

        let determined = area
            .iter()
            .map(|c| grid.get_cell_at(area.get_coord(c)))
            .filter(|c| c.is_determined() && c.get_value() == mark.to_value())
            .count();

        let possible = area
            .iter()
            .map(|c| grid.get_cell_at(area.get_coord(c)))
            .filter(|c| c.is_possible(mark))
            .count();

        if determined > 1 {
            panic!("More than one {} in area, from {} to {}", mark, first, last);
        } else if determined == 1 && possible > 0 {
            panic!(
                "Determined {} with possible values, from {} to {}",
                mark, first, last
            );
        } else if determined == 0 && possible == 0 {
            panic!(
                "No possible values for {}, from {} to {}",
                mark, first, last
            );
        }
    }
}

pub fn is_valid(grid: &Grid) -> bool {
    for r in grid.iter_rows() {
        if !is_valid_area(grid, r) {
            return false;
        }
    }
    for c in grid.iter_columns() {
        if !is_valid_area(grid, c) {
            return false;
        }
    }
    for s in grid.iter_squares() {
        if !is_valid_area(grid, s) {
            return false;
        }
    }

    true
}

pub fn is_valid_area<T: CellCollection>(grid: &Grid, area: T) -> bool {
    let mut determined = [0; 10];
    let mut possibles = [0; 10];

    for index in area.iter() {
        let coord = area.get_coord(index);
        let cell = grid.get_cell_at(coord);

        if cell.is_determined() {
            let value = cell.get_value();
            if determined[value] >= 1 {
                return false;
            }

            determined[value] = determined[value] + 1;
        } else {
            for value in cell.iter_possible() {
                let v = value.to_value();
                possibles[v] = possibles[v] + 1;
            }
        }
    }

    for i in 1..9 {
        // If not determined, then it needs to be possible
        if determined[i] == 0 {
            if possibles[i] == 0 {
                return false;
            }
        } else {
            // If determined, then it can't be possible
            if possibles[i] > 0 {
                return false;
            }
        }
    }

    return true;
}
