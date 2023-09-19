use crate::grid::{
    cell::Cell,
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

    fn solve(&self, grid: Grid) -> SolverResult {
        let mut current = grid.clone();
        let mut changed = false;

        //Loop through all the cells
        for row in GRID_HEIGHT_RANGE {
            for col in GRID_WIDTH_RANGE {
                let coord = Coord::new(row, col);
                let cell = current.get_cell_at(coord);

                if cell.get_count() != 1 {
                    continue;
                }

                match cell.possibilities.iter_possible().next() {
                    Some(value) => {
                        let c = Cell::new_with_value(value.to_value());
                        current.set_cell_at(coord, &c);
                        changed = true;
                    }
                    None => {}
                }
            }
        }

        let result = match changed {
            false => SolveResult::Nothing,
            true => SolveResult::Updated,
        };

        SolverResult::new(current, result)
    }
}
