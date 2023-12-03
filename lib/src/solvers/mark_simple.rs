use crate::grid::{cell_collection::CellCollection, grid::Grid, mark::Mark};

use super::solver::{Solver, SolverResult};

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

    fn solve(&self, grid: &Grid) -> SolverResult {
        let mut current = grid.clone();

        for i in grid.iter() {
            let cell = current.get_cell(i);

            //If the cell is determined, mark off that square, row and column
            if let Some(value) = cell.value() {
                let turnoff = Mark::from_value(value);
                let coord = current.get_coord(i);

                //Mark off the row
                let row = current.get_row(coord.get_row());
                let col = current.get_column(coord.get_col());
                let square = current.get_square_at(coord);

                for i in 0..9 {
                    current.unset_possible_at(row.get_coord(i), turnoff);
                    current.unset_possible_at(col.get_coord(i), turnoff);
                    current.unset_possible_at(square.get_coord(i), turnoff);
                }
            }
        }

        SolverResult::nothing(current)
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
        let mut grid = Grid::new();

        grid.set_cell_at(coord, Cell::new_with_value(mark.to_value()));

        println!("{}\n{}", get_url(&grid), utility::ascii_grid(&grid));
        let solver = super::MarkSimple::new();
        let result = solver.solve(&grid);
        let modified = result.grid;

        // Checks the rows
        for row in modified.iter_rows() {
            let row_index = row.row_index();
            let possible = row.count_possible(&modified, mark);

            if row_index == coord.get_row() {
                assert_eq!(possible, 0, "Row {} is not marked off", row_index);
            } else {
                assert!(possible > 0, "Row {} should be still possible", row_index);
            }
        }

        // Checks the columns
        for col in modified.iter_columns() {
            let col_index = col.col_index();
            let possible = col.count_possible(&modified, mark);

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
        for square in modified.iter_squares() {
            let possible = square.count_possible(&modified, mark);

            if square.is_coord_in_square(coord) {
                assert_eq!(possible, 0, "Square is not marked off");
            } else {
                assert!(possible > 0, "Square should be still possible");
            }
        }
    }
}
