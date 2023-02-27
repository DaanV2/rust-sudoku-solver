use crate::grid::square::Square;

pub fn get_square_row_neighbors(square: &Square) -> Option<[Square; 2]> {
    let row = square.row;
    let col: usize = square.col;
    let result: [Square; 2];

    match col {
        0 => {
            result = [
                Square::new(row, 3, square.grid),
                Square::new(row, 6, square.grid),
            ];
        }
        3 => {
            result = [
                Square::new(row, 0, square.grid),
                Square::new(row, 6, square.grid),
            ];
        }
        6 => {
            result = [
                Square::new(row, 0, square.grid),
                Square::new(row, 3, square.grid),
            ];
        }
        _ => return None,
    }

    Some(result)
}

pub fn get_square_col_neighbors(square: &Square) -> Option<[Square; 2]> {
    let row: usize = square.row;
    let col = square.col;
    let result: [Square; 2];

    match row {
        0 => {
            result = [
                Square::new(3, col, square.grid),
                Square::new(6, col, square.grid),
            ];
        }
        3 => {
            result = [
                Square::new(0, col, square.grid),
                Square::new(6, col, square.grid),
            ];
        }
        6 => {
            result = [
                Square::new(0, col, square.grid),
                Square::new(3, col, square.grid),
            ];
        }
        _ => return None,
    }

    Some(result)
}
