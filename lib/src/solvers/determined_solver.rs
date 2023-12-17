use crate::grid::{
    cell_collection::CellCollection, column::Column, grid::Grid, mark::Mark, row::Row,
    slice::Slice, square::Square,
};

use super::solver::{SolveResult, Solver};

// The solver that turns solved cells into determined cells.
// EC if only 1 possibility is left
pub struct DeterminedSolver {}

impl Solver for DeterminedSolver {
    fn name(&self) -> &'static str {
        "Determined Solver"
    }

    fn solve(&self, grid: &mut Grid) -> SolveResult {
        DeterminedSolver::solve(grid)
    }
}

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

        SolveResult::from_changed(ch1 | ch2 | ch3)
    }

    pub fn solve_for_mark(grid: &mut Grid, mark: Mark) -> SolveResult {
        let ch1 = DeterminedSolver::solve_rows_for_mark(grid, mark);
        let ch2 = DeterminedSolver::solve_columns_for_mark(grid, mark);
        let ch3 = DeterminedSolver::solve_squares_for_mark(grid, mark);

        SolveResult::from_changed(ch1 | ch2 | ch3)
    }

    pub fn solve_rows(grid: &mut Grid) -> bool {
        solve_area(grid, Row::iter_row())
    }

    pub fn solve_rows_for_mark(grid: &mut Grid, mark: Mark) -> bool {
        solve_area_for_mark(grid, Row::iter_row(), mark)
    }

    pub fn solve_columns(grid: &mut Grid) -> bool {
        solve_area(grid, Column::iter_col())
    }

    pub fn solve_columns_for_mark(grid: &mut Grid, mark: Mark) -> bool {
        solve_area_for_mark(grid, Column::iter_col(), mark)
    }

    pub fn solve_squares(grid: &mut Grid) -> bool {
        solve_area(grid, Square::iter_squares())
    }

    pub fn solve_squares_for_mark(grid: &mut Grid, mark: Mark) -> bool {
        solve_area_for_mark(grid, Square::iter_squares(), mark)
    }
}

#[inline(always)]
fn solve_area<U: CellCollection, T: Iterator<Item = U>>(grid: &mut Grid, iter: T) -> bool {
    let mut changed = false;

    for area in iter {
        changed |= set_if_possible_area(grid, &area);
    }

    return changed;
}

#[inline(always)]
fn solve_area_for_mark<U: CellCollection, T: Iterator<Item = U>>(
    grid: &mut Grid,
    iter: T,
    mark: Mark,
) -> bool {
    let mut changed = false;

    for area in iter {
        changed |= set_if_possible_area_for_mark(grid, &area, mark);
    }

    return changed;
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

#[inline(always)]
fn set_if_possible_area_for_mark<T: CellCollection>(grid: &mut Grid, area: &T, mark: Mark) -> bool {
    let only_possible = Slice::from(grid, area).only_possible();
    let mut changed = false;

    let marked = only_possible.only_possible_value(mark);
    if marked.count() == 1 {
        let index = marked.first_possible(mark);
        let coord = area.get_coord(index);
        let value = mark.to_value();
        changed = true;

        grid.place_value_at(coord, value);
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
    fn only_1_possible() {
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
