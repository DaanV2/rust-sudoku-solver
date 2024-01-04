use super::{
    cell::Cell, constants::GRID_SIZE, coords::Coord, grid::Grid, mark::Mark, square::Square,
};

pub const GRID_MASK: Grid = Grid::from([Cell::mask(); GRID_SIZE]);
pub const UNSET_BIT_POSSIBLE_MASK: [Grid; 9] = generate_unset_possible_masks();
pub const SET_BIT_POSSIBLE_MASK: [Grid; 9] = generate_set_possible_masks();
pub const INFLUENCE_MASK: [Grid; GRID_SIZE] = generate_influence_masks();
pub const UNSET_INFLUENCE_MASK: [[Grid; GRID_SIZE]; 9] = generate_unset_influence_masks_all();

pub fn get_unset_influence_mask(coord: Coord, value: u16) -> Grid {
    debug_assert!(value >= 1 && value <= 9, "Value must be between 1 and 9");
    let index = (value - 1) as usize;

    unsafe {
        let set = UNSET_INFLUENCE_MASK.get_unchecked(index);
        let mask = set.get_unchecked(coord.get_index());
        return *mask;
    }
}

pub const fn get_unset_possible_mask(mark: Mark) -> Grid {
    UNSET_BIT_POSSIBLE_MASK[mark.to_index() as usize]
}

pub const fn get_unset_possible_mask_for_value(value: usize) -> Grid {
    UNSET_BIT_POSSIBLE_MASK[value - 1]
}

pub const fn get_set_possible_mask(mark: Mark) -> Grid {
    SET_BIT_POSSIBLE_MASK[mark.to_index() as usize]
}

pub const fn get_set_possible_mask_for_value(value: usize) -> Grid {
    SET_BIT_POSSIBLE_MASK[value - 1]
}

pub const fn get_influence_mask(coord: Coord) -> Grid {
    INFLUENCE_MASK[coord.get_index() as usize]
}

