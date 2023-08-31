use crate::util::shared::{AdjustBy1, Comparison};

pub trait BitwiseU8 {
    fn bit_is_set_at(self, index: usize) -> bool;
    fn bit_0_is_set(self) -> bool;
    fn bit_1_is_set(self) -> bool;
    fn bit_2_is_set(self) -> bool;
    fn bit_3_is_set(self) -> bool;
    fn bit_4_is_set(self) -> bool;
    fn bit_5_is_set(self) -> bool;
    fn bit_6_is_set(self) -> bool;
    fn bit_7_is_set(self) -> bool;

    fn set_bit_at(&mut self, index: usize) -> ();
    fn unset_bit_at(&mut self, index: usize) -> ();
}

impl BitwiseU8 for u8 {
    fn bit_is_set_at(self, index: usize) -> bool {
        self & (1 << index) != 0
    }

    fn bit_0_is_set(self) -> bool {
        self & 1 != 0
    }

    fn bit_1_is_set(self) -> bool {
        self & (1 << 1) != 0
    }

    fn bit_2_is_set(self) -> bool {
        self & (1 << 2) != 0
    }

    fn bit_3_is_set(self) -> bool {
        self & (1 << 3) != 0
    }

    fn bit_4_is_set(self) -> bool {
        self & (1 << 4) != 0
    }

    fn bit_5_is_set(self) -> bool {
        self & (1 << 5) != 0
    }

    fn bit_6_is_set(self) -> bool {
        self & (1 << 6) != 0
    }

    fn bit_7_is_set(self) -> bool {
        self & (1 << 7) != 0
    }

    fn set_bit_at(&mut self, index: usize) -> () {
        if index > 7 {
            panic!("Out of bounds");
        }
        *self |= 1 << index;
    }

    fn unset_bit_at(&mut self, index: usize) -> () {
        if index > 7 {
            panic!("Out of bounds");
        }
        *self &= !(1 << index);
    }
}

impl Comparison for u8 {
    fn is_zero(self) -> bool {
        self == 0
    }
}

impl AdjustBy1 for u8 {
    fn decrement(&mut self) -> () {
        *self = self.wrapping_sub(1);
        ()
    }

    fn increment(&mut self) -> () {
        *self = self.wrapping_add(1);
        ()
    }
}
