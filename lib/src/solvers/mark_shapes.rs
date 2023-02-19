use crate::grid::{grid::Grid, mark::Mark, searchable::Searchable};

use super::solver::{SolveResult, Solver, SolverResult};

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
        solve_private(grid)
    }
}
