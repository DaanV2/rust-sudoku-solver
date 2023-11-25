use super::mark::Mark;

// If the cell is determined, the value is stored here
// 0000 0000 0000 vvvv
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

/// A cell in the grid
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cell {
    /// 4 for bits for the value, the rest for the possibilities
    data: u16,
}

/// The default value for a cell, no value and all possibilities on
const DEFAULT_CELL_VALUE: u16 = 0b1111_1111_1000_0000;

/// The mask used to read / write the value
const CELL_MASK: usize = 0b0000_0000_0000_1111;

impl Cell {
    #[inline(always)]
    fn possible_to_bit(value: usize) -> usize {
        // 1 is possible = 0000 0000 1000 0000

        1 << (value + 6)
    }

    /// Creates a new empty cell, with all possibilities on
    pub fn new() -> Cell {
        Cell {
            data: DEFAULT_CELL_VALUE,
        }
    }

    /// Creates a new cell with a value, and all possibilities off
    pub fn new_empty() -> Cell {
        Cell { data: 0 }
    }

    /// Creates a new cell with a value, and all possibilities off
    /// Assumes the value is between 1 and 9
    pub fn new_with_value(value: usize) -> Cell {
        Cell { data: value as u16 }
    }

    /// Creates a new cell with a mark as a value, and all possibilities off
    pub fn new_from_mark_as_value(mark: Mark) -> Cell {
        let v = mark.to_value();

        Self::new_with_value(v)
    }

    /// Returns true if the cell is determined or not
    pub fn is_determined(self) -> bool {
        (self.data as usize & CELL_MASK) != 0
    }

    /// Returns true if the given value is possible for this cell
    pub fn is_possible(self, value: Mark) -> bool {
        let v = value.to_data();
        let d = self.data as usize;

        d & v == v
    }

    /// Returns true if the given value is possible for this cell
    /// Assumes the value is between 1 and 9
    pub fn is_possible_value(self, value: usize) -> bool {
        let b = Cell::possible_to_bit(value);
        let d = self.data as usize;

        d & b == b
    }

    /// Stores the given value in the cell, sets all possibilities off
    /// Assumes the value is between 1 and 9
    pub fn set_value(&mut self, value: usize) {
        self.data = value as u16;
    }

    /// Returns the value of this cell
    /// Assumes the cell is determined
    pub fn get_value(self) -> usize {
        self.data as usize
    }

    pub fn value(self) -> Option<usize> {
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
        let d = self.data as usize;

        self.data = (d | v) as u16;
    }

    /// Un sets the given value as possible for this cell
    /// Assumes the value is between 1 and 9 and the cell is not determined
    pub fn unset_possible(&mut self, value: Mark) {
        let v = value.to_data();
        let d = self.data as usize;

        self.data = (d & !v) as u16;
    }

    /// Returns the amount of possibilities for this cell
    pub fn get_count(self) -> u32 {
        self.data.count_ones()
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

            let index = value.trailing_zeros() as usize - 7;
            let mark = Mark::from_index(index);
            let mask = mark as u16;
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

#[cfg(test)]
mod test {
    use crate::grid::mark::Mark;

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

        assert_eq!(cell.get_count(), 9);

        cell.unset_possible(Mark::N1);
        assert_eq!(cell.get_count(), 8);

        cell.unset_possible(Mark::N2);
        assert_eq!(cell.get_count(), 7);

        cell.unset_possible(Mark::N3);
        assert_eq!(cell.get_count(), 6);

        cell.unset_possible(Mark::N4);
        assert_eq!(cell.get_count(), 5);

        cell.unset_possible(Mark::N5);
        assert_eq!(cell.get_count(), 4);

        cell.unset_possible(Mark::N6);
        assert_eq!(cell.get_count(), 3);

        cell.unset_possible(Mark::N7);
        assert_eq!(cell.get_count(), 2);

        cell.unset_possible(Mark::N8);
        assert_eq!(cell.get_count(), 1);

        cell.unset_possible(Mark::N9);
        assert_eq!(cell.get_count(), 0);
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
}
