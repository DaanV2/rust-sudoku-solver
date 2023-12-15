use rand::{rngs::ThreadRng, Rng};
use sudoku_solver_lib::grid::{cell::Cell, cell_collection::CellCollection, grid::Grid};

pub fn remove_cells_amount(rng: &mut ThreadRng, grid: &mut Grid, amount: usize) {
    let mut removed = 0;

    while removed < amount {
        let index = rng.gen_range(0..grid.max());
        let cell = &grid.get_cell(index);

        if cell.is_determined() {
            grid.set_cell(index, &Cell::new());
            removed += 1;
        }
    }
}
