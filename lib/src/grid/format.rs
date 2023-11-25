use super::{constants::GRID_WIDTH, coords::Coord};

// Calculate the index of a cell in the grid from its row and column
#[inline(always)]
pub const fn get_index(coord: Coord) -> usize {
    get_index_from(coord.get_row(), coord.get_col())
}

// Calculate the index of a cell in the grid from its row and column
#[inline(always)]
pub const fn get_index_from(row: usize, col: usize) -> usize {
    row * GRID_WIDTH + col
}
