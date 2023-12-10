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
    pub fn to_index(self) -> u16 {
        Mark::to_value(self) - 1
    }

    // Returns the value of the given index
    pub fn from_index(index: u16) -> Mark {
        Mark::from_value(index + 1)
    }

    pub fn to_value(self) -> u16 {
        ((self as usize).trailing_zeros() - 6) as u16
    }

    pub fn from_value(value: u16) -> Mark {
        let v = (1 << (value + 6)) as usize;
        unsafe { std::mem::transmute(v) }
    }

    // Returns raw data of the mark
    pub fn to_data(self) -> u16 {
        self as u16
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
            assert_eq!(value, index as u16);
        }
    }

    #[test]
    fn test_value_fixed() {
        assert_eq!(Mark::N1.to_value(), 1);
        assert_eq!(Mark::N2.to_value(), 2);
    }

    #[test]
    fn test_value() {
        for (index, mark) in Mark::iter().enumerate() {
            let value = mark.to_value();
            let mark2 = Mark::from_value(value);

            assert_eq!(value, index as u16 + 1, "from mark to value");
            assert_eq!(mark2, mark, "from mark to mark");
        }
    }

    #[test]
    fn test_index_fixed() {
        assert_eq!(Mark::N1.to_index(), 0);
        assert_eq!(Mark::N2.to_index(), 1);
    }

    #[test]
    fn test_index_value() {
        for index in 0..9 {
            let mark = Mark::from_index(index);
            let value = mark.to_index();

            assert_eq!(value, index);
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
