use super::possibility::{Possibility, Mark};



pub struct Cell {
    pub value: u8,
    pub possibilities: Possibility
}

impl Cell {
    pub fn new() -> Cell {
        Cell {
            value: 0,
            possibilities: Possibility::new()
        }
    }

    pub fn copy(&self) -> Cell {
        Cell {
            value: self.value,
            possibilities: self.possibilities.copy()
        }
    }

    pub fn is_determined(&self) -> bool {
        self.value != 0
    }

    pub fn is_possible(&self, value: Mark) -> bool {
        self.possibilities.is_possible(value)
    }

    pub fn set(&mut self, value: Mark) {
        self.value = value as u8;
        self.possibilities.all_off();
        self.possibilities.set(value);
    }

    pub fn unset(&mut self, value: Mark) {
        self.possibilities.unset(value);
    }

    pub fn set_state(&mut self, value: Mark, on: bool) {
        self.possibilities.set_state(value, on);
    }

    pub fn get_state(&self, value: Mark) -> bool {
        self.possibilities.get_state(value)
    }

    pub fn get_count(&self) -> u32 {
        self.possibilities.get_count()
    }
}