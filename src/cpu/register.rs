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

    pub fn decrement(&mut self) -> () {
        self.0 = self.0.wrapping_sub(1);
        ()
    }

    pub fn increment(&mut self) -> () {
        self.0 = self.0.wrapping_add(1);
        ()
    }

    pub fn is_zero(&self) -> bool {
        self.0 == 0
    }

    pub fn bit_0_is_set(&self) -> bool {
        self.0 & 0b0000_0001 != 0
    }

    pub fn bit_1_is_set(&self) -> bool {
        self.0 & 0b0000_0010 != 0
    }

    pub fn bit_2_is_set(&self) -> bool {
        self.0 & 0b0000_0100 != 0
    }

    pub fn bit_3_is_set(&self) -> bool {
        self.0 & 0b0000_1000 != 0
    }

    pub fn bit_4_is_set(&self) -> bool {
        self.0 & 0b0001_0000 != 0
    }

    pub fn bit_5_is_set(&self) -> bool {
        self.0 & 0b0010_0000 != 0
    }

    pub fn bit_6_is_set(&self) -> bool {
        self.0 & 0b0100_0000 != 0
    }

    pub fn bit_7_is_set(&self) -> bool {
        self.0 & 0b1000_0000 != 0
    }
}
