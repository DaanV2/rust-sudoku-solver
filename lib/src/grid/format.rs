use super::{
    constants::{GRID_HEIGHT, GRID_WIDTH},
    coords::Coord,
};

// Calculate the index of a cell in the grid from its row and column
pub fn get_index(row: usize, col: usize) -> usize {
    (row * GRID_WIDTH + col) as usize
}

// Calculate the row and column of a cell in the grid from its index
pub fn to_row_col(index: usize) -> Coord {
    Coord::new(index / GRID_WIDTH, index % GRID_HEIGHT)
}

pub fn get_square_index(row: usize, col: usize) -> usize {
    (row / 3) * 3 + col / 3
}
