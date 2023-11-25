#[cfg(test)]
mod test {
    use crate::{
        grid::{cell::Cell, cell_collection::CellCollection, grid::Grid},
        test::util::general_tests,
    };

    #[test]
    fn test_1_square_missing() {
        let grid: &mut Grid = &mut general_tests::filled_sudoku();

        clear_square(grid, 3, 3);

        general_tests::test_should_solve(*grid);
    }

    #[test]
    fn test_2_square_missing() {
        let grid: &mut Grid = &mut general_tests::filled_sudoku();

        clear_square(grid, 3, 3);
        clear_square(grid, 0, 6);

        general_tests::test_should_solve(*grid);
    }

    #[test]
    fn test_3_square_missing() {
        let grid: &mut Grid = &mut general_tests::filled_sudoku();

        clear_square(grid, 3, 3);
        clear_square(grid, 0, 6);
        clear_square(grid, 6, 0);

        general_tests::test_should_solve(*grid);
    }

    fn clear_square(grid: &mut Grid, row: usize, col: usize) {
        let sq = grid.get_square(row, col);
        for index in sq.iter() {
            let coord = sq.get_coord(index);
            grid.set_cell_at(coord, Cell::new_with_value(0));
        }
    }
}
