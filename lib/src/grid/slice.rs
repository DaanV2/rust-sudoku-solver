use std::{
    fmt::Display,
    ops::{BitAnd, BitOr},
};

use super::{cell::Cell, cell_collection::CellCollection, grid::Grid, mark::Mark};

const SLICE_SIZE: usize = 16;
#[allow(dead_code)]
const SLICE_ACTUAL_SIZE: usize = 9;

#[derive(Clone)]
pub struct Slice {
    pub items: [Cell; SLICE_SIZE],
}

pub const SLICE_EMPTY: Slice = Slice::new();

impl Slice {
    pub const fn new() -> Self {
        Slice {
            items: [Cell::new_empty(); SLICE_SIZE],
        }
    }

    pub fn from<T: CellCollection>(grid: &Grid, area: &T) -> Slice {
        if area.max() >= SLICE_SIZE {
            panic!("Slice must be of size {}", SLICE_SIZE);
        }
        let mut slice = Slice::new();

        for i in area.iter() {
            let coord = area.get_coord(i);
            slice.items[i] = grid.get_cell_at(coord).clone()
        }

        slice
    }

    pub const fn create_mask_full(cell: Cell) -> Slice {
        Slice {
            items: [cell; SLICE_SIZE],
        }
    }

    pub const fn create_mask_threes(c1: Cell, c2: Cell, c3: Cell) -> Slice {
        let mut s = Slice::new();
        s.items[0] = c1;
        s.items[1] = c1;
        s.items[2] = c1;
        s.items[3] = c2;
        s.items[4] = c2;
        s.items[5] = c2;
        s.items[6] = c3;
        s.items[7] = c3;
        s.items[8] = c3;

        s
    }

    /// Returns the cell at the given index assumes the index is valid
    pub fn get(&self, index: usize) -> Cell {
        unsafe {
            return self.items.get_unchecked(index).clone();
        }
    }

    /// Returns the number of cells that are not empty
    pub fn count(&self) -> usize {
        let mut count = 0;

        for i in self.iter() {
            if self.items[i].has_any() {
                count += 1;
            }
        }

        count as usize
    }

    /// Returns the number of cells that are empty
    pub fn count_empty(&self) -> usize {
        let mut count = 0;

        for i in self.iter() {
            if self.items[i].is_empty() {
                count += 1;
            }
        }

        count as usize
    }

    /// Returns if the given value is determined in this slice
    pub fn any_determined_value(&self, value: u16) -> bool {
        for c in self.items.iter() {
            if c.get_value() == value {
                return true;
            }
        }

        false
    }

    /// Returns if the given mark is possible in this slice
    pub fn any_possible(&self, mark: Mark) -> bool {
        let mut result = Cell::new_empty();

        for c in self.items.iter() {
            result = result | *c;
        }

        result.is_possible(mark)
    }

    pub fn count_possible(&self, mark: Mark) -> usize {
        let mut count = 0;

        for c in self.items.iter() {
            if c.is_possible(mark) {
                count += 1;
            }
        }

        count
    }

    pub fn count_determined(&self) -> usize {
        let mut count = 0;

        for c in self.items.iter() {
            if c.is_determined() {
                count += 1;
            }
        }

        count
    }

    pub fn count_determined_value(&self, value: u16) -> usize {
        let mut count = 0;

        for c in self.items.iter() {
            if c.get_value() == value {
                count += 1;
            }
        }

        count
    }

    /// Returns the first index of a cell that is possible for the given mark, assumes there is at least one
    pub fn first_possible(&self, mark: Mark) -> usize {
        for i in self.iter() {
            if self.items[i].is_possible(mark) {
                return i;
            }
        }

        0
    }

    pub fn is_determined(&self, value: u16) -> bool {
        for c in self.items.iter() {
            if c.get_value() == value {
                return true;
            }
        }

        false
    }

    pub fn is_fully_determined(&self) -> bool {
        let mut determined = true;

        for c in self.items.iter() {
            determined &= c.is_determined();
        }

        determined
    }

    pub fn iter(&self) -> std::ops::Range<usize> {
        0..self.items.len()
    }

    pub fn only_possible(&self) -> Slice {
        let mut slice = self.clone();

        for i in slice.iter() {
            slice.items[i] = slice.items[i].only_possible();
        }

        slice
    }

    pub fn only_possible_value(&self, mark: Mark) -> Slice {
        let mut slice = self.clone();
        let mask = Cell::new_with_possible(mark);

        for i in self.iter() {
            slice.items[i] = slice.items[i] & mask;
        }

        slice
    }

    pub fn only_determined(&self) -> SliceValue {
        let mut slice = SliceValue::new();

        for i in slice.iter() {
            slice.items[i] = self.items[i].only_determined().get_value() as u8;
        }

        slice
    }

