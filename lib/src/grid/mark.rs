use std::slice::Iter;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[repr(u16)]
pub enum Mark {
    N1 = 0b00000000_00000001, //1
    N2 = 0b00000000_00000010, //2
    N3 = 0b00000000_00000100, //4
    N4 = 0b00000000_00001000, //8
    N5 = 0b00000000_00010000, //16
    N6 = 0b00000000_00100000, //32
    N7 = 0b00000000_01000000, //64
    N8 = 0b00000000_10000000, //128
    N9 = 0b00000001_00000000, //256
}

impl Default for Mark {
    fn default() -> Self {
        Self::N1
    }
}

impl Mark {
    // Returns an iterator over all possible values
    pub fn iter() -> Iter<'static, Mark> {
        static FIELDS: [Mark; 9] = [
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
        FIELDS.iter()
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
    pub fn from_index(index: usize) -> Mark {
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

    pub fn to_value(&self) -> u8 {
        match self {
            Mark::N1 => 1,
            Mark::N2 => 2,
            Mark::N3 => 3,
            Mark::N4 => 4,
            Mark::N5 => 5,
            Mark::N6 => 6,
            Mark::N7 => 7,
            Mark::N8 => 8,
            Mark::N9 => 9,
        }
    }

    pub fn from_value(value: u8) -> Mark {
        match value {
            1 => Mark::N1,
            2 => Mark::N2,
            3 => Mark::N3,
            4 => Mark::N4,
            5 => Mark::N5,
            6 => Mark::N6,
            7 => Mark::N7,
            8 => Mark::N8,
            9 => Mark::N9,
            _ => Mark::N1,
        }
    }
}

impl From<u32> for Mark {
    fn from(value: u32) -> Self {
        Mark::from_value(value as u8)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_index() {
        for (index, mark) in Mark::iter().enumerate() {
            let value = mark.to_index();
            let mark2 = &Mark::from_index(value);

            assert_eq!(mark2, mark);
            assert_eq!(value, index);
        }
    }

    #[test]
    fn test_value() {
        for (index, mark) in Mark::iter().enumerate() {
            let value = mark.to_value();
            let mark2 = &Mark::from_value(value);

            assert_eq!(mark2, mark);
            assert_eq!(value, index as u8 + 1);
        }
    }
}
