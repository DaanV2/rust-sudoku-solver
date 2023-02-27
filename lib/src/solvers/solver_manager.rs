use super::solver::{AnnotatedSolverResult, SolveResult, Solver, SolverResult};
use crate::grid::grid::Grid;

pub struct SolverManager {
    pub solvers: Vec<Box<dyn Solver>>,

    pub config: SolverManagerConfig,
}

pub struct SolverManagerConfig {
    pub max_iterations: usize,
}

impl SolverManager {
    pub fn new() -> Self {
        Self {
            solvers: vec![
                // Setups for other solvers
                super::mark_reset::MarkReset::new_box(),
                super::mark_simple::MarkSimple::new_box(),
                // super::mark_shapes::MarkShapes::new_box(),
                // Solvers
                super::determined_solver::DeterminedSolver::new_box(),
                // Finalizers
                super::is_solved::IsSolved::new_box(),
            ],
            config: SolverManagerConfig {
                max_iterations: 1000,
            },
        }
    }

    pub fn solve(&self, grid: Grid) -> AnnotatedSolverResult {
        let mut current = SolverResult {
            result: SolveResult::Updated,
            grid,
        };
        let mut iteration = 0;

        //While the grid has been updated, keep solving
        while SolveResult::Updated == current.result {
            let result = self.solve_round(current);
            current = current.combine(result);

            if current.result == SolveResult::Solved {
                break;
            }

            iteration += 1;

            if iteration >= self.config.max_iterations {
                break;
            }
        }

        AnnotatedSolverResult {
            grid: current.grid,
            result: current.result,
            iterations: iteration,
        }
    }

    pub fn solve_round(&self, mut grid: SolverResult) -> SolverResult {
        //Reset the result
        grid.result = SolveResult::Nothing;

        for solver in &self.solvers {
            let solver_result = solver.solve(grid.grid);
            grid = grid.combine(solver_result);

            if grid.result == SolveResult::Solved {
                break;
            }
        }

        return grid;
    }
}
