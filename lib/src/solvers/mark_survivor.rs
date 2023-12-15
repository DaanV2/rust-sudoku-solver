use crate::grid::{cell_collection::CellCollection, grid::Grid};

use super::solver::{SolveResult, Solver};

/** MarkSurvivor checks if a there is only one possibility left and turns that into a determined value */
pub struct MarkSurvivor {}

impl Solver for MarkSurvivor {
    fn name(&self) -> &'static str {
        "Mark Shapes"
    }

    fn solve(&self, grid: &mut Grid) -> SolveResult {
        MarkSurvivor::solve(grid)
    }
}

impl MarkSurvivor {
    pub fn new() -> Self {
        Self {}
    }

    pub fn new_box() -> Box<Self> {
        Box::new(Self::new())
    }

    pub fn solve(grid: &mut Grid) -> SolveResult {
        let mut result = SolveResult::Nothing;

        //Loop through all the cells
        for i in grid.iter() {
            let coord = grid.get_coord(i);
            let cell = grid.get_cell_at(coord);
            if cell.is_determined() {
                continue;
            }
            if cell.possible_count() != 1 {
                continue;
            }

            //If there is only one possible value, set it
            match cell.iter_possible().next() {
                Some(mark) => {
                    grid.place_value_at(coord, mark.to_value());
                    result = SolveResult::Updated;
                }
                None => {}
            }
        }

        result
    }
}
