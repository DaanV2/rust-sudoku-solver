use super::solver::{SolveResult, Solver};
use crate::grid::{cell::Cell, cell_collection::CellCollection, grid::Grid};

pub struct MarkReset {}

impl MarkReset {
    pub fn new() -> Self {
        Self {}
    }

    pub fn new_box() -> Box<Self> {
        Box::new(Self::new())
    }

    pub fn solve(grid: &mut Grid) -> SolveResult {
        for i in grid.iter() {
            let mut cell = *grid.get_cell(i);

            // If the cell is not determined, then we need to reset the marks
            if !cell.is_determined() {
                cell = Cell::new();
            }

            grid.set_cell(i, &cell);
        }

        SolveResult::Nothing
    }
}

impl Solver for MarkReset {
    fn name(&self) -> &'static str {
        "Mark Resetter"
    }

    fn solve(&self, grid: &mut Grid) -> SolveResult {
        MarkReset::solve(grid)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{solvers::mark_reset::MarkReset, test::util::general_tests};

    #[test]
    fn test_mark_reset() {
        let grid = &mut general_tests::filled_sudoku();

        let index = 12;
        let cell = Cell::new_with_value(0);

        grid.set_cell(index, &cell);

        let original = grid.get_cell(index);
        //Checking it has been set properly
        assert_eq!(original.possible_count(), 0, "Cell should be empty");

        MarkReset::solve(grid);

        //Checking it has been reset
        let set = grid.get_cell(index);
        assert_eq!(set.possible_count(), 9, "Cell should be set again");
    }
}
