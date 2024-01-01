use rand::{rngs::StdRng, seq::IteratorRandom, RngCore, SeedableRng};

use crate::grid::{cell_collection::CellCollection, grid::Grid, square::Square};

use super::{solver::SolveResult, solver_manager::SolverManager};

/// A solver that uses a random number generator to solve the puzzle, forcefully goes through each square
///
/// ```rust
/// let mut solver = FastSolver::new_random();
/// ```
pub struct FastSolver<T: RngCore> {
    pub rng: T,
    pub solvers: SolverManager,
}

impl<T: RngCore> FastSolver<T> {
    /// Creates a new generator
    pub fn new(rng: T) -> Self {
        let mut solvers = SolverManager::new();
        solvers.config.max_iterations = 10;

        Self { solvers, rng }
    }

    pub fn solve(&mut self, source: &Grid) -> Grid {
        loop {
            match self.solve_round(source) {
                Some(grid) => return grid,
                None => continue,
            }
        }
    }

    fn solve_round(&mut self, source: &Grid) -> Option<Grid> {
        let grid = &mut source.clone();
        self.solvers.pre_solve(grid);

        for sq in Square::iter_squares() {
            let mut count = 3;
            loop {
                let buf = &mut grid.clone();
                let result = self.determine_area(buf, sq);
                if result == SolveResult::Solved {
                    return Some(buf.clone());
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

        let result = self.solvers.solve(grid.clone());
        return match result.result {
            SolveResult::Solved => Some(result.grid),
            _ => None,
        };
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
}

impl FastSolver<StdRng> {
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