// Generate functions
const fn generate_influence_masks() -> [Grid; GRID_SIZE] {
    let mut masks = [Grid::empty(); GRID_SIZE];

    // Row 0
    masks[0] = generate_influence_mask(Coord::new(0, 0));
    masks[1] = generate_influence_mask(Coord::new(0, 1));
    masks[2] = generate_influence_mask(Coord::new(0, 2));
    masks[3] = generate_influence_mask(Coord::new(0, 3));
    masks[4] = generate_influence_mask(Coord::new(0, 4));
    masks[5] = generate_influence_mask(Coord::new(0, 5));
    masks[6] = generate_influence_mask(Coord::new(0, 6));
    masks[7] = generate_influence_mask(Coord::new(0, 7));
    masks[8] = generate_influence_mask(Coord::new(0, 8));

    // Row 1
    masks[9] = generate_influence_mask(Coord::new(1, 0));
    masks[10] = generate_influence_mask(Coord::new(1, 1));
    masks[11] = generate_influence_mask(Coord::new(1, 2));
    masks[12] = generate_influence_mask(Coord::new(1, 3));
    masks[13] = generate_influence_mask(Coord::new(1, 4));
    masks[14] = generate_influence_mask(Coord::new(1, 5));
    masks[15] = generate_influence_mask(Coord::new(1, 6));
    masks[16] = generate_influence_mask(Coord::new(1, 7));
    masks[17] = generate_influence_mask(Coord::new(1, 8));

    // Row 2
    masks[18] = generate_influence_mask(Coord::new(2, 0));
    masks[19] = generate_influence_mask(Coord::new(2, 1));
    masks[20] = generate_influence_mask(Coord::new(2, 2));
    masks[21] = generate_influence_mask(Coord::new(2, 3));
    masks[22] = generate_influence_mask(Coord::new(2, 4));
    masks[23] = generate_influence_mask(Coord::new(2, 5));
    masks[24] = generate_influence_mask(Coord::new(2, 6));
    masks[25] = generate_influence_mask(Coord::new(2, 7));
    masks[26] = generate_influence_mask(Coord::new(2, 8));

    // Row 3
    masks[27] = generate_influence_mask(Coord::new(3, 0));
    masks[28] = generate_influence_mask(Coord::new(3, 1));
    masks[29] = generate_influence_mask(Coord::new(3, 2));
    masks[30] = generate_influence_mask(Coord::new(3, 3));
    masks[31] = generate_influence_mask(Coord::new(3, 4));
    masks[32] = generate_influence_mask(Coord::new(3, 5));
    masks[33] = generate_influence_mask(Coord::new(3, 6));
    masks[34] = generate_influence_mask(Coord::new(3, 7));
    masks[35] = generate_influence_mask(Coord::new(3, 8));

    // Row 4
    masks[36] = generate_influence_mask(Coord::new(4, 0));
    masks[37] = generate_influence_mask(Coord::new(4, 1));
    masks[38] = generate_influence_mask(Coord::new(4, 2));
    masks[39] = generate_influence_mask(Coord::new(4, 3));
    masks[40] = generate_influence_mask(Coord::new(4, 4));
    masks[41] = generate_influence_mask(Coord::new(4, 5));
    masks[42] = generate_influence_mask(Coord::new(4, 6));
    masks[43] = generate_influence_mask(Coord::new(4, 7));
    masks[44] = generate_influence_mask(Coord::new(4, 8));

    // Row 5
    masks[45] = generate_influence_mask(Coord::new(5, 0));
    masks[46] = generate_influence_mask(Coord::new(5, 1));
    masks[47] = generate_influence_mask(Coord::new(5, 2));
    masks[48] = generate_influence_mask(Coord::new(5, 3));
    masks[49] = generate_influence_mask(Coord::new(5, 4));
    masks[50] = generate_influence_mask(Coord::new(5, 5));
    masks[51] = generate_influence_mask(Coord::new(5, 6));
    masks[52] = generate_influence_mask(Coord::new(5, 7));
    masks[53] = generate_influence_mask(Coord::new(5, 8));

    // Row 6
    masks[54] = generate_influence_mask(Coord::new(6, 0));
    masks[55] = generate_influence_mask(Coord::new(6, 1));
    masks[56] = generate_influence_mask(Coord::new(6, 2));
    masks[57] = generate_influence_mask(Coord::new(6, 3));
    masks[58] = generate_influence_mask(Coord::new(6, 4));
    masks[59] = generate_influence_mask(Coord::new(6, 5));
    masks[60] = generate_influence_mask(Coord::new(6, 6));
    masks[61] = generate_influence_mask(Coord::new(6, 7));
    masks[62] = generate_influence_mask(Coord::new(6, 8));

    // Row 7
    masks[63] = generate_influence_mask(Coord::new(7, 0));
    masks[64] = generate_influence_mask(Coord::new(7, 1));
    masks[65] = generate_influence_mask(Coord::new(7, 2));
    masks[66] = generate_influence_mask(Coord::new(7, 3));
    masks[67] = generate_influence_mask(Coord::new(7, 4));
    masks[68] = generate_influence_mask(Coord::new(7, 5));
    masks[69] = generate_influence_mask(Coord::new(7, 6));
    masks[70] = generate_influence_mask(Coord::new(7, 7));
    masks[71] = generate_influence_mask(Coord::new(7, 8));

    // Row 8
    masks[72] = generate_influence_mask(Coord::new(8, 0));
    masks[73] = generate_influence_mask(Coord::new(8, 1));
    masks[74] = generate_influence_mask(Coord::new(8, 2));
    masks[75] = generate_influence_mask(Coord::new(8, 3));
    masks[76] = generate_influence_mask(Coord::new(8, 4));
    masks[77] = generate_influence_mask(Coord::new(8, 5));
    masks[78] = generate_influence_mask(Coord::new(8, 6));
    masks[79] = generate_influence_mask(Coord::new(8, 7));
    masks[80] = generate_influence_mask(Coord::new(8, 8));

    masks
}

const fn generate_unset_influence_masks_all() -> [[Grid; GRID_SIZE]; 9] {
    let mut result = [[Grid::empty(); GRID_SIZE]; 9];

    result[0] = generate_unset_influence_masks(Mark::N1);
    result[1] = generate_unset_influence_masks(Mark::N2);
    result[2] = generate_unset_influence_masks(Mark::N3);
    result[3] = generate_unset_influence_masks(Mark::N4);
    result[4] = generate_unset_influence_masks(Mark::N5);
    result[5] = generate_unset_influence_masks(Mark::N6);
    result[6] = generate_unset_influence_masks(Mark::N7);
    result[7] = generate_unset_influence_masks(Mark::N8);
    result[8] = generate_unset_influence_masks(Mark::N9);

    return result;
}

