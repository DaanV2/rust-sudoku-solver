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

#[cfg(test)]
mod test {
    use super::Row;
    use crate::grid::{cell::Cell, constants::GRID_HEIGHT_RANGE, searchable::Searchable};

    #[test]
    fn test_coords() {
        for row_index in GRID_HEIGHT_RANGE {
            let row = Row::new(row_index, [Cell::new(); 81]);

            for col_index in GRID_HEIGHT_RANGE {
                let coord = row.get_coord(col_index);
                assert_eq!(coord.row, row_index);
                assert_eq!(coord.col, col_index);
            }
        }
    }
}
