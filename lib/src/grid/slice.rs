use std::{
    fmt::Display,
    ops::{BitAnd, BitOr},
};

use super::{
    cell::{Cell, CELL_BITS_SIZE},
    cell_collection::CellCollection,
    grid::Grid,
    mark::Mark,
};

const SLICE_SIZE: usize = 16;
const SLICE_BIT_SIZE: usize = CELL_BITS_SIZE * SLICE_SIZE;
#[allow(dead_code)]
const SLICE_ACTUAL_SIZE: usize = 9;

const ITEMS_U64: usize = SLICE_BIT_SIZE / u64::BITS as usize;

#[derive(Debug, Clone)]
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

    /// Creates a slice from the given cell collection
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

    /// Fills a slice with the given cell
    pub const fn create_mask_full(cell: Cell) -> Slice {
        Slice {
            items: [cell; SLICE_SIZE],
        }
    }

    /// Fills a slice with the given cell, in sets of three
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
            count += self.items[i].has_any() as usize
        }

        count
    }

    /// Returns the number of cells that are empty
    pub fn count_empty(&self) -> usize {
        let mut count = 0;

        for i in self.iter() {
            count += self.items[i].is_empty() as usize;
        }

        count as usize
    }

    /// Returns if the given mark is possible in this slice
    #[inline(always)]
    pub fn any_possible(&self, mark: Mark) -> bool {
        self.only_possible_value(mark) != SLICE_EMPTY
    }

    /// Counts the number of cells that are possible for the given mark
    pub fn count_possible(&self, mark: Mark) -> usize {
        let result = self.only_possible_value(mark);
        let mut count = 0;

        for cell in result.items.iter() {
            let b = cell.get_value();
            count += (b > 0) as usize;
        }

        count
    }

    /// Counts the number of cells that are determined
    pub fn count_determined(&self) -> usize {
        let mut count = 0;
        let temp = self.only_determined();

        for cell in temp.items.iter() {
            let b = cell.get_value();
            count += (b > 0) as usize;
        }

        count
    }

    /// Counts the number of cells that are determined to the given value
    pub fn count_determined_value(&self, value: u16) -> usize {
        let mut count = 0;
        let temp = self.clone();

        for c in temp.items.iter() {
            count += (c.get_value() == value) as usize;
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

    /// Returns true or false if the given value is determined in this slice
    pub fn is_determined(&self, value: u16) -> bool {
        let mut out = false;

        for c in self.items.iter() {
            out |= c.get_value() == value
        }

        out
    }

    /// Returns an iterator over the entire slice
    pub fn iter(&self) -> std::ops::Range<usize> {
        0..self.items.len()
    }

    /// Returns a slice, which has all determined cells removed
    pub fn only_possible(&self) -> Slice {
        let mut slice = self.clone();

        for i in slice.iter() {
            slice.items[i] = slice.items[i].only_possible();
        }

        slice
    }

    /// Returns a slice, which has all determined cells removed, with only the given mark
    pub fn only_possible_value(&self, mark: Mark) -> Slice {
        let mask = Cell::new_with_value(mark.to_data());
        let mut slice = self.clone();

        for i in slice.iter() {
            slice.items[i] = slice.items[i] & mask;
        }

        slice
    }

    /// Returns a slice, which has all possible cells removed
    pub fn only_determined(&self) -> Slice {
        let mut slice = self.clone();

        for i in slice.iter() {
            slice.items[i] = slice.items[i].only_determined();
        }

        slice
    }

    /// Returns a slice, which has all possible cells removed, with only the given value
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

    /// Returns a slice of u64's, which represent the slice
    pub fn to_u64_slice(&self) -> [u64; ITEMS_U64] {
        let mut out = [0; ITEMS_U64];

        unsafe {
            let ptr = out.as_mut_ptr() as *mut u8;
            let slice_ptr = self.items.as_ptr() as *const u8;
            std::ptr::copy_nonoverlapping(slice_ptr, ptr, SLICE_BIT_SIZE / 8);
        }

        out
    }

    /// Returns a u64, which is the or fold of all u64's in the slice
    pub fn to_u64_or(&self) -> u64 {
        let data = self.to_u64_slice();
        let result = data.iter().fold(0, |acc, &x| acc | x);

        result
    }

    /// Returns a u64, which is the and fold of all u64's in the slice
    pub fn to_u64_and(&self) -> u64 {
        let data = self.to_u64_slice();
        let result = data.iter().fold(u64::MAX, |acc, &x| acc & x);

        result
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

#[cfg(test)]
mod tests {
    use crate::{
        grid::{cell::Cell, column::Column, coords::Coord, mark::Mark, row::Row},
        test::util::general_tests,
    };

    use super::{Slice, SLICE_ACTUAL_SIZE};

    fn slice_example() -> Slice {
        let mut s = Slice::new();

        for i in 0..SLICE_ACTUAL_SIZE {
            let v = (i as u16) + 1;
            s.items[i] = Cell::new_with_value((v % 9) + 1);
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
