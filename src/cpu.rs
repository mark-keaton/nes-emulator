use crate::cpu::memory::Memory;
use crate::cpu::opscodes::{OpCode, OPCODES_MAP};
use crate::cpu::processor_status::ProcessorStatus;
use crate::cpu::register::Register;
use std::collections::HashMap;

mod memory;
mod opscodes;
mod processor_status;
mod register;

pub struct CPU {
    pub register_a: Register,
    pub register_x: Register,
    pub register_y: Register,
    pub status: ProcessorStatus,
    pub program_counter: u16,
    pub memory: Memory,
}

impl CPU {
    /// When the NES is powered on or reset, the CPU fetches the
    /// 16-bit address stored at 0xFFFC (low byte) and 0xFFFD (high byte)
    /// and sets the program_counter to this address.
    /// This is where execution begins.
    const RESET_VECTOR: u16 = 0xFFFC;

    pub fn new() -> Self {
        CPU {
            register_a: Register::new(0),
            register_x: Register::new(0),
            register_y: Register::new(0),
            status: ProcessorStatus::new(0),
            program_counter: 0,
            memory: Memory::new(),
        }
    }

    pub fn load_and_run(&mut self, program: Vec<u8>) {
        self.load(program);
        self.reset();
        self.run();
    }

    pub fn load(&mut self, program: Vec<u8>) {
        self.memory.load_program(program);
        self.memory.write_u16(CPU::RESET_VECTOR, 0x8000)
    }

    pub fn reset(&mut self) {
        self.register_a = Register::new(0);
        self.register_x = Register::new(0);
        self.status = ProcessorStatus::new(0);
        self.program_counter = self.memory.read_u16(CPU::RESET_VECTOR);
    }

    pub fn run(&mut self) {
        let ref opcodes: HashMap<u8, &'static OpCode> = *OPCODES_MAP;

        loop {
            let code = self.memory.read(self.program_counter);
            let opcode = opcodes
                .get(&code)
                .expect(&format!("OpCode {:x} is not recognized", code));

            self.program_counter += 1;
            let program_counter_state = self.program_counter;

            match code {
                0x00 => {
                    return;
                }
                0xE0 | 0xE4 | 0xEC => {
                    opscodes::registers::cpx(self, &opcode.mode);
                    self.program_counter += 1;
                }
                0xC0 | 0xC4 | 0xCC => {
                    opscodes::registers::cpy(self, &opcode.mode);
                    self.program_counter += 1;
                }
                0xCA => {
                    opscodes::registers::dex(self);
                }
                0x88 => {
                    opscodes::registers::dey(self);
                }
                0xE8 => {
                    opscodes::registers::inx(self);
                }
                0xA9 | 0xA5 | 0xB5 | 0xAD | 0xBD | 0xB9 | 0xA1 | 0xB1 => {
                    opscodes::registers::lda(self, &opcode.mode);
                    self.program_counter += 1;
                }
                0xA2 | 0xA6 | 0xB6 | 0xAE | 0xBE => {
                    opscodes::registers::ldx(self, &opcode.mode);
                    self.program_counter += 1;
                }
                0xA0 | 0xA4 | 0xB4 | 0xAC | 0xBC => {
                    opscodes::registers::ldy(self, &opcode.mode);
                    self.program_counter += 1;
                }
                0x85 | 0x95 | 0x8d | 0x9d | 0x99 | 0x81 | 0x91 => {
                    opscodes::registers::sta(self, &opcode.mode);
                }
                0xAA => {
                    opscodes::registers::tax(self);
                }
                _ => todo!(),
            }

            if program_counter_state == self.program_counter {
                self.program_counter += (opcode.len - 1) as u16;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0xaa_tax_move_a_to_x() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x0a, 0xaa, 0x00]);

        assert_eq!(cpu.register_x.0, 10)
    }

