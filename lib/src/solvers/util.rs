use crate::grid::square::Square;

pub fn get_square_row_neighbors(square: &Square) -> Option<[Square; 2]> {
    let row = square.row;
    let col: usize = square.col;

    return match col {
        0 => Some([Square::new(row, 3), Square::new(row, 6)]),
        3 => Some([Square::new(row, 0), Square::new(row, 6)]),
        6 => Some([Square::new(row, 0), Square::new(row, 3)]),
        _ => None,
    };
}

pub fn get_square_col_neighbors(square: &Square) -> Option<[Square; 2]> {
    let row: usize = square.row;
    let col = square.col;

    return match row {
        0 => Some([Square::new(3, col), Square::new(6, col)]),
        3 => Some([Square::new(0, col), Square::new(6, col)]),
        6 => Some([Square::new(0, col), Square::new(3, col)]),
        _ => return None,
    };
}
