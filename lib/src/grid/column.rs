use super::{
    cell::Cell, constants::GRID_SIZE, coords::Coord, format::get_index, searchable::Searchable,
    square::Square,
};

pub struct Column {
    //The column index
    col: usize,
    //The entire grid
    grid: [Cell; GRID_SIZE],
}

impl Column {
    pub fn new(col: usize, grid: [Cell; GRID_SIZE]) -> Self {
        Self { col, grid }
    }

    pub fn get_square(&self, index: usize) -> Square {
        Square::from(index, self.col, self.grid)
    }
}

impl Searchable for Column {
    fn get_cell(&self, index: usize) -> &Cell {
        &self.grid[get_index(self.get_coord(index))]
    }

    fn get_coord(&self, index: usize) -> Coord {
        Coord::new(index, self.col)
    }
}

#[cfg(test)]
mod test {
    use super::Column;
    use crate::grid::{cell::Cell, constants::GRID_HEIGHT_RANGE, searchable::Searchable};

    #[test]
    fn test_coords() {
        for col_index in GRID_HEIGHT_RANGE {
            let row = Column::new(col_index, [Cell::new(); 81]);

            for row_index in GRID_HEIGHT_RANGE {
                let coord = row.get_coord(row_index);
                assert_eq!(coord.row, row_index);
                assert_eq!(coord.col, col_index);
            }
        }
    }
}
