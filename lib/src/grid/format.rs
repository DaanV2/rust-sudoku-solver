use super::{
    constants::{GRID_HEIGHT, GRID_WIDTH},
    coords::Coord,
};

// Calculate the index of a cell in the grid from its row and column
#[inline(always)]
pub fn get_index(coord: Coord) -> usize {
    (coord.row * GRID_WIDTH + coord.col) as usize
}

// Calculate the row and column of a cell in the grid from its index
#[inline(always)]
pub fn to_row_col(index: usize) -> Coord {
    Coord::new(index / GRID_WIDTH, index % GRID_HEIGHT)
}

#[cfg(test)]
mod tests {
    use crate::grid::{
        constants::{GRID_HEIGHT, GRID_WIDTH},
        coords::Coord,
        format::{get_index, to_row_col},
    };

    #[test]
    fn test_index() {
        for row in 0..GRID_HEIGHT {
            for col in 0..GRID_WIDTH {
                let coord = Coord::new(row, col);
                let index = get_index(coord);
                let coord2 = to_row_col(index);

                assert_eq!(coord2, coord);
            }
        }
    }
}
