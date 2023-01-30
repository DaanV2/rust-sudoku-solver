use super::{cell::Cell, mark::Mark};

const GRID_HEIGHT: usize = 9;
const GRID_WIDTH: usize = 9;
const GRID_SIZE: usize = GRID_HEIGHT * GRID_WIDTH;

pub struct Grid {
    // The grid is a vector of vectors of cells
    grid: [Cell; GRID_SIZE],
}

pub fn get_index(row: usize, col: usize) -> usize {
    (row * GRID_WIDTH + col) as usize
}

pub fn to_row_col(index: usize) -> (usize, usize) {
    (index / GRID_WIDTH, index % GRID_HEIGHT)
}

impl Grid {
    pub fn new() -> Grid {
        Grid {
            grid: [Cell::new(); GRID_SIZE],
        }
    }

    pub fn copy(&self) -> Grid {
        Grid { 
            grid: self.grid.clone()
        }
    }

    pub fn get_cell(&self, row: usize, col: usize) -> &Cell {
        &self.grid[get_index(row, col)]
    }

    pub fn set_cell(&mut self, row: usize, col: usize, cell: Cell) {
        self.grid[get_index(row, col)] = cell;
    }

    pub fn get_row(&self, row: usize) -> Row {
        Row {
            row,
            grid: self.grid,
        }
    }

    pub fn get_column(&self, col: usize) -> Column {
        Column {
            col,
            grid: self.grid,
        }
    }

    pub fn get_box(&self, row: usize, col: usize) -> Box {
        Box {
            row: row - row % 3,
            col: col - col % 3,
            grid: self.grid,
        }
    }
}

impl Default for Grid {
    fn default() -> Self {
        Self::new()
    }
}

pub trait Searchable {
    fn get_cell(&self, index: usize) -> &Cell;
    fn get_coords(&self, index: usize) -> (usize, usize);

    fn find_value(&self, value: u8) -> Option<usize> {
        for i in 0..9 {
            if self.get_cell(i).value == value {
                return Some(i);
            }
        }
        None
    }

    fn has_value(&self, value: u8) -> bool {
        for i in 0..9 {
            if self.get_cell(i).value == value {
                return true;
            }
        }
        false
    }

    fn has_possible(&self, value: Mark) -> bool {
        for i in 0..9 {
            if self.get_cell(i).is_possible(value) {
                return true;
            }
        }
        false
    }
}

pub struct Row {
    //The row index
    row : usize,
    //The entire grid
    grid: [Cell; GRID_SIZE],
}

impl Searchable for Row {
    fn get_cell(&self, index: usize) -> &Cell {
        &self.grid[get_index(self.row, index)]
    }

    fn get_coords(&self, index: usize) -> (usize, usize) {
        (self.row, index)
    }
}

pub struct Column {
    //The column index
    col : usize,
    //The entire grid
    grid: [Cell; GRID_SIZE],
}

impl Searchable for Column {
    fn get_cell(&self, index: usize) -> &Cell {
        &self.grid[get_index(index, self.col)]
    }

    fn get_coords(&self, index: usize) -> (usize, usize) {
        (index, self.col)
    }
}

pub struct Box {
    row : usize,
    col : usize,
    grid: [Cell; GRID_SIZE],
}

impl Searchable for Box {
    fn get_cell(&self, index: usize) -> &Cell {
        let row = self.row + index / 3;
        let col = self.col + index % 3;
        &self.grid[get_index(row, col)]
    }

    fn get_coords(&self, index: usize) -> (usize, usize) {
        let row = self.row + index / 3;
        let col = self.col + index % 3;
        (row, col)
    }
}
