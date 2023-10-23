use super::{mark::Mark, possibility::Possibility};

/// A cell in the grid
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cell {
    /// The possibilities for this cell
    pub possibilities: Possibility,
    /// The value of this cell, 0 if not determined
    pub value: u8,
}

impl Cell {
    /// Creates a new empty cell, with all possibilities on
    pub fn new() -> Cell {
        Cell {
            possibilities: Possibility::new(),
            value: 0,
        }
    }

    /// Creates a new cell with a value, and all possibilities off
    pub fn new_with_value(value: u8) -> Cell {
        Cell {
            possibilities: Possibility::empty(),
            value: value,
        }
    }

    /// Returns true if the cell is determined or not
    pub fn is_determined(&self) -> bool {
        self.value > 0
    }

    /// Returns true if the given value is possible for this cell
    pub fn is_possible(&self, value: Mark) -> bool {
        if self.is_determined() {
            return false;
        }

        self.possibilities.is_possible(value)
    }

    /// Stores the given value in the cell, sets all possibilities off
    pub fn set_value(&mut self, value: u8) {
        self.value = value;
        self.possibilities = Possibility::empty();
    }

    /// Sets the given value as possible for this cell
    pub fn set_possible(&mut self, value: Mark) {
        self.possibilities.set_possible(value);
    }

    pub fn set_possible_state(&mut self, value: Mark, state: bool) {
        if self.is_determined() {
            return;
        }

        self.possibilities.set_possible_state(value, state);
    }

    pub fn unset_possible(&mut self, value: Mark) {
        if self.is_determined() {
            return;
        }

        self.possibilities.unset_possible(value);
    }

    pub fn get_count(&self) -> u32 {
        self.possibilities.get_count()
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

    #[test]
    fn test_set_value() {
        for i in 1..10 {
            let mut cell = super::Cell::new();
            assert_eq!(cell.is_determined(), false);
            assert_eq!(cell.possibilities.get_count(), 9);

            cell.set_value(i as u8);
            assert_eq!(cell.is_determined(), true);
            assert_eq!(cell.possibilities.get_count(), 0);
        }
    }

    #[test]
    fn test_size_object() {
        let cell = super::Cell::new();
        let size = std::mem::size_of_val(&cell);

        assert_eq!(size, 4);
    }

    #[test]
    fn test_set_and_unset_possibility() {
        let mut cell = super::Cell::new();
        cell.possibilities.all_off();

        for i in 1..10 {
            let mark = Mark::from_value(i);
            cell.set_possible(Mark::from_value(i));

            assert_eq!(cell.is_possible(mark), true);
        }

        for i in 1..10 {
            let mark = Mark::from_value(i);
            cell.unset_possible(Mark::from_value(i));

            assert_eq!(cell.is_possible(mark), false);
        }
    }

    #[test]
    fn test_get_count() {
        let mut cell = super::Cell::new();
        cell.possibilities.all_off();

        for i in 1..10 {
            cell.set_possible(Mark::from_value(i));
            assert_eq!(cell.get_count(), i as u32, "i: {}", i);
        }
    }
}
