use super::mark::Mark;

#[derive(Clone, Copy, PartialEq)]
pub struct Possibility {
    value: u16,
}

impl std::fmt::Debug for Possibility {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Possibility")
            .field("1", &self.is_possible(Mark::N1))
            .field("2", &self.is_possible(Mark::N2))
            .field("3", &self.is_possible(Mark::N3))
            .field("4", &self.is_possible(Mark::N4))
            .field("5", &self.is_possible(Mark::N5))
            .field("6", &self.is_possible(Mark::N6))
            .field("7", &self.is_possible(Mark::N7))
            .field("8", &self.is_possible(Mark::N8))
            .field("9", &self.is_possible(Mark::N9))
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

    pub fn is_empty(&self) -> bool {
        self.value == 0
    }

    pub fn from(value: Mark) -> Possibility {
        let mut p = Possibility::empty();
        p.set_possible(value);
        return p;
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
    pub fn set_possible(&mut self, value: Mark) {
        self.value |= value as u16;
    }

    // Sets all possibilities to false except the given value
    pub fn unset_possible(&mut self, value: Mark) {
        self.value &= !(value as u16);
    }

    // Sets all possibilities to the given value
    pub fn set_possible_state(&mut self, value: Mark, state: bool) {
        if state {
            self.set_possible(value);
        } else {
            self.unset_possible(value);
        }
    }

    // Returns the number of possibilities that are true
    pub fn get_count(&self) -> u32 {
        let v = self.value as usize;
        v.count_ones()
    }

    // Returns the value of the possibilities
    pub fn get_value(&self) -> u16 {
        self.value
    }

    pub fn iter_possible(&self) -> impl Iterator<Item = Mark> {
        let mut value = self.value;
        std::iter::from_fn(move || {
            if value == 0 {
                return None;
            }

            let mark = Mark::from_index(value.trailing_zeros() as usize);
            let mask = mark as u16;
            value &= !(mask);
            Some(mark)
        })
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
        p.set_possible(Mark::N1);
        p.set_possible(Mark::N2);
        p.set_possible(Mark::N3);

        assert_eq!(p.value, 0b00000000_00000111);
        assert_eq!(p.get_count(), 3);
    }

    #[test]
    fn set_each_field_works() {
        let mut p = Possibility::new();

        for m in Mark::iter() {
            p.all_off();
            p.set_possible(*m);

            assert_eq!(p.get_count(), 1);
            assert_eq!(p.is_possible(*m), true);

            p.unset_possible(*m);

            assert_eq!(p.get_count(), 0);
            assert_eq!(p.is_possible(*m), false);
        }
    }

    #[test]
    fn iter_possible() {
        let mut p = Possibility::new();
        p.all_off();
        p.set_possible(Mark::N1);
        p.set_possible(Mark::N2);
        p.set_possible(Mark::N3);

        let mut iter = p.iter_possible();
        assert_eq!(iter.next(), Some(Mark::N1));
        assert_eq!(iter.next(), Some(Mark::N2));
        assert_eq!(iter.next(), Some(Mark::N3));
        assert_eq!(iter.next(), None);
    }
}
