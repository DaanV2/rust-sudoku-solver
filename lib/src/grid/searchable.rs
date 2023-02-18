use super::{cell::Cell, mark::Mark};

pub trait Searchable {
    fn get_cell(&self, index: usize) -> &Cell;
    fn get_coords(&self, index: usize) -> (usize, usize);
    fn max(&self) -> usize {
        9
    }

    fn find_value(&self, value: u8) -> Option<usize> {
        for i in 0..9 {
            if self.get_cell(i).value == value {
                return Some(i);
            }
        }
        None
    }

    fn has_value(&self, value: u8) -> bool {
        for i in 0..9 {
            if self.get_cell(i).value == value {
                return true;
            }
        }
        false
    }

    fn has_possible(&self, value: Mark) -> bool {
        for i in 0..9 {
            if self.get_cell(i).is_possible(value) {
                return true;
            }
        }
        false
    }

    fn iter(&self) -> Box<dyn Iterator<Item = usize>> {
        Box::new((0..self.max()).into_iter())
    }
}