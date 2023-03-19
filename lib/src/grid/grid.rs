use std::fmt::Display;

use super::{
    cell::Cell,
    cell_collection::CellCollection,
    column::Column,
    constants::GRID_SIZE,
    coords::Coord,
    format::{get_index, to_row_col},
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

    pub fn copy(&self) -> Grid {
        Grid {
            cells: self.cells.clone(),
        }
    }

    /// Retrieves the cell at the given index
    pub fn get_cell(&self, index: usize) -> &Cell {
        &self.cells[index]
    }

    /// Sets the cell at the given index
    pub fn set_cell(&mut self, index: usize, cell: &Cell) {
        self.cells[index] = *cell;
    }

    /// Retrieves the cell at the given coordinate
    pub fn get_cell_at(&self, coord: Coord) -> &Cell {
        &self.get_cell(get_index(&coord))
    }

    /// Sets the cell at the given coordinate
    pub fn set_cell_at(&mut self, coord: Coord, cell: &Cell) {
        self.set_cell(get_index(&coord), cell);
    }

    pub fn get_row(&self, row: usize) -> Row {
        Row::new(row, self.cells)
    }

    pub fn get_column(&self, col: usize) -> Column {
        Column::new(col, self.cells)
    }

    pub fn get_square_at(&self, coord: Coord) -> Square {
        Square::from(coord.row, coord.col, self.cells)
    }

    pub fn get_square(&self, row: usize, col: usize) -> Square {
        Square::from(row, col, self.cells)
    }

    pub fn is_possible_at(&self, coord: Coord, mark: Mark) -> bool {
        self.get_cell_at(coord).is_possible(mark)
    }

    pub fn is_possible(&self, index: usize, mark: Mark) -> bool {
        self.get_cell(index).is_possible(mark)
    }

    pub fn set_possible_at(&mut self, coord: Coord, mark: Mark) {
        self.set_possible(get_index(&coord), mark);
    }

    pub fn unset_possible_at(&mut self, coord: Coord, mark: Mark) {
        self.unset_possible(get_index(&coord), mark);
    }

    pub fn set_possible(&mut self, index: usize, mark: Mark) {
        let new_cell: &mut Cell = &mut self.get_cell(index).clone();
        if new_cell.is_determined() {
            return;
        }

        new_cell.set_possible(mark);
        self.set_cell(index, &new_cell);
    }

    pub fn unset_possible(&mut self, index: usize, mark: Mark) {
        let new_cell: &mut Cell = &mut self.get_cell(index).clone();
        if new_cell.is_determined() || new_cell.possibilities.is_empty() {
            return;
        }

        new_cell.unset_possible(mark);
        self.set_cell(index, &new_cell);
    }

    pub fn iter_rows(&self) -> impl Iterator<Item = Row> + '_ {
        (0..9).map(move |row| self.get_row(row))
    }

    pub fn iter_columns(&self) -> impl Iterator<Item = Column> + '_ {
        (0..9).map(move |col| self.get_column(col))
    }

    pub fn iter_squares(&self) -> impl Iterator<Item = Square> + '_ {
        static COORDS: [Coord; 9] = [
            Coord { row: 0, col: 0 },
            Coord { row: 0, col: 3 },
            Coord { row: 0, col: 6 },
            Coord { row: 3, col: 0 },
            Coord { row: 3, col: 3 },
            Coord { row: 3, col: 6 },
            Coord { row: 6, col: 0 },
            Coord { row: 6, col: 3 },
            Coord { row: 6, col: 6 },
        ];

        COORDS.iter().map(move |coord| self.get_square_at(*coord))
    }
}

impl Default for Grid {
    fn default() -> Self {
        Self::new()
    }
}

impl CellCollection for Grid {
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
            mark::Mark,
        },
        test::util::general_tests,
    };
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

                let row_cell = row.get_cell(col_index);
                assert_eq!(cell, row_cell);
                assert_eq!(coord.row, row_index);
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

                let column_cell = column.get_cell(row_index);
                assert_eq!(cell, column_cell);
                assert_eq!(coord.col, col_index);
            }
        }
    }
}
