use crate::util::shared::{AdjustBy1, Comparison};
use crate::util::u8_ext::BitwiseU8;
use std::fmt;

#[derive(Copy, Clone)]
pub struct ProcessorStatus(pub u8);

impl fmt::Debug for ProcessorStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:08b}", self.0)
    }
}

impl fmt::Display for ProcessorStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:08b}", self.0)
    }
}

impl ProcessorStatus {
    pub fn new(val: u8) -> Self {
        ProcessorStatus(val)
    }

    pub fn set_bit_at(&mut self, index: usize) -> () {
        if index > 7 {
            panic!("Out of bounds");
        }
        self.0 |= 1 << index;
    }

    pub fn unset_bit_at(&mut self, index: usize) -> () {
        if index > 7 {
            panic!("Out of bounds");
        }
        self.0 &= !(1 << index);
    }

    pub fn get_carry_flag(&self) -> u8 {
        self.bit_0_is_set() as u8
    }

    pub fn set_carry_flag(&mut self, state: bool) -> () {
        match state {
            true => self.set_bit_at(0),
            false => self.unset_bit_at(0),
        }
    }

    pub fn get_zero_flag(&self) -> u8 {
        self.bit_1_is_set() as u8
    }

    pub fn set_zero_flag(&mut self, state: bool) -> () {
        match state {
            true => self.set_bit_at(1),
            false => self.unset_bit_at(1),
        }
    }

    pub fn get_interupt_disable_flag(&self) -> u8 {
        self.bit_2_is_set() as u8
    }

    pub fn get_decimal_flag(&self) -> u8 {
        self.bit_3_is_set() as u8
    }

    pub fn get_overflow_flag(&self) -> u8 {
        self.bit_6_is_set() as u8
    }

    pub fn set_overflow_flag(&mut self, state: bool) -> () {
        match state {
            true => self.set_bit_at(6),
            false => self.unset_bit_at(6),
        }
    }

    pub fn get_negative_flag(&self) -> u8 {
        self.bit_7_is_set() as u8
    }

    pub fn set_negative_flag(&mut self, state: bool) -> () {
        match state {
            true => self.set_bit_at(7),
            false => self.unset_bit_at(7),
        }
    }
}

impl AdjustBy1 for ProcessorStatus {
    fn decrement(&mut self) -> () {
        self.0 = self.0.wrapping_sub(1);
        ()
    }

    fn increment(&mut self) -> () {
        self.0 = self.0.wrapping_add(1);
        ()
    }
}

impl Comparison for ProcessorStatus {
    fn is_zero(self) -> bool {
        self.0 == 0
    }
}

impl BitwiseU8 for ProcessorStatus {
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
