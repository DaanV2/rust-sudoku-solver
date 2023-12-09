use std::cmp::min;

use serde::{Deserialize, Serialize};
use sudoku_solver_lib::{
    grid::{cell::Cell, cell_collection::CellCollection, constants, grid::Grid, mark::Mark},
    solvers::solver::AnnotatedSolverResult,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct GridInput {
    pub cells: Vec<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GridOutput {
    pub iterations: u32,
    pub result: u32,
    pub cells: Vec<CellOutput>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CellOutput {
    value: u8,
    possible: Possible,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Possible {
    p1: bool,
    p2: bool,
    p3: bool,
    p4: bool,
    p5: bool,
    p6: bool,
    p7: bool,
    p8: bool,
    p9: bool,
}

impl GridInput {
    pub fn is_valid(&self) -> bool {
        if self.cells.len() != constants::GRID_SIZE {
            return false;
        }

        true
    }

    pub fn to_grid(&self) -> Grid {
        let mut grid = Grid::new();
        let max = min(self.cells.len(), constants::GRID_SIZE);

        for index in 0..max {
            let v = self.cells[index] as usize;
            let new_cell = &mut Cell::new();

            if v != 0 {
                new_cell.set_value(v);
            }

            grid.set_cell(index, new_cell);
        }

        grid
    }
}

impl GridOutput {
    pub fn from_grid(result: AnnotatedSolverResult) -> GridOutput {
        let mut cells = Vec::new();
        let grid = result.grid;

        for index in 0..grid.max() {
            let cell = grid.get_cell(index);
            let cell_output = CellOutput::from_cell(cell);

            cells.push(cell_output);
        }

        GridOutput {
            iterations: result.iterations as u32,
            result: result.result as u32,
            cells,
        }
    }
}

impl CellOutput {
    pub fn from_cell(cell: &Cell) -> CellOutput {
        CellOutput {
            value: cell.get_value() as u8,
            possible: Possible::from_cell(cell),
        }
    }
}

impl Possible {
    pub fn from_cell(cell: &Cell) -> Possible {
        Possible {
            p1: cell.is_possible(Mark::N1),
            p2: cell.is_possible(Mark::N2),
            p3: cell.is_possible(Mark::N3),
            p4: cell.is_possible(Mark::N4),
            p5: cell.is_possible(Mark::N5),
            p6: cell.is_possible(Mark::N6),
            p7: cell.is_possible(Mark::N7),
            p8: cell.is_possible(Mark::N8),
            p9: cell.is_possible(Mark::N9),
        }
    }
}
