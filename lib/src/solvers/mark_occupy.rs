use crate::grid::{
    cell::Cell, cell_collection::CellCollection, column::Column, grid::Grid, mark::Mark, row::Row,
    slice::Slice,
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

        for row in (0..9).step_by(3) {
            changed |= solve_set(grid, Row::new(row), Row::new(row + 1), Row::new(row + 2));
        }

        for column in (0..9).step_by(3) {
            changed |= solve_set(
                grid,
                Column::new(column),
                Column::new(column + 1),
                Column::new(column + 2),
            );
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

struct SquareData {
    pub set1: Cell,
    pub set2: Cell,
    pub set3: Cell,
}

impl SquareData {
    #[inline(always)]
    pub fn from(set1: &Slice, set2: &Slice, set3: &Slice, square: usize) -> Self {
        let c = square * 3;

        SquareData {
            set1: set1.get(c) | set1.get(c + 1) | set1.get(c + 1),
            set2: set2.get(c) | set2.get(c + 1) | set2.get(c + 1),
            set3: set3.get(c) | set3.get(c + 1) | set3.get(c + 1),
        }
    }

    pub fn all(&self) -> Cell {
        self.set1 | self.set2 | self.set3
    }
}

fn solve_set<T: CellCollection>(grid: &mut Grid, set1: T, set2: T, set3: T) -> bool {
    let s1 = Slice::from(grid, &set1);
    let s2 = Slice::from(grid, &set2);
    let s3 = Slice::from(grid, &set3);

    let mut changed = false;

    let data1 = SquareData::from(&s1, &s2, &s3, 0);
    let data2 = SquareData::from(&s1, &s2, &s3, 1);
    let data3 = SquareData::from(&s1, &s2, &s3, 2);

    let all = data1.all() | data2.all() | data3.all();

    for mark in all.only_possible().iter_possible() {
        let c1 = count(mark, data1.set1, data2.set1, data3.set1);
        let c2 = count(mark, data1.set2, data2.set2, data3.set2);
        let c3 = count(mark, data1.set3, data2.set3, data3.set3);
    }

    changed
}

pub fn count(mark: Mark, data1: Cell, data2: Cell, data3: Cell) -> usize {
    return data1.is_possible(mark) as usize
        + data2.is_possible(mark) as usize
        + data3.is_possible(mark) as usize;
}
