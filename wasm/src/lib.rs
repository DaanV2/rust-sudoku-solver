mod sudoku;

use sudoku::cells::Cell;
use sudoku_solver_lib::solvers::solver_manager;
use wasm_bindgen::prelude::*;

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

#[wasm_bindgen]
pub fn solve_once(grid: Vec<Cell>) -> Vec<Cell> {
    let grid = Cell::to_sudoku_grid(grid);

    let solver = solver_manager::SolverManager::new();
    let result = solver.solve_simple(grid);

    Cell::from_grid(result.grid)
}

#[wasm_bindgen]
pub fn solve(grid: Vec<Cell>) -> Vec<Cell> {
    let grid = Cell::to_sudoku_grid(grid);

    let solver = solver_manager::SolverManager::new();
    let result = solver.solve(grid);

    Cell::from_grid(result.grid)
}
