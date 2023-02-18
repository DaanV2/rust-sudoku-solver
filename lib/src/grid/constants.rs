use std::ops::Range;

pub const GRID_HEIGHT: usize = 9;
pub const GRID_WIDTH: usize = 9;
pub const GRID_SIZE: usize = GRID_HEIGHT * GRID_WIDTH;

pub const GRID_HEIGHT_RANGE: Range<usize> = 0..GRID_HEIGHT;
pub const GRID_WIDTH_RANGE: Range<usize> = 0..GRID_WIDTH;
