use super::{
    cell::Cell, constants::GRID_SIZE, coords::Coord, format::get_index, searchable::Searchable,
    square::Square,
};

pub struct Column {
    //The column index
    col: usize,
    //The entire grid
    grid: [Cell; GRID_SIZE],
}

impl Column {
    pub fn new(col: usize, grid: [Cell; GRID_SIZE]) -> Self {
        Self { col, grid }
    }

    pub fn get_square(&self, index: usize) -> Square {
        Square::from(index, self.col, self.grid)
    }
}

impl Searchable for Column {
    fn get_cell(&self, index: usize) -> &Cell {
        &self.grid[get_index(index, self.col)]
    }

    fn get_coords(&self, index: usize) -> Coord {
        Coord::new(index, self.col)
    }
}
