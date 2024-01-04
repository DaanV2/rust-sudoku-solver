use crate::grid::{
    cell::Cell, cell_collection::CellCollection, column::Column, constants::GRID_HEIGHT_RANGE,
    grid::Grid, mark::Mark, row::Row,
};

use super::solver::{SolveResult, Solver};

/** Checks rows and columns, and determines if a mark value is occupied by a certain area:
 *
 *
 * x x x | x x x | x x x
 * x x x | x   x | x x x
 *       |       | x x x
 *
 * From these three we can conclude that the right square can only be placed in the bottom row
*/

pub struct MarkOccupy {}

impl MarkOccupy {
    pub fn new() -> Self {
        Self {}
    }

    pub fn new_box() -> Box<Self> {
        Box::new(Self::new())
    }

    pub fn solve(grid: &mut Grid) -> SolveResult {
        let mut changed = false;

        for index in GRID_HEIGHT_RANGE.step_by(3) {
            let r1 = &Row::new(index + 0);
            let r2 = &Row::new(index + 1);
            let r3 = &Row::new(index + 2);

            changed |= solve_set(grid, r1, r2, r3);
            changed |= solve_set(grid, r2, r1, r3);
            changed |= solve_set(grid, r3, r1, r2);
        }

        for index in GRID_HEIGHT_RANGE.step_by(3) {
            let c1 = &Column::new(index + 0);
            let c2 = &Column::new(index + 1);
            let c3 = &Column::new(index + 2);

            changed |= solve_set(grid, c1, c2, c3);
            changed |= solve_set(grid, c2, c1, c3);
            changed |= solve_set(grid, c3, c1, c2);
        }

        SolveResult::from_changed(changed)
    }
}

impl Solver for MarkOccupy {
    fn name(&self) -> &'static str {
        "Mark Occupy"
    }

    fn solve(&self, grid: &mut Grid) -> SolveResult {
        MarkOccupy::solve(grid)
    }
}

#[inline(always)]
fn solve_set<T: CellCollection>(grid: &mut Grid, check: &T, other1: &T, other2: &T) -> bool {
    fn get_cell<T: CellCollection>(grid: &Grid, marked: &T, index: usize) -> Cell {
        let coord = marked.get_coord(index);
        *grid.get_cell_at(coord)
    }

    let c1 = get_cell(grid, check, 0) | get_cell(grid, check, 1) | get_cell(grid, check, 2);
    let c2 = get_cell(grid, check, 3) | get_cell(grid, check, 4) | get_cell(grid, check, 5);
    let c3 = get_cell(grid, check, 6) | get_cell(grid, check, 7) | get_cell(grid, check, 8);

    fn which_square(c1: Cell, c2: Cell, c3: Cell, mark: Mark) -> Option<usize> {
        // This mark is only possible in square one?
        return match (
            c1.is_possible(mark),
            c2.is_possible(mark),
            c3.is_possible(mark),
        ) {
            (true, false, false) => Some(0),
            (false, true, false) => Some(3),
            (false, false, true) => Some(6),
            _ => None,
        };
    }

    // Check if only possible in one square
    let mut changed = false;
    let possibles: Cell = (c1 | c2 | c3).only_possible();

    for mark in possibles.iter_possible() {
        // Isolate only the cells that have this mark
        if let Some(square) = which_square(c1, c2, c3, mark) {
            // We found a square that can only be in one place
            unset_three(grid, square, other1, mark);
            unset_three(grid, square, other2, mark);
            changed = true;
        }
    }

    changed
}

pub fn unset_three<T: CellCollection>(grid: &mut Grid, start: usize, set: &T, mark: Mark) {
    grid.unset_possible_at(set.get_coord(start + 0), mark);
    grid.unset_possible_at(set.get_coord(start + 1), mark);
    grid.unset_possible_at(set.get_coord(start + 2), mark);
}
