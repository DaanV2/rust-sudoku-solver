use crate::grid::{
    cell::Cell, cell_collection::CellCollection, column::Column, coords::Coord, grid::Grid,
    mark::Mark, row::Row, slice::Slice, square::Square,
};

use super::solver::{SolveResult, Solver};

/** Determines if a row or col in a square is certain reserved, if so, the other squares are marked off */
pub struct MarkShapes {}

impl Solver for MarkShapes {
    fn name(&self) -> &'static str {
        "Mark Shapes"
    }

    fn solve(&self, grid: &mut Grid) -> SolveResult {
        MarkShapes::solve(grid)
    }
}

impl MarkShapes {
    pub fn new() -> Self {
        Self {}
    }

    pub fn new_box() -> Box<Self> {
        Box::new(Self::new())
    }

    pub fn solve(grid: &mut Grid) -> SolveResult {
        let mut changed = false;

        for square in Square::iter_squares() {
            changed = changed | MarkShapes::solve_square(grid, &square);
        }

        SolveResult::from_changed(changed)
    }

    pub fn solve_for_mark(grid: &mut Grid, mark: Mark) -> SolveResult {
        let mut changed = false;

        for square in Square::iter_squares() {
            changed = changed | MarkShapes::solve_square_for_mark(grid, &square, mark);
        }

        SolveResult::from_changed(changed)
    }

    pub fn solve_square(grid: &mut Grid, square: &Square) -> bool {
        let s = Slice::from(grid, square);
        let mut changed = false;

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
                changed = changed | mark_off_rows(&square, grid, index, mark);
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
                changed = changed | mark_off_columns(&square, grid, index, mark);
            }
        }

        changed
    }

    pub fn solve_square_for_mark(grid: &mut Grid, square: &Square, mark: Mark) -> bool {
        let s = Slice::from(grid, square);
        let mut changed = false;

        // Rows
        let row_0 = or_cells(&s, index(0, 0), index(0, 1), index(0, 2));
        let row_1 = or_cells(&s, index(1, 0), index(1, 1), index(1, 2));
        let row_2 = or_cells(&s, index(2, 0), index(2, 1), index(2, 2));

        // Columns
        let col_0 = or_cells(&s, index(0, 0), index(1, 0), index(2, 0));
        let col_1 = or_cells(&s, index(0, 1), index(1, 1), index(2, 1));
        let col_2 = or_cells(&s, index(0, 2), index(1, 2), index(2, 2));

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
            changed = changed | mark_off_rows(&square, grid, index, mark);
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
            changed = changed | mark_off_columns(&square, grid, index, mark);
        }

        changed
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
fn mark_off_rows(square: &Square, grid: &mut Grid, row: usize, mark: Mark) -> bool {
    let row_start = square.row;
    let row_index = row_start + row;
    let row_data = Row::new(row_index);
    let skip: std::ops::Range<usize> = square.col..(square.col + 3);
    let mut changed = false;

    //Unset the row but not in the square
    for col in row_data.iter() {
        if skip.contains(&col) {
            continue;
        }
        let c = row_data.get_coord(col);
        changed = changed | mark_off_at(grid, c, mark);
    }

    changed
}

/// Marks off the columns except for the square
#[inline]
fn mark_off_columns(square: &Square, grid: &mut Grid, col: usize, mark: Mark) -> bool {
    let col_start = square.col;
    let col_index = col_start + col;
    let column = Column::new(col_index);
    let skip = square.row..(square.row + 3);
    let mut changed = false;

    //Unset the column but not in the square
    for row in column.iter() {
        if skip.contains(&row) {
            continue;
        }
        let c = column.get_coord(row);
        changed = changed | mark_off_at(grid, c, mark);
    }

    changed
}

#[inline(always)]
fn mark_off_at(grid: &mut Grid, coord: Coord, mark: Mark) -> bool {
    let cell = grid.get_cell_at(coord);

    if !cell.is_possible(mark) {
        return false;
    }

    grid.unset_possible_at(coord, mark);
    return true;
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

        let s = Slice::from(&grid, &Row::new(0));
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

    #[test]
    fn test_specific_case_1() {
        let grid = &mut utility::parse_from_ascii(
            r#"3 . . | . . . | . . .
               6 8 . | . . 1 | . . .
               . . . | . . . | . . .
               ------|-------|------
               . . 6 | . . 5 | . . 7
               . . . | . . 2 | . . .
               . . . | . . . | . . .
               ------|-------|------
               . 1 . | . . . | . . 4
               . 2 . | . . . | . . .
               . 6 . | . . . | . . ."#,
        );

        //Run through the basics
        let grid = &mut general_tests::process_through(
            grid,
            vec![MarkReset::new_box(), MarkSimple::new_box()],
        );

        let solver = MarkShapes::new();
        let solved = solver.solve(grid);
        println!("{}", solved);

        // Bottom left square should have 3 in the right column
        assert!(grid.get_cell_at(Coord::new(6, 2)).is_possible(Mark::N3));
        assert!(grid.get_cell_at(Coord::new(7, 2)).is_possible(Mark::N3));
        assert!(grid.get_cell_at(Coord::new(8, 2)).is_possible(Mark::N3));
        // But not the first column
        assert!(grid.get_cell_at(Coord::new(6, 0)).is_possible(Mark::N3) == false);
        assert!(grid.get_cell_at(Coord::new(7, 0)).is_possible(Mark::N3) == false);
        assert!(grid.get_cell_at(Coord::new(8, 0)).is_possible(Mark::N3) == false);

        // Middle left square should have 3 in the middle column
        assert!(grid.get_cell_at(Coord::new(3, 1)).is_possible(Mark::N3));
        assert!(grid.get_cell_at(Coord::new(4, 1)).is_possible(Mark::N3));
        assert!(grid.get_cell_at(Coord::new(5, 1)).is_possible(Mark::N3));
        // But not the first and last column
        assert!(grid.get_cell_at(Coord::new(3, 0)).is_possible(Mark::N3) == false);
        assert!(grid.get_cell_at(Coord::new(4, 0)).is_possible(Mark::N3) == false);
        assert!(grid.get_cell_at(Coord::new(5, 0)).is_possible(Mark::N3) == false);
        assert!(grid.get_cell_at(Coord::new(3, 2)).is_possible(Mark::N3) == false);
        assert!(grid.get_cell_at(Coord::new(4, 2)).is_possible(Mark::N3) == false);
        assert!(grid.get_cell_at(Coord::new(5, 2)).is_possible(Mark::N3) == false);
    }
}
