use crate::grid::{mark::Mark, searchable::Searchable};

use super::solver::Solver;

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
    fn solve(&self, grid: crate::grid::grid::Grid) -> super::solver::SolverResult {
        let mut current: crate::grid::grid::Grid = grid.clone();

        for i in grid.iter() {
            let cell = current.get(i);

            //If the cell is determined, mark off that square, row and column
            if cell.is_determined() {
                let turnoff = Mark::from_index(cell.value as usize);
                let coord = current.get_coord(i);

                //Mark off the row
                for j in current.get_row(coord.row).iter_coords() {
                    let mut new_cell = current.get_cell(j).clone();
                    new_cell.set_state(turnoff, false);
                    current.set_cell(j, new_cell);
                }

                //Mark off the column
                for j in current.get_column(coord.col).iter_coords() {
                    let mut new_cell = current.get_cell(j).clone();
                    new_cell.set_state(turnoff, false);
                    current.set_cell(j, new_cell);
                }

                //Mark off the square
                for j in current.get_square(coord.row, coord.col).iter_coords() {
                    let mut new_cell = current.get_cell(j).clone();
                    new_cell.set_state(turnoff, false);
                    current.set_cell(j, new_cell);
                }
            }
        }

        super::solver::SolverResult {
            result: super::solver::SolveResult::Nothing,
            grid: current,
        }
    }
}
