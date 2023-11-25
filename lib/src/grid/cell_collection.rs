use super::{cell::Cell, coords::Coord, mark::Mark};

/// A trait for collection of cells
pub trait CellCollection {
    /// Returns the cell at the given index
    fn get_cell(&self, index: usize) -> Cell;
    /// Sets the cell at the given index
    fn get_coord(&self, index: usize) -> Coord;

    /// Returns the maximum number of cells in this collection
    fn max(&self) -> usize {
        9
    }

    /// Finds the first cell with the given value
    fn find_value(&self, value: usize) -> Option<usize> {
        for i in 0..self.max() {
            if self.get_cell(i).get_value() == value {
                return Some(i);
            }
        }
        None
    }

    /// Returns true if the given value is present in this collection
    fn has_value(&self, value: usize) -> bool {
        match self.find_value(value) {
            Some(_) => true,
            None => false,
        }
    }

    /// Returns true if the given value is possible in this collection
    fn has_possible(&self, value: Mark) -> bool {
        for i in 0..self.max() {
            if self.get_cell(i).is_possible(value) {
                return true;
            }
        }
        false
    }

    /// Returns the amount of cells with the given possible value
    fn count_possible(&self, value: Mark) -> u32 {
        let mut count = 0;
        for i in 0..self.max() {
            if self.get_cell(i).is_possible(value) {
                count += 1;
            }
        }
        count
    }

    /// Returns the amount of cells that are determined
    fn count_determined(&self) -> u32 {
        let mut count = 0;
        for i in 0..self.max() {
            if self.get_cell(i).is_determined() {
                count += 1;
            }
        }
        count
    }

    /// Returns a new iterator over the cells in this collection
    fn iter(&self) -> Box<dyn Iterator<Item = usize>> {
        Box::new((0..self.max()).into_iter())
    }

    /// Returns a new iterator over the coordinates in this collection
    fn iter_coords(&self) -> Box<dyn Iterator<Item = Coord> + '_> {
        Box::new((0..self.max()).into_iter().map(move |i| self.get_coord(i)))
    }

    /// Returns a new iterator over the cells in this collection
    fn iter_cells(&self) -> Box<dyn Iterator<Item = Cell> + '_> {
        Box::new((0..self.max()).into_iter().map(move |i| self.get_cell(i)))
    }
}
