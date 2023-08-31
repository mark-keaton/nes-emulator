use crate::util::shared::{AdjustBy1, Comparison};
use crate::util::u8_ext::BitwiseU8;
use std::fmt;

#[derive(Copy, Clone)]
pub struct Register(pub u8);

impl fmt::Debug for Register {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:08b}", self.0)
    }
}

impl fmt::Display for Register {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:08b}", self.0)
    }
}

impl Register {
    pub fn new(val: u8) -> Self {
        Register(val)
    }
}

impl AdjustBy1 for Register {
    fn decrement(&mut self) -> () {
        self.0 = self.0.wrapping_sub(1);
        ()
    }

    fn increment(&mut self) -> () {
        self.0 = self.0.wrapping_add(1);
        ()
    }
}

impl Comparison for Register {
    fn is_zero(self) -> bool {
        self.0 == 0
    }
}

impl BitwiseU8 for Register {
    fn bit_is_set_at(self, index: usize) -> bool {
        self.0 & (1 << index) != 0
    }

    fn bit_0_is_set(self) -> bool {
        self.0 & 0b0000_0001 != 0
    }

    fn bit_1_is_set(self) -> bool {
        self.0 & 0b0000_0010 != 0
    }

    fn bit_2_is_set(self) -> bool {
        self.0 & 0b0000_0100 != 0
    }

    fn bit_3_is_set(self) -> bool {
        self.0 & 0b0000_1000 != 0
    }

    fn bit_4_is_set(self) -> bool {
        self.0 & 0b0001_0000 != 0
    }

    fn bit_5_is_set(self) -> bool {
        self.0 & 0b0010_0000 != 0
    }

    fn bit_6_is_set(self) -> bool {
        self.0 & 0b0100_0000 != 0
    }

    fn bit_7_is_set(self) -> bool {
        self.0 & 0b1000_0000 != 0
    }

    fn set_bit_at(&mut self, index: usize) -> () {
        if index > 7 {
            panic!("Out of bounds");
        }
        self.0 |= 1 << index;
    }

    fn unset_bit_at(&mut self, index: usize) -> () {
        if index > 7 {
            panic!("Out of bounds");
        }
        self.0 &= !(1 << index);
    }
}
