use crate::grid::{
    cell::Cell, cell_collection::CellCollection, coords::Coord, grid::Grid, mark::Mark,
};

use super::solver::{SolveResult, Solver, SolverResult};

// The solver that turns solved cells into determined cells.
// EC if only 1 possibility is left
pub struct DeterminedSolver {}

impl DeterminedSolver {
    pub fn new() -> Self {
        Self {}
    }

    pub fn new_box() -> Box<Self> {
        Box::new(Self::new())
    }
}

impl Solver for DeterminedSolver {
    fn name(&self) -> &'static str {
        "Determined Solver"
    }

    fn solve(&self, grid: &Grid) -> SolverResult {
        let current: &mut Grid = &mut grid.clone();
        let mut result = SolveResult::Nothing;

        for index in current.iter() {
            let coord = current.get_coord(index);
            let cell = current.get_cell_at(coord);
            if cell.is_determined() {
                continue;
            }
            if set_if_possible_all(current, cell, coord) {
                result = SolveResult::Updated;
            }
        }

        SolverResult::new(*current, result)
    }
}

fn set_if_possible_all(grid: &mut Grid, cell: Cell, current: Coord) -> bool {
    let (r, c) = current.get_row_col();

    for mark in cell.iter_possible() {
        if set_if_possible(grid, &grid.get_row(r), current, mark) {
            return true;
        }
        if set_if_possible(grid, &grid.get_column(c), current, mark) {
            return true;
        }
        if set_if_possible(grid, &grid.get_square_at(current), current, mark) {
            return true;
        }
    }

    return false;
}

fn set_if_possible<T: CellCollection>(grid: &mut Grid, area: &T, coord: Coord, mark: Mark) -> bool {
    //Loop through the rest of the area to see if the mark is possible anywhere else
    if is_only_possible_at(grid, area, coord, mark) {
        let value = mark.to_value();
        grid.place_value_at(coord, value);
        return true;
    }

    false
}

fn is_only_possible_at<T: CellCollection>(grid: &Grid, area: &T, coord: Coord, mark: Mark) -> bool {
    for i in area.iter().rev() {
        let c = area.get_coord(i);
        if c == coord {
            continue;
        }
        let cell = grid.get_cell_at(c);
        if cell.is_possible(mark) {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod test {
    use crate::{
        grid::{cell::Cell, cell_collection::CellCollection, mark::Mark},
        solvers::solver::{SolveResult, Solver},
        test::util::general_tests,
    };

    #[test]
    fn test_can_solve() {
        let grid = &mut general_tests::filled_sudoku();

        let index = 64;
        let coord = grid.get_coord(index);

        let cell = grid.get_cell_at(coord);
        let mut new_cell;

        if let Some(value) = cell.value() {
            new_cell = Cell::new_empty();
            new_cell.set_possible(Mark::from_value(value));
        } else {
            panic!("Cell should be determined");
        }

        grid.set_cell_at(coord, new_cell);

        let solver = super::DeterminedSolver::new();
        let output = solver.solve(&grid);

        assert_eq!(output.result, SolveResult::Updated);

        let check_cell = output.grid.get_cell_at(coord);

        assert_eq!(check_cell, cell);
    }

    #[test]
    fn test_single_missing_number() {
        let mut grid = general_tests::filled_sudoku();

        general_tests::remove_number(&mut grid, 5);

        println!("{}", grid);
        let solver = super::DeterminedSolver::new();
        let result = solver.solve(&grid);

        assert_eq!(result.result, SolveResult::Updated);

        //Check that all cells with value 5 are determined
        for index in result.grid.iter() {
            let cell = result.grid.get_cell(index);

            assert!(cell.is_determined(), "Cell at {} is not determined", index)
        }
    }

    #[test]
    fn test_double_missing_number() {
        let mut grid = general_tests::filled_sudoku();

        general_tests::remove_number(&mut grid, 5);
        general_tests::remove_number(&mut grid, 1);

        println!("{}", grid);

        let result = super::DeterminedSolver::new().solve(&grid);

        println!("{}", result.grid);

        assert_eq!(result.result, SolveResult::Updated);

        //Check that all cells with value 5 are determined
        for index in result.grid.iter() {
            let cell = result.grid.get_cell(index);

            assert!(cell.is_determined(), "Cell at {} is not determined", index)
        }
    }

    #[test]
    fn test_only_1_possible() {
        let mut grid = general_tests::filled_sudoku();

        for i in 0..10 {
            general_tests::remove_number(&mut grid, i);
        }

        println!("{}", grid);
        let result = super::DeterminedSolver::new().solve(&grid);

        println!("{}", result.grid);
        assert_eq!(result.result, SolveResult::Updated);

        //Check that all cells with value 5 are determined
        for index in result.grid.iter() {
            let cell = result.grid.get_cell(index);

            assert!(cell.is_determined(), "Cell at {} is not determined", index)
        }
    }
}
