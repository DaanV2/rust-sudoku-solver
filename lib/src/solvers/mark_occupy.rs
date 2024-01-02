use crate::grid::{
    cell_collection::CellCollection, column::Column, constants::GRID_HEIGHT_RANGE, coords::Coord,
    grid::Grid, mark::Mark, row::Row, slice::Slice,
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
    let s1 = Slice::from(grid, check);

    // Check if only possible in one square
    let mut changed = false;
    let possibles = s1.only_possible().or_all();

    for mark in possibles.iter_possible() {
        // Isolate only the cells that have this mark
        if let Some(square) = which_square(s1, mark) {
            // We found a square that can only be in one place
            changed |= unset_three(grid, square, other1, mark);
            changed |= unset_three(grid, square, other2, mark);
        }
    }

    changed
}

#[inline(always)]
fn which_square(marked: Slice, mark: Mark) -> Option<usize> {
    // This mark is only possible in square one?
    let c1 = marked.get(0) | marked.get(1) | marked.get(2);
    let c2 = marked.get(3) | marked.get(4) | marked.get(5);
    let c3 = marked.get(6) | marked.get(7) | marked.get(8);

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

pub fn unset_three<T: CellCollection>(grid: &mut Grid, start: usize, set: &T, mark: Mark) -> bool {
    let mut changed = false;

    changed |= mark_off_at(grid, set.get_coord(start + 0), mark);
    changed |= mark_off_at(grid, set.get_coord(start + 1), mark);
    changed |= mark_off_at(grid, set.get_coord(start + 2), mark);

    changed
}

#[inline(always)]
fn mark_off_at(grid: &mut Grid, coord: Coord, mark: Mark) -> bool {
    let old = *grid.get_cell_at(coord);
    grid.unset_possible_at(coord, mark);

    let n = *grid.get_cell_at(coord);

    return n != old;
}
