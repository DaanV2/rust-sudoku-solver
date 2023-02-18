use super::{
    constants::{GRID_HEIGHT, GRID_WIDTH},
    coords::Coord,
};

// Calculate the index of a cell in the grid from its row and column
#[inline(always)]
pub fn get_index(coord: Coord) -> usize {
    (coord.get_row() * GRID_WIDTH + coord.get_col()) as usize
}

// Calculate the row and column of a cell in the grid from its index
#[inline(always)]
pub fn to_row_col(index: usize) -> Coord {
    Coord::new(index / GRID_WIDTH, index % GRID_HEIGHT)
}
