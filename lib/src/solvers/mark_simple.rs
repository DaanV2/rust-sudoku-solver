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

    fn solve(&self, grid: Grid) -> SolverResult {
        let mut current = grid.clone();

        for i in grid.iter() {
            let cell = current.get_cell(i);

            //If the cell is determined, mark off that square, row and column
            if let Some(value) = cell.value() {
                let turnoff = Mark::from_value(value);
                let coord = current.get_coord(i);

                //Mark off the row
                for j in current.get_row(coord.row).iter_coords() {
                    current.unset_possible_at(j, turnoff);
                }

                //Mark off the column
                for j in current.get_column(coord.col).iter_coords() {
                    current.unset_possible_at(j, turnoff);
                }

                //Mark off the square
                for j in current.get_square(coord.row, coord.col).iter_coords() {
                    current.unset_possible_at(j, turnoff);
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
            cell::Cell,
            cell_collection::CellCollection,
            constants::{GRID_HEIGHT_RANGE, GRID_WIDTH_RANGE},
            coords::Coord,
            grid::Grid,
            mark::Mark,
        },
        solvers::solver::Solver,
    };

    #[test]
    fn test_solve() {
        let mut grid = Grid::new();
        let coord = Coord::new(4, 3);

        grid.set_cell_at(coord, Cell::new_with_value(5));

        let solver = super::MarkSimple::new();
        let result = solver.solve(grid);
        let modified = result.grid;

        //Check that the row is marked off
        for row in GRID_HEIGHT_RANGE {
            if row == coord.row {
                continue;
            }

            let c = modified.get_cell_at(Coord::new(row, 3));
            assert_eq!(c.is_possible(Mark::N5), false);
        }

        //Check that the column is marked off
        for col in GRID_WIDTH_RANGE {
            if col == coord.col {
                continue;
            }

            let c = modified.get_cell_at(Coord::new(4, col));
            assert_eq!(c.is_possible(Mark::N5), false);
        }

        //Check that the square is marked off
        let square = modified.get_square(coord.row, coord.col);
        for c in square.iter_coords() {
            if coord.row == c.row && coord.row == c.col {
                continue;
            }

            let c = modified.get_cell_at(c);
            assert_eq!(c.is_possible(Mark::N5), false);
        }
    }
}
