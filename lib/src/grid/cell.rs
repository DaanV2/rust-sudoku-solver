use std::{
    fmt::Display,
    ops::{BitAnd, BitOr, BitXor},
};

use super::mark::Mark;

// If the cell is determined, the value is stored here
// pppp pppp p000 vvvv
// If the cell is not determined, the possibilities are stored the top
// 1 is possible = 0000 0000 1000 0000
// 2 is possible = 0000 0001 0000 0000
// 3 is possible = 0000 0010 0000 0000
// 4 is possible = 0000 0100 0000 0000
// 5 is possible = 0000 1000 0000 0000
// 6 is possible = 0001 0000 0000 0000
// 7 is possible = 0010 0000 0000 0000
// 8 is possible = 0100 0000 0000 0000
// 9 is possible = 1000 0000 0000 0000

type InnerCell = u16;

/// A cell in the grid
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cell {
    /// 4 for bits for the value, the rest for the possibilities
    data: InnerCell,
}

/// The default value for a cell, no value and all possibilities on
const DEFAULT_CELL_VALUE: InnerCell = 0b1111_1111_1000_0000;

/// The mask used to read / write the value
const CELL_VALUE_MASK: InnerCell = 0b0000_0000_0000_1111;
const CELL_POSSIBLE_MASK: InnerCell = DEFAULT_CELL_VALUE;

impl Cell {
    #[inline(always)]
    pub const fn possible_to_bit(value: InnerCell) -> InnerCell {
        // 1 is possible = 0000 0000 1000 0000

        1 << (value + 6)
    }

    /// Creates a new empty cell, with all possibilities on
    pub const fn new() -> Cell {
        Cell {
            data: DEFAULT_CELL_VALUE,
        }
    }

    /// Creates a new cell with a value, and all possibilities off
    pub const fn new_empty() -> Cell {
        Cell::new_with_value(0)
    }

    /// Creates a new cell with a value, and all possibilities off
    /// Assumes the value is between 1 and 9
    pub const fn new_with_value(data: InnerCell) -> Cell {
        Cell { data }
    }

    /// Creates a new cell with a mark turned on as a possibility, and all  other possibilities off
    pub const fn new_with_possible(mark: Mark) -> Cell {
        let v = mark.to_data();

        Self::new_with_value(v)
    }

    /// Creates a new cell with a mark as a value, and all possibilities off
    pub const fn new_from_mark_as_value(mark: Mark) -> Cell {
        let v = mark.to_value();

        Self::new_with_value(v)
    }

    /// Creates a new cell with a mark as a value, and all possibilities off
    pub const fn mask() -> Cell {
        Cell {
            data: 0b1111_1111_1111_1111,
        }
    }

    /// Returns true if the cell is empty or not
    pub fn is_empty(self) -> bool {
        self.data == 0
    }

    /// Returns true if the cell has any possibilities or not
    pub fn has_any(self) -> bool {
        self.data > 0
    }

    /// Returns true if the cell is determined or not
    pub fn is_determined(self) -> bool {
        (self.data & CELL_VALUE_MASK) > 0
    }

    /// Returns true if the given value is possible for this cell
    pub fn is_possible(self, value: Mark) -> bool {
        let v = value.to_data();
        let d = self.data;

        d & v != 0
    }

    /// Returns true if the given value is possible for this cell
    /// Assumes the value is between 1 and 9
    pub fn is_possible_value(self, value: InnerCell) -> bool {
        let b = Cell::possible_to_bit(value);
        let d = self.data;

        d & b != 0
    }

    /// Stores the given value in the cell, sets all possibilities off
    /// Assumes the value is between 1 and 9
    pub fn set_value(&mut self, value: u16) {
        self.data = value as InnerCell;
    }

    /// Returns the value of this cell
    /// Assumes the cell is determined
    pub const fn get_value(self) -> u16 {
        self.data
    }

    /// Returns the value of this cell, if determined
    pub fn value(self) -> Option<u16> {
        if self.is_determined() {
            Some(self.get_value())
        } else {
            None
        }
    }

    /// Sets the given value as possible for this cell
    /// Assumes the value is between 1 and 9 and the cell is not determined
    pub fn set_possible(&mut self, value: Mark) {
        let v = value.to_data();
        let d = self.data;

        self.data = (d | v) as InnerCell;
    }

    /// Un sets the given value as possible for this cell
    /// Assumes the value is between 1 and 9 and the cell is not determined
    pub fn unset_possible(&mut self, value: Mark) {
        let v = value.to_data();
        let d = self.data;

        self.data = (d & !v) as InnerCell;
    }

    /// Returns the amount of possibilities for this cell, assumes the cell is not determined
    pub fn possible_count(self) -> u32 {
        self.data.count_ones()
    }

    /// Filters all possibilities from this cell, removing the value
    pub fn only_possible(&self) -> Self {
        self.clone() & Cell::new_with_value(CELL_POSSIBLE_MASK)
    }

    /// Filters out all possibilities from this cell, keeping the value
    pub fn only_determined(&self) -> Self {
        self.clone() & Cell::new_with_value(CELL_VALUE_MASK)
    }

    /// Iterates over all possible values for this cell
    pub fn iter_possible(self) -> impl Iterator<Item = Mark> {
        let mut value = self.data;
        if self.is_determined() {
            value = 0;
        }

        std::iter::from_fn(move || {
            if value == 0 {
                return None;
            }

            let index = value.trailing_zeros() as u16 - 7;
            let mark = Mark::from_index(index);
            let mask = mark as InnerCell;
            value &= !(mask);
            Some(mark)
        })
    }
}

