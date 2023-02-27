use super::solver::{Solver, SolverResult};
use crate::grid::{cell::Cell, cell_collection::CellCollection, grid::Grid};

pub struct MarkReset {}

impl MarkReset {
    pub fn new() -> Self {
        Self {}
    }

    pub fn new_box() -> Box<Self> {
        Box::new(Self::new())
    }
}

impl Solver for MarkReset {
    fn solve(&self, grid: Grid) -> SolverResult {
        let mut current: Grid = grid.clone();

        for i in grid.iter() {
            let cell = current.get_cell(i);

            // If the cell is not determined, then we need to reset the marks
            if !cell.is_determined() {
                current.set_cell(i, &Cell::new());
            }
        }

        SolverResult::nothing(grid)
    }
}
