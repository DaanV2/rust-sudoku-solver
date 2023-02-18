use super::{
    cell::Cell, constants::GRID_SIZE, coords::Coord, format::get_index, searchable::Searchable,
};

pub struct Square {
    //The row index the square starts at
    row: usize,
    //The column index the square starts at
    col: usize,
    grid: [Cell; GRID_SIZE],
}

impl Square {
    pub fn new(row: usize, col: usize, grid: [Cell; GRID_SIZE]) -> Self {
        Self { row, col, grid }
    }

    pub fn from(row: usize, col: usize, grid: [Cell; GRID_SIZE]) -> Self {
        let row_offset = row - row % 3;
        let col_offset = col - col % 3;

        Square::new(row_offset, col_offset, grid)
    }
}

impl Searchable for Square {
    fn get_cell(&self, index: usize) -> &Cell {
        let row = self.row + index / 3;
        let col = self.col + index % 3;
        &self.grid[get_index(row, col)]
    }

    fn get_coords(&self, index: usize) -> Coord {
        let row = self.row + index / 3;
        let col = self.col + index % 3;
        Coord::new(row, col)
    }
}
