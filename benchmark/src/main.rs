use std::time::Instant;

use sudoku_solver_lib::{
    generators::generators::Generator, grid::grid::Grid, solvers::solver_manager::SolverManager,
};

use crate::data::datapoint::DataPoint;

pub mod data;
pub mod setup;

const RNG_SEED: u64 = 77143266753986;
const SIZE: usize = 5000;

fn main() {
    let mut points = Vec::with_capacity(20);

    for _ in 0..5 {
        let point = run_random_test(SIZE);
        points.push(point);
    }

    // for i in (10..80).step_by(5) {
    //     let point = run_test(SIZE, i);
    //     points.push(point);
    // }

    // Output the results to run.csv
    let mut csv = String::new();
    csv.push_str(&DataPoint::csv_headers());
    csv.push('\n');
    for point in points.iter() {
        csv.push_str(&point.to_csv());
        csv.push('\n');
    }

    std::fs::write("run.csv", csv).expect("Unable to write file");

    let avg = DataPoint::avg(points);
    println!("Average:\n{}", avg.to_csv());
}

#[allow(dead_code)]
fn run_random_test(size: usize) -> DataPoint {
    println!("Running test with {} grids and random cells removed", size);
    let step = size / 10;

    let mut grids = Vec::with_capacity(size);

    let mut generator = Generator::new_with_seed(RNG_SEED);
    println!("Generating {} grids...", size);

    let start_time = Instant::now();
    let mut count = 0;

    while count < size {
        if count % step == 0 {
            println!("Generated {} grids", count);
        }
        if let Some(grid) = generator.generate() {
            // println!("{}", grid);
            // validate_grid(&grid);
            let g: &mut Grid = &mut grid.clone();

            generator.remove_cells(g);
            grids.push(g.clone());
            count += 1;
        }
    }

    let generation_time = start_time.elapsed();

    let mut point = DataPoint::empty();
    point.generation_time = generation_time.as_nanos();
    point.generation_time_per = generation_time.as_nanos() / grids.len() as u128;

    point = solve(grids, point);
    println!("Done! ");

    point
}

#[allow(dead_code)]
fn run_test(size: usize, remove_cells: usize) -> DataPoint {
    println!(
        "Running test with {} grids and {} cells removed",
        size, remove_cells
    );
    let step = size / 10;

    let mut grids = Vec::with_capacity(size);

    let mut generator = Generator::new_with_seed(RNG_SEED);
    println!("Generating {} grids...", size);

    let start_time = Instant::now();
    let mut count = 0;

    while count < size {
        if count % step == 0 {
            println!("Generated {} grids", count);
        }
        if let Some(grid) = generator.generate() {
            // validate_grid(&grid);
            let g: &mut Grid = &mut grid.clone();

            generator.remove_cells_amount(g, remove_cells);
            grids.push(g.clone());
            count += 1;
        }
    }

    let generation_time = start_time.elapsed();

    let mut point = DataPoint::empty();
    point.generation_time = generation_time.as_nanos();
    point.generation_time_per = generation_time.as_nanos() / grids.len() as u128;

    point = solve(grids, point);
    println!("Done! ");

    point
}

fn solve(grids: Vec<Grid>, point: DataPoint) -> DataPoint {
    let solver = SolverManager::new();
    let size = grids.len();
    let step = size / 10;
    let mut iterations = 0;

    let mut solved = 0;
    let mut nothing = 0;
    let mut error = 0;
    let mut updated = 0;

    println!("Solving...");
    let start_time = Instant::now();

    //Solve all of them
    for i in 0..size {
        if i % step == 0 {
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
    let size128 = size as u128;

    DataPoint {
        updated,
        solved,
        nothing,
        error,
        iterations,
        size,
        solve_time: solve_time.as_nanos(),
        solve_time_per: solve_time.as_nanos() / size128,
        generation_time: point.generation_time,
        generation_time_per: point.generation_time_per,
    }
}
