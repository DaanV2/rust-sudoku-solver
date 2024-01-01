use crate::grid::{cell::Cell, cell_collection::CellCollection, grid::Grid};

use super::solver::{SolveResult, Solver};

pub struct IsSolved {}

impl IsSolved {
    pub fn new() -> Self {
        Self {}
    }

    pub fn new_box() -> Box<Self> {
        Box::new(Self::new())
    }

    pub fn solve(grid: &Grid) -> SolveResult {
        let mut check_possible = Cell::new_empty();
        let mut check_filled = true;

        for i in grid.iter() {
            let c = grid.get_cell(i);
            check_possible = check_possible | *c;
            check_filled &= c.get_value() > 0;
        }

        if check_possible.only_possible().is_empty() {
            if !check_filled {
                return SolveResult::Error;
            }
            return SolveResult::Solved;
        }

        return SolveResult::Nothing;
    }
}

impl Solver for IsSolved {
    fn name(&self) -> &'static str {
        "Is Solved"
    }

    fn solve(&self, grid: &mut Grid) -> SolveResult {
        IsSolved::solve(grid)
    }
}

#[cfg(test)]
mod tests {
    use super::IsSolved;
    use crate::{grid::cell::Cell, test::util::general_tests};

    #[test]
    fn test_filled_is_solved() {
        let grid = &mut general_tests::filled_sudoku();
        let result = IsSolved::solve(grid);

        assert_eq!(result, super::SolveResult::Solved);
    }

    #[test]
    fn test_missing_one() {
        let grid = &mut general_tests::filled_sudoku();
        grid.set_cell(33, &Cell::new());

        let result = IsSolved::solve(grid);

        assert_eq!(result, super::SolveResult::Nothing);
    }
}
