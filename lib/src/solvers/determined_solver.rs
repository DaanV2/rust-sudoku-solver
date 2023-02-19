use crate::grid::{
    cell::Cell,
    constants::{GRID_HEIGHT_RANGE, GRID_WIDTH_RANGE},
    grid::Grid,
    mark::Mark,
    searchable::Searchable,
};

use super::solver::{Solver, SolverResult};

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
    fn solve(&self, grid: Grid) -> SolverResult {
        let current: &mut Grid = &mut grid.clone();
        let mut changed = false;

        //For each row
        for row_index in GRID_HEIGHT_RANGE {
            let row = current.get_row(row_index);
            let r = check_searchable(current, &row);
            changed = changed | r;
        }

        //For each column
        for col in GRID_WIDTH_RANGE {
            let col = current.get_column(col);
            let r = check_searchable(current, &col);
            changed = changed | r;
        }

        //For each square
        for x in 0..3 {
            for y in 0..3 {
                let square = current.get_square(x * 3, y * 3);
                let r = check_searchable(current, &square);
                changed = changed | r;
            }
        }

        if changed {
            SolverResult::update(*current)
        } else {
            SolverResult::nothing(*current)
        }
    }
}

fn check_searchable<T: Searchable>(grid: &mut Grid, area: &T) -> bool {
    for index in area.iter() {
        let cell = area.get_cell(index);
        if cell.is_determined() {
            continue;
        }

        for mark in Mark::iter() {
            if !cell.is_possible(*mark) {
                continue;
            }

            //Loop through the rest of the area to see if the mark is possible anywhere else
            if is_only_possible_at(area, *mark, index) {
                let coord = area.get_coord(index);
                let value = mark.to_value();
                let new_cell = Cell::new_with_value(value);

                grid.set_cell_at(coord, &new_cell);
                return true;
            }
        }
    }

    false
}

fn is_only_possible_at<T: Searchable>(area: &T, mark: Mark, index: usize) -> bool {
    let mark_value = mark.to_value();

    for other_index in area.iter() {
        if other_index == index {
            continue;
        }

        let cell = area.get_cell(other_index);
        if cell.value == mark_value {
            return false;
        }
        if cell.is_possible(mark) {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod test {
    use crate::{
        grid::{
            cell::Cell, mark::Mark, possibility::Possibility, searchable::Searchable,
            test_util::test_util,
        },
        solvers::solver::{SolveResult, Solver},
    };

    #[test]
    fn test_can_solve() {
        let mut grid = test_util::filled_sudoku();

        let index = 64;
        let coord = grid.get_coord(index);

        let cell = grid.get_cell_at(coord);
        let value = cell.value;

        let new_cell = Cell {
            value: 0,
            possibilities: Possibility::from(Mark::from_value(value)),
        };

        grid.set_cell_at(coord, &new_cell);

        let solver = super::DeterminedSolver::new();
        let output = solver.solve(grid);

        assert_eq!(output.result, SolveResult::Updated);

        let check_cell = output.grid.get_cell_at(coord);

        assert_eq!(check_cell.value, value);
    }
}