    #[test]
    fn test_0xa9_lda_immediate_load_data() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x05, 0x00]);
        assert_eq!(cpu.register_a.0, 0x05);
        assert_eq!(cpu.status.bit_1_is_set(), false);
        assert_eq!(cpu.status.bit_7_is_set(), false);
    }

    #[test]
    fn test_0xa9_lda_zero_flag() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x00, 0x00]);
        assert_eq!(cpu.status.bit_1_is_set(), true);
    }

    #[test]
    fn test_0xe8_inx_increments_x_register() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xE8, 0xE8, 0x00]);
        assert_eq!(cpu.register_x.0, 2);
    }

    #[test]
    fn test_5_ops_working_together() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0xc0, 0xaa, 0xe8, 0x00]);

        assert_eq!(cpu.register_x.0, 0xc1)
    }

    #[test]
    fn test_inx_overflow() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0xff, 0xaa, 0xe8, 0xe8, 0x00]);

        assert_eq!(cpu.register_x.0, 1)
    }

    #[test]
    fn test_0xca_dex_decrements_x_register() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa2, 0x02, 0xca, 0x00]);

        assert_eq!(cpu.register_x.0, 1);
    }

    #[test]
    fn test_dex_underflow() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa2, 0x00, 0xca, 0x00]);

        assert_eq!(cpu.register_x.0, 255);
    }

    #[test]
    fn test_0x88_dey_decrements_x_register() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa0, 0x02, 0x88, 0x00]);

        assert_eq!(cpu.register_y.0, 1);
    }

    #[test]
    fn test_dey_underflow() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa0, 0x00, 0x88, 0x00]);

        assert_eq!(cpu.register_y.0, 255);
    }

    #[test]
    fn test_cpx_x_greater_than_memory() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xA2, 0x80, 0xE0, 0x40, 0x00]);

        assert_eq!(cpu.status.bit_0_is_set(), true); // Carry flag set
        assert_eq!(cpu.status.bit_1_is_set(), false); // Zero flag not set
        assert_eq!(cpu.status.bit_7_is_set(), false); // Negative flag not set
    }

    #[test]
    fn test_cpx_x_equals_memory() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xA2, 0x40, 0xE0, 0x40, 0x00]);

        assert_eq!(cpu.status.bit_0_is_set(), true); // Carry flag set
        assert_eq!(cpu.status.bit_1_is_set(), true); // Zero flag set
        assert_eq!(cpu.status.bit_7_is_set(), false); // Negative flag not set
    }

    #[test]
    fn test_cpx_x_less_than_memory() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xA2, 0x01, 0xE0, 0x40, 0x00]);

        assert_eq!(cpu.status.bit_0_is_set(), false); // Carry flag not set
        assert_eq!(cpu.status.bit_1_is_set(), false); // Zero flag not set
        assert_eq!(cpu.status.bit_7_is_set(), true); // Negative flag set
    }

    #[test]
    fn test_cpy_y_greater_than_memory() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xA0, 0x80, 0xC0, 0x40, 0x00]);

        assert_eq!(cpu.status.bit_0_is_set(), true); // Carry flag set
        assert_eq!(cpu.status.bit_1_is_set(), false); // Zero flag not set
        assert_eq!(cpu.status.bit_7_is_set(), false); // Negative flag not set
    }

    #[test]
    fn test_cpy_y_equals_memory() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xA0, 0x40, 0xC0, 0x40, 0x00]);

        assert_eq!(cpu.status.bit_0_is_set(), true); // Carry flag set
        assert_eq!(cpu.status.bit_1_is_set(), true); // Zero flag set
        assert_eq!(cpu.status.bit_7_is_set(), false); // Negative flag not set
    }

    #[test]
    fn test_cpy_y_less_than_memory() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xA0, 0x01, 0xC0, 0x40, 0x00]);

        assert_eq!(cpu.status.bit_0_is_set(), false); // Carry flag not set
        assert_eq!(cpu.status.bit_1_is_set(), false); // Zero flag not set
        assert_eq!(cpu.status.bit_7_is_set(), true); // Negative flag set
    }
}
