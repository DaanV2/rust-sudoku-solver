use super::mark::Mark;

#[derive(Clone, Copy, PartialEq)]
pub struct Possibility {
    value: u16,
}

impl std::fmt::Debug for Possibility {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Possibility")
            .field("1", &self.get_state(Mark::N1))
            .field("2", &self.get_state(Mark::N2))
            .field("3", &self.get_state(Mark::N3))
            .field("4", &self.get_state(Mark::N4))
            .field("5", &self.get_state(Mark::N5))
            .field("6", &self.get_state(Mark::N6))
            .field("7", &self.get_state(Mark::N7))
            .field("8", &self.get_state(Mark::N8))
            .field("9", &self.get_state(Mark::N9))
            .finish()
    }
}

impl Possibility {
    pub fn new() -> Possibility {
        Possibility {
            value: 0b00000001_11111111,
        }
    }

    pub fn empty() -> Possibility {
        Possibility { value: 0 }
    }

    pub fn from(value: Mark) -> Possibility {
        let mut p = Possibility::empty();
        p.set(value);
        p
    }

    pub fn copy(&self) -> Possibility {
        Possibility { value: self.value }
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
        let v = self.value as usize;
        v.count_ones()
    }
}

impl Default for Possibility {
    fn default() -> Self {
        Self::new()
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
