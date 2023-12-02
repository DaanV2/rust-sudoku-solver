use std::fmt::Display;

use super::{
    cell::Cell,
    cell_collection::CellCollection,
    column::Column,
    constants::{GRID_HEIGHT_RANGE, GRID_SIZE, GRID_WIDTH_RANGE},
    coords::Coord,
    mark::Mark,
    row::Row,
    square::Square,
    utility::utility,
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

    /// Retrieves the cell at the given index
    pub fn get_cell(&self, index: usize) -> Cell {
        return self.cells[index];
    }

    /// Sets the cell at the given index
    pub fn set_cell(&mut self, index: usize, cell: Cell) {
        if index < self.cells.len() {
            self.cells[index] = cell;
        }
    }

    /// Retrieves the cell at the given coordinate
    pub fn get_cell_at(&self, coord: Coord) -> Cell {
        self.get_cell(coord.get_index())
    }

    /// Sets the cell at the given coordinate
    pub fn set_cell_at(&mut self, coord: Coord, cell: Cell) {
        self.set_cell(coord.get_index(), cell);
    }

    /// Retrieves the row at the given index
    pub fn get_row(&self, row: usize) -> Row {
        Row::new(row)
    }

    /// Retrieves the column at the given index
    pub fn get_column(&self, col: usize) -> Column {
        Column::new(col)
    }

    /// Retrieves the square at the given row and column
    pub fn get_square_at(&self, coord: Coord) -> Square {
        let (row, col) = coord.get_row_col();
        Square::from(row, col)
    }

    /// Retrieves the square at the given row and column
    pub fn get_square(&self, row: usize, col: usize) -> Square {
        Square::from(row, col)
    }

    /// Returns true if the given value is present in this collection
    pub fn is_possible_at(&self, coord: Coord, mark: Mark) -> bool {
        self.get_cell_at(coord).is_possible(mark)
    }

    /// Returns true if the given value is possible for this cell
    pub fn is_possible(&self, index: usize, mark: Mark) -> bool {
        self.get_cell(index).is_possible(mark)
    }

    /// Returns true if the given value is possible for this cell
    pub fn set_possible_at(&mut self, coord: Coord, mark: Mark) {
        self.set_possible(coord.get_index(), mark);
    }

    /// Un sets the given value as possible for this cell
    pub fn unset_possible_at(&mut self, coord: Coord, mark: Mark) {
        self.unset_possible(coord.get_index(), mark);
    }

    /// Returns true if the given value is present in this collection
    pub fn set_possible(&mut self, index: usize, mark: Mark) {
        let old_cell = self.get_cell(index);
        if old_cell.is_determined() {
            return;
        }

        let mut new_cell = old_cell.clone();
        new_cell.set_possible(mark);
        self.set_cell(index, new_cell);
    }

    /// Un sets the given value as possible for this cell
    pub fn unset_possible(&mut self, index: usize, mark: Mark) {
        let old_cell = self.get_cell(index);
        if old_cell.is_determined() {
            return;
        }

        let mut new_cell = old_cell.clone();
        new_cell.unset_possible(mark);
        self.set_cell(index, new_cell);
    }

    /// Iterates over all rows
    pub fn iter_rows(&self) -> impl Iterator<Item = Row> + '_ {
        GRID_HEIGHT_RANGE.map(move |row| self.get_row(row))
    }

    /// Iterates over all columns
    pub fn iter_columns(&self) -> impl Iterator<Item = Column> + '_ {
        GRID_WIDTH_RANGE.map(move |col| self.get_column(col))
    }

    /// Iterates over all squares
    pub fn iter_squares(&self) -> impl Iterator<Item = Square> + '_ {
        Square::iter_coords().map(move |coord| self.get_square_at(coord))
    }

    /// Iterates over all cells and counts the determined cells
    pub fn count_determined(&self) -> usize {
        let mut sum: usize = 0;
        for c in self.cells {
            if c.is_determined() {
                sum += 1;
            }
        }

        sum
    }
}

impl Default for Grid {
    fn default() -> Self {
        Self::new()
    }
}

impl CellCollection for Grid {
    fn get_coord(&self, index: usize) -> Coord {
        Coord::from_index(index)
    }

    fn max(&self) -> usize {
        GRID_SIZE
    }

    fn iter(&self) -> std::ops::Range<usize> {
        0..GRID_SIZE
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", utility::ascii_grid(self))
    }
}

#[cfg(test)]
mod tests {
    use super::Grid;
    use crate::{
        grid::{
            cell_collection::CellCollection,
            constants::{GRID_HEIGHT_RANGE, GRID_WIDTH_RANGE},
        },
        test::util::general_tests,
    };
    use std::mem::size_of_val;

    #[test]
    fn output_gridsize_obj() {
        let grid = Grid::new();

        println!("Grid object size: {}", size_of_val(&grid));
    }

    #[test]
    fn cell_same_at_index() {
        let grid = general_tests::filled_sudoku();

        let index = 64;
        let coord = grid.get_coord(index);

        assert_eq!(grid.get_cell(index), grid.get_cell_at(coord));
    }

    #[test]
    fn test_get_row() {
        for row_index in GRID_HEIGHT_RANGE {
            let grid = general_tests::filled_sudoku();
            let row = grid.get_row(row_index);

            for col_index in GRID_WIDTH_RANGE {
                let coord = row.get_coord(col_index);
                let cell = grid.get_cell_at(coord);

                let row_cell = grid.get_cell_at(coord);

                assert_eq!(cell, row_cell);
                assert_eq!(coord.get_row(), row_index);
            }
        }
    }

    #[test]
    fn test_get_column() {
        for col_index in GRID_WIDTH_RANGE {
            let grid = general_tests::filled_sudoku();
            let column = grid.get_column(col_index);

            for row_index in GRID_HEIGHT_RANGE {
                let coord = column.get_coord(row_index);
                let cell = grid.get_cell_at(coord);

                let column_cell = grid.get_cell_at(coord);
                assert_eq!(cell, column_cell);
                assert_eq!(coord.get_col(), col_index);
            }
        }
    }
}
