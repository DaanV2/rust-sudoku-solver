use crate::grid::{
    cell::Cell, cell_collection::CellCollection, grid::Grid, mark::Mark, slice::Slice,
    square::Square,
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

    pub fn solve(grid: &mut Grid) -> SolveResult {
        for square in Square::iter_squares() {
            MarkShapes::solve_square(grid, &square);
        }

        SolveResult::Nothing
    }

    pub fn solve_square(grid: &mut Grid, square: &Square) {
        let s: Slice = Slice::from(grid, square);

        // Rows
        let row_0 = or_cells(&s, index(0, 0), index(0, 1), index(0, 2));
        let row_1 = or_cells(&s, index(1, 0), index(1, 1), index(1, 2));
        let row_2 = or_cells(&s, index(2, 0), index(2, 1), index(2, 2));

        // Columns
        let col_0 = or_cells(&s, index(0, 0), index(1, 0), index(2, 0));
        let col_1 = or_cells(&s, index(0, 1), index(1, 1), index(2, 1));
        let col_2 = or_cells(&s, index(0, 2), index(1, 2), index(2, 2));

        let or = row_0 | row_1 | row_2 | col_0 | col_1 | col_2;

        for mark in or.only_possible().iter_possible() {
            //If a row is certain, mark off the other squares, and the other are not possible
            if let Some(index) = match (
                row_0.is_possible(mark),
                row_1.is_possible(mark),
                row_2.is_possible(mark),
            ) {
                (true, false, false) => Some(0),
                (false, true, false) => Some(1),
                (false, false, true) => Some(2),
                _ => None,
            } {
                mark_off_rows(&square, grid, index, mark);
            }

            //If a col is certain, mark off the other squares, and the other are not possible
            if let Some(index) = match (
                col_0.is_possible(mark),
                col_1.is_possible(mark),
                col_2.is_possible(mark),
            ) {
                (true, false, false) => Some(0),
                (false, true, false) => Some(1),
                (false, false, true) => Some(2),
                _ => None,
            } {
                mark_off_columns(&square, grid, index, mark);
            }
        }
    }
}

impl Solver for MarkShapes {
    fn name(&self) -> &'static str {
        "Mark Shapes"
    }

    fn solve(&self, grid: &mut Grid) -> SolveResult {
        MarkShapes::solve(grid)
    }
}

#[inline(always)]
fn index(row: usize, col: usize) -> usize {
    row * 3 + col
}

#[inline(always)]
fn or_cells(slice: &Slice, c1: usize, c2: usize, c3: usize) -> Cell {
    slice.items[c1] | slice.items[c2] | slice.items[c3]
}

/// Marks off the rows except for the square
#[inline]
fn mark_off_rows(square: &Square, grid: &mut Grid, row: usize, mark: Mark) {
    let row_start = square.row;
    let row_index = row_start + row;
    let row_data = grid.get_row(row_index);

    let skip = square.col..(square.col + 3);

    //Unset the row but not in the square
    for col in row_data.iter() {
        if skip.contains(&col) {
            continue;
        }
        let c = row_data.get_coord(col);
        grid.unset_possible_at(c, mark)
    }
}

/// Marks off the columns except for the square
#[inline]
fn mark_off_columns(square: &Square, grid: &mut Grid, col: usize, mark: Mark) {
    let col_start = square.col;
    let col_index = col_start + col;
    let column = grid.get_column(col_index);

    let skip = square.row..(square.row + 3);

    //Unset the column but not in the square
    for row in column.iter() {
        if skip.contains(&row) {
            continue;
        }
        let c = column.get_coord(row);
        grid.unset_possible_at(c, mark)
    }
}

#[cfg(test)]
mod test {
    use std::vec;

    use super::*;
    use crate::{
        grid::{coords::Coord, utility::utility},
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
    fn test_simple_square() {
        let square = Square::new(0, 3);
        let mut grid = base_grid();

        //Run through the basics
        let grid = &mut general_tests::process_through(
            &mut grid,
            vec![MarkReset::new_box(), MarkSimple::new_box()],
        );

        MarkShapes::solve_square(grid, &square);

        //Top row in the middle should not have 4 5 6
        for c in 0..3 {
            let coord = Coord::new(0, c + 3);
            let cell = grid.get_cell_at(coord);

            assert!(cell.is_possible(Mark::N4), "Cell is not still possible: 4");
            assert!(cell.is_possible(Mark::N5), "Cell is not still possible: 5");
            assert!(cell.is_possible(Mark::N6), "Cell is not still possible: 6");
        }

        let s = Slice::from(&grid, &grid.get_row(0));
        println!("Slice: {}", s);

        //First and last 3 should not have 4 5 6
        for c in 0..3 {
            let cell = s.items[c];

            assert!(!cell.is_possible(Mark::N4), "Cell is still possible: 4");
            assert!(!cell.is_possible(Mark::N5), "Cell is still possible: 5");
            assert!(!cell.is_possible(Mark::N6), "Cell is still possible: 6");
        }

        for c in 6..9 {
            let cell = s.items[c];

            assert!(!cell.is_possible(Mark::N4), "Cell is still possible: 4");
            assert!(!cell.is_possible(Mark::N5), "Cell is still possible: 5");
            assert!(!cell.is_possible(Mark::N6), "Cell is still possible: 6");
        }
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

            println!("Cell: {}", cell);

            assert!(!cell.is_possible(Mark::N4), "Cell is still possible: 4");
            assert!(!cell.is_possible(Mark::N5), "Cell is still possible: 5");
            assert!(!cell.is_possible(Mark::N6), "Cell is still possible: 6");
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
