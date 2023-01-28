

struct Grid {
    // The grid is a vector of vectors of cells
    grid: [Cell; 9*9],
}

impl Grid {
    pub fn new() -> Grid {
        Grid {
            grid: [Cell::new(); 9*9]
        }
    }

    pub fn copy(&self) -> Grid {
        Grid {
            grid: self.grid
        }
    }

    pub fn get_cell(&self, row: u8, col: u8) -> &Cell {
        &self.grid[(row * 9 + col) as usize]
    }

    pub fn get_cell_mut(&mut self, row: u8, col: u8) -> &mut Cell {
        &mut self.grid[(row * 9 + col) as usize]
    }

    pub fn to_index(row: usize, col: usize) -> usize {
        (row * 9 + col)
    }

    pub fn to_row_col(index: usize) -> (usize, usize) {
        (index / 9, index % 9)
    }

    pub fn get_block_from_index(index: usize) -> usize {
        let (row, col) = Grid::to_row_col(index);
        
    }

    pub fn row_iter(&self, row: usize) -> impl Iterator<Item = &Cell> {
        self.grid[row * 9..(row + 1) * 9].iter()
    }

    pub fn col_iter(&self, col: usize) -> impl Iterator<Item = &Cell> {
        (0..9).map(move |row| &self.grid[row * 9 + col])
    }

    pub fn block_iter(&self, block: usize) -> impl Iterator<Item = &Cell> {
        let row = block / 3;
        let col = block % 3;
        (0..3).flat_map(move |r| (0..3).map(move |c| &self.grid[(row * 3 + r) * 9 + col * 3 + c]))
    }
}