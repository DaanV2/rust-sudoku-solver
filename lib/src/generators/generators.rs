use rand::{rngs::StdRng, seq::IteratorRandom, Rng, RngCore, SeedableRng};

use crate::{
    grid::{cell::Cell, cell_collection::CellCollection, constants::GRID_SIZE, grid::Grid},
    solvers::{solver::SolveResult, solver_manager::SolverManager},
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
        let grid = &mut Grid::new();
        let mut iter = 0;
        let mut result = self.solvers.pre_solve(grid);

        // Pick a cell, Checks its possible values, and picks a random value from the possible values
        while !SolveResult::is_done(result) && iter < GRID_SIZE {
            iter += 1;
            let index = self.rng.gen_range(0..grid.max());
            let cell = &grid.get_cell(index);

            if cell.is_determined() {
                continue;
            }

            let determined = grid.count_determined();
            if determined > (GRID_SIZE / 3) {
                break;
            }

            let iter = cell.iter_possible();
            if let Some(value) = iter.choose(&mut self.rng) {
                grid.place_value(index, value.to_value());

                // Solve some cells, if it fails, remove the cell
                _ = self.solvers.pre_solve(grid);
                result = self.solvers.solve_round(grid);
                match result {
                    SolveResult::Error => {
                        grid.set_cell(index, &Cell::new());
                    }
                    _ => {}
                }
            }
        }

        if !SolveResult::is_done(result) {
            let r = self.solvers.solve(grid.clone());
            match r.result {
                SolveResult::Solved => {
                    return Some(r.grid);
                }
                _ => {}
            }
        }
        match result {
            SolveResult::Solved => Some(grid.clone()),
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
                grid.set_cell(index, &Cell::new());
                removed += 1;
            }
        }
    }
}

impl Generator<StdRng> {
    /// Creates a new generator with a random seed
    pub fn new_random() -> Self {
        let rng = StdRng::from_entropy();
        Self::new(rng)
    }

    pub fn new_with_seed(seed: u64) -> Self {
        let rng = StdRng::seed_from_u64(seed);
        Self::new(rng)
    }
}
