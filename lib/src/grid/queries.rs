use super::{cell_collection::CellCollection, grid::Grid, mark::Mark, slice::Slice};

pub fn count_determine_value<T: CellCollection>(grid: &Grid, area: T, value: u16) -> usize {
    if area.max() == 9 {
        let s = Slice::from(grid, &area);

        return s.count_determined_value(value);
    }

    let mut count = 0;
    for i in area.iter() {
        let c = area.get_coord(i);
        let cell = grid.get_cell_at(c);

        if cell.get_value() == value {
            count += 1;
        }
    }

    count
}

pub fn count_possible<T: CellCollection>(grid: &Grid, area: T, mark: Mark) -> usize {
    if area.max() == 9 {
        let s = Slice::from(grid, &area);

        return s.count_possible(mark);
    }

    let mut count = 0;
    for i in area.iter() {
        let c = area.get_coord(i);
        let cell = grid.get_cell_at(c);

        if cell.is_possible(mark) {
            count += 1;
        }
    }

    count
}