impl Default for Cell {
    fn default() -> Self {
        Self::new()
    }
}

impl BitOr for Cell {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Cell {
            data: self.data | rhs.data,
        }
    }
}

impl BitAnd for Cell {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Cell {
            data: self.data & rhs.data,
        }
    }
}

impl BitXor for Cell {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Cell {
            data: self.data ^ rhs.data,
        }
    }
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;

        for mark in Mark::iter() {
            if !self.is_possible(mark) {
                write!(f, "_")?;
                continue;
            }
            write!(f, "{}", mark)?;
        }
        write!(f, ",")?;
        write!(f, "{}", self.only_determined().get_value())?;
        write!(f, "]")
    }
}

#[cfg(test)]
mod test {
    use crate::grid::{cell::CELL_VALUE_MASK, mark::Mark};

    use super::Cell;

    #[test]
    pub fn test_print_objsize() {
        println!("Cell: {}", std::mem::size_of::<Cell>());
    }

    #[test]
    pub fn test_possible_to_bit() {
        assert_eq!(Cell::possible_to_bit(1), 0b0000_0000_1000_0000);
        assert_eq!(Cell::possible_to_bit(2), 0b0000_0001_0000_0000);
        assert_eq!(Cell::possible_to_bit(3), 0b0000_0010_0000_0000);
        assert_eq!(Cell::possible_to_bit(4), 0b0000_0100_0000_0000);
        assert_eq!(Cell::possible_to_bit(5), 0b0000_1000_0000_0000);
        assert_eq!(Cell::possible_to_bit(6), 0b0001_0000_0000_0000);
        assert_eq!(Cell::possible_to_bit(7), 0b0010_0000_0000_0000);
        assert_eq!(Cell::possible_to_bit(8), 0b0100_0000_0000_0000);
        assert_eq!(Cell::possible_to_bit(9), 0b1000_0000_0000_0000);
    }

    #[test]
    pub fn test_get_count() {
        let mut cell = Cell::new();

        assert_eq!(cell.possible_count(), 9);

        cell.unset_possible(Mark::N1);
        assert_eq!(cell.possible_count(), 8);

        cell.unset_possible(Mark::N2);
        assert_eq!(cell.possible_count(), 7);

        cell.unset_possible(Mark::N3);
        assert_eq!(cell.possible_count(), 6);

        cell.unset_possible(Mark::N4);
        assert_eq!(cell.possible_count(), 5);

        cell.unset_possible(Mark::N5);
        assert_eq!(cell.possible_count(), 4);

        cell.unset_possible(Mark::N6);
        assert_eq!(cell.possible_count(), 3);

        cell.unset_possible(Mark::N7);
        assert_eq!(cell.possible_count(), 2);

        cell.unset_possible(Mark::N8);
        assert_eq!(cell.possible_count(), 1);

        cell.unset_possible(Mark::N9);
        assert_eq!(cell.possible_count(), 0);
    }

    #[test]
    pub fn test_iter_possible() {
        let cell = Cell::new();

        let mut iter = cell.iter_possible();

        assert_eq!(iter.next(), Some(Mark::N1));
        assert_eq!(iter.next(), Some(Mark::N2));
        assert_eq!(iter.next(), Some(Mark::N3));
        assert_eq!(iter.next(), Some(Mark::N4));
        assert_eq!(iter.next(), Some(Mark::N5));
        assert_eq!(iter.next(), Some(Mark::N6));
        assert_eq!(iter.next(), Some(Mark::N7));
        assert_eq!(iter.next(), Some(Mark::N8));
        assert_eq!(iter.next(), Some(Mark::N9));
        assert_eq!(iter.next(), None);
    }

    #[test]
    pub fn test_determined() {
        for i in 1..=9 {
            let cell = Cell::new_with_value(i);

            assert!(cell.is_determined());
            assert_eq!(cell.get_value(), i as u16);
        }
    }

    #[test]
    pub fn test_not_determined() {
        let cell = Cell::new();

        assert!(!cell.is_determined());

        for i in 1..=9 {
            let mark = Mark::from_index(i - 1);
            assert!(cell.is_possible(mark));
            assert!(!cell.is_determined());
        }
    }

    #[test]
    pub fn test_bit_or() {
        let c1 = Cell::new();
        let c2 = Cell::new_with_value(3);

        let c3 = c1 | c2;

        assert_eq!(c3.get_value() & (CELL_VALUE_MASK as u16), 3);
        assert!(c3.is_possible(Mark::N1), "1");
        assert!(c3.is_possible(Mark::N2), "2");
        assert!(c3.is_possible(Mark::N3), "3");
        assert!(c3.is_possible(Mark::N4), "4");
        assert!(c3.is_possible(Mark::N5), "5");
        assert!(c3.is_possible(Mark::N6), "6");
        assert!(c3.is_possible(Mark::N7), "7");
        assert!(c3.is_possible(Mark::N8), "8");
        assert!(c3.is_possible(Mark::N9), "9");
    }

    #[test]
    pub fn test_bit_or2() {
        let mut c1 = Cell::new_empty();
        c1.set_possible(Mark::N1);
        let mut c2 = Cell::new_empty();
        c2.set_possible(Mark::N9);

        let c3 = c1 | c2;

        assert!(c3.is_possible(Mark::N1), "1");
        assert!(c3.is_possible(Mark::N9), "9");
    }
}
