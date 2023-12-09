use crate::grid::{
    cell_collection::CellCollection, coords::Coord, grid::Grid, mark::Mark, square::Square,
};

use super::solver::{SolveResult, Solver};

/** Determines if a row or col in a square is certain reserved, if so, the other squares are marked off */
pub struct MarkShapes {}

impl MarkShapes {
    pub fn new() -> Self {
        Self {}
    }

    pub fn new_box() -> Box<Self> {
        Box::new(Self::new())
    }
}

impl Solver for MarkShapes {
    fn name(&self) -> &'static str {
        "Mark Shapes"
    }

    fn solve(&self, grid: &mut Grid) -> SolveResult {
        for coord in Square::iter_coords() {
            for mark in Mark::iter() {
                let square = grid.get_square_at(coord);
                check_square(&square, grid, mark);
            }
        }

        SolveResult::Nothing
    }
}

fn check_square(square: &Square, grid: &mut Grid, mark: Mark) {
    let mut row_0 = false;
    let mut row_1 = false;
    let mut row_2 = false;
    let mut col_0 = false;
    let mut col_1 = false;
    let mut col_2 = false;

    let coord = square.get_coord(0);

    //Row 1
    (row_0, col_0) = check_cell(grid, mark, coord.offset(0, 0), row_0, col_0);
    (row_0, col_1) = check_cell(grid, mark, coord.offset(0, 1), row_0, col_1);
    (row_0, col_2) = check_cell(grid, mark, coord.offset(0, 2), row_0, col_2);

    //Row 2
    (row_1, col_0) = check_cell(grid, mark, coord.offset(1, 0), row_1, col_0);
    (row_1, col_1) = check_cell(grid, mark, coord.offset(1, 1), row_1, col_1);
    (row_1, col_2) = check_cell(grid, mark, coord.offset(1, 2), row_1, col_2);

    //Row 3
    (row_2, col_0) = check_cell(grid, mark, coord.offset(2, 0), row_2, col_0);
    (row_2, col_1) = check_cell(grid, mark, coord.offset(2, 1), row_2, col_1);
    (row_2, col_2) = check_cell(grid, mark, coord.offset(2, 2), row_2, col_2);

    //If a row is certain, mark off the other squares, and the other are not possible
    let index = match (row_0, row_1, row_2) {
        (true, false, false) => 0,
        (false, true, false) => 1,
        (false, false, true) => 2,
        _ => -1,
    };
    if index >= 0 {
        mark_off_rows(square, grid, index as usize, mark);
    }

    //If a col is certain, mark off the other squares, and the other are not possible
    let index = match (col_0, col_1, col_2) {
        (true, false, false) => 0,
        (false, true, false) => 1,
        (false, false, true) => 2,
        _ => -1,
    };
    if index >= 0 {
        mark_off_columns(square, grid, index as usize, mark);
    }
}

#[inline(always)]
fn check_cell(grid: &Grid, mark: Mark, coord: Coord, row: bool, col: bool) -> (bool, bool) {
    let cell = grid.get_cell_at(coord);
    let p = cell.is_possible(mark);

    (row | p, col | p)
}

/// Marks off the rows except for the square
#[inline]
fn mark_off_rows(square: &Square, grid: &mut Grid, row: usize, mark: Mark) {
    let row_start = square.row;
    let row_index = row_start + row;
    let row_data = grid.get_row(row_index);

    //Unset the row but not in the square
    for index in row_data.iter() {
        let c = row_data.get_coord(index);
        if square.is_column_in_square(c.get_col()) {
            continue;
        }

        grid.unset_possible_at(c, mark)
    }
}

/// Marks off the columns except for the square
#[inline]
fn mark_off_columns(square: &Square, grid: &mut Grid, col: usize, mark: Mark) {
    let col_start = square.col;
    let col_index = col_start + col;
    let column = grid.get_column(col_index);

    //Unset the column but not in the square
    for index in column.iter() {
        let c = column.get_coord(index);

        if square.is_row_in_square(c.get_row()) {
            continue;
        }

        grid.unset_possible_at(c, mark)
    }
}

#[cfg(test)]
mod test {
    use std::vec;

    use super::*;
    use crate::{
        grid::utility::utility,
        solvers::{mark_reset::MarkReset, mark_simple::MarkSimple},
        test::util::general_tests,
    };

    fn base_grid() -> Grid {
        return utility::parse_from_ascii(
            "
            . . . | . . . | . . .
            . . . | 7 8 9 | . . .
            . . . | 1 2 3 | . . .
            ",
        );
    }

    #[test]
    fn test_mark_shapes() {
        let mut grid = base_grid();

        //Run through the basics
        grid = general_tests::process_through(
            &mut grid,
            vec![
                MarkReset::new_box(),
                MarkSimple::new_box(),
                MarkShapes::new_box(),
            ],
        );

        //Top row should not have 4 5 6
        for c in 0..3 {
            let coord = Coord::new(0, c);
            let cell = grid.get_cell_at(coord);
            assert!(!cell.is_possible(Mark::N4));
            assert!(!cell.is_possible(Mark::N5));
            assert!(!cell.is_possible(Mark::N6));
        }

        //Top row should not have 4, 5 6
        for c in 0..3 {
            let coord = Coord::new(0, c + 6);
            let cell = grid.get_cell_at(coord);
            assert!(!cell.is_possible(Mark::N4));
            assert!(!cell.is_possible(Mark::N5));
            assert!(!cell.is_possible(Mark::N6));
        }

        //Middle square, Top row should have 4, 5, 6 and only those
        for c in 0..3 {
            let coord = Coord::new(0, c + 3);
            let cell = grid.get_cell_at(coord);
            assert!(cell.is_possible(Mark::N4), "Coord: {}", coord);
            assert!(cell.is_possible(Mark::N5), "Coord: {}", coord);
            assert!(cell.is_possible(Mark::N6), "Coord: {}", coord);

            //Make sure the other numbers are not possible
            assert!(!cell.is_possible(Mark::N1), "Coord: {}", coord);
            assert!(!cell.is_possible(Mark::N2), "Coord: {}", coord);
            assert!(!cell.is_possible(Mark::N3), "Coord: {}", coord);
            assert!(!cell.is_possible(Mark::N7), "Coord: {}", coord);
            assert!(!cell.is_possible(Mark::N8), "Coord: {}", coord);
            assert!(!cell.is_possible(Mark::N9), "Coord: {}", coord);
        }
    }

    #[test]
    fn test_mark_shapes_touched_nothing() {
        let grid = &mut general_tests::filled_sudoku();

        general_tests::remove_number(grid, 5);

        //Run through the basics
        let processed = &mut general_tests::process_through(
            grid,
            vec![MarkReset::new_box(), MarkSimple::new_box()],
        );

        let solver = MarkShapes::new();
        let solved = solver.solve(processed);
        println!("{}", solved);

        //Empty grids should still be possible for only 5
        for index in grid.iter() {
            let coord = grid.get_coord(index);
            let c = grid.get_cell_at(coord);
            if !c.is_determined() {
                let p = c.is_possible(Mark::N5);

                assert_eq!(p, true, "Coord: {}", coord);
            }
        }
    }
}
