#[cfg(test)]
pub mod general_tests {

    use rand::Rng;

    use crate::grid::cell::Cell;
    use crate::grid::cell_collection::CellCollection;
    use crate::grid::coords::Coord;
    use crate::grid::grid::Grid;
    use crate::grid::mark::Mark;
    use crate::grid::utility::utility::{self, parse_from_ascii};
    use crate::solvers::solver::{SolveResult, Solver};
    use crate::solvers::solver_manager::SolverManager;

    pub fn test_should_solve(grid: Grid) {
        println!("{}", utility::ascii_grid(&grid));

        let solver = SolverManager::new();
        let output = solver.solve(grid);

        println!("iterations: {:?}", output.iterations);
        println!("{}", utility::ascii_grid(&output.grid));

        assert_eq!(output.result, SolveResult::Solved, "Grid should be solved");
    }

    pub fn remove_cells(grid: &mut Grid) {
        let mut rng = rand::thread_rng();
        let amount = rng.gen_range((grid.max() / 2)..grid.max());

        remove_cells_amount(grid, amount);
    }

    pub fn remove_cells_amount(grid: &mut Grid, amount: usize) {
        let mut rng = rand::thread_rng();
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

    pub fn remove_number(grid: &mut Grid, number: u8) {
        let mark = Mark::from_value(number);

        //Reset all cells with nr 5 to empty
        for i in grid.iter() {
            let cell = grid.get_cell(i);
            if cell.value == number {
                let c = &mut Cell::new();
                c.set_possible(mark);

                grid.set_cell(i, c);
            }
        }
    }

    pub fn process_through(grid: &mut Grid, solves: Vec<Box<dyn Solver>>) -> Grid {
        let mut result = grid.clone();

        for solver in solves {
            let output = solver.solve(result);
            result = output.grid;
        }

        result
    }

    pub fn filled_sudoku() -> Grid {
        parse_from_ascii(
            "4 3 5 | 2 6 9 | 7 8 1\n\
             6 8 2 | 5 7 1 | 4 9 3\n\
             1 9 7 | 8 3 4 | 5 6 2\n\
             ------|-------|------\n\
             8 2 6 | 1 9 5 | 3 4 7\n\
             3 7 4 | 6 8 2 | 9 1 5\n\
             9 5 1 | 7 4 3 | 6 2 8\n\
             ------|-------|------\n\
             5 1 9 | 3 2 6 | 8 7 4\n\
             2 4 8 | 9 5 7 | 1 3 6\n\
             7 6 3 | 4 1 8 | 2 5 9",
        )
    }

    //Test that filled_sudoku returns a grid with all cells determined, and that the values are with their coords
    #[test]
    fn test_filled_sudoku() {
        let grid = filled_sudoku();

        for row in 0..9 {
            for col in 0..9 {
                let coord = Coord::new(row, col);
                let cell = grid.get_cell_at(coord);

                assert!(cell.is_determined());
            }
        }
    }
}
