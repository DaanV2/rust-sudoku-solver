use super::{
    cell::Cell, constants::GRID_SIZE, coords::Coord, format::get_index, searchable::Searchable,
    square::Square,
};

pub struct Row {
    //The row index
    row: usize,
    //The entire grid
    grid: [Cell; GRID_SIZE],
}

impl Row {
    pub fn new(row: usize, grid: [Cell; GRID_SIZE]) -> Self {
        Self { row, grid }
    }

    pub fn get_square(&self, index: usize) -> Square {
        Square::from(self.row, index, self.grid)
    }
}

impl Searchable for Row {
    fn get_cell(&self, index: usize) -> &Cell {
        &self.grid[get_index(self.get_coord(index))]
    }

    fn get_coord(&self, index: usize) -> Coord {
        Coord::new(self.row, index)
    }
}
