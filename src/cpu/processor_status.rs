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

    /// Carry flag
    pub fn set_0(&mut self, _state: bool) -> () {
        todo!()
    }

    /// Zero flag
    pub fn set_1(&mut self, state: bool) -> () {
        self.set_zero_flag(state)
    }

    /// Interrupt disable flag
    pub fn set_2(&mut self, _state: bool) -> () {
        todo!()
    }

    /// Decimal flag
    pub fn set_3(&mut self, _state: bool) -> () {
        todo!()
    }

    /// No CPU effect, see: the B flag
    pub fn set_4(&mut self, _state: bool) -> () {
        todo!()
    }

    /// No CPU effect, see: the B flag
    pub fn set_5(&mut self, _state: bool) -> () {
        todo!()
    }

    /// Overflow flag
    pub fn set_6(&mut self, _state: bool) -> () {
        todo!()
    }

    /// Negative flag
    pub fn set_7(&mut self, state: bool) -> () {
        self.set_negative_flag(state)
    }

    pub fn set_bit_at(&mut self, index: usize) -> () {
        self.0 = match index {
            0 => self.0 | 0b0000_0001,
            1 => self.0 | 0b0000_0010,
            2 => self.0 | 0b0000_0100,
            3 => self.0 | 0b0000_1000,
            4 => self.0 | 0b0001_0000,
            5 => self.0 | 0b0010_0000,
            6 => self.0 | 0b0100_0000,
            7 => self.0 | 0b1000_0000,
            _ => panic!("Out of bounds"),
        };
        ()
    }

    pub fn unset_bit_at(&mut self, index: usize) -> () {
        self.0 = match index {
            0 => self.0 & 0b1111_1110,
            1 => self.0 & 0b1111_1101,
            2 => self.0 & 0b1111_1011,
            3 => self.0 & 0b1111_0111,
            4 => self.0 & 0b1110_1111,
            5 => self.0 & 0b1101_1111,
            6 => self.0 & 0b1011_1111,
            7 => self.0 & 0b0111_1111,
            _ => panic!("Out of bounds"),
        };
        ()
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
