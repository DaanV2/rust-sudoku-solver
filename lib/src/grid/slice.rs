use std::fmt::Display;

use super::{cell::Cell, cell_collection::CellCollection, grid::Grid, mark::Mark};

#[derive(Clone)]
pub struct Slice {
    pub cells: [Cell; 9],
}

impl Slice {
    pub fn new() -> Slice {
        Slice {
            cells: [Cell::new_empty(); 9],
        }
    }

    /// Returns the number of cells that are not empty
    pub fn count(&self) -> usize {
        let mut count = 0;

        for i in self.iter() {
            if !self.cells[i].is_empty() {
                count += 1;
            }
        }

        count
    }

    /// Returns if the given value is determined in this slice
    pub fn any_determined_value(&self, value: u16) -> bool {
        for cell in self.cells.iter() {
            if cell.get_value() == value {
                return true;
            }
        }

        false
    }

    pub fn any_possible(&self, mark: Mark) -> bool {
        let mut data: u16 = 0;
        let v = mark.to_data();

        for cell in self.cells.iter() {
            data |= cell.get_value();
        }

        data & v == v
    }

    pub fn count_possible(&self, mark: Mark) -> usize {
        let mut count = 0;

        for cell in self.cells.iter() {
            if cell.is_possible(mark) {
                count += 1;
            }
        }

        count
    }

    pub fn count_determined(&self) -> usize {
        let mut count = 0;

        for cell in self.cells.iter() {
            if cell.is_determined() {
                count += 1;
            }
        }

        count
    }

    pub fn count_determined_value(&self, value: u16) -> usize {
        let mut count = 0;

        for cell in self.cells.iter() {
            if cell.get_value() == value {
                count += 1;
            }
        }

        count
    }

    pub fn from<T: CellCollection>(grid: &Grid, area: &T) -> Slice {
        if area.max() != 9 {
            panic!("Slice must be of size 9");
        }
        let mut slice = Slice::new();

        for i in area.iter() {
            let coord = area.get_coord(i);
            slice.cells[i] = grid.get_cell_at(coord).clone();
        }

        slice
    }

    /// Returns the first index of a cell that is possible for the given mark, assumes there is at least one
    pub fn first_possible(&self, mark: Mark) -> usize {
        for i in self.iter() {
            if self.cells[i].is_possible(mark) {
                return i;
            }
        }

        0
    }

    pub fn is_determined(&self, value: u16) -> bool {
        for cell in self.cells.iter() {
            if cell.get_value() == value {
                return true;
            }
        }

        false
    }

    pub fn is_possible(&self, mark: Mark) -> bool {
        let mut possible = false;

        for cell in self.cells.iter() {
            possible &= cell.is_possible(mark);
        }

        possible
    }

    pub fn is_fully_determined(&self) -> bool {
        let mut determined = true;

        for cell in self.cells.iter() {
            determined &= cell.is_determined();
        }

        determined
    }

    pub fn iter(&self) -> std::ops::Range<usize> {
        0..self.cells.len()
    }

    pub fn only_possible(&self) -> Slice {
        let mut slice = self.clone();

        for i in slice.iter() {
            if slice.cells[i].is_determined() {
                slice.cells[i] = Cell::new_empty();
            }
        }

        slice
    }

    pub fn only_possible_value(&self, mark: Mark) -> Slice {
        let mut slice = self.clone();

        for i in slice.iter() {
            if !slice.cells[i].is_possible(mark) {
                slice.cells[i] = Cell::new_empty();
            }
        }

        slice
    }

    pub fn only_determined(&self) -> Slice {
        let mut slice = self.clone();

        for i in slice.iter() {
            if !slice.cells[i].is_determined() {
                slice.cells[i] = Cell::new_empty();
            }
        }

        slice
    }

    pub fn only_determined_value(&self, value: u16) -> Slice {
        let mut slice = self.clone();

        for i in slice.iter() {
            if slice.cells[i].get_value() != value {
                slice.cells[i] = Cell::new_empty();
            }
        }

        slice
    }

    pub fn or_all(&self) -> Cell {
        let mut data = Cell::new_empty();

        for cell in self.cells.iter() {
            data = data | cell.clone();
        }

        data
    }

    /// Returns the first index and
    pub fn search_count_possible(&self, mark: Mark) -> (usize, usize) {
        let mut count = 0;
        let mut index = 0;

        for i in self.iter() {
            if self.cells[i].is_possible(mark) {
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
            s.push_str(&self.cells[i].to_string());
            s.push_str(" ");
        }

        write!(f, "{}", s)
    }
}

#[cfg(test)]
mod tests {
    use crate::grid::{cell::Cell, mark::Mark};

    use super::Slice;

    fn slice_example() -> Slice {
        let mut s = Slice::new();

        for i in s.iter() {
            s.cells[i] = Cell::new_with_value((i as u16) + 1);
        }

        s
    }

    #[test]
    pub fn test_only_possible() {
        let mut s = slice_example();

        s.cells[5] = Cell::new();
        s.cells[6] = Cell::new();

        let s = s.only_possible();
        let count = s.count();

        assert_eq!(count, 2);
    }

    #[test]
    pub fn test_only_possible_value() {
        let mut s = slice_example();

        s.cells[5] = Cell::new();
        s.cells[6] = Cell::new();

        let s = s.only_possible_value(Mark::N7);
        let count = s.count();

        assert_eq!(count, 2);
    }

    #[test]
    pub fn test_only_determined() {
        let mut s = slice_example();

        s.cells[5] = Cell::new();
        s.cells[6] = Cell::new();

        let s = s.only_determined();
        let count = s.count();

        assert_eq!(count, 7);
    }

    #[test]
    pub fn test_only_determined_value() {
        let mut s = slice_example();

        s.cells[5] = Cell::new();
        s.cells[6] = Cell::new();

        let s = s.only_determined_value(1);
        let count = s.count();

        assert_eq!(count, 1);
    }
}