    pub fn only_determined_value(&self, value: u16) -> Slice {
        let mut slice = self.clone();

        for i in slice.iter() {
            if slice.items[i].get_value() != value {
                slice.items[i] = Cell::new_empty();
            }
        }

        slice
    }

    /// Returns a cell that is or'ed with all the cells in the slice
    pub fn or_all(&self) -> Cell {
        let mut data = Cell::new_empty();

        for c in self.items.iter() {
            data = data | c.clone();
        }

        data
    }

    /// Returns the first index and count of a cell that is possible for the given mark, assumes there is at least one
    pub fn search_count_possible(&self, mark: Mark) -> (usize, usize) {
        let mut count = 0;
        let mut index = 0;

        for i in self.iter() {
            if self.items[i].is_possible(mark) {
                count += 1;
                index = i;
            }
        }

        (index, count)
    }
}

impl Display for Slice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();

        for i in self.iter() {
            s.push_str(&self.items[i].to_string());
            s.push_str(" ");
        }

        write!(f, "{}", s)
    }
}

impl BitOr for Slice {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        let mut slice = Slice::new();

        for i in slice.iter() {
            slice.items[i] = self.items[i] | rhs.items[i];
        }

        slice
    }
}

impl BitAnd for Slice {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        let mut slice = Slice::new();

        for i in slice.iter() {
            slice.items[i] = self.items[i] & rhs.items[i];
        }

        slice
    }
}

impl PartialEq for Slice {
    fn eq(&self, other: &Self) -> bool {
        self.items == other.items
    }
}

impl Copy for Slice {}

pub struct SliceValue {
    pub items: [u8; 9],
}

impl SliceValue {
    pub fn new() -> Self {
        SliceValue { items: [0; 9] }
    }

    pub fn iter(&self) -> std::ops::Range<usize> {
        0..self.items.len()
    }

    pub fn any(&self) -> bool {
        for i in self.iter() {
            if self.items[i] != 0 {
                return true;
            }
        }

        false
    }

    pub fn all(&self) -> bool {
        for i in self.iter() {
            if self.items[i] == 0 {
                return false;
            }
        }

        true
    }

    pub fn count(&self) -> usize {
        let mut count = 0;

        for i in self.iter() {
            if self.items[i] != 0 {
                count += 1;
            }
        }

        count
    }

    pub fn count_value(&self, value: u8) -> usize {
        let mut count = 0;

        for i in self.iter() {
            if self.items[i] == value {
                count += 1;
            }
        }

        count
    }
}

impl BitOr for SliceValue {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        let mut slice = SliceValue::new();

        for i in slice.iter() {
            slice.items[i] = self.items[i] | rhs.items[i];
        }

        slice
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        grid::{cell::Cell, column::Column, coords::Coord, mark::Mark, row::Row},
        test::util::general_tests,
    };

    use super::Slice;

    fn slice_example() -> Slice {
        let mut s = Slice::new();

        for i in s.iter() {
            s.items[i] = Cell::new_with_value((i as u16) + 1);
        }

        s
    }

    #[test]
    pub fn test_from() {
        let grid = general_tests::filled_sudoku();

        let s = Slice::from(&grid, &Row::new(0));
        for i in 0..9 {
            assert_eq!(s.items[i], *grid.get_cell_at(Coord::new(0, i)));
        }

        let s = Slice::from(&grid, &Column::new(0));
        for i in 0..9 {
            assert_eq!(s.items[i], *grid.get_cell_at(Coord::new(i, 0)));
        }
    }

    #[test]
    pub fn test_only_possible() {
        let mut s = slice_example();

        s.items[5] = Cell::new();
        s.items[6] = Cell::new();

        let s = s.only_possible();
        let count = s.count();

        assert_eq!(count, 2);
    }

    #[test]
    pub fn test_only_possible_value() {
        let mut s = slice_example();

        s.items[5] = Cell::new();
        s.items[6] = Cell::new();

        let s = s.only_possible_value(Mark::N7);
        let count = s.count();

        assert_eq!(count, 2);
    }

    #[test]
    pub fn test_only_determined() {
        let mut s = slice_example();

        s.items[5] = Cell::new();
        s.items[6] = Cell::new();

        let s = s.only_determined();
        let count = s.count();

        assert_eq!(count, 7);
    }

    #[test]
    pub fn test_only_determined_value() {
        let mut s = slice_example();

        s.items[5] = Cell::new();
        s.items[6] = Cell::new();

        let s = s.only_determined_value(1);
        let count = s.count();

        assert_eq!(count, 1);
    }
}
