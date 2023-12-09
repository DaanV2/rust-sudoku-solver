use std::fmt::Display;

use super::{cell::Cell, cell_collection::CellCollection, grid::Grid, mark::Mark};

pub struct Slice {
    pub cells: [Cell; 9],
}

impl Slice {
    pub fn new() -> Slice {
        Slice {
            cells: [Cell::new_empty(); 9],
        }
    }

    pub fn is_possible(&self, mark: Mark) -> bool {
        let mut possible = false;

        for cell in self.cells.iter() {
            possible &= cell.is_possible(mark);
        }

        possible
    }

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

    pub fn count_possible(&self, mark: Mark) -> usize {
        let mut count = 0;

        for cell in self.cells.iter() {
            if cell.is_possible(mark) {
                count += 1;
            }
        }

        count
    }

    pub fn any_possible(&self, mark: Mark) -> bool {
        for cell in self.cells.iter() {
            if cell.is_possible(mark) {
                return true;
            }
        }

        false
    }

    pub fn is_determined(&self, value: usize) -> bool {
        for cell in self.cells.iter() {
            if cell.get_value() == value {
                return true;
            }
        }

        false
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

    pub fn count_determined_value(&self, value: usize) -> usize {
        let mut count = 0;

        for cell in self.cells.iter() {
            if cell.get_value() == value {
                count += 1;
            }
        }

        count
    }

    pub fn any_determined_value(&self, value: usize) -> bool {
        for cell in self.cells.iter() {
            if cell.get_value() == value {
                return true;
            }
        }

        false
    }

    pub fn is_fully_determined(&self) -> bool {
        for cell in self.cells.iter() {
            if !cell.is_determined() {
                return false;
            }
        }

        true
    }

    pub fn iter(&self) -> std::ops::Range<usize> {
        0..self.cells.len()
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
}

impl Display for Slice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();

        for i in self.iter() {
            s.push_str(&self.cells[i].get_value().to_string());
        }

        write!(f, "{}", s)
    }
}
