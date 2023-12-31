use std::{
    fmt::{Display, Formatter},
    ops::{BitOr, BitOrAssign},
};

use crate::grid::grid::Grid;

pub trait Solver {
    fn name(&self) -> &'static str;
    /// Solves the given grid and returns the result.
    fn solve(&self, grid: &mut Grid) -> SolveResult;
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

impl Display for AnnotatedSolverResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "result: {}, iters: {}\n{}",
            self.result, self.iterations, self.grid
        )
    }
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
        if self as usize >= other as usize {
            return self;
        }

        return other;
    }

    pub fn is_done(self) -> bool {
        match self {
            SolveResult::Solved | SolveResult::Error => true,
            _ => false,
        }
    }

    /// Returns updated if changed is true, otherwise nothing.
    pub fn from_changed(changed: bool) -> Self {
        match changed {
            true => SolveResult::Updated,
            false => SolveResult::Nothing,
        }
    }
}

impl Display for SolveResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SolveResult::Nothing => write!(f, "Nothing"),
            SolveResult::Updated => write!(f, "Updated"),
            SolveResult::Solved => write!(f, "Solved"),
            SolveResult::Error => write!(f, "Error"),
        }
    }
}

impl BitOr for SolveResult {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        self.combine(rhs)
    }
}

impl BitOrAssign for SolveResult {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = self.combine(rhs);
    }
}

#[cfg(test)]
mod test {
    use crate::solvers::solver::SolveResult;

    #[test]
    pub fn test_combine_nothing() {
        let current = SolveResult::Nothing;
        assert_eq!(current.combine(SolveResult::Nothing), SolveResult::Nothing);
        assert_eq!(current.combine(SolveResult::Updated), SolveResult::Updated);
        assert_eq!(current.combine(SolveResult::Solved), SolveResult::Solved);
        assert_eq!(current.combine(SolveResult::Error), SolveResult::Error);
    }

    #[test]
    pub fn test_combine_updated() {
        let current = SolveResult::Updated;
        assert_eq!(current.combine(SolveResult::Nothing), SolveResult::Updated);
        assert_eq!(current.combine(SolveResult::Updated), SolveResult::Updated);
        assert_eq!(current.combine(SolveResult::Solved), SolveResult::Solved);
        assert_eq!(current.combine(SolveResult::Error), SolveResult::Error);
    }

    #[test]
    pub fn test_combine_solved() {
        let current = SolveResult::Solved;
        assert_eq!(current.combine(SolveResult::Nothing), SolveResult::Solved);
        assert_eq!(current.combine(SolveResult::Updated), SolveResult::Solved);
        assert_eq!(current.combine(SolveResult::Solved), SolveResult::Solved);
        assert_eq!(current.combine(SolveResult::Error), SolveResult::Error);
    }

    #[test]
    pub fn test_combine_error() {
        let current = SolveResult::Error;
        assert_eq!(current.combine(SolveResult::Nothing), SolveResult::Error);
        assert_eq!(current.combine(SolveResult::Updated), SolveResult::Error);
        assert_eq!(current.combine(SolveResult::Solved), SolveResult::Error);
        assert_eq!(current.combine(SolveResult::Error), SolveResult::Error);
    }
}
