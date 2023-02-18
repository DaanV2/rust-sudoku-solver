use crate::grid::grid::Grid;

pub trait Solver {
    /// Solves the given grid and returns the result.
    fn solve(&self, grid: Grid) -> SolverResult;
}

#[derive(Debug, Clone, Copy)]
pub struct SolverResult {
    pub result: SolveResult,
    pub grid: Grid,
}

impl SolverResult {
    pub fn combine(&self, other: SolverResult) -> Self {
        Self {
            result: self.result.combine(other.result),
            grid: self.grid,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum SolveResult {
    Nothing = 0,
    Updated = 1,
    Solved = 2,
}

impl SolveResult {
    pub fn combine(self, other: SolveResult) -> Self {
        if other as usize >= self as usize {
            return other;
        }

        return self;
    }
}
