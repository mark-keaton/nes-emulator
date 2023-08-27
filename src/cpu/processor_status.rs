pub struct ProcessorStatus(u8);

impl ProcessorStatus {
    pub fn new(val: u8) -> Self {
        ProcessorStatus(val)
    }

    pub fn value(&self) -> u8 {
        self.0
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
    pub fn set_0(&mut self, state: bool) -> Self {
        todo!()
    }

    /// Zero flag
    pub fn set_1(&mut self, state: bool) -> Self {
        self.set_zero_flag(state)
    }

    /// Interrupt disable flag
    pub fn set_2(&mut self, state: bool) -> Self {
        todo!()
    }

    /// Decimal flag
    pub fn set_3(&mut self, state: bool) -> Self {
        todo!()
    }

    /// No CPU effect, see: the B flag
    pub fn set_4(&mut self, state: bool) -> Self {
        todo!()
    }

    /// No CPU effect, see: the B flag
    pub fn set_5(&mut self, state: bool) -> Self {
        todo!()
    }

    /// Overflow flag
    pub fn set_6(&mut self, state: bool) -> Self {
        todo!()
    }

    /// Negative flag
    pub fn set_7(&mut self, state: bool) -> Self {
        self.set_negative_flag(state)
    }

    pub fn set_bit_at(&mut self, index: usize) -> Self {
        match index {
            0 => ProcessorStatus(self.0 | 0b0000_0001),
            1 => ProcessorStatus(self.0 | 0b0000_0010),
            2 => ProcessorStatus(self.0 | 0b0000_0100),
            3 => ProcessorStatus(self.0 | 0b0000_1000),
            4 => ProcessorStatus(self.0 | 0b0001_0000),
            5 => ProcessorStatus(self.0 | 0b0010_0000),
            6 => ProcessorStatus(self.0 | 0b0100_0000),
            7 => ProcessorStatus(self.0 | 0b1000_0000),
            _ => panic!("Out of bounds"),
        }
    }

    pub fn unset_bit_at(&mut self, index: usize) -> Self {
        match index {
            0 => ProcessorStatus(self.0 & 0b1111_1110),
            1 => ProcessorStatus(self.0 & 0b1111_1101),
            2 => ProcessorStatus(self.0 & 0b1111_1011),
            3 => ProcessorStatus(self.0 & 0b1111_0111),
            4 => ProcessorStatus(self.0 & 0b1110_1111),
            5 => ProcessorStatus(self.0 & 0b1101_1111),
            6 => ProcessorStatus(self.0 & 0b1011_1111),
            7 => ProcessorStatus(self.0 & 0b0111_1111),
            _ => panic!("Out of bounds"),
        }
    }

    pub fn set_zero_flag(&mut self, state: bool) -> ProcessorStatus {
        match state {
            true => self.set_bit_at(1),
            false => self.unset_bit_at(1),
        }
    }

    pub fn set_negative_flag(&mut self, state: bool) -> Self {
        match state {
            true => self.set_bit_at(7),
            false => self.unset_bit_at(7),
        }
    }
}
