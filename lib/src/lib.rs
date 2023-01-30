pub mod grid;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use crate::grid::grid::{Grid, Searchable};

    use super::*;

    #[test]
    fn it_works() {
        let grid = Grid::new();

        if !grid.get_row(3).has_possible(grid::mark::Mark::N1) {
            panic!("Row 3 should have 1 as a possible value");
        }
    }
}
