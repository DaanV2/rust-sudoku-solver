use rand::{rngs::StdRng, seq::IteratorRandom, Rng, RngCore, SeedableRng};

use crate::{
    grid::{cell::Cell, cell_collection::CellCollection, grid::Grid, square::Square},
    solvers::{solver::SolveResult, solver_manager::SolverManager},
};

pub struct Generator<T: RngCore> {
    pub solvers: SolverManager,
    pub rng: T,
}

impl<T: RngCore> Generator<T> {
    /// Creates a new generator
    pub fn new(rng: T) -> Self {
        let mut solvers = SolverManager::new();
        solvers.config.max_iterations = 100;

        Self { solvers, rng }
    }

    /// Generates a new grid
    pub fn generate(&mut self) -> Grid {
        loop {
            let grid = &mut Grid::new();

            for sq in Square::iter_squares() {
                let mut count = 3;
                loop {
                    let buf = &mut grid.clone();
                    let result = self.determine_area(buf, sq);
                    if result == SolveResult::Solved {
                        return buf.clone();
                    }
                    if result == SolveResult::Error {
                        count -= 1;
                        if count == 0 {
                            break;
                        }

                        continue;
                    }

                    // println!("{}", buf);
                    *grid = buf.clone();
                    break;
                }
            }
        }
    }

    fn determine_area<U: CellCollection>(&mut self, grid: &mut Grid, area: U) -> SolveResult {
        for index in area.iter() {
            let coord = area.get_coord(index);
            let cell = &grid.get_cell_at(coord);
            if cell.is_determined() {
                continue;
            }

            let iter = cell.iter_possible();

            match iter.choose(&mut self.rng) {
                Some(value) => grid.place_value_at(coord, value.to_value()),
                None => return SolveResult::Error,
            }
        }

        return self.solvers.pre_solve(grid) | self.solvers.solve_round(grid);
    }

    /// Removes a random amount of cells from the grid
    pub fn remove_cells(&mut self, grid: &mut Grid) {
        let amount = self.rng.gen_range(0..80);

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generator() {
        let mut generator = Generator::new_with_seed(77143266753986);
        let grid = generator.generate();

        println!("{}", grid);

        assert_eq!(grid.count_determined(), 81);
    }
}
