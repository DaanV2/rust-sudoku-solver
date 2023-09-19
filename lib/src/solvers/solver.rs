use crate::grid::grid::Grid;

pub trait Solver {
    fn name(&self) -> &'static str;
    /// Solves the given grid and returns the result.
    fn solve(&self, grid: Grid) -> SolverResult;
}

#[derive(Debug, Clone, Copy)]
pub struct SolverResult {
    pub result: SolveResult,
    pub grid: Grid,
}

impl SolverResult {
    pub fn new(grid: Grid, result: SolveResult) -> Self {
        Self {
            result: result,
            grid: grid,
        }
    }

    #[inline]
    pub fn combine(&self, other: SolverResult) -> Self {
        SolverResult::new(other.grid, self.result.combine(other.result))
    }

    #[inline]
    pub fn update(grid: Grid) -> Self {
        SolverResult::new(grid, SolveResult::Updated)
    }

    #[inline]
    pub fn nothing(grid: Grid) -> Self {
        SolverResult::new(grid, SolveResult::Nothing)
    }

    #[inline]
    pub fn solved(grid: Grid) -> Self {
        SolverResult::new(grid, SolveResult::Solved)
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
    Error = 3,
}

impl SolveResult {
    pub fn combine(self, other: SolveResult) -> Self {
        if other as usize >= self as usize {
            return other;
        }

        return self;
    }
}
