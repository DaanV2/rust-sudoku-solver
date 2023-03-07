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

                row.unset_all_possible(grid, mark)
            }
        }

        let row = grid.get_row(row_index);
        row.unset_all_possible(grid, mark);
    }
}

fn mark_off_other_columns(square: &Square, grid: &mut Grid, col: usize, mark: Mark) {
    let col_start = square.col;

    for c in 0..3 {
        let col_index = col_start + c;
        if c == col {
            //Unset the column but not in the square
            let col = grid.get_column(col_index);

            for r in col.iter_coords() {
                if square.is_row_in_square(r.row) {
                    continue;
                }

                col.unset_all_possible(grid, mark)
            }
        }

        let col = grid.get_column(col_index);
        col.unset_all_possible(grid, mark);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{
        grid::test_util::test_util,
        solvers::{mark_reset::MarkReset, mark_simple::MarkSimple},
    };

    #[test]
    fn test_mark_shapes() {
        let mut grid = test_util::parse_from_ascii(
            "
            . . . . . . . . .
            . . . 7 8 9 . . .
            . . . 1 2 3 . . .
            ",
        );
        println!("{}", test_util::ascii_grid(&grid));

        //Run through the basics
        let mut result = MarkReset::new().solve(grid);
        result = MarkSimple::new().solve(result.grid);

        let solver = MarkShapes::new();
        result = solver.solve(result.grid);

        //Top row should not have 4, 5 6
        grid = result.grid;
        let sq1 = grid.get_square(0, 0);
        for c in 0..3 {
            let coord = Coord::new(0, c);
            let cell = sq1.get_cell_at(coord);
            assert!(!cell.is_possible(Mark::N4));
            assert!(!cell.is_possible(Mark::N5));
            assert!(!cell.is_possible(Mark::N6));
        }

        //Top row should not have 4, 5 6
        let sq3 = result.grid.get_square(0, 6);
        for c in 0..3 {
            let coord = Coord::new(0, c);
            let cell = sq3.get_cell_at(coord);
            assert!(!cell.is_possible(Mark::N4));
            assert!(!cell.is_possible(Mark::N5));
            assert!(!cell.is_possible(Mark::N6));
        }

        //Top row should have 4, 5, 6 and only those
        let sq2 = result.grid.get_square(0, 3);
        for c in 0..3 {
            let coord = Coord::new(0, c);
            let cell = sq2.get_cell_at(coord);
            assert!(cell.is_possible(Mark::N4));
            assert!(cell.is_possible(Mark::N5));
            assert!(cell.is_possible(Mark::N6));

            assert!(!cell.is_possible(Mark::N1));
            assert!(!cell.is_possible(Mark::N2));
            assert!(!cell.is_possible(Mark::N3));
            assert!(!cell.is_possible(Mark::N7));
            assert!(!cell.is_possible(Mark::N8));
            assert!(!cell.is_possible(Mark::N9));
        }
    }
}
