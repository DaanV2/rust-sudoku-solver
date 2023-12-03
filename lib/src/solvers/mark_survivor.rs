use crate::grid::{
    constants::{GRID_HEIGHT_RANGE, GRID_WIDTH_RANGE},
    coords::Coord,
    grid::Grid,
};

use super::solver::{SolveResult, Solver, SolverResult};

/** MarkSurvivor checks if a there is only one possibility left and turns that into a determined value */
pub struct MarkSurvivor {}

impl MarkSurvivor {
    pub fn new() -> Self {
        Self {}
    }

    pub fn new_box() -> Box<Self> {
        Box::new(Self::new())
    }
}

impl Solver for MarkSurvivor {
    fn name(&self) -> &'static str {
        "Mark Shapes"
    }

    fn solve(&self, grid: &Grid) -> SolverResult {
        let mut current = grid.clone();
        let mut result = SolveResult::Nothing;

        //Loop through all the cells
        for row in GRID_HEIGHT_RANGE {
            for col in GRID_WIDTH_RANGE {
                let coord = Coord::new(row, col);
                let cell = current.get_cell_at(coord);

                if cell.get_count() != 1 {
                    continue;
                }

                match cell.iter_possible().next() {
                    Some(mark) => {
                        current.place_value_at(coord, mark.to_value());
                        result = SolveResult::Updated;
                    }
                    None => {}
                }
            }
        }

        SolverResult::new(current, result)
    }
}