const fn generate_unset_influence_masks(mark: Mark) -> [Grid; GRID_SIZE] {
    let mut masks = [Grid::empty(); GRID_SIZE];

    // Row 0
    masks[0] = generate_unset_influence_mask(Coord::new(0, 0), mark);
    masks[1] = generate_unset_influence_mask(Coord::new(0, 1), mark);
    masks[2] = generate_unset_influence_mask(Coord::new(0, 2), mark);
    masks[3] = generate_unset_influence_mask(Coord::new(0, 3), mark);
    masks[4] = generate_unset_influence_mask(Coord::new(0, 4), mark);
    masks[5] = generate_unset_influence_mask(Coord::new(0, 5), mark);
    masks[6] = generate_unset_influence_mask(Coord::new(0, 6), mark);
    masks[7] = generate_unset_influence_mask(Coord::new(0, 7), mark);
    masks[8] = generate_unset_influence_mask(Coord::new(0, 8), mark);

    // Row 1
    masks[9] = generate_unset_influence_mask(Coord::new(1, 0), mark);
    masks[10] = generate_unset_influence_mask(Coord::new(1, 1), mark);
    masks[11] = generate_unset_influence_mask(Coord::new(1, 2), mark);
    masks[12] = generate_unset_influence_mask(Coord::new(1, 3), mark);
    masks[13] = generate_unset_influence_mask(Coord::new(1, 4), mark);
    masks[14] = generate_unset_influence_mask(Coord::new(1, 5), mark);
    masks[15] = generate_unset_influence_mask(Coord::new(1, 6), mark);
    masks[16] = generate_unset_influence_mask(Coord::new(1, 7), mark);
    masks[17] = generate_unset_influence_mask(Coord::new(1, 8), mark);

    // Row 2
    masks[18] = generate_unset_influence_mask(Coord::new(2, 0), mark);
    masks[19] = generate_unset_influence_mask(Coord::new(2, 1), mark);
    masks[20] = generate_unset_influence_mask(Coord::new(2, 2), mark);
    masks[21] = generate_unset_influence_mask(Coord::new(2, 3), mark);
    masks[22] = generate_unset_influence_mask(Coord::new(2, 4), mark);
    masks[23] = generate_unset_influence_mask(Coord::new(2, 5), mark);
    masks[24] = generate_unset_influence_mask(Coord::new(2, 6), mark);
    masks[25] = generate_unset_influence_mask(Coord::new(2, 7), mark);
    masks[26] = generate_unset_influence_mask(Coord::new(2, 8), mark);

    // Row 3
    masks[27] = generate_unset_influence_mask(Coord::new(3, 0), mark);
    masks[28] = generate_unset_influence_mask(Coord::new(3, 1), mark);
    masks[29] = generate_unset_influence_mask(Coord::new(3, 2), mark);
    masks[30] = generate_unset_influence_mask(Coord::new(3, 3), mark);
    masks[31] = generate_unset_influence_mask(Coord::new(3, 4), mark);
    masks[32] = generate_unset_influence_mask(Coord::new(3, 5), mark);
    masks[33] = generate_unset_influence_mask(Coord::new(3, 6), mark);
    masks[34] = generate_unset_influence_mask(Coord::new(3, 7), mark);
    masks[35] = generate_unset_influence_mask(Coord::new(3, 8), mark);

    // Row 4
    masks[36] = generate_unset_influence_mask(Coord::new(4, 0), mark);
    masks[37] = generate_unset_influence_mask(Coord::new(4, 1), mark);
    masks[38] = generate_unset_influence_mask(Coord::new(4, 2), mark);
    masks[39] = generate_unset_influence_mask(Coord::new(4, 3), mark);
    masks[40] = generate_unset_influence_mask(Coord::new(4, 4), mark);
    masks[41] = generate_unset_influence_mask(Coord::new(4, 5), mark);
    masks[42] = generate_unset_influence_mask(Coord::new(4, 6), mark);
    masks[43] = generate_unset_influence_mask(Coord::new(4, 7), mark);
    masks[44] = generate_unset_influence_mask(Coord::new(4, 8), mark);

    // Row 5
    masks[45] = generate_unset_influence_mask(Coord::new(5, 0), mark);
    masks[46] = generate_unset_influence_mask(Coord::new(5, 1), mark);
    masks[47] = generate_unset_influence_mask(Coord::new(5, 2), mark);
    masks[48] = generate_unset_influence_mask(Coord::new(5, 3), mark);
    masks[49] = generate_unset_influence_mask(Coord::new(5, 4), mark);
    masks[50] = generate_unset_influence_mask(Coord::new(5, 5), mark);
    masks[51] = generate_unset_influence_mask(Coord::new(5, 6), mark);
    masks[52] = generate_unset_influence_mask(Coord::new(5, 7), mark);
    masks[53] = generate_unset_influence_mask(Coord::new(5, 8), mark);

    // Row 6
    masks[54] = generate_unset_influence_mask(Coord::new(6, 0), mark);
    masks[55] = generate_unset_influence_mask(Coord::new(6, 1), mark);
    masks[56] = generate_unset_influence_mask(Coord::new(6, 2), mark);
    masks[57] = generate_unset_influence_mask(Coord::new(6, 3), mark);
    masks[58] = generate_unset_influence_mask(Coord::new(6, 4), mark);
    masks[59] = generate_unset_influence_mask(Coord::new(6, 5), mark);
    masks[60] = generate_unset_influence_mask(Coord::new(6, 6), mark);
    masks[61] = generate_unset_influence_mask(Coord::new(6, 7), mark);
    masks[62] = generate_unset_influence_mask(Coord::new(6, 8), mark);

    // Row 7
    masks[63] = generate_unset_influence_mask(Coord::new(7, 0), mark);
    masks[64] = generate_unset_influence_mask(Coord::new(7, 1), mark);
    masks[65] = generate_unset_influence_mask(Coord::new(7, 2), mark);
    masks[66] = generate_unset_influence_mask(Coord::new(7, 3), mark);
    masks[67] = generate_unset_influence_mask(Coord::new(7, 4), mark);
    masks[68] = generate_unset_influence_mask(Coord::new(7, 5), mark);
    masks[69] = generate_unset_influence_mask(Coord::new(7, 6), mark);
    masks[70] = generate_unset_influence_mask(Coord::new(7, 7), mark);
    masks[71] = generate_unset_influence_mask(Coord::new(7, 8), mark);

    // Row 8
    masks[72] = generate_unset_influence_mask(Coord::new(8, 0), mark);
    masks[73] = generate_unset_influence_mask(Coord::new(8, 1), mark);
    masks[74] = generate_unset_influence_mask(Coord::new(8, 2), mark);
    masks[75] = generate_unset_influence_mask(Coord::new(8, 3), mark);
    masks[76] = generate_unset_influence_mask(Coord::new(8, 4), mark);
    masks[77] = generate_unset_influence_mask(Coord::new(8, 5), mark);
    masks[78] = generate_unset_influence_mask(Coord::new(8, 6), mark);
    masks[79] = generate_unset_influence_mask(Coord::new(8, 7), mark);
    masks[80] = generate_unset_influence_mask(Coord::new(8, 8), mark);

    masks
}

