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
        for i in current.iter().rev() {
            let coord = current.get_coord(i);
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

        result
    }
}
