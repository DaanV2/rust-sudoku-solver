#[cfg(test)]
mod test {
    use crate::{
        grid::{cell::Cell, grid::Grid, searchable::Searchable, test_util::test_util},
        solvers::{solver::SolveResult, solver_manager::SolverManager},
    };

    #[test]
    fn test_1() {
        test_amount(1)
    }

    #[test]
    fn test_10() {
        test_amount(10);
    }

    #[test]
    fn test_20() {
        test_amount(20);
    }

    // #[test]
    // fn test_30() {
    //     test_amount(30);
    // }

    // #[test]
    // fn test_40() {
    //     test_amount(40);
    // }

    // #[test]
    // fn test_50() {
    //     test_amount(50);
    // }

    // #[test]
    // fn test_60() {
    //     test_amount(60);
    // }

    #[test]
    fn test_1_area_missing() {
        let grid: &mut Grid = &mut test_util::filled_sudoku();

        clear_square(grid, 3, 3);

        test_should_solve(*grid);
    }

    #[test]
    fn test_2_area_missing() {
        let grid: &mut Grid = &mut test_util::filled_sudoku();

        clear_square(grid, 3, 3);
        clear_square(grid, 0, 6);

        test_should_solve(*grid);
    }

    #[test]
    fn test_3_area_missing() {
        let grid: &mut Grid = &mut test_util::filled_sudoku();

        clear_square(grid, 3, 3);
        clear_square(grid, 0, 6);
        clear_square(grid, 6, 0);

        test_should_solve(*grid);
    }

    fn clear_square(grid: &mut Grid, row: usize, col: usize) {
        let sq = grid.get_square(row, col);
        for index in sq.iter() {
            let coord = sq.get_coord(index);
            grid.set_cell_at(coord, &Cell::new_with_value(0));
        }
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
        println!("{}", test_util::ascii_grid(&output.grid));

        assert_eq!(output.result, SolveResult::Solved);
    }
}
