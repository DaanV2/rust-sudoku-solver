use std::time::Instant;

use rand::{rngs::StdRng, SeedableRng};
use sudoku_solver_lib::{
    generators::generators::Generator, grid::grid::Grid, solvers::solver_manager::SolverManager,
};

pub mod setup;

const RNG_SEED: u64 = 77143266753986;
const SIZE: usize = 5000;

fn main() {
    let mut start_time = Instant::now();
    let mut grids = Vec::with_capacity(SIZE);

    let rng = StdRng::seed_from_u64(RNG_SEED);
    let mut generator = Generator::new(rng);
    println!("Generating {} grids...", SIZE);

    let mut count = 0;

    while count < SIZE {
        if count % 100 == 0 {
            println!("Generated {} grids", count);
        }
        if let Some(grid) = generator.generate() {
            for _ in 0..10 {
                let g: &mut Grid = &mut grid.clone();

                generator.remove_cells(g);
                grids.push(g.clone());
                count += 1;
            }
        }
    }

    let generation_time = start_time.elapsed();
    println!("Done! ");
    println!("Solving...");
    let solver = SolverManager::new();
    let mut iterations = 0;

    let mut solved = 0;
    let mut nothing = 0;
    let mut error = 0;
    let mut updated = 0;

    start_time = Instant::now();

    //Solve all of them
    for i in 0..grids.len() {
        if i % 100 == 0 {
            println!("Solving grid {}...", i);
        }
        let grid = grids.get(i).unwrap().clone();
        let r = solver.solve(grid);

        iterations += r.iterations;
        match r.result {
            sudoku_solver_lib::solvers::solver::SolveResult::Solved => solved += 1,
            sudoku_solver_lib::solvers::solver::SolveResult::Nothing => nothing += 1,
            sudoku_solver_lib::solvers::solver::SolveResult::Error => error += 1,
            sudoku_solver_lib::solvers::solver::SolveResult::Updated => updated += 1,
        }
    }

    let solve_time = start_time.elapsed();
    let size = SIZE as u128;

    println!("Results:");
    println!("  Generation time: {}ns", generation_time.as_nanos());
    println!(
        "  Generation time per: {}ns",
        (generation_time.as_nanos() / size)
    );
    println!("  Solve time: {}ns", solve_time.as_nanos());
    println!("  Solve time per: {}ns", solve_time.as_nanos() / size);
    println!("  Iterations: {}", iterations);
    println!("  Solved: {}", solved);
    println!("  Nothing: {}", nothing);
    println!("  Error: {}", error);
    println!("  Updated: {}", updated);
    println!(
        "csv: {},{},{},{},{},{},{},{},{},{}",
        size,
        generation_time.as_nanos(),
        solve_time.as_nanos(),
        generation_time.as_nanos() / size,
        solve_time.as_nanos() / size,
        iterations,
        solved,
        nothing,
        error,
        updated
    );
}
