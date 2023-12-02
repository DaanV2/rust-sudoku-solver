use super::coords::Coord;

/// A trait for collection of cells
pub trait CellCollection {
    /// Sets the cell at the given index
    fn get_coord(&self, index: usize) -> Coord;
    /// Returns a new iterator over the cells in this collection
    fn iter(&self) -> std::ops::Range<usize>;
    /// Returns the maximum number of cells in this collection
    fn max(&self) -> usize;
}
