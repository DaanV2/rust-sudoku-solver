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
    cells: [Cell; GRID_SIZE],
}

impl Grid {
    pub fn new() -> Grid {
        Grid {
            cells: [Cell::new(); GRID_SIZE],
        }
    }

    pub fn from(cells: [Cell; GRID_SIZE]) -> Grid {
        Grid { cells: cells }
    }

    pub fn copy(&self) -> Grid {
        Grid {
            cells: self.cells.clone(),
        }
    }

    /// Retrieves the cell at the given index
    pub fn get(&self, index: usize) -> &Cell {
        &self.cells[index]
    }

    /// Sets the cell at the given index
    pub fn set(&mut self, index: usize, cell: Cell) {
        self.cells[index] = cell;
    }

    /// Retrieves the cell at the given coordinate
    pub fn get_cell(&self, coord: Coord) -> &Cell {
        &self.cells[get_index(coord)]
    }

    /// Sets the cell at the given coordinate
    pub fn set_cell(&mut self, coord: Coord, cell: Cell) {
        self.cells[get_index(coord)] = cell;
    }

    pub fn get_row(&self, row: usize) -> Row {
        Row::new(row, self.cells)
    }

    pub fn get_column(&self, col: usize) -> Column {
        Column::new(col, self.cells)
    }

    pub fn get_square(&self, row: usize, col: usize) -> Square {
        Square::from(row, col, self.cells)
    }
}

impl Default for Grid {
    fn default() -> Self {
        Self::new()
    }
}

impl Searchable for Grid {
    fn get_cell(&self, index: usize) -> &Cell {
        &self.cells[index]
    }

    fn get_coord(&self, index: usize) -> Coord {
        to_row_col(index)
    }

    fn max(&self) -> usize {
        self.cells.len()
    }
}

#[cfg(test)]
mod tests {
    use super::Grid;
    use crate::grid::{mark::Mark, searchable::Searchable, test_util::test_util};
    use std::mem::size_of_val;

    #[test]
    fn it_works() {
        let grid = Grid::new();

        if !grid.get_row(3).has_possible(Mark::N1) {
            panic!("Row 3 should have 1 as a possible value");
        }
    }

    #[test]
    fn output_gridsize_obj() {
        let grid = Grid::new();

        println!("Grid object size: {}", size_of_val(&grid));
    }

    #[test]
    fn cell_same_at_index() {
        let grid = test_util::filled_sudoku();

        let index = 64;
        let coord = grid.get_coord(index);

        assert_eq!(grid.get(index), grid.get_cell(coord));
    }
}