const fn generate_influence_mask(coord: Coord) -> Grid {
    let cell_mask = Cell::mask();
    let mut cells = [Cell::new_empty(); GRID_SIZE];

    let (row, col) = coord.get_row_col();
    let square = Square::from(row, col);

    //Row
    cells[Coord::new(row, 0).get_index()] = cell_mask;
    cells[Coord::new(row, 1).get_index()] = cell_mask;
    cells[Coord::new(row, 2).get_index()] = cell_mask;
    cells[Coord::new(row, 3).get_index()] = cell_mask;
    cells[Coord::new(row, 4).get_index()] = cell_mask;
    cells[Coord::new(row, 5).get_index()] = cell_mask;
    cells[Coord::new(row, 6).get_index()] = cell_mask;
    cells[Coord::new(row, 7).get_index()] = cell_mask;
    cells[Coord::new(row, 8).get_index()] = cell_mask;

    cells[Coord::new(0, col).get_index()] = cell_mask;
    cells[Coord::new(1, col).get_index()] = cell_mask;
    cells[Coord::new(2, col).get_index()] = cell_mask;
    cells[Coord::new(3, col).get_index()] = cell_mask;
    cells[Coord::new(4, col).get_index()] = cell_mask;
    cells[Coord::new(5, col).get_index()] = cell_mask;
    cells[Coord::new(6, col).get_index()] = cell_mask;
    cells[Coord::new(7, col).get_index()] = cell_mask;
    cells[Coord::new(8, col).get_index()] = cell_mask;

    cells[square.get_coord_at(0, 0).get_index()] = cell_mask;
    cells[square.get_coord_at(0, 1).get_index()] = cell_mask;
    cells[square.get_coord_at(0, 2).get_index()] = cell_mask;
    cells[square.get_coord_at(1, 0).get_index()] = cell_mask;
    cells[square.get_coord_at(1, 1).get_index()] = cell_mask;
    cells[square.get_coord_at(1, 2).get_index()] = cell_mask;
    cells[square.get_coord_at(2, 0).get_index()] = cell_mask;
    cells[square.get_coord_at(2, 1).get_index()] = cell_mask;
    cells[square.get_coord_at(2, 2).get_index()] = cell_mask;

    Grid::from(cells)
}

