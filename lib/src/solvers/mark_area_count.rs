use crate::grid::{
    cell::Cell, cell_collection::CellCollection, coords::Coord, grid::Grid, mark::Mark,
    square::Square,
};

use super::solver::{SolveResult, Solver, SolverResult};

/** A solver takes a mark and its areas, and counts and tries to eliminate other marks from that area */
pub struct MarkAreaCount {}

impl MarkAreaCount {
    pub fn new() -> Self {
        Self {}
    }

    pub fn new_box() -> Box<Self> {
        Box::new(Self::new())
    }
}

impl Solver for MarkAreaCount {
    fn name(&self) -> &'static str {
        "Mark Area Count"
    }

    fn solve(&self, grid: Grid) -> SolverResult {
        let instance = &mut MarkAreaCountInstance::new(grid);
        let result = instance.check_marks_area();

        SolverResult::new(instance.grid, result)
    }
}

#[derive(Copy, Clone)]
struct MarkAreaCountDataItem {
    pub check: bool,
    pub cell: Cell,
    pub coord: Coord,
}

impl MarkAreaCountDataItem {
    pub fn new() -> Self {
        Self {
            check: false,
            cell: Cell::new(),
            coord: Coord::new(0, 0),
        }
    }
}

struct MarkAreaCountInstance {
    grid: Grid,

    data: [MarkAreaCountDataItem; 9],
}

impl MarkAreaCountInstance {
    pub fn new(grid: Grid) -> Self {
        Self {
            grid,
            data: [MarkAreaCountDataItem::new(); 9],
        }
    }

    pub fn check_marks_area(&mut self) -> SolveResult {
        for mark in Mark::iter() {
            for sq in Square::iter_square_coords() {
                let square = self.grid.get_square_at(sq);
                let result = self.check_area(square, *mark);
                if result != SolveResult::Nothing {
                    return result;
                }
            }
        }

        return SolveResult::Nothing;
    }

    fn check_area<T>(&mut self, area: T, mark: Mark) -> SolveResult
    where
        T: CellCollection,
    {
        let mut count = 0;
        let mark_value = mark.to_value();

        //Transfer data
        for i in 0..self.data.len() {
            let coord = area.get_coord(i);
            let cell = area.get_cell(i);

            //Already defined, nothing to do
            if cell.value == mark_value {
                return SolveResult::Nothing;
            }
            let mut check = false;
            if cell.is_possible(mark) {
                count += 1;
                check = true;
            }

            self.data[i] = MarkAreaCountDataItem {
                check: check,
                cell: *cell,
                coord,
            };
        }

        //If we got here, and have nothing and nothing was defined with that value. We have an error?
        if count == 0 {
            return SolveResult::Error;
        }

        return SolveResult::Nothing;
    }
}
