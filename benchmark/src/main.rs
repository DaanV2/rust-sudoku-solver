use setup::util::remove_cells_amount;
use sudoku_solver_lib::grid::utility::utility;

pub mod setup;

fn main() {
    let grid = utility::filled_sudoku();
    let mut grids = [grid; 1000000];
    let mut rng = Seeder::;

    for grid in grids.iter_mut() {
        remove_cells_amount(grid, 40);
    }
}
