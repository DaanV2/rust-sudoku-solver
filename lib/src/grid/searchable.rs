use super::{cell::Cell, coords::Coord, mark::Mark};

pub trait Searchable {
    fn get_cell(&self, index: usize) -> &Cell;
    fn get_coord(&self, index: usize) -> Coord;
    fn max(&self) -> usize {
        9
    }

    fn find_value(&self, value: u8) -> Option<usize> {
        for i in 0..self.max() {
            if self.get_cell(i).value == value {
                return Some(i);
            }
        }
        None
    }

    fn has_value(&self, value: u8) -> bool {
        for i in 0..self.max() {
            if self.get_cell(i).value == value {
                return true;
            }
        }
        false
    }

    fn has_possible(&self, value: Mark) -> bool {
        for i in 0..self.max() {
            if self.get_cell(i).is_possible(value) {
                return true;
            }
        }
        false
    }

    fn count_possible(&self, value: Mark) -> u32 {
        let mut count = 0;
        for i in 0..self.max() {
            if self.get_cell(i).is_possible(value) {
                count += 1;
            }
        }
        count
    }

    fn iter(&self) -> Box<dyn Iterator<Item = usize>> {
        Box::new((0..self.max()).into_iter())
    }

    fn iter_coords(&self) -> Box<dyn Iterator<Item = Coord> + '_> {
        Box::new((0..self.max()).into_iter().map(move |i| self.get_coord(i)))
    }

    fn iter_cells(&self) -> Box<dyn Iterator<Item = &Cell> + '_> {
        Box::new((0..self.max()).into_iter().map(move |i| self.get_cell(i)))
    }
}
