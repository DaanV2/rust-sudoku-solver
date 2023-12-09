use sudoku_solver_lib::grid::mark::Mark;
use wasm_bindgen::prelude::*;

#[derive(Clone, Copy, Debug)]
#[wasm_bindgen]
pub struct Possibilities {
    pub p1: bool,
    pub p2: bool,
    pub p3: bool,
    pub p4: bool,
    pub p5: bool,
    pub p6: bool,
    pub p7: bool,
    pub p8: bool,
    pub p9: bool,
}

#[derive(Clone, Copy, Debug)]
#[wasm_bindgen]
pub struct Cell {
    pub value: usize,
    pub possibilities: Possibilities,
}

impl Cell {
    pub fn new() -> Cell {
        Cell {
            value: 0,
            possibilities: Possibilities {
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
        }
    }

    pub fn to_sudoku_grid(cells: Vec<i32>) -> sudoku_solver_lib::grid::grid::Grid {
        let mut grid = sudoku_solver_lib::grid::grid::Grid::new();

        // Check cells is 81
        if cells.len() != 81 {
            panic!("Cells must be 81");
        }

        for (i, v) in cells.iter().enumerate() {
            grid.place_value(i as usize, (*v) as usize);
        }

        return grid;
    }

    pub fn from_grid(grid: sudoku_solver_lib::grid::grid::Grid) -> Vec<Cell> {
        let mut cells = Vec::new();

        for i in 0..81 {
            let c = grid.get_cell(i);
            let cell = Cell::from_sudoku(c);
            cells.push(cell);
        }

        return cells;
    }

    pub fn to_sudoku(&self) -> sudoku_solver_lib::grid::cell::Cell {
        if self.value > 0 {
            return sudoku_solver_lib::grid::cell::Cell::new_with_value(self.value as usize);
        }

        let mut c = sudoku_solver_lib::grid::cell::Cell::new_empty();

        if self.possibilities.p1 {
            c.set_possible(Mark::N1)
        }
        if self.possibilities.p2 {
            c.set_possible(Mark::N2)
        }
        if self.possibilities.p3 {
            c.set_possible(Mark::N3)
        }
        if self.possibilities.p4 {
            c.set_possible(Mark::N4)
        }
        if self.possibilities.p5 {
            c.set_possible(Mark::N5)
        }
        if self.possibilities.p6 {
            c.set_possible(Mark::N6)
        }
        if self.possibilities.p7 {
            c.set_possible(Mark::N7)
        }
        if self.possibilities.p8 {
            c.set_possible(Mark::N8)
        }
        if self.possibilities.p9 {
            c.set_possible(Mark::N9)
        }

        return c;
    }

    pub fn from_sudoku(c: &sudoku_solver_lib::grid::cell::Cell) -> Cell {
        let mut cell = Cell::new();

        if c.is_determined() {
            cell.value = c.get_value() as usize;
            return cell;
        }

        cell.possibilities.p1 = c.is_possible(Mark::N1);
        cell.possibilities.p2 = c.is_possible(Mark::N2);
        cell.possibilities.p3 = c.is_possible(Mark::N3);
        cell.possibilities.p4 = c.is_possible(Mark::N4);
        cell.possibilities.p5 = c.is_possible(Mark::N5);
        cell.possibilities.p6 = c.is_possible(Mark::N6);
        cell.possibilities.p7 = c.is_possible(Mark::N7);
        cell.possibilities.p8 = c.is_possible(Mark::N8);
        cell.possibilities.p9 = c.is_possible(Mark::N9);

        return cell;
    }
}
