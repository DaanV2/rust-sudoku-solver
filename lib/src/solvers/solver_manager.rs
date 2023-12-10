use super::{
    determined_solver::DeterminedSolver,
    is_solved::IsSolved,
    mark_reset::MarkReset,
    mark_shapes::MarkShapes,
    mark_simple::MarkSimple,
    mark_survivor::MarkSurvivor,
    solver::{AnnotatedSolverResult, SolveResult},
    validator::is_valid,
};
use crate::grid::{cell_collection::CellCollection, grid::Grid};

pub struct SolverManagerConfig {
    pub max_iterations: usize,
}

impl SolverManagerConfig {
    pub fn new() -> Self {
        Self {
            max_iterations: 1000,
        }
    }
}

pub struct SolverManager {
    pub config: SolverManagerConfig,
}

impl SolverManager {
    /// Creates a new solver manager with default settings
    pub fn new() -> Self {
        let default_config = SolverManagerConfig::new();

        SolverManager::new_with_config(default_config)
    }

    /// Creates a new solver manager with the given config
    pub fn new_with_config(config: SolverManagerConfig) -> Self {
        Self { config }
    }

    pub fn pre_solve(&self, grid: &mut Grid) -> SolveResult {
        if MarkReset::solve(grid) == SolveResult::Solved {
            return SolveResult::Solved;
        }

        MarkSimple::solve(grid)
    }

    pub fn solve_round(&self, grid: &mut Grid) -> SolveResult {
        //Markers

        if MarkShapes::solve(grid) == SolveResult::Solved {
            return SolveResult::Solved;
        }
        // if MarkAreaCount::solve(grid) == SolveResult::Solved {
        //     return SolveResult::Solved;
        // }
        // Solvers
        if MarkSurvivor::solve(grid) == SolveResult::Solved {
            return SolveResult::Solved;
        }
        if DeterminedSolver::solve(grid) == SolveResult::Solved {
            return SolveResult::Solved;
        }
        //Finalizers
        IsSolved::solve(grid)
    }

    pub fn solve(&self, grid: Grid) -> AnnotatedSolverResult {
        let mut current = &mut grid.clone();
        let mut result = self.solve_simple(current);

        if result.result != SolveResult::Solved {
            loop {
                result = self.try_some_stuff(current, result.iterations);
                if result.result == SolveResult::Solved
                    || result.iterations >= self.config.max_iterations
                {
                    return result;
                }
                current = &mut result.grid;
            }
        }

        result
    }

    pub fn solve_simple(&self, grid: &mut Grid) -> AnnotatedSolverResult {
        self.solve_internal(grid, 0)
    }

    fn solve_internal(&self, grid: &mut Grid, start_iteration: usize) -> AnnotatedSolverResult {
        let mut iteration = start_iteration;
        //Pre-solve
        let mut current = self.pre_solve(grid);

        // Pre solvers can do a lot of work, but not mark it as solved or updated
        if current == SolveResult::Nothing {
            current = SolveResult::Updated;
        }

        //While the grid has been updated, keep solving
        while SolveResult::Updated == current {
            current = self.solve_round(grid);
            if current == SolveResult::Solved {
                break;
            }

            iteration += 1;

            if iteration >= self.config.max_iterations {
                break;
            }
        }

        AnnotatedSolverResult {
            grid: grid.clone(),
            result: current,
            iterations: iteration,
        }
    }

    fn try_some_stuff(&self, grid: &Grid, start_iteration: usize) -> AnnotatedSolverResult {
        let best_result = &mut grid.clone();
        let mut solved_amount = grid.count_determined();
        let mut iterations = start_iteration + 1;

        //Used as a buffer
        let new_grid = &mut Grid::empty();

        //Just set some cells to see if it works
        for index in grid.iter() {
            let cell = grid.get_cell(index);

            if cell.is_determined() {
                continue;
            }

            for mark in cell.iter_possible() {
                grid.clone_to(new_grid);
                new_grid.place_value(index, mark.to_value());

                let result = self.solve_internal(new_grid, start_iteration);
                if result.result == SolveResult::Error || !is_valid(&result.grid) {
                    continue;
                }
                if result.result == SolveResult::Solved {
                    return result;
                }

                if result.grid.count_determined() > solved_amount {
                    result.grid.clone_to(best_result);
                    solved_amount = best_result.count_determined();
                    iterations = result.iterations;
                }
            }
        }

        AnnotatedSolverResult {
            grid: best_result.clone(),
            result: SolveResult::Nothing,
            iterations: iterations,
        }
    }
}
