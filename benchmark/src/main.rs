use std::time::Instant;

use sudoku_solver_lib::{
    generators::generators::Generator,
    grid::grid::Grid,
    solvers::{fast_solver::FastSolver, validator::is_valid},
};

use crate::data::datapoint::DataPoint;

pub mod data;
pub mod setup;

const RNG_SEED: u64 = 77143266753986;
const SOLVE_SEED: u64 = 85822788013146;
const SIZE: usize = 10000;

struct GridSet {
    #[allow(dead_code)]
    pub original: Grid,
    pub to_solve: Grid,
}

impl GridSet {
    pub fn new(original: Grid, to_solve: Grid) -> GridSet {
        GridSet { original, to_solve }
    }
}

fn main() {
    let mut points = Vec::with_capacity(20);

    for i in 0..10 {
        println!("Running test {}", i);
        let point = run_random_test(SIZE);
        points.push(point);
    }

    // for i in (10..=80).step_by(5) {
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
        let grid = generator.generate();
        // println!("{}", grid);
        // validate_grid(&grid);
        let g: &mut Grid = &mut grid.clone();

        generator.remove_cells(g);
        grids.push(GridSet::new(grid, g.clone()));
        count += 1;
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
        let grid = generator.generate();
        let g: &mut Grid = &mut grid.clone();

        generator.remove_cells_amount(g, remove_cells);
        grids.push(GridSet::new(grid, g.clone()));
        count += 1;
    }

    let generation_time = start_time.elapsed();

    let mut point = DataPoint::empty();
    point.generation_time = generation_time.as_nanos();
    point.generation_time_per = generation_time.as_nanos() / grids.len() as u128;

    point = solve(grids, point);
    println!("Done! ");

    point
}

fn solve(grids: Vec<GridSet>, point: DataPoint) -> DataPoint {
    let mut solver = FastSolver::new_with_seed(SOLVE_SEED);
    let size = grids.len();
    let step = size / 10;

    let mut solved = 0;
    let mut error = 0;

    println!("Solving...");
    let start_time = Instant::now();

    // let mut file = File::create("specifics.rs").unwrap();

    //Solve all of them
    for i in 0..size {
        if i % step == 0 {
            println!("Solving grid {}...", i);
        }
        let grid = grids.get(i).unwrap().to_solve.clone();
        let r = solver.solve(&grid);

        match is_valid(&r) {
            true => solved += 1,
            false => error += 1,
        }
    }

    let solve_time = start_time.elapsed();
    let size128 = size as u128;

    DataPoint {
        error,
        generation_time_per: point.generation_time_per,
        generation_time: point.generation_time,
        iterations: 0,
        nothing: 0,
        size,
        solve_time_per: solve_time.as_nanos() / size128,
        solve_time: solve_time.as_nanos(),
        solved,
        updated: 0,
    }
}
