use std::ops::{BitAnd, BitOr, BitXor};

use super::cell::Cell;

pub struct Flags16 {
    flags: u16,
}

impl Flags16 {
    pub fn new() -> Self {
        Flags16 { flags: 0 }
    }

    pub fn set_bit(&mut self, index: usize) {
        let bit = Cell::possible_to_bit(index as u16);
        self.flags |= bit;
    }

    pub fn get_bit(&self, index: usize) -> bool {
        let bit = Cell::possible_to_bit(index as u16);
        self.flags & bit == bit
    }

    pub fn any(&self) -> bool {
        self.flags != 0
    }
}

impl From<Cell> for Flags16 {
    fn from(cell: Cell) -> Self {
        Flags16 {
            flags: cell.get_value() as u16,
        }
    }
}

impl BitOr for Flags16 {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Flags16 {
            flags: self.flags | rhs.flags,
        }
    }
}

impl BitAnd for Flags16 {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Flags16 {
            flags: self.flags & rhs.flags,
        }
    }
}

impl BitXor for Flags16 {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Flags16 {
            flags: self.flags ^ rhs.flags,
        }
    }
}
