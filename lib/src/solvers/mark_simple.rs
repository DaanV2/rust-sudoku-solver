use crate::grid::{cell_collection::CellCollection, coords::Coord, grid::Grid};

use super::solver::{SolveResult, Solver};

/** Marks off the row, column and square of a determined cell */
pub struct MarkSimple {}

impl MarkSimple {
    pub fn new() -> Self {
        Self {}
    }

    pub fn new_box() -> Box<Self> {
        Box::new(Self::new())
    }
}

impl Solver for MarkSimple {
    fn name(&self) -> &'static str {
        "Mark Simple"
    }

    fn solve(&self, grid: &mut Grid) -> SolveResult {
        for i in grid.iter().rev() {
            let cell = grid.get_cell(i);

            //If the cell is determined, mark off that square, row and column
            if cell.is_determined() {
                grid.mark_off(Coord::from_index(i));
            }
        }

        SolveResult::Nothing
    }
}

#[cfg(test)]
mod test {
    use crate::{
        grid::{
            cell::Cell, cell_collection::CellCollection, coords::Coord, grid::Grid, mark::Mark,
            utility::utility,
        },
        solvers::solver::Solver,
        test::util::general_tests::get_url,
    };

    #[test]
    fn test_solve_square_1() {
        let coord = Coord::new(1, 1);
        test_at_coord(coord, Mark::N1);
    }

    #[test]
    fn test_solve_square_2() {
        let coord = Coord::new(2, 4);
        test_at_coord(coord, Mark::N2);
    }

    #[test]
    fn test_solve_square_3() {
        let coord = Coord::new(0, 7);
        test_at_coord(coord, Mark::N3);
    }

    #[test]
    fn test_solve_square_4() {
        let coord = Coord::new(3, 1);
        test_at_coord(coord, Mark::N4);
    }

    #[test]
    fn test_solve_square_5() {
        let coord = Coord::new(4, 3);
        test_at_coord(coord, Mark::N5);
    }

    #[test]
    fn test_solve_square_6() {
        let coord = Coord::new(5, 8);
        test_at_coord(coord, Mark::N6);
    }

    #[test]
    fn test_solve_square_7() {
        let coord = Coord::new(7, 1);
        test_at_coord(coord, Mark::N7);
    }

    #[test]
    fn test_solve_square_8() {
        let coord = Coord::new(8, 3);
        test_at_coord(coord, Mark::N8);
    }

    #[test]
    fn test_solve_square_9() {
        let coord = Coord::new(6, 8);
        test_at_coord(coord, Mark::N9);
    }

    fn test_at_coord(coord: Coord, mark: Mark) {
        let grid = &mut Grid::new();

        grid.set_cell_at(coord, Cell::new_with_value(mark.to_value()));

        println!("{}\n{}", get_url(&grid), utility::ascii_grid(&grid));
        let solver = super::MarkSimple::new();
        solver.solve(grid);

        // Checks the rows
        for row in grid.iter_rows() {
            let row_index = row.row_index();
            let possible = row.count_possible(grid, mark);

            if row_index == coord.get_row() {
                assert_eq!(possible, 0, "Row {} is not marked off", row_index);
            } else {
                assert!(possible > 0, "Row {} should be still possible", row_index);
            }
        }

        // Checks the columns
        for col in grid.iter_columns() {
            let col_index = col.col_index();
            let possible = col.count_possible(grid, mark);

            if col_index == coord.get_col() {
                assert_eq!(possible, 0, "Column {} is not marked off", col_index);
            } else {
                assert!(
                    possible > 0,
                    "Column {} should be still possible",
                    col_index
                );
            }
        }

        // Checks the squares
        for square in grid.iter_squares() {
            let possible = square.count_possible(grid, mark);

            if square.is_coord_in_square(coord) {
                assert_eq!(possible, 0, "Square is not marked off");
            } else {
                assert!(possible > 0, "Square should be still possible");
            }
        }
    }
}
