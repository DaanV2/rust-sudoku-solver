use rand::{seq::IteratorRandom, Rng, RngCore};

use crate::{
    grid::{cell::Cell, cell_collection::CellCollection, constants::GRID_SIZE, grid::Grid},
    solvers::{
        solver::{SolveResult, SolverResult},
        solver_manager::SolverManager,
    },
};

pub struct Generator<T: RngCore> {
    pub solvers: SolverManager,
    pub rng: T,
}

impl<T: RngCore> Generator<T> {
    /// Creates a new generator
    pub fn new(rng: T) -> Self {
        let mut solver = SolverManager::new();
        solver.config.max_iterations = 10;

        Self {
            solvers: solver,
            rng: rng,
        }
    }

    /// Generates a new grid
    pub fn generate(&mut self) -> Option<Grid> {
        let grid = Grid::new();
        let mut result = SolverResult::new(grid, SolveResult::Nothing);
        let mut iter = 0;
        result = self.solvers.pre_solve(result);

        // Pick a cell, Checks its possible values, and picks a random value from the possible values
        while !SolveResult::is_done(result.result) && iter < GRID_SIZE {
            iter += 1;
            let index = self.rng.gen_range(0..result.grid.max());
            let cell = &result.grid.get_cell(index);

            if cell.is_determined() {
                continue;
            }

            let determined = result.grid.count_determined();
            if determined > (GRID_SIZE / 3) {
                break;
            }

            let iter = cell.iter_possible();
            if let Some(value) = iter.choose(&mut self.rng) {
                let c = Cell::new_from_mark_as_value(value);

                result.grid.set_cell(index, c);

                // Solve some cells, if it fails, remove the cell
                let mut r = self.solvers.pre_solve(result);
                r = self.solvers.solve_round(r);
                match r.result {
                    SolveResult::Error => {
                        result.grid.set_cell(index, Cell::new());
                    }
                    _ => {
                        result = r;
                    }
                }
            }
        }

        if !SolveResult::is_done(result.result) {
            let r = self.solvers.solve(result.grid);
            result = SolverResult {
                grid: r.grid,
                result: r.result,
            }
        }
        match result.result {
            SolveResult::Solved => Some(result.grid),
            _ => None,
        }
    }

    /// Removes a random amount of cells from the grid
    pub fn remove_cells(&mut self, grid: &mut Grid) {
        let amount = self.rng.gen_range(0..grid.max());

        self.remove_cells_amount(grid, amount);
    }

    /// Removes a random amount of cells from the grid
    pub fn remove_cells_amount(&mut self, grid: &mut Grid, amount: usize) {
        let mut removed = 0;

        while removed < amount {
            let index = self.rng.gen_range(0..grid.max());
            let cell = &grid.get_cell(index);

            if cell.is_determined() {
                grid.set_cell(index, Cell::new());
                removed += 1;
            }
        }
    }
}