const fn generate_unset_influence_mask(coord: Coord, mark: Mark) -> Grid {
    let mut cells = [Cell::mask(); GRID_SIZE];

    let (row, col) = coord.get_row_col();
    let square = Square::from(row, col);

    let cm = Cell::new_with_value(mark.to_data());
    let cell_mask = Cell::new_with_value(Cell::mask().get_value() ^ cm.get_value());

    //Row
    cells[Coord::new(row, 0).get_index()] = cell_mask;
    cells[Coord::new(row, 1).get_index()] = cell_mask;
    cells[Coord::new(row, 2).get_index()] = cell_mask;
    cells[Coord::new(row, 3).get_index()] = cell_mask;
    cells[Coord::new(row, 4).get_index()] = cell_mask;
    cells[Coord::new(row, 5).get_index()] = cell_mask;
    cells[Coord::new(row, 6).get_index()] = cell_mask;
    cells[Coord::new(row, 7).get_index()] = cell_mask;
    cells[Coord::new(row, 8).get_index()] = cell_mask;

    cells[Coord::new(0, col).get_index()] = cell_mask;
    cells[Coord::new(1, col).get_index()] = cell_mask;
    cells[Coord::new(2, col).get_index()] = cell_mask;
    cells[Coord::new(3, col).get_index()] = cell_mask;
    cells[Coord::new(4, col).get_index()] = cell_mask;
    cells[Coord::new(5, col).get_index()] = cell_mask;
    cells[Coord::new(6, col).get_index()] = cell_mask;
    cells[Coord::new(7, col).get_index()] = cell_mask;
    cells[Coord::new(8, col).get_index()] = cell_mask;

    cells[square.get_coord_at(0, 0).get_index()] = cell_mask;
    cells[square.get_coord_at(0, 1).get_index()] = cell_mask;
    cells[square.get_coord_at(0, 2).get_index()] = cell_mask;
    cells[square.get_coord_at(1, 0).get_index()] = cell_mask;
    cells[square.get_coord_at(1, 1).get_index()] = cell_mask;
    cells[square.get_coord_at(1, 2).get_index()] = cell_mask;
    cells[square.get_coord_at(2, 0).get_index()] = cell_mask;
    cells[square.get_coord_at(2, 1).get_index()] = cell_mask;
    cells[square.get_coord_at(2, 2).get_index()] = cell_mask;

    Grid::from(cells)
}

const fn generate_set_possible_masks() -> [Grid; 9] {
    let mut masks = [Grid::empty(); 9];

    masks[0] = generate_set_possible_mask(Mark::N1);
    masks[1] = generate_set_possible_mask(Mark::N2);
    masks[2] = generate_set_possible_mask(Mark::N3);
    masks[3] = generate_set_possible_mask(Mark::N4);
    masks[4] = generate_set_possible_mask(Mark::N5);
    masks[5] = generate_set_possible_mask(Mark::N6);
    masks[6] = generate_set_possible_mask(Mark::N7);
    masks[7] = generate_set_possible_mask(Mark::N8);
    masks[8] = generate_set_possible_mask(Mark::N9);

    masks
}

const fn generate_set_possible_mask(mark: Mark) -> Grid {
    let d = mark.to_data();
    let mask = Cell::new_with_value(d);
    Grid::from([mask; GRID_SIZE])
}

const fn generate_unset_possible_masks() -> [Grid; 9] {
    let mut masks = [Grid::empty(); 9];

    masks[0] = generate_unset_possible_mask(Mark::N1);
    masks[1] = generate_unset_possible_mask(Mark::N2);
    masks[2] = generate_unset_possible_mask(Mark::N3);
    masks[3] = generate_unset_possible_mask(Mark::N4);
    masks[4] = generate_unset_possible_mask(Mark::N5);
    masks[5] = generate_unset_possible_mask(Mark::N6);
    masks[6] = generate_unset_possible_mask(Mark::N7);
    masks[7] = generate_unset_possible_mask(Mark::N8);
    masks[8] = generate_unset_possible_mask(Mark::N9);

    masks
}

const fn generate_unset_possible_mask(mark: Mark) -> Grid {
    let d = mark.to_data();
    let mask = Cell::new_with_value(!d);
    Grid::from([mask; GRID_SIZE])
}
