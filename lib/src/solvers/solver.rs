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
            grid: other.grid,
        }
    }

    pub fn update(grid: Grid) -> Self {
        Self {
            result: SolveResult::Updated,
            grid: grid,
        }
    }

    pub fn nothing(grid: Grid) -> Self {
        Self {
            result: SolveResult::Nothing,
            grid: grid,
        }
    }

    pub fn solved(grid: Grid) -> Self {
        Self {
            result: SolveResult::Solved,
            grid: grid,
        }
    }
}

pub struct AnnotatedSolverResult {
    pub result: SolveResult,
    pub grid: Grid,
    pub iterations: usize,
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
