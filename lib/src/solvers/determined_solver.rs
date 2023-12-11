use crate::grid::{
    cell_collection::CellCollection, column::Column, grid::Grid, row::Row, slice::Slice,
    square::Square,
};

use super::solver::{SolveResult, Solver};

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

    pub fn solve(grid: &mut Grid) -> SolveResult {
        let ch1 = DeterminedSolver::solve_rows(grid);
        let ch2 = DeterminedSolver::solve_columns(grid);
        let ch3 = DeterminedSolver::solve_squares(grid);

        ch1.combine(ch2).combine(ch3)
    }

    pub fn solve_rows(grid: &mut Grid) -> SolveResult {
        let mut changed = false;

        // Check rows
        for row in Row::iter_row() {
            changed |= set_if_possible_area(grid, &row);
        }

        SolveResult::from_changed(changed)
    }

    pub fn solve_columns(grid: &mut Grid) -> SolveResult {
        let mut changed = false;

        // Check columns
        for col in Column::iter_col() {
            changed |= set_if_possible_area(grid, &col);
        }

        SolveResult::from_changed(changed)
    }

    pub fn solve_squares(grid: &mut Grid) -> SolveResult {
        let mut changed = false;

        // Check squares
        for sq in Square::iter_squares() {
            changed |= set_if_possible_area(grid, &sq);
        }

        SolveResult::from_changed(changed)
    }
}

impl Solver for DeterminedSolver {
    fn name(&self) -> &'static str {
        "Determined Solver"
    }

    fn solve(&self, grid: &mut Grid) -> SolveResult {
        DeterminedSolver::solve(grid)
    }
}

#[inline(always)]
fn set_if_possible_area<T: CellCollection>(grid: &mut Grid, area: &T) -> bool {
    let only_possible = Slice::from(grid, area).only_possible();
    let or_all = only_possible.or_all();
    let mut changed = false;

    for mark in or_all.iter_possible() {
        let marked = only_possible.only_possible_value(mark);
        if marked.count() == 1 {
            let index = marked.first_possible(mark);
            let coord = area.get_coord(index);
            let value = mark.to_value();
            changed = true;

            grid.place_value_at(coord, value);
        }
    }

    return changed;
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

        let index = 2;
        let coord = grid.get_coord(index);

        let cell = grid.get_cell_at(coord).clone();
        let mut new_cell;

        if let Some(value) = cell.value() {
            new_cell = Cell::new_empty();
            new_cell.set_possible(Mark::from_value(value));
        } else {
            panic!("Cell should be determined");
        }

        grid.set_cell_at(coord, &new_cell);

        let solver = super::DeterminedSolver::new();
        let output = solver.solve(grid);

        let check_cell = grid.get_cell_at(coord);

        assert_eq!(check_cell, &cell);
        assert_eq!(output, SolveResult::Updated);
    }

    #[test]
    fn test_single_missing_number() {
        let grid = &mut general_tests::filled_sudoku();

        general_tests::remove_number(grid, 5);

        let solver = super::DeterminedSolver::new();
        let result = solver.solve(grid);

        assert_eq!(result, SolveResult::Updated);

        //Check that all cells with value 5 are determined
        for index in grid.iter() {
            let cell = grid.get_cell(index);

            assert!(cell.is_determined(), "Cell at {} is not determined", index)
        }
    }

    #[test]
    fn test_double_missing_number() {
        let grid = &mut general_tests::filled_sudoku();

        general_tests::remove_number(grid, 5);
        general_tests::remove_number(grid, 1);

        let result = super::DeterminedSolver::new().solve(grid);

        assert_eq!(result, SolveResult::Updated);

        //Check that all cells with value 5 are determined
        for index in grid.iter() {
            let cell = grid.get_cell(index);

            assert!(cell.is_determined(), "Cell at {} is not determined", index)
        }
    }

    #[test]
    fn test_only_1_possible() {
        let grid = &mut general_tests::filled_sudoku();

        for i in 0..10 {
            general_tests::remove_number(grid, i);
        }

        println!("{}", grid);
        let result = super::DeterminedSolver::new().solve(grid);

        println!("{}", grid);
        assert_eq!(result, SolveResult::Updated);

        //Check that all cells with value 5 are determined
        for index in grid.iter() {
            let cell = grid.get_cell(index);

            assert!(cell.is_determined(), "Cell at {} is not determined", index)
        }
    }
}
