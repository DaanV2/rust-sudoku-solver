use super::solver::{AnnotatedSolverResult, SolveResult, Solver, SolverResult};
use crate::grid::{cell::Cell, cell_collection::CellCollection, grid::Grid};

pub struct SolverManager {
    pub pre_solvers: Vec<Box<dyn Solver>>,
    pub solvers: Vec<Box<dyn Solver>>,

    pub config: SolverManagerConfig,
}

pub struct SolverManagerConfig {
    pub max_iterations: usize,
}

impl SolverManagerConfig {
    pub fn new() -> Self {
        Self {
            max_iterations: 100,
        }
    }
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
            pre_solvers: vec![
                // Setups for other solvers
                super::mark_reset::MarkReset::new_box(),
            ],
            solvers: vec![
                //Markers
                super::mark_simple::MarkSimple::new_box(),
                super::mark_shapes::MarkShapes::new_box(),
                super::mark_area_count::MarkAreaCount::new_box(),
                super::mark_survivor::MarkSurvivor::new_box(),
                // Solvers
                super::determined_solver::DeterminedSolver::new_box(),
                //Finalizers
                super::is_solved::IsSolved::new_box(),
            ],
        }
    }

    pub fn pre_solve(&self, current: SolverResult) -> SolverResult {
        apply_solvers(current, &self.pre_solvers)
    }

    pub fn solve_round(&self, current: SolverResult) -> SolverResult {
        return apply_solvers(current, &self.solvers);
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

            for mark in cell.possibilities.iter_possible() {
                let mut new_grid = grid.clone();
                let c = Cell::new_with_value(mark.to_value());

                new_grid.set_cell(index, &c);

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
fn apply_solvers(mut current: SolverResult, solvers: &Vec<Box<dyn Solver>>) -> SolverResult {
    //Reset the result
    current.result = SolveResult::Nothing;

    for solver in solvers {
        //println!("Solver: {}", solver.name());
        let old_result = current.result;
        current = solver.solve(current.grid);
        current.result = old_result.combine(current.result);

        if current.result == SolveResult::Solved {
            break;
        }
    }

    return current;
}
