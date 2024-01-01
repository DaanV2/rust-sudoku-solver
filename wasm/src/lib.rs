mod sudoku;

use std::ops::BitXor;

use sudoku::cells::Cell;
use sudoku_solver_lib::{generators::generators::Generator, solvers::solver_manager};
use wasm_bindgen::prelude::*;

/// Create a new empty grid.
#[wasm_bindgen]
pub fn new_grid() -> Vec<Cell> {
    let mut cells = Vec::new();

    for _ in 0..81 {
        let c = Cell {
            value: 0,
            possibilities: sudoku::cells::Possibilities {
                p1: true,
                p2: true,
                p3: true,
                p4: true,
                p5: true,
                p6: true,
                p7: true,
                p8: true,
                p9: true,
            },
        };

        cells.push(c);
    }

    cells
}

/// Solve a grid.
#[wasm_bindgen]
pub fn solve_once(grid: Vec<i32>) -> Vec<Cell> {
    let grid = &mut Cell::to_sudoku_grid(grid);

    let solver = solver_manager::SolverManager::new();
    let result = solver.solve_simple(grid);

    Cell::from_grid(result.grid)
}

/// Solve a sudoku grid.
#[wasm_bindgen]
pub fn solve(grid: Vec<i32>) -> Vec<Cell> {
    let grid = Cell::to_sudoku_grid(grid);

    let solver = solver_manager::SolverManager::new();
    let result = solver.solve(grid);
    println!("Solved: {}", result);

    Cell::from_grid(result.grid)
}

/// Generate a new grid with a random seed and difficulty.
#[wasm_bindgen]
pub fn generate() -> Vec<Cell> {
    let mut generator = Generator::new_random();

    let grid = generator.generate();
    let mut g = grid.clone();
    generator.remove_cells(&mut g);

    return Cell::from_grid(g);
}

/// Generate a new grid with a specific difficulty and seed. If the difficulty is 0, it will be a full grid.
#[wasm_bindgen]
pub fn generate_with(difficulty: i32, seed: i32) -> Vec<Cell> {
    if seed == 0 {
        panic!("Seed cannot be 0");
    }
    let mut seed = seed as u64;
    seed |= seed.bitxor(u64::MAX) << 32;

    let mut generator = Generator::new_with_seed(seed);

    let grid = generator.generate();
    let mut g = grid.clone();

    if difficulty != 0 {
        generator.remove_cells_amount(&mut g, difficulty as usize);
    }

    return Cell::from_grid(g);
}

#[cfg(test)]
mod test {
    use crate::solve;

    #[test]
    pub fn test_solve() {
        let input = vec![
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 9, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ];

        let output = solve(input);

        for c in output.iter() {
            println!("{:?}", c);
            assert_ne!(c.value, 0);
        }
    }
}
