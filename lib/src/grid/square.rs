use super::{
    cell::Cell, cell_collection::CellCollection, constants::GRID_SIZE, coords::Coord,
    format::get_index,
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Square {
    //The row index the square starts at
    pub row: usize,
    //The column index the square starts at
    pub col: usize,
    pub grid: [Cell; GRID_SIZE],
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

    pub fn get_cell_at(&self, coord: Coord) -> &Cell {
        let c = Coord::new(self.row + coord.row, self.col + coord.col);
        &self.grid[get_index(c)]
    }

    pub fn get_coord_at(&self, row: usize, col: usize) -> Coord {
        Coord::new(self.row + row, self.col + col)
    }

    pub fn is_column_in_square(&self, col: usize) -> bool {
        col >= self.col && col < self.col + 3
    }

    pub fn is_row_in_square(&self, row: usize) -> bool {
        row >= self.row && row < self.row + 3
    }

    pub fn is_coord_in_square(&self, coord: Coord) -> bool {
        self.is_row_in_square(coord.row) && self.is_column_in_square(coord.col)
    }
}

impl CellCollection for Square {
    fn get_cell(&self, index: usize) -> &Cell {
        let coord = self.get_coord(index);
        &self.grid[get_index(coord)]
    }

    fn get_coord(&self, index: usize) -> Coord {
        let row = self.row + index / 3;
        let col = self.col + index % 3;
        Coord::new(row, col)
    }
}

impl Default for Square {
    fn default() -> Self {
        Self {
            row: 0,
            col: 0,
            grid: [Cell::default(); GRID_SIZE],
        }
    }
}
