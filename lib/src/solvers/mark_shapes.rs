use crate::grid::{
    cell_collection::CellCollection, coords::Coord, grid::Grid, mark::Mark, square::Square,
};

use super::solver::{Solver, SolverResult};

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

    fn solve(&self, grid: Grid) -> SolverResult {
        let current = &mut grid.clone();

        for mark in Mark::iter() {
            for square in grid.iter_squares() {
                check_square(&square, current, *mark);
            }
        }

        SolverResult::nothing(*current)
    }
}

fn check_square(square: &Square, grid: &mut Grid, mark: Mark) {
    for row in 0..3 {
        if only_possible_in_row(square, row, mark) {
            mark_off_other_rows(square, grid, row, mark);
            break;
        }
    }

    for col in 0..3 {
        if only_possible_in_column(square, col, mark) {
            mark_off_other_columns(square, grid, col, mark);
            break;
        }
    }
}

fn only_possible_in_row(square: &Square, row: usize, mark: Mark) -> bool {
    let mut in_row = 0;
    for r in 0..3 {
        for c in 0..3 {
            let coord = Coord::new(r, c);
            let cell = square.get_cell_at(coord);
            let is_possible = cell.is_possible(mark);

            //If in the row then its where we want it
            if r == row {
                if is_possible {
                    in_row += 1;
                }
            //If not in the row then only possible in the row is a problem
            } else {
                if is_possible {
                    return false;
                }
            }
        }
    }

    return in_row > 0;
}

fn only_possible_in_column(square: &Square, col: usize, mark: Mark) -> bool {
    let mut in_col = 0;
    for r in 0..3 {
        for c in 0..3 {
            let coord = Coord::new(r, c);
            let cell = square.get_cell_at(coord);
            let is_possible = cell.is_possible(mark);

            //If in the column then its where we want it
            if c == col {
                if is_possible {
                    in_col += 1;
                }
            //If not in the column then only possible in the column is a problem
            } else {
                if is_possible {
                    return false;
                }
            }
        }
    }

    return in_col > 0;
}

fn mark_off_other_rows(square: &Square, grid: &mut Grid, row: usize, mark: Mark) {
    let row_start = square.row;

    for r in 0..3 {
        let row_index = row_start + r;
        if r == row {
            //Unset the row but not in the square
            let row = grid.get_row(row_index);

            for c in row.iter_coords() {
                if square.is_column_in_square(c.col) {
                    continue;
                }

                grid.unset_possible_at(c, mark)
            }
        } else {
            let row = grid.get_row(row_index);
            row.unset_all_possible(grid, mark);
        }
    }
}

fn mark_off_other_columns(square: &Square, grid: &mut Grid, col: usize, mark: Mark) {
    let col_start = square.col;

    for c in 0..3 {
        let col_index = col_start + c;
        let column = grid.get_column(col_index);

        if c == col {
            //Unset the column but not in the square
            for r in column.iter_coords() {
                if square.is_row_in_square(r.row) {
                    continue;
                }

                grid.unset_possible_at(r, mark)
            }
        } else {
            column.unset_all_possible(grid, mark);
        }
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
            . . . . . . . . .
            . . . 7 8 9 . . .
            . . . 1 2 3 . . .
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
        let sq1 = grid.get_square(0, 0);
        for c in 0..3 {
            let coord = Coord::new(0, c);
            let cell = sq1.get_cell_at(coord);
            assert!(!cell.is_possible(Mark::N4));
            assert!(!cell.is_possible(Mark::N5));
            assert!(!cell.is_possible(Mark::N6));
        }

        //Top row should not have 4, 5 6
        let sq3 = grid.get_square(0, 6);
        for c in 0..3 {
            let coord = Coord::new(0, c);
            let cell = sq3.get_cell_at(coord);
            assert!(!cell.is_possible(Mark::N4));
            assert!(!cell.is_possible(Mark::N5));
            assert!(!cell.is_possible(Mark::N6));
        }

        //Top row should have 4, 5, 6 and only those
        let sq2 = grid.get_square(0, 3);
        for c in 0..3 {
            let coord = Coord::new(0, c);
            let cell = sq2.get_cell_at(coord);
            assert!(cell.is_possible(Mark::N4));
            assert!(cell.is_possible(Mark::N5));
            assert!(cell.is_possible(Mark::N6));

            //Make sure the other numbers are not possible
            assert!(!cell.is_possible(Mark::N1));
            assert!(!cell.is_possible(Mark::N2));
            assert!(!cell.is_possible(Mark::N3));
            assert!(!cell.is_possible(Mark::N7));
            assert!(!cell.is_possible(Mark::N8));
            assert!(!cell.is_possible(Mark::N9));
        }
    }

    #[test]
    fn test_mark_shapes_touched_nothing() {
        let mut grid = general_tests::filled_sudoku();

        //Run through the basics
        grid = general_tests::process_through(
            &mut grid,
            vec![MarkReset::new_box(), MarkSimple::new_box()],
        );

        grid = MarkShapes::new_box().solve(grid).grid;

        //Empty grids should still be possible for only 5
        for c in grid.iter_cells() {
            if !c.is_determined() {
                let p = c.possibilities.get_value();

                assert_eq!(p, Mark::N5 as u16);
            }
        }
    }
}
