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