use super::solver::{AnnotatedSolverResult, SolveResult, Solver, SolverResult};
use crate::grid::{cell::Cell, cell_collection::CellCollection, grid::Grid};

pub struct SolverManagerConfig {
    pub max_iterations: usize,
}

impl SolverManagerConfig {
    pub fn new() -> Self {
        Self {
            max_iterations: 200,
        }
    }
}

pub struct SolverManager {
    pub config: SolverManagerConfig,

    determined_solver: super::determined_solver::DeterminedSolver,
    is_solved: super::is_solved::IsSolved,
    mark_area_count: super::mark_area_count::MarkAreaCount,
    mark_reset: super::mark_reset::MarkReset,
    mark_shapes: super::mark_shapes::MarkShapes,
    mark_simple: super::mark_simple::MarkSimple,
    mark_survivor: super::mark_survivor::MarkSurvivor,
}

impl SolverManager {
    /// Creates a new solver manager with default settings
    pub fn new() -> Self {
        let default_config = SolverManagerConfig::new();

        SolverManager::new_with_config(default_config)
    }

    /// Creates a new solver manager with the given config
    pub fn new_with_config(config: SolverManagerConfig) -> Self {
        Self {
            config: config,
            determined_solver: super::determined_solver::DeterminedSolver::new(),
            is_solved: super::is_solved::IsSolved::new(),
            mark_area_count: super::mark_area_count::MarkAreaCount::new(),
            mark_reset: super::mark_reset::MarkReset::new(),
            mark_shapes: super::mark_shapes::MarkShapes::new(),
            mark_simple: super::mark_simple::MarkSimple::new(),
            mark_survivor: super::mark_survivor::MarkSurvivor::new(),
        }
    }

    pub fn pre_solve(&self, current: SolverResult) -> SolverResult {
        let mut current = current;
        current.result = SolveResult::Nothing;
        let current = apply_solver(&current, &self.mark_reset);

        return current;
    }

    pub fn solve_round(&self, current: SolverResult) -> SolverResult {
        let mut current = current;
        current.result = SolveResult::Nothing;

        //Markers
        let current = apply_solver(&current, &self.mark_simple);
        if current.result == SolveResult::Solved {
            return current;
        }
        let current = apply_solver(&current, &self.mark_shapes);
        if current.result == SolveResult::Solved {
            return current;
        }
        let current = apply_solver(&current, &self.mark_area_count);
        if current.result == SolveResult::Solved {
            return current;
        }
        let current = apply_solver(&current, &self.mark_survivor);
        if current.result == SolveResult::Solved {
            return current;
        }
        // Solvers
        let current = apply_solver(&current, &self.determined_solver);
        if current.result == SolveResult::Solved {
            return current;
        }
        //Finalizers
        let current = apply_solver(&current, &self.is_solved);
        if current.result == SolveResult::Solved {
            return current;
        }

        return current;
    }

    pub fn solve(&self, grid: Grid) -> AnnotatedSolverResult {
        let result = self.solve_simple(grid);

        if result.result != SolveResult::Solved {
            let mut r = result;
            loop {
                r = self.try_some_stuff(r.grid, r.iterations);
                if r.result == SolveResult::Solved || r.iterations >= self.config.max_iterations {
                    return r;
                }
            }
        }

        result
    }

    pub fn solve_simple(&self, grid: Grid) -> AnnotatedSolverResult {
        self.solve_internal(grid, 0)
    }

    fn solve_internal(&self, grid: Grid, start_iteration: usize) -> AnnotatedSolverResult {
        let mut current = SolverResult {
            result: SolveResult::Updated,
            grid,
        };
        let mut iteration = start_iteration;
        //Pre-solve
        current = self.pre_solve(current);

        // Pre solvers can do a lot of work, but not mark it as solved or updated
        if current.result == SolveResult::Nothing {
            current.result = SolveResult::Updated;
        }

        //While the grid has been updated, keep solving
        while SolveResult::Updated == current.result {
            current = self.solve_round(current);
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

    fn try_some_stuff(&self, grid: Grid, start_iteration: usize) -> AnnotatedSolverResult {
        let mut best_result: Grid = grid;
        let mut solved_amount = grid.count_determined();
        let mut iterations = start_iteration + 1;

        //Just set some cells to see if it works
        for index in grid.iter() {
            let cell = grid.get_cell(index);

            if cell.is_determined() {
                continue;
            }

            for mark in cell.iter_possible() {
                let mut new_grid = grid.clone();
                let c = Cell::new_with_value(mark.to_value());

                new_grid.set_cell(index, c);

                let result = self.solve_internal(new_grid, start_iteration);

                match result.result {
                    SolveResult::Solved => return result,
                    SolveResult::Error => continue,
                    _ => {}
                }

                if result.grid.count_determined() > solved_amount {
                    solved_amount = result.grid.count_determined();
                    best_result = result.grid;
                    iterations = result.iterations;
                }
            }
        }

        AnnotatedSolverResult {
            grid: best_result,
            result: SolveResult::Nothing,
            iterations: iterations,
        }
    }
}

#[inline(always)]
fn apply_solver<T: Solver>(current: &SolverResult, solver: &T) -> SolverResult {
    let old_result = current.result;
    let mut new = solver.solve(&current.grid);
    new.result = old_result.combine(new.result);

    return new;
}
