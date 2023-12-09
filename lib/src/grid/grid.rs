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

    pub fn empty() -> Grid {
        Grid {
            cells: [Cell::new_empty(); GRID_SIZE],
        }
    }

    pub fn from(cells: [Cell; GRID_SIZE]) -> Grid {
        Grid { cells: cells }
    }

    /// Retrieves the cell at the given index
    pub fn get_cell(&self, index: usize) -> &Cell {
        unsafe {
            return self.cells.get_unchecked(index);
        }
    }

    /// Sets the cell at the given index
    pub fn set_cell(&mut self, index: usize, cell: &Cell) {
        unsafe {
            cell.clone_into(self.cells.get_unchecked_mut(index));
        }
    }

    /// Retrieves the cell at the given coordinate
    pub fn get_cell_at(&self, coord: Coord) -> &Cell {
        self.get_cell(coord.get_index())
    }

    /// Sets the cell at the given coordinate
    pub fn set_cell_at(&mut self, coord: Coord, cell: &Cell) {
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
        let mut new_cell = self.get_cell(index).clone();
        new_cell.set_possible(mark);
        if !new_cell.is_determined() {
            self.set_cell(index, &new_cell);
        }
    }

    /// Un sets the given value as possible for this cell
    pub fn unset_possible(&mut self, index: usize, mark: Mark) {
        let mut new_cell = self.get_cell(index).clone();
        new_cell.unset_possible(mark);
        self.set_cell(index, &new_cell);
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

    pub fn clone_to(&self, to: &mut Grid) {
        for index in self.iter() {
            to.set_cell(index, self.get_cell(index));
        }
    }

    #[inline(always)]
    pub fn place_value(&mut self, index: usize, value: usize) {
        let coord = Coord::from_index(index);
        self.place_value_at(coord, value);
    }

    #[inline(always)]
    pub fn place_value_at(&mut self, coord: Coord, value: usize) {
        self.set_cell_at(coord, &Cell::new_with_value(value));

        self.mark_off(coord);
    }

    #[inline(always)]
    pub fn mark_off(&mut self, coord: Coord) {
        let mark = Mark::from_value(self.get_cell_at(coord).get_value());
        let (row, col) = coord.get_row_col();

        // Mark off row
        for c in GRID_WIDTH_RANGE.rev() {
            self.unset_possible_at(Coord::new(row, c), mark);
        }

        // Mark off column
        for r in GRID_HEIGHT_RANGE.rev() {
            self.unset_possible_at(Coord::new(r, col), mark);
        }

        // Mark off square
        let square = self.get_square_at(coord);
        for c in square.iter().rev() {
            self.unset_possible_at(square.get_coord(c), mark);
        }
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
            cell::Cell,
            cell_collection::CellCollection,
            constants::{GRID_HEIGHT_RANGE, GRID_WIDTH_RANGE},
            coords::Coord,
            mark::Mark,
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
    fn cell_get_set() {
        let mut grid = general_tests::filled_sudoku();
        let index: usize = 64;
        let mut c = Cell::new_with_value(9);
        c.set_possible(Mark::N3);
        c.set_possible(Mark::N4);

        grid.set_cell(index, &c);
        let t = grid.get_cell(index);

        assert_eq!(t, &c);
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

    #[test]
    fn test_place() {
        let mut grid = Grid::new();

        let index = 64;
        let coord = grid.get_coord(index);

        grid.place_value_at(coord, 9);

        assert_eq!(grid.get_cell_at(coord).get_value(), 9);

        //Check row
        for col in GRID_WIDTH_RANGE {
            let at = Coord::new(coord.get_row(), col);
            if at == coord {
                continue;
            }

            let c = grid.get_cell_at(at);
            assert_eq!(c.is_possible(Mark::N9), false);
        }

        //Check column
        for row in GRID_HEIGHT_RANGE {
            let at = Coord::new(row, coord.get_col());
            if at == coord {
                continue;
            }

            let c = grid.get_cell_at(at);
            assert_eq!(c.is_possible(Mark::N9), false);
        }

        //Check square
        let square = grid.get_square_at(coord);
        for c in square.iter() {
            let at = square.get_coord(c);
            if at == coord {
                continue;
            }

            let c = grid.get_cell_at(at);
            assert_eq!(c.is_possible(Mark::N9), false);
        }
    }
}
