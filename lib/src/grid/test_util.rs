pub mod test_util {
    use rand::Rng;

    use crate::grid::{
        cell::Cell,
        constants::{GRID_HEIGHT_RANGE, GRID_SIZE},
        coords::Coord,
        grid::Grid,
        searchable::Searchable,
    };

    pub fn filled_sudoku() -> Grid {
        let cells: [Cell; GRID_SIZE] = [
            //435269781
            Cell::new_with_value(4),
            Cell::new_with_value(3),
            Cell::new_with_value(5),
            Cell::new_with_value(2),
            Cell::new_with_value(6),
            Cell::new_with_value(9),
            Cell::new_with_value(7),
            Cell::new_with_value(8),
            Cell::new_with_value(1),
            //682571493
            Cell::new_with_value(6),
            Cell::new_with_value(8),
            Cell::new_with_value(2),
            Cell::new_with_value(5),
            Cell::new_with_value(7),
            Cell::new_with_value(1),
            Cell::new_with_value(4),
            Cell::new_with_value(9),
            Cell::new_with_value(3),
            //197834562
            Cell::new_with_value(1),
            Cell::new_with_value(9),
            Cell::new_with_value(7),
            Cell::new_with_value(8),
            Cell::new_with_value(3),
            Cell::new_with_value(4),
            Cell::new_with_value(5),
            Cell::new_with_value(6),
            Cell::new_with_value(2),
            //826195347
            Cell::new_with_value(8),
            Cell::new_with_value(2),
            Cell::new_with_value(6),
            Cell::new_with_value(1),
            Cell::new_with_value(9),
            Cell::new_with_value(5),
            Cell::new_with_value(3),
            Cell::new_with_value(4),
            Cell::new_with_value(7),
            //374682915
            Cell::new_with_value(3),
            Cell::new_with_value(7),
            Cell::new_with_value(4),
            Cell::new_with_value(6),
            Cell::new_with_value(8),
            Cell::new_with_value(2),
            Cell::new_with_value(9),
            Cell::new_with_value(1),
            Cell::new_with_value(5),
            //951743628
            Cell::new_with_value(9),
            Cell::new_with_value(5),
            Cell::new_with_value(1),
            Cell::new_with_value(7),
            Cell::new_with_value(4),
            Cell::new_with_value(3),
            Cell::new_with_value(6),
            Cell::new_with_value(2),
            Cell::new_with_value(8),
            //519326874
            Cell::new_with_value(5),
            Cell::new_with_value(1),
            Cell::new_with_value(9),
            Cell::new_with_value(3),
            Cell::new_with_value(2),
            Cell::new_with_value(6),
            Cell::new_with_value(8),
            Cell::new_with_value(7),
            Cell::new_with_value(4),
            //248957136
            Cell::new_with_value(2),
            Cell::new_with_value(4),
            Cell::new_with_value(8),
            Cell::new_with_value(9),
            Cell::new_with_value(5),
            Cell::new_with_value(7),
            Cell::new_with_value(1),
            Cell::new_with_value(3),
            Cell::new_with_value(6),
            //763418259
            Cell::new_with_value(7),
            Cell::new_with_value(6),
            Cell::new_with_value(3),
            Cell::new_with_value(4),
            Cell::new_with_value(1),
            Cell::new_with_value(8),
            Cell::new_with_value(2),
            Cell::new_with_value(5),
            Cell::new_with_value(9),
        ];

        Grid::from(cells)
    }

    pub fn remove_cells(grid: &mut Grid) {
        let mut rng = rand::thread_rng();
        let amount = rng.gen_range((grid.max() / 2)..grid.max());

        remove_cells_amount(grid, amount);
    }

    pub fn remove_cells_amount(grid: &mut Grid, amount: usize) {
        let mut rng = rand::thread_rng();
        let mut removed = 0;

        while removed < amount {
            let index = rng.gen_range(0..grid.max());
            let cell = &grid.get(index);

            if cell.is_determined() {
                grid.set(index, Cell::new());
                removed += 1;
            }
        }
    }

    pub fn ascii_grid(grid: &Grid) -> String {
        let mut result = String::new();

        for y in GRID_HEIGHT_RANGE {
            for x in GRID_HEIGHT_RANGE {
                let coord = Coord::new(x, y);
                let cell = grid.get_cell(coord);
                let value = cell.value;

                if cell.is_determined() {
                    result.push_str(&format!("{} ", value));
                } else {
                    result.push_str(". ");
                }

                if x == 2 || x == 5 {
                    result.push_str("| ");
                }
            }
            result.push_str("\n");

            if y == 2 || y == 5 {
                result.push_str("------|-------|------\n");
            }
        }

        result
    }
}
