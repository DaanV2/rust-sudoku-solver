use crate::grid::{cell_collection::CellCollection, grid::Grid};

use super::solver::{SolveResult, Solver};

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

    fn solve(&self, grid: &mut Grid) -> SolveResult {
        let mut current = grid.clone();
        let mut result = SolveResult::Nothing;

        //Loop through all the cells
        for i in current.iter() {
            let coord = current.get_coord(i);
            let cell = current.get_cell_at(coord);
            if cell.is_determined() {
                continue;
            }
            if cell.possible_count() != 1 {
                continue;
            }

            //If there is only one possible value, set it
            match cell.iter_possible().next() {
                Some(mark) => {
                    current.place_value_at(coord, mark.to_value());
                    result = SolveResult::Updated;
                }
                None => {}
            }
        }

        result
    }
}
