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
    fn name(&self) -> &'static str {
        "Mark Resetter"
    }

    fn solve(&self, grid: Grid) -> SolverResult {
        let mut current = grid.clone();

        for i in grid.iter() {
            let cell = current.get_cell(i);

            // If the cell is not determined, then we need to reset the marks
            if !cell.is_determined() {
                current.set_cell(i, &Cell::new());
            }
        }

        SolverResult::nothing(current)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{
        grid::possibility::Possibility, solvers::mark_reset::MarkReset, test::util::general_tests,
    };

    #[test]
    fn test_mark_reset() {
        let mut grid = general_tests::filled_sudoku();

        let index = 12;
        let cell = Cell::new_with_value(0);

        grid.set_cell(index, &cell);

        let original = grid.get_cell(index);
        //Checking it has been set properly
        assert_eq!(
            original.possibilities,
            Possibility::empty(),
            "Cell should be empty"
        );

        let solver = MarkReset::new();
        let result = solver.solve(grid);

        //Checking it has been reset
        let set = result.grid.get_cell(index);
        assert_eq!(
            set.possibilities,
            Possibility::new(),
            "Cell should be set again"
        );
    }
}
