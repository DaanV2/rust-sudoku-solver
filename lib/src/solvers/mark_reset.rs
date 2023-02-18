use crate::grid::{grid::Grid, searchable::Searchable};

use super::solver::{SolverResult, Solver, SolveResult};




pub struct MarkReset {
}

impl MarkReset {
    pub fn new() -> Self {
        Self {
        }
    }

    pub fn new_box() -> Box<Self> {
        Box::new(Self::new())
    }
}

impl Solver for MarkReset {
    fn solve(&self, grid: Grid) -> SolverResult {
        let mut current: Grid = grid.clone();
        
        for i in grid.iter() {
            let cell = current.get(i);

            // If the cell is not determined, then we need to reset the marks
            if !cell.is_determined() {
                let mut new_cell = cell.clone();
                new_cell.reset_possibilities();

                current.set(i, new_cell)
            }
        }

        SolverResult {
            result: SolveResult::Nothing,
            grid: current,
        }
    }
}