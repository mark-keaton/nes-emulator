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
    pub register_s: Register,
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
            register_s: Register::new(0),
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
        self.register_s = Register::new(0xFF); // Stack
        self.register_x = Register::new(0);
        self.register_y = Register::new(0);
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
                0x69 | 0x65 | 0x75 | 0x6D | 0x7D | 0x79 | 0x61 | 0x71 => {
                    opscodes::arithmetic_logic::adc(self, &opcode.mode);
                }
                0x29 | 0x25 | 0x35 | 0x2D | 0x3D | 0x39 | 0x21 | 0x31 => {
                    opscodes::arithmetic_logic::and(self, &opcode.mode);
                }
                0x0A | 0x06 | 0x16 | 0x0E | 0x1E => {
                    opscodes::arithmetic_logic::asl(self, &opcode.mode);
                }
                0xE0 | 0xE4 | 0xEC => {
                    opscodes::registers::cpx(self, &opcode.mode);
                }
                0xC0 | 0xC4 | 0xCC => {
                    opscodes::registers::cpy(self, &opcode.mode);
                }
                0xCA => {
                    opscodes::registers::dex(self);
                }
                0x88 => {
                    opscodes::registers::dey(self);
                }
                0xE6 | 0xF6 | 0xEE | 0xFE => {
                    opscodes::registers::inc(self, &opcode.mode);
                }
                0xE8 => {
                    opscodes::registers::inx(self);
                }
                0xC8 => {
                    opscodes::registers::iny(self);
                }
                0xA9 | 0xA5 | 0xB5 | 0xAD | 0xBD | 0xB9 | 0xA1 | 0xB1 => {
                    opscodes::registers::lda(self, &opcode.mode);
                }
                0xA2 | 0xA6 | 0xB6 | 0xAE | 0xBE => {
                    opscodes::registers::ldx(self, &opcode.mode);
                }
                0xA0 | 0xA4 | 0xB4 | 0xAC | 0xBC => {
                    opscodes::registers::ldy(self, &opcode.mode);
                }
                0x48 => {
                    opscodes::registers::pha(self);
                }
                0x38 => {
                    opscodes::status_register::sec(self);
                }
                0x85 | 0x95 | 0x8d | 0x9d | 0x99 | 0x81 | 0x91 => {
                    opscodes::registers::sta(self, &opcode.mode);
                }
                0x86 | 0x96 | 0x8e => {
                    opscodes::registers::stx(self, &opcode.mode);
                }
                0x84 | 0x94 | 0x8c => {
                    opscodes::registers::sty(self, &opcode.mode);
                }
                0xAA => {
                    opscodes::registers::tax(self);
                }
                0xA8 => {
                    opscodes::registers::tay(self);
                }
                0xBA => {
                    opscodes::registers::tsx(self);
                }
                0x8A => {
                    opscodes::registers::txa(self);
                }
                0x9A => {
                    opscodes::registers::txs(self);
                }
                0x98 => {
                    opscodes::registers::tya(self);
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
    use crate::util::u8_ext::*;

    #[test]
    fn test_adc_basic() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xA9, 0x30, 0x69, 0x40, 0x00]); // LDA #0x30 -> ADC #0x40 -> BRK
        assert_eq!(cpu.register_a.0, 0x70);
        assert_eq!(cpu.status.bit_0_is_set(), false);
        assert_eq!(cpu.status.bit_1_is_set(), false);
        assert_eq!(cpu.status.bit_6_is_set(), false);
        assert_eq!(cpu.status.bit_7_is_set(), false);
    }

    #[test]
    fn test_adc_with_carry() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xA9, 0xFF, 0x38, 0x69, 0x01, 0x00]);
        assert_eq!(cpu.register_a.0, 0x01);
        assert_eq!(cpu.status.bit_0_is_set(), true);
        assert_eq!(cpu.status.bit_1_is_set(), false);
        assert_eq!(cpu.status.bit_6_is_set(), false);
        assert_eq!(cpu.status.bit_7_is_set(), false);
    }

    #[test]
    fn test_adc_overflow() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xA9, 0x7F, 0x69, 0x02, 0x00]); // LDA #0x7F -> ADC #0x02 -> BRK
        assert_eq!(cpu.register_a.0, 0x81);
        assert_eq!(cpu.status.bit_0_is_set(), false);
        assert_eq!(cpu.status.bit_1_is_set(), false);
        assert_eq!(cpu.status.bit_6_is_set(), true);
        assert_eq!(cpu.status.bit_7_is_set(), true);
    }

    #[test]
    fn test_and_basic() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xA9, 0x0C, 0x85, 0x10, 0xA9, 0x0A, 0x25, 0x10, 0x00]);
        assert_eq!(cpu.register_a.0, 0x08); // 0x0C AND 0x0A = 0x08
    }

    #[test]
    fn test_and_zero_result() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xA9, 0x02, 0x85, 0x10, 0xA9, 0x04, 0x25, 0x10, 0x00]);
        assert_eq!(cpu.register_a.0, 0x00); // 0x02 AND 0x04 = 0x00
    }

    #[test]
    fn test_and_zero_flag() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xA9, 0x02, 0x85, 0x10, 0xA9, 0x04, 0x25, 0x10, 0x00]);
        assert_eq!(cpu.status.bit_1_is_set(), true); // Zero flag should be set due to result being 0x00
    }

    #[test]
    fn test_and_negative_flag() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xA9, 0x80, 0x85, 0x10, 0xA9, 0xF0, 0x25, 0x10, 0x00]);
        assert_eq!(cpu.status.bit_7_is_set(), true); // Negative flag should be set due to bit 7 of result being 1
    }

    #[test]
    fn test_asl_basic() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xA9, 0x01, 0x0A, 0x00]); // LDA #0x01, ASL A
        assert_eq!(cpu.register_a.0, 0x02); // 0x01 shifted left is 0x02
    }

    #[test]
    fn test_asl_zero_result() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xA9, 0x00, 0x0A, 0x00]); // LDA #0x00, ASL A
        assert_eq!(cpu.register_a.0, 0x00); // 0x00 shifted left remains 0x00
    }

    #[test]
    fn test_asl_zero_flag() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xA9, 0x00, 0x0A, 0x00]); // LDA #0x00, ASL A
        assert_eq!(cpu.status.bit_1_is_set(), true); // Zero flag should be set due to result being 0x00
    }

    #[test]
    fn test_asl_carry_flag() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xA9, 0x80, 0x0A, 0x00]); // LDA #0x80, ASL A
        assert_eq!(cpu.status.bit_0_is_set(), true); // Carry flag should be set as bit 7 was 1 and shifted out
    }

    #[test]
    fn test_asl_negative_flag() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xA9, 0x40, 0x0A, 0x00]); // LDA #0x40, ASL A
        assert_eq!(cpu.status.bit_7_is_set(), true); // Negative flag should be set as result's bit 7 is set
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
    fn test_5_ops_working_together() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0xc0, 0xaa, 0xe8, 0x00]);

        assert_eq!(cpu.register_x.0, 0xc1)
    }

    #[test]
    fn test_inx_increments_x_register() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xE8, 0xE8, 0x00]);
        assert_eq!(cpu.register_x.0, 2);
    }

    #[test]
    fn test_inx_overflow() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0xff, 0xaa, 0xe8, 0xe8, 0x00]);

        assert_eq!(cpu.register_x.0, 1)
    }

    #[test]
    fn test_iny_increments_y_register() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xC8, 0xC8, 0x00]);
        assert_eq!(cpu.register_y.0, 2);
    }

    #[test]
    fn test_iny_overflow() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa0, 0xff, 0xaa, 0xc8, 0xc8, 0x00]);

        assert_eq!(cpu.register_y.0, 1)
    }

    #[test]
    fn test_inc_zero_page() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xA9, 0x05, 0x85, 0x10, 0xE6, 0x10, 0x00]);
        assert_eq!(cpu.memory.read(0x10), 0x06); // Memory at 0x10 should be incremented to 0x06
    }

    #[test]
    fn test_inc_wraparound() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xA9, 0xFF, 0x85, 0x20, 0xE6, 0x20, 0x00]);
        assert_eq!(cpu.memory.read(0x20), 0x00); // Memory at 0x20 should wraparound to 0x00
    }

    #[test]
    fn test_inc_zero_flag() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xA9, 0xFF, 0x85, 0x30, 0xE6, 0x30, 0x00]);
        assert_eq!(cpu.status.bit_1_is_set(), true); // Zero flag should be set
    }

    #[test]
    fn test_inc_negative_flag() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xA9, 0x7F, 0x85, 0x40, 0xE6, 0x40, 0x00]);
        assert_eq!(cpu.status.bit_7_is_set(), true); // Negative flag should be set
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

    #[test]
    fn test_sta_zero_page() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xA9, 0x05, 0x85, 0x10, 0x00]);
        assert_eq!(cpu.memory.read(0x10), 0x05); // Memory at 0x10 should be 0x05
    }

    #[test]
    fn test_sta_negative_value() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xA9, 0x80, 0x85, 0x20, 0x00]);
        assert_eq!(cpu.memory.read(0x20), 0x80); // Memory at 0x20 should be 0x80
    }

    #[test]
    fn test_sta_zero_value() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xA9, 0x00, 0x85, 0x30, 0x00]);
        assert_eq!(cpu.memory.read(0x30), 0x00); // Memory at 0x30 should be 0x00
    }

    #[test]
    fn test_tax_move_a_to_x() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x0a, 0xaa, 0x00]);

        assert_eq!(cpu.register_x.0, 10)
    }

    #[test]
    fn test_tay_move_a_to_y() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x0a, 0xa8, 0x00]);

        assert_eq!(cpu.register_y.0, 10)
    }

    #[test]
    fn test_tsx_transfer_stack_to_x() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x05, 0x48, 0xba, 0x00]);

        assert_eq!(cpu.register_x.0, 0xFE); // Assuming initial stack pointer was 0xFF
        assert_eq!(cpu.memory.read(0x0100 + 0xFF), 0x05); // Memory at 0xFE should be 0x05
    }

    #[test]
    fn test_txa_transfer_x_to_a() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![
            0xA2, 0x0A, // LDX #0x0A
            0x8A, 0x00,
        ]); // TXA

        assert_eq!(cpu.register_a.0, 0x0A);
    }

    #[test]
    fn test_txs_transfer_x_to_s() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![
            0xA2, 0x0A, // LDX #0x0A
            0x9A, 0x00,
        ]); // TXS

        assert_eq!(cpu.register_s.0, 0x0A);
    }

    #[test]
    fn test_tya_transfer_y_to_a() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![
            0xA0, 0x0A, // LDY #0x0A
            0x98, 0x00,
        ]); // TYA

        assert_eq!(cpu.register_a.0, 0x0A);
    }
}
