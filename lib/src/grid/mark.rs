use std::{
    fmt::{Display, Formatter},
    ops::Shl,
};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[repr(usize)]
pub enum Mark {
    N1 = 0b0000_0000_1000_0000, //1
    N2 = 0b0000_0001_0000_0000, //2
    N3 = 0b0000_0010_0000_0000, //4
    N4 = 0b0000_0100_0000_0000, //8
    N5 = 0b0000_1000_0000_0000, //16
    N6 = 0b0001_0000_0000_0000, //32
    N7 = 0b0010_0000_0000_0000, //64
    N8 = 0b0100_0000_0000_0000, //128
    N9 = 0b1000_0000_0000_0000, //256
}

impl Default for Mark {
    fn default() -> Self {
        Self::N1
    }
}

impl Display for Mark {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_value())
    }
}

impl Mark {
    // Returns an iterator over all possible values
    pub fn iter() -> impl Iterator<Item = Mark> {
        let mut value = Mark::N1 as usize;

        std::iter::from_fn(move || {
            if value > (Mark::N9 as usize) {
                return None;
            }

            let mark = unsafe { std::mem::transmute(value) };
            value <<= 1;
            Some(mark)
        })
    }

    // Returns the index of the given value
    pub fn to_index(self) -> usize {
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

    pub fn to_value(self) -> usize {
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

    pub fn from_value(value: usize) -> Mark {
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

    // Returns raw data of the mark
    pub fn to_data(self) -> usize {
        self as usize
    }
}

impl From<u32> for Mark {
    fn from(value: u32) -> Self {
        Mark::from_value(value as usize)
    }
}

impl Shl<u32> for Mark {
    type Output = Mark;

    fn shl(self, rhs: u32) -> Self::Output {
        //Convert unsafe to usize, shift it and convert back to Mark
        let mut value = self as usize;
        value = value << rhs;
        unsafe { std::mem::transmute(value) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_index() {
        for (index, mark) in Mark::iter().enumerate() {
            let value = mark.to_index();
            let mark2 = Mark::from_index(value);

            assert_eq!(mark2, mark);
            assert_eq!(value, index);
        }
    }

    #[test]
    fn test_value() {
        for (index, mark) in Mark::iter().enumerate() {
            let value = mark.to_value();
            let mark2 = Mark::from_value(value);

            assert_eq!(mark2, mark);
            assert_eq!(value, index as usize + 1);
        }
    }

    #[test]
    fn test_iter() {
        let mut iter = Mark::iter();

        assert_eq!(iter.next(), Some(Mark::N1));
        assert_eq!(iter.next(), Some(Mark::N2));
        assert_eq!(iter.next(), Some(Mark::N3));
        assert_eq!(iter.next(), Some(Mark::N4));
        assert_eq!(iter.next(), Some(Mark::N5));
        assert_eq!(iter.next(), Some(Mark::N6));
        assert_eq!(iter.next(), Some(Mark::N7));
        assert_eq!(iter.next(), Some(Mark::N8));
        assert_eq!(iter.next(), Some(Mark::N9));
        assert_eq!(iter.next(), None);
    }
}
