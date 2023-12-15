use super::{coords::Coord, grid::Grid, mark::Mark};

/// A trait for collection of cells
pub trait CellCollection {
    /// Sets the cell at the given index
    fn get_coord(&self, index: usize) -> Coord;
    /// Returns a new iterator over the cells in this collection
    fn iter(&self) -> std::ops::Range<usize>;
    /// Returns the maximum number of cells in this collection
    fn max(&self) -> usize;

    fn count_possible(&self, grid: &Grid, mark: Mark) -> usize {
        let mut count = 0;

        for i in self.iter() {
            let coord = self.get_coord(i);
            if grid.is_possible_at(coord, mark) {
                count += 1;
            }
        }

        count
    }

    fn count_determined(&self, grid: &Grid) -> usize {
        let mut count = 0;

        for i in self.iter() {
            let coord = self.get_coord(i);
            let cell = grid.get_cell_at(coord);
            if cell.is_determined() {
                count += 1;
            }
        }

        count
    }

    fn count_determined_value(&self, grid: &Grid, value: u16) -> usize {
        let mut count = 0;

        for i in self.iter() {
            let coord = self.get_coord(i);
            let cell = grid.get_cell_at(coord);
            if let Some(v) = cell.value() {
                if v == value {
                    count += 1;
                }
            }
        }

        count
    }
}
