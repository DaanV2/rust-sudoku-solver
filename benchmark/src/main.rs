use std::time::Instant;

use rand::{rngs::StdRng, SeedableRng};
use sudoku_solver_lib::{
    generators::generators::Generator, grid::grid::Grid, solvers::solver_manager::SolverManager,
};

pub mod setup;

const RNG_SEED: u64 = 77143266753986;
const SIZE: usize = 200;

fn main() {
    let mut start_time = Instant::now();
    let mut grids = Vec::with_capacity(SIZE);

    let rng = StdRng::seed_from_u64(RNG_SEED);
    let mut generator = Generator::new(rng);
    println!("Generating {} grids...", SIZE);

    let mut count = 0;

    while count < SIZE {
        println!("--- Generating a grid {} ---", count);
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

    start_time = Instant::now();

    //Solve all of them
    for i in 0..grids.len() {
        println!("Solving grid {}...", i);
        let grid = grids.get(i).unwrap().copy();
        solver.solve(grid);
    }

    let solve_time = start_time.elapsed();

    println!("Results:");
    println!("  Generation time: {}ms", generation_time.as_millis());
    println!("  Solve time: {}ms", solve_time.as_millis());
    println!(
        "csv: {}\t{}\t{}",
        SIZE,
        generation_time.as_millis(),
        solve_time.as_millis()
    );
}
