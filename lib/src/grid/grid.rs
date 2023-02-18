use super::{
    cell::Cell,
    column::Column,
    constants::GRID_SIZE,
    coords::Coord,
    format::{get_index, to_row_col},
    row::Row,
    searchable::Searchable,
    square::Square,
};

#[derive(Debug, Clone, Copy)]
pub struct Grid {
    // The grid is a vector of vectors of cells
    grid: [Cell; GRID_SIZE],
}

impl Grid {
    pub fn new() -> Grid {
        Grid {
            grid: [Cell::new(); GRID_SIZE],
        }
    }

    pub fn copy(&self) -> Grid {
        Grid {
            grid: self.grid.clone(),
        }
    }

    /// Retrieves the cell at the given index
    pub fn get(&self, index: usize) -> &Cell {
        &self.grid[index]
    }

    /// Sets the cell at the given index
    pub fn set(&mut self, index: usize, cell: Cell) {
        self.grid[index] = cell;
    }

    /// Retrieves the cell at the given coordinate
    pub fn get_cell(&self, coord: Coord) -> &Cell {
        &self.grid[get_index(coord)]
    }

    /// Sets the cell at the given coordinate
    pub fn set_cell(&mut self, coord: Coord, cell: Cell) {
        self.grid[get_index(coord)] = cell;
    }

    pub fn get_row(&self, row: usize) -> Row {
        Row::new(row, self.grid)
    }

    pub fn get_column(&self, col: usize) -> Column {
        Column::new(col, self.grid)
    }

    pub fn get_square(&self, row: usize, col: usize) -> Square {
        Square::from(row, col, self.grid)
    }
}

impl Default for Grid {
    fn default() -> Self {
        Self::new()
    }
}

impl Searchable for Grid {
    fn get_cell(&self, index: usize) -> &Cell {
        &self.grid[index]
    }

    fn get_coord(&self, index: usize) -> Coord {
        to_row_col(index)
    }

    fn max(&self) -> usize {
        self.grid.len()
    }
}

#[cfg(test)]
mod tests {
    use crate::grid::{mark::Mark, searchable::Searchable};

    use super::Grid;

    #[test]
    fn it_works() {
        let grid = Grid::new();

        if !grid.get_row(3).has_possible(Mark::N1) {
            panic!("Row 3 should have 1 as a possible value");
        }
    }
}
