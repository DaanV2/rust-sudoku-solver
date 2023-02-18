pub mod grid;
pub mod solvers;

#[cfg(test)]
mod test {
    use crate::{
        grid::{cell::Cell, grid::Grid, searchable::Searchable, test_util::test_util},
        solvers::{solver::SolveResult, solver_manager::SolverManager},
    };

    #[test]
    fn test_1() {
        test_amount(1);
    }

    #[test]
    fn test_10() {
        test_amount(3);
    }

    #[test]
    fn test_20() {
        test_amount(20);
    }

    #[test]
    fn test_30() {
        test_amount(30);
    }

    #[test]
    fn test_1_area_missing() {
        let mut grid = test_util::filled_sudoku();

        let sq = grid.get_square(3, 3);
        for index in sq.iter() {
            let coord = sq.get_coord(index);
            grid.set_cell(coord, Cell::new_with_value(0));
        }

        test_should_solve(grid);
    }

    fn test_amount(amount: usize) {
        println!("test_amount({})", amount);
        let mut grid = test_util::filled_sudoku();

        test_util::remove_cells_amount(&mut grid, amount);

        test_should_solve(grid);
    }

    fn test_should_solve(grid: Grid) {
        println!("{}", test_util::ascii_grid(&grid));

        let solver = SolverManager::new();
        let output = solver.solve(grid);

        println!("iterations: {:?}", output.iterations);
        println!("{}", test_util::ascii_grid(&grid));

        assert_eq!(output.result, SolveResult::Solved);
    }
}
