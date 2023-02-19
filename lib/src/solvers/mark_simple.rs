use crate::grid::{grid::Grid, mark::Mark, searchable::Searchable};

use super::solver::{SolveResult, Solver, SolverResult};

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
    fn solve(&self, grid: Grid) -> SolverResult {
        let mut current = grid.clone();

        for i in grid.iter() {
            let cell = current.get(i);

            //If the cell is determined, mark off that square, row and column
            if cell.is_determined() {
                let turnoff = Mark::from_index(cell.value as usize);
                let coord = current.get_coord(i);

                //Mark off the row
                for j in current.get_row(coord.row).iter_coords() {
                    let mut new_cell = current.get_cell(j).clone();
                    new_cell.unset(turnoff);
                    current.set_cell(j, new_cell);
                }

                //Mark off the column
                for j in current.get_column(coord.col).iter_coords() {
                    let mut new_cell = current.get_cell(j).clone();
                    new_cell.unset(turnoff);
                    current.set_cell(j, new_cell);
                }

                //Mark off the square
                for j in current.get_square(coord.row, coord.col).iter_coords() {
                    let mut new_cell = current.get_cell(j).clone();
                    new_cell.unset(turnoff);
                    current.set_cell(j, new_cell);
                }
            }
        }

        SolverResult {
            result: SolveResult::Nothing,
            grid: current,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{
        grid::{cell::Cell, constants::GRID_HEIGHT_RANGE, coords::Coord, grid::Grid, mark::Mark},
        solvers::solver::Solver,
    };

    #[test]
    fn test_solve() {
        let mut grid = Grid::new();
        let coord = Coord::new(4, 3);

        grid.set_cell(coord, Cell::new_with_value(5));

        let solver = super::MarkSimple::new();
        let result = solver.solve(grid);
        let modified = result.grid;

        //Check that the row is marked off
        for row in GRID_HEIGHT_RANGE {
            if row == coord.row {
                continue;
            }

            let c = modified.get_cell(Coord::new(row, 3));
            assert_eq!(c.is_possible(Mark::N5), false);
        }

        //Check that the column is marked off
        for col in GRID_HEIGHT_RANGE {
            if col == coord.col {
                continue;
            }

            let c = modified.get_cell(Coord::new(4, col));
            assert_eq!(c.is_possible(Mark::N5), false);
        }

        //Check that the square is marked off
        for row in 3..6 {
            for col in 0..3 {
                if row == coord.row && col == coord.col {
                    continue;
                }

                let c = modified.get_cell(Coord::new(row, col));
                assert_eq!(c.is_possible(Mark::N5), false);
            }
        }
    }
}
