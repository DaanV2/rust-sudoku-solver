use std::slice::Iter;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[repr(u16)]
pub enum Mark {
    N1 = 0b00000000_00000001,
    N2 = 0b00000000_00000010,
    N3 = 0b00000000_00000100,
    N4 = 0b00000000_00001000,
    N5 = 0b00000000_00010000,
    N6 = 0b00000000_00100000,
    N7 = 0b00000000_01000000,
    N8 = 0b00000000_10000000,
    N9 = 0b00000001_00000000,
}

impl Default for Mark {
    fn default() -> Self {
        Self::N1
    }
}

impl Mark {
    // Returns an iterator over all possible values
    pub fn iter() -> Iter<'static, Mark> {
        static Fields: [Mark; 9] = [
            Mark::N1,
            Mark::N2,
            Mark::N3,
            Mark::N4,
            Mark::N5,
            Mark::N6,
            Mark::N7,
            Mark::N8,
            Mark::N9,
        ];
        Fields.iter()
    }

    // Returns the index of the given value
    pub fn to_index(&self) -> usize {
        match self {
            Mark::N1 => 0,
            Mark::N2 => 1,
            Mark::N3 => 2,
            Mark::N4 => 3,
            Mark::N5 => 4,
            Mark::N6 => 5,
            Mark::N7 => 6,
            Mark::N8 => 7,
            Mark::N9 => 8,
        }
    }

    // Returns the value of the given index
    pub fn from_index(index: usize) -> Self {
        match index {
            0 => Mark::N1,
            1 => Mark::N2,
            2 => Mark::N3,
            3 => Mark::N4,
            4 => Mark::N5,
            5 => Mark::N6,
            6 => Mark::N7,
            7 => Mark::N8,
            8 => Mark::N9,
            _ => Mark::N1,
        }
    }
}

pub struct Possibility {
    value: u16
}

impl Possibility {
    pub fn new() -> Possibility {
        Possibility{ 
            value: 0b00000001_11111111
        }
    }

    pub fn copy(&self) -> Possibility {
        Possibility{ 
            value: self.value
        }
    }

    pub fn is_possible(&self, value: Mark) -> bool {
        self.value & (value as u16) != 0
    }

    // Sets all possibilities to false
    pub fn all_off(&mut self) {
        self.value = 0;
    }

    // Sets all possibilities to true
    pub fn all_on(&mut self) {
        self.value = 0b00000001_11111111;
    }

    // Sets all possibilities to true except the given value
    pub fn set(&mut self, value: Mark) {
        self.value |= value as u16;
    }

    // Sets all possibilities to false except the given value
    pub fn unset(&mut self, value: Mark) {
        self.value &= !(value as u16);
    }

    // Sets all possibilities to the given value
    pub fn set_state(&mut self, value: Mark, on: bool) {
        if on {
            self.set(value);
        } else {
            self.unset(value);
        }
    }

    // Returns the state of the given value
    pub fn get_state(&self, value: Mark) -> bool {
        self.value & (value as u16) != 0
    }

    // Returns the number of possibilities that are true
    pub fn get_count(&self) -> u32 {
        self.value.count_ones()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_off_works() {
        let mut p = Possibility::new();
        p.all_off();

        assert_eq!(p.value, 0);
        assert_eq!(p.get_count(), 0);
    }

    #[test]
    fn all_on_works() {
        let mut p = Possibility::new();
        p.all_off();
        p.all_on();

        assert_eq!(p.value, 0b00000001_11111111);
        assert_eq!(p.get_count(), 9);
    }

    #[test]
    fn set_works() {
        let mut p = Possibility::new();
        p.all_off();
        p.set(Mark::N1);
        p.set(Mark::N2);
        p.set(Mark::N3);

        assert_eq!(p.value, 0b00000000_00000111);
        assert_eq!(p.get_count(), 3);
    }

    #[test]
    fn set_each_field_works() {
        let mut p = Possibility::new();

        
        for m in Mark::iter() {
            p.all_off();
            p.set(*m);

            assert_eq!(p.get_count(), 1);
            assert_eq!(p.get_state(*m), true);

            p.unset(*m);

            assert_eq!(p.get_count(), 0);
            assert_eq!(p.get_state(*m), false);
        }
    }
}