use crate::grid::{
    cell_collection::CellCollection, column::Column, constants::GRID_SIZE, coords::Coord,
    grid::Grid, row::Row, slice::Slice, square::Square,
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
}

struct Change {
    pub coord: Coord,
    pub value: u16,
}

impl Solver for DeterminedSolver {
    fn name(&self) -> &'static str {
        "Determined Solver"
    }

    fn solve(&self, grid: &mut Grid) -> SolveResult {
        let changes = &mut Vec::<Change>::with_capacity(GRID_SIZE);

        // Check rows
        for row in Row::iter_row() {
            set_if_possible_area(changes, grid, &row);
        }

        // Check columns
        for col in Column::iter_col() {
            set_if_possible_area(changes, grid, &col);
        }

        // Check squares
        for sq in Square::iter_squares() {
            set_if_possible_area(changes, grid, &sq);
        }

        // for index in current.iter() {
        //     let coord = current.get_coord(index);
        //     let cell = current.get_cell_at(coord);
        //     if cell.is_determined() {
        //         continue;
        //     }
        //     if set_if_possible_all(current, cell, coord) {
        //         result = SolveResult::Updated;
        //     }
        // }
        for change in changes.iter() {
            grid.place_value_at(change.coord, change.value);
        }

        return match changes.len() {
            0 => SolveResult::Nothing,
            _ => SolveResult::Updated,
        };
    }
}

#[inline(always)]
fn set_if_possible_area<T: CellCollection>(vecs: &mut Vec<Change>, grid: &mut Grid, area: &T) {
    let slice: Slice = Slice::from(grid, area);
    let determined = slice.only_determined().count();

    if determined == 9 {
        return;
    }
    let or = slice.or_all().only_possible();

    for mark in or.iter_possible() {
        let (index, count) = slice.search_count_possible(mark);
        if count == 1 {
            let coord = area.get_coord(index);
            let value = mark.to_value();
            vecs.push(Change { coord, value });
        }
    }
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
