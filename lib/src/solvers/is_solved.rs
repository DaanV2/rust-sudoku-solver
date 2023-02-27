use crate::grid::{cell_collection::CellCollection, grid::Grid};

use super::solver::{SolveResult, Solver, SolverResult};

pub struct IsSolved {}

impl IsSolved {
    pub fn new() -> Self {
        Self {}
    }

    pub fn new_box() -> Box<Self> {
        Box::new(Self::new())
    }
}

impl Solver for IsSolved {
    fn solve(&self, grid: Grid) -> SolverResult {
        let current: Grid = grid.clone();
        let mut result = SolveResult::Solved;

        for i in grid.iter() {
            let cell = current.get_cell(i);

            // If the cell is not determined, then we need to reset the marks
            if !cell.is_determined() {
                result = SolveResult::Nothing;
                break;
            }
        }

        SolverResult {
            result,
            grid: current,
        }
    }
}
