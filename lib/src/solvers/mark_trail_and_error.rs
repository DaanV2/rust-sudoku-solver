use crate::grid::{
    cell_collection::CellCollection, grid::Grid, mark::Mark, slice::Slice, square::Square,
};

use super::solver::{SolveResult, Solver};

/**
 * MarkTrailAndError takes a number, and checks if any of the possible cell left to fill in,
 * will cause other squares to lose out any possibility to fill in of their cell
 */

pub struct MarkTrailAndError {}

impl Solver for MarkTrailAndError {
    fn name(&self) -> &'static str {
        "Mark Trail And Error"
    }

    fn solve(&self, grid: &mut Grid) -> SolveResult {
        MarkTrailAndError::solve(grid)
    }
}

impl MarkTrailAndError {
    pub fn new() -> Self {
        Self {}
    }

    pub fn new_box() -> Box<Self> {
        Box::new(Self::new())
    }

    pub fn solve(grid: &mut Grid) -> SolveResult {
        let mut changed = false;

        for mark in Mark::iter() {
            changed = changed | MarkTrailAndError::solve_for_mark(grid, mark);
        }

        SolveResult::from_changed(changed)
    }

    pub fn solve_for_mark(grid: &mut Grid, mark: Mark) -> bool {
        let buffer = &mut Grid::empty();
        let mut squares_determined = [false; 9];
        let mut changed = false;

        let determined = grid.count_determined();

        for sq_index in Square::iter() {
            let sq = Square::from_square_index(sq_index);

            squares_determined[sq_index] = Slice::from(grid, &sq).is_determined(mark.to_value());
        }

        let squares_count = squares_determined.iter().filter(|x| **x).count();
        if squares_count < 3 || squares_count > 7 || determined < 25 {
            // 8 and 9 amount statistics are almost 100% never hitting
            // Under determined 25 is the most in-effective
            return false;
        }

        for sq_index in Square::iter() {
            let sq = Square::from_square_index(sq_index);
            if squares_determined[sq_index] {
                continue;
            }

            for c in sq.iter().map(|s| sq.get_coord(s)) {
                let original_cell = grid.get_cell_at(c);
                if !original_cell.is_possible(mark) {
                    continue;
                }

                buffer.clone_from(grid);
                // Place the mark
                buffer.place_value_at(c, mark.to_value());

                // Solve some stuff
                // let mut result;
                // loop {
                //     result = SolveResult::Nothing;
                //     result = result | MarkShapes::solve_for_mark(buffer, mark);
                //     result = result | DeterminedSolver::solve_for_mark(buffer, mark);
                //     if result != SolveResult::Updated {
                //         break;
                //     }
                // }
                // if result == SolveResult::Error {
                //     continue;
                // }

                // Check all the square were possible are now determined, or have at least one possible left
                // If there is any square not determined or any possible left, this cell is causing errors and needs to be marked off
                for old_sq_index in Square::iter() {
                    if squares_determined[sq_index] || old_sq_index == sq_index {
                        continue;
                    }
                    let sq = Square::from_square_index(old_sq_index);
                    let a = Slice::from(buffer, &sq);
                    //Any possible then continue
                    if a.any_possible(mark) {
                        continue;
                    }
                    if a.is_determined(mark.to_value()) {
                        continue;
                    }
                    // This square is not determined, and has no possible left, unset this cell
                    // println!("Marking {} at {} as impossible", mark, c);
                    // println!("Because it causes {} to be impossible\ndata: {}", sq, a);
                    // println!("original cell: {}", buffer);
                    // println!("grid:\n{}", buffer);

                    // for r in Row::iter_row() {
                    //     let rs = Slice::from(buffer, &r);
                    //     println!("{}", rs);
                    // }

                    grid.unset_possible_at(c, mark);
                    changed = true;
                    break;
                }
            }
        }

        changed
    }
}
