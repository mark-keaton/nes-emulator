use crate::cpu::addressing_mode::AddressingMode;
use crate::cpu::memory::Memory;
use crate::cpu::opscodes::{OpCode, OPCODES_MAP};
use crate::cpu::processor_status::ProcessorStatus;
use crate::cpu::register::Register;
use std::collections::HashMap;

mod addressing_mode;
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
                0x90 => {
                    opscodes::control_flow::bcc(self, &opcode.mode);
                }
                0xB0 => {
                    opscodes::control_flow::bcs(self, &opcode.mode);
                }
                0xF0 => {
                    opscodes::control_flow::beq(self, &opcode.mode);
                }
                0x24 | 0x2C => {
                    opscodes::arithmetic_logic::bit(self, &opcode.mode);
                }
                0x18 => {
                    opscodes::status_register::clc(self);
                }
                0xC9 | 0xC5 | 0xD5 | 0xCD | 0xDD | 0xD9 | 0xC1 | 0xD1 => {
                    opscodes::arithmetic_logic::cmp(self, &opcode.mode);
                }
                0xE0 | 0xE4 | 0xEC => {
                    opscodes::registers::cpx(self, &opcode.mode);
                }
                0xC0 | 0xC4 | 0xCC => {
                    opscodes::registers::cpy(self, &opcode.mode);
                }
                0xC6 | 0xD6 | 0xCE | 0xDE => {
                    opscodes::arithmetic_logic::dec(self, &opcode.mode);
                }
                0xCA => {
                    opscodes::registers::dex(self);
                }
                0x88 => {
                    opscodes::registers::dey(self);
                }
                0x49 | 0x45 | 0x55 | 0x4D | 0x5D | 0x59 | 0x41 | 0x51 => {
                    opscodes::arithmetic_logic::eor(self, &opcode.mode);
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
                0x4A | 0x46 | 0x56 | 0x4E | 0x5E => {
                    opscodes::arithmetic_logic::lsr(self, &opcode.mode);
                }
                0x09 | 0x05 | 0x15 | 0x0D | 0x1D | 0x19 | 0x01 | 0x11 => {
                    opscodes::arithmetic_logic::ora(self, &opcode.mode);
                }
                0x48 => {
                    opscodes::registers::pha(self);
                }
                0x2A | 0x26 | 0x36 | 0x2E | 0x3E => {
                    opscodes::arithmetic_logic::rol(self, &opcode.mode);
                }
                0x6A | 0x66 | 0x76 | 0x6E | 0x7E => {
                    opscodes::arithmetic_logic::ror(self, &opcode.mode);
                }
                0xE9 | 0xE5 | 0xF5 | 0xED | 0xFD | 0xF9 | 0xE1 | 0xF1 => {
                    opscodes::arithmetic_logic::sbc(self, &opcode.mode);
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

            if program_counter_state == self.program_counter
                || opcode.mode == AddressingMode::Relative
            {
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
    fn test_bcc_branch_not_taken() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0x38, 0x90, 0x02, 0xA9, 0xFF, 0x00]); // SEC (set carry), BCC +2, LDA #0xFF
        assert_eq!(cpu.register_a.0, 0xFF); // Since BCC wasn't taken, LDA should be executed.
    }

    #[test]
    fn test_bcc_branch_taken() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0x18, 0x90, 0x02, 0xA9, 0xFF, 0xA9, 0xAA, 0x00]); // CLC (clear carry), BCC +2, LDA #0xFF, LDA #0xAA
        assert_eq!(cpu.register_a.0, 0xAA); // BCC should be taken, skipping the first LDA and executing the second LDA
    }

    #[test]
    fn test_bcc_negative_offset_set_carry_with_adc() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![
            0xA9, 0xFE, // LDA #0xFE (Load A with 0xFE)
            0x69, 0x01, // ADC #0x01 (Add 1, which won't overflow with the initial value)
            0x90, 0xFC, // BCC -4 (Go back 4 bytes if carry is clear)
            0xA9, 0xAA, // LDA #0xAA
            0x00, // BRK or another ending instruction
        ]);

        // On the first pass, 0x7F + 0x01 = 0x80 doesn't set the carry flag, so BCC is taken, going back 6 bytes.
        // On the second pass, 0x80 + 0x01 = 0x81 does set the carry flag, so BCC isn't taken, and execution continues to LDA #0xAA.
        assert_eq!(cpu.register_a.0, 0xAA);
        assert_eq!(cpu.program_counter, 0x8009);
    }

    #[test]
    fn test_bcs_carry_set_by_adc() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![
            0xA9, 0xFF, // LDA #0xFF (Load A with 0xFF)
            0x69, 0x01, // ADC #0x01 (Add 1, which will cause a carry due to overflow)
            0xB0, 0x02, // BCS +2 (Go forward 4 bytes if carry is set)
            0xA9, 0x11, // LDA #0x11
            0xA9, 0xAA, // LDA #0xAA
            0x00, // BRK or another ending instruction
        ]);

        // 0xFF + 0x01 = 0x00 with the carry flag set. Therefore, BCS is taken and skips to LDA #0xAA.
        assert_eq!(cpu.register_a.0, 0xAA);
        assert_eq!(cpu.program_counter, 0x800B);
    }

    #[test]
    fn test_bcs_carry_not_set() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![
            0xA9, 0x7F, // LDA #0x7F (Load A with 0x7F)
            0x69, 0x01, // ADC #0x01 (Add 1, which won't cause a carry)
            0xB0, 0x02, // BCS +2 (Go forward 2 bytes if carry is set)
            0xA9, 0x11, // LDA #0x11
            0x00, // BRK or another ending instruction
        ]);

        // 0x7F + 0x01 = 0x80 without setting the carry flag. Therefore, BCS isn't taken and the LDA #0x11 is executed.
        assert_eq!(cpu.register_a.0, 0x11);
        assert_eq!(cpu.program_counter, 0x8009);
    }

    #[test]
    fn test_bcs_negative_offset_set_carry_with_adc() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![
            0xA9, 0x7F, // LDA #0x7F (Load A with 0x7F)
            0x69, 0x01, // ADC #0x01 (Add 1, which won't cause a carry)
            0x69, 0x80, // ADC #0x80 (Add 128, which will cause a carry due to overflow)
            0xB0, 0xFC, // BCS -4 (Go back 4 bytes if carry is set)
            0xA9, 0xAA, // LDA #0xAA
            0x00, // BRK or another ending instruction
        ]);

        // The first ADC won't set the carry flag, but the second ADC will. BCS then goes back 4 bytes to the second ADC, which still sets the carry. Execution then moves to LDA #0xAA.
        assert_eq!(cpu.register_a.0, 0xAA);
        assert_eq!(cpu.program_counter, 0x800B);
    }

    #[test]
    fn test_beq_positive_offset_zero_flag_set() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![
            0xA9, 0x01, // LDA #0x01
            0x49, 0x01, // EOR #0x01 (XOR with itself to set zero flag)
            0xF0, 0x02, // BEQ +2 (Skip 2 bytes ahead if zero flag is set)
            0xA9, 0xBB, // LDA #0xBB (This will be skipped)
            0xA9, 0xCC, // LDA #0xCC (This should be executed)
            0x00, // BRK or another ending instruction
        ]);
        assert_eq!(cpu.register_a.0, 0xCC);
        assert_eq!(cpu.program_counter, 0x800B); // PC should be right after the second LDA
    }

    #[test]
    fn test_beq_negative_offset_zero_flag_set() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![
            0xA9, 0x01, // LDA #0x01
            0x49, 0x01, // EOR #0x01 (XOR with itself to set zero flag)
            0xF0, 0xFE, // BEQ -2 (Skip back 2 bytes if zero flag is set)
            0xA9, 0xCC, // LDA #0xCC (This should be executed)
            0x00, // BRK or another ending instruction
        ]);
        assert_eq!(cpu.register_a.0, 0xCC); // EOR should have been executed again, resulting in zero
        assert_eq!(cpu.program_counter, 0x8009); // PC should be right after the BEQ
    }

    #[test]
    fn test_beq_not_taken_zero_flag_clear() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![
            0xA9, 0x7F, // LDA #0x7F (Load a non-zero value)
            0xF0,
            0x02, // BEQ +2 (Skip 2 bytes ahead if zero flag is set; should not be taken)
            0xA9, 0xCC, // LDA #0xCC (This should be executed)
            0x00, // BRK or another ending instruction
        ]);
        assert_eq!(cpu.register_a.0, 0x7F);
        assert_eq!(cpu.program_counter, 0x8007); // PC should be right after the LDA
    }

    #[test]
    fn test_bit_zero_flag_set() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xA9, 0x0F, 0x85, 0x10, 0xA9, 0xF0, 0x24, 0x10, 0x00]); // LDA #0x0F, STA $10, LDA #0xF0, BIT $10
        assert_eq!(cpu.status.bit_1_is_set(), true); // Zero flag should be set as the result of A AND M is 0x00
    }

    #[test]
    fn test_bit_negative_flag_set() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xA9, 0x80, 0x85, 0x10, 0xA9, 0x80, 0x24, 0x10, 0x00]); // LDA #0x80, STA $10, LDA #0x80, BIT $10
        assert_eq!(cpu.status.bit_7_is_set(), true); // Negative flag should be set as bit 7 of memory is set
    }

    #[test]
    fn test_bit_overflow_flag_set() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xA9, 0x40, 0x85, 0x10, 0xA9, 0x40, 0x24, 0x10, 0x00]); // LDA #0x40, STA $10, LDA #0x40, BIT $10
        assert_eq!(cpu.status.bit_6_is_set(), true); // Overflow flag should be set as bit 6 of memory is set
    }

    #[test]
    fn test_bit_no_flags_set() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xA9, 0x20, 0x85, 0x10, 0xA9, 0x20, 0x24, 0x10, 0x00]); // LDA #0x20, STA $10, LDA #0x20, BIT $10
        assert_eq!(cpu.status.bit_1_is_set(), false); // Zero flag should be clear
        assert_eq!(cpu.status.bit_7_is_set(), false); // Negative flag should be clear
        assert_eq!(cpu.status.bit_6_is_set(), false); // Overflow flag should be clear
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
    fn test_cmp_equal() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xA9, 0x42, 0xC9, 0x42, 0x00]); // LDA #0x42, CMP #0x42
        assert_eq!(cpu.status.bit_1_is_set(), true); // Zero flag should be set as A equals M
        assert_eq!(cpu.status.bit_0_is_set(), true); // Carry flag should be set as A >= M
        assert_eq!(cpu.status.bit_7_is_set(), false); // Negative flag should be clear
    }

    #[test]
    fn test_cmp_less_than() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xA9, 0x20, 0xC9, 0x40, 0x00]); // LDA #0x20, CMP #0x40
        assert_eq!(cpu.status.bit_1_is_set(), false); // Zero flag should be clear as A != M
        assert_eq!(cpu.status.bit_0_is_set(), false); // Carry flag should be clear as A < M
        assert_eq!(cpu.status.bit_7_is_set(), true); // Negative flag should be set due to subtraction result
    }

    #[test]
    fn test_cmp_greater_than() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xA9, 0x50, 0xC9, 0x30, 0x00]); // LDA #0x50, CMP #0x30
        assert_eq!(cpu.status.bit_1_is_set(), false); // Zero flag should be clear as A != M
        assert_eq!(cpu.status.bit_0_is_set(), true); // Carry flag should be set as A > M
        assert_eq!(cpu.status.bit_7_is_set(), false); // Negative flag should be clear
    }

    #[test]
    fn test_dec_basic() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xA2, 0x01, 0x86, 0x10, 0xCE, 0x10, 0x00]); // LDX #0x01, STX $10, DEC $10
        assert_eq!(cpu.memory.read(0x10), 0x00); // Memory at $10 should now be 0x00
        assert_eq!(cpu.status.bit_1_is_set(), true); // Zero flag should be set as the result is 0x00
        assert_eq!(cpu.status.bit_7_is_set(), false); // Negative flag should be clear
    }

    #[test]
    fn test_dec_zero_to_negative() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xA2, 0x00, 0x86, 0x10, 0xCE, 0x10, 0x00]); // LDX #0x00, STX $10, DEC $10
        assert_eq!(cpu.memory.read(0x10), 0xFF); // Memory at $10 should now be 0xFF
        assert_eq!(cpu.status.bit_1_is_set(), false); // Zero flag should be clear
        assert_eq!(cpu.status.bit_7_is_set(), true); // Negative flag should be set as the result is negative
    }

    #[test]
    fn test_dec_negative() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xA2, 0xFF, 0x86, 0x10, 0xCE, 0x10, 0x00]); // LDX #0xFF, STX $10, DEC $10
        assert_eq!(cpu.memory.read(0x10), 0xFE); // Memory at $10 should now be 0xFE
        assert_eq!(cpu.status.bit_1_is_set(), false); // Zero flag should be clear
        assert_eq!(cpu.status.bit_7_is_set(), true); // Negative flag should still be set as the result remains negative
    }

    #[test]
    fn test_eor_basic() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xA9, 0x0F, 0x49, 0xF0, 0x00]); // LDA #0x0F, EOR #0xF0
        assert_eq!(cpu.register_a.0, 0xFF); // Result should be 0xFF
        assert_eq!(cpu.status.bit_1_is_set(), false); // Zero flag should be clear
        assert_eq!(cpu.status.bit_7_is_set(), true); // Negative flag should be set
    }

    #[test]
    fn test_eor_zero_result() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xA9, 0xAA, 0x49, 0xAA, 0x00]); // LDA #0xAA, EOR #0xAA
        assert_eq!(cpu.register_a.0, 0x00); // Result should be 0x00
        assert_eq!(cpu.status.bit_1_is_set(), true); // Zero flag should be set
        assert_eq!(cpu.status.bit_7_is_set(), false); // Negative flag should be clear
    }

    #[test]
    fn test_eor_no_change() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xA9, 0x55, 0x49, 0x00, 0x00]); // LDA #0x55, EOR #0x00
        assert_eq!(cpu.register_a.0, 0x55); // Result should still be 0x55
        assert_eq!(cpu.status.bit_1_is_set(), false); // Zero flag should be clear
        assert_eq!(cpu.status.bit_7_is_set(), false); // Negative flag should be clear
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
    fn test_lsr_basic() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xA9, 0x02, 0x4A, 0x00]); // LDA #0x02, LSR A
        assert_eq!(cpu.register_a.0, 0x01); // Result should be 0x01
        assert_eq!(cpu.status.bit_1_is_set(), false); // Zero flag should be clear
        assert_eq!(cpu.status.bit_0_is_set(), false); // Carry flag should be clear
        assert_eq!(cpu.status.bit_7_is_set(), false); // Negative flag should be clear
    }

    #[test]
    fn test_lsr_carry_set() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xA9, 0x01, 0x4A, 0x00]); // LDA #0x01, LSR A
        assert_eq!(cpu.register_a.0, 0x00); // Result should be 0x00
        assert_eq!(cpu.status.bit_1_is_set(), true); // Zero flag should be set
        assert_eq!(cpu.status.bit_0_is_set(), true); // Carry flag should be set as bit 0 was shifted into it
        assert_eq!(cpu.status.bit_7_is_set(), false); // Negative flag should be clear
    }

    #[test]
    fn test_lsr_shift_into_carry() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xA9, 0x81, 0x4A, 0x00]); // LDA #0x81, LSR A
        assert_eq!(cpu.register_a.0, 0x40); // Result should be 0x40
        assert_eq!(cpu.status.bit_1_is_set(), false); // Zero flag should be clear
        assert_eq!(cpu.status.bit_0_is_set(), true); // Carry flag should be set as bit 0 of 0x81 was shifted into it
        assert_eq!(cpu.status.bit_7_is_set(), false); // Negative flag should be clear
    }

    #[test]
    fn test_ora_basic() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xA9, 0x02, 0x09, 0x01, 0x00]); // LDA #0x02, ORA #0x01
        assert_eq!(cpu.register_a.0, 0x03); // Result should be 0x03 (0b0000_0010 | 0b0000_0001 = 0b0000_0011)
        assert_eq!(cpu.status.bit_1_is_set(), false); // Zero flag should be clear
        assert_eq!(cpu.status.bit_7_is_set(), false); // Negative flag should be clear
    }

    #[test]
    fn test_ora_negative_result() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xA9, 0x80, 0x09, 0x01, 0x00]); // LDA #0x80, ORA #0x01
        assert_eq!(cpu.register_a.0, 0x81); // Result should be 0x81 (0b1000_0000 | 0b0000_0001 = 0b1000_0001)
        assert_eq!(cpu.status.bit_1_is_set(), false); // Zero flag should be clear
        assert_eq!(cpu.status.bit_7_is_set(), true); // Negative flag should be set due to bit 7 being 1
    }

    #[test]
    fn test_ora_zero_result() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xA9, 0x00, 0x09, 0x00, 0x00]); // LDA #0x00, ORA #0x00
        assert_eq!(cpu.register_a.0, 0x00); // Result should be 0x00
        assert_eq!(cpu.status.bit_1_is_set(), true); // Zero flag should be set
        assert_eq!(cpu.status.bit_7_is_set(), false); // Negative flag should be clear
    }

    #[test]
    fn test_rol_basic() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xA9, 0x40, 0x2A, 0x00]); // LDA #0x40, ROL A
        assert_eq!(cpu.register_a.0, 0x80); // Result should be 0x80 (0b0100_0000 rotated left becomes 0b1000_0000)
        assert_eq!(cpu.status.bit_1_is_set(), false); // Zero flag should be clear
        assert_eq!(cpu.status.bit_7_is_set(), true); // Negative flag should be set due to bit 7 being 1
        assert_eq!(cpu.status.bit_0_is_set(), false); // Carry flag should be clear
    }

    #[test]
    fn test_rol_carry_set() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xA9, 0x80, 0x2A, 0x00]); // LDA #0x80, ROL A
        assert_eq!(cpu.register_a.0, 0x00); // Result should be 0x00 (0b1000_0000 rotated left becomes 0b0000_0000 with carry set)
        assert_eq!(cpu.status.bit_1_is_set(), true); // Zero flag should be set
        assert_eq!(cpu.status.bit_7_is_set(), false); // Negative flag should be clear
        assert_eq!(cpu.status.bit_0_is_set(), true); // Carry flag should be set because of the rotation
    }

    #[test]
    fn test_rol_with_initial_carry() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xA9, 0x40, 0x38, 0x2A, 0x00]); // LDA #0x40, SEC (set carry), ROL A
        assert_eq!(cpu.register_a.0, 0x81); // Result should be 0x81 (0b0100_0000 rotated left becomes 0b1000_0000 and carry inserted into bit 0)
        assert_eq!(cpu.status.bit_1_is_set(), false); // Zero flag should be clear
        assert_eq!(cpu.status.bit_7_is_set(), true); // Negative flag should be set due to bit 7 being 1
        assert_eq!(cpu.status.bit_0_is_set(), false); // Carry flag should be clear
    }

    #[test]
    fn test_ror_basic() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xA9, 0x01, 0x6A, 0x00]); // LDA #0x01, ROR A
        assert_eq!(cpu.register_a.0, 0x00); // Result should be 0x00 (0b0000_0001 rotated right becomes 0b0000_0000)
        assert_eq!(cpu.status.bit_1_is_set(), true); // Zero flag should be set
        assert_eq!(cpu.status.bit_7_is_set(), false); // Negative flag should be clear
        assert_eq!(cpu.status.bit_0_is_set(), true); // Carry flag should be set
    }

    #[test]
    fn test_ror_carry_set() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xA9, 0x02, 0x6A, 0x00]); // LDA #0x02, ROR A
        assert_eq!(cpu.register_a.0, 0x01); // Result should be 0x01 (0b0000_0010 rotated right becomes 0b0000_0001)
        assert_eq!(cpu.status.bit_1_is_set(), false); // Zero flag should be clear
        assert_eq!(cpu.status.bit_7_is_set(), false); // Negative flag should be clear
        assert_eq!(cpu.status.bit_0_is_set(), false); // Carry flag should be clear
    }

    #[test]
    fn test_ror_with_initial_carry() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xA9, 0x01, 0x38, 0x6A, 0x00]); // LDA #0x01, SEC (set carry), ROR A
        assert_eq!(cpu.register_a.0, 0x80); // Result should be 0x80 (0b0000_0001 rotated right becomes 0b1000_0000 with carry inserted into bit 7)
        assert_eq!(cpu.status.bit_1_is_set(), false); // Zero flag should be clear
        assert_eq!(cpu.status.bit_7_is_set(), true); // Negative flag should be set due to bit 7 being 1
        assert_eq!(cpu.status.bit_0_is_set(), true); // Carry flag should be set because of the rotation
    }

    #[test]
    fn test_sbc_basic() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xA9, 0x10, 0xE9, 0x05, 0x00]); // LDA #0x10, SBC #0x05
        assert_eq!(cpu.register_a.0, 0x0B); // Result should be 0x0B (0x10 - 0x05 = 0x0B)
        assert_eq!(cpu.status.bit_1_is_set(), false); // Zero flag should be clear
        assert_eq!(cpu.status.bit_7_is_set(), false); // Negative flag should be clear
        assert_eq!(cpu.status.bit_0_is_set(), true); // Carry flag should be set, as no borrow was required
    }

    #[test]
    fn test_sbc_borrow_required() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xA9, 0x05, 0xE9, 0x10, 0x00]); // LDA #0x05, SBC #0x10
        assert_eq!(cpu.register_a.0, 0xF5); // Result will wrap and be 0xF5 (0x05 - 0x10 = -0x0B or 0xF5 in two's complement)
        assert_eq!(cpu.status.bit_1_is_set(), false); // Zero flag should be clear
        assert_eq!(cpu.status.bit_7_is_set(), true); // Negative flag should be set because result is negative in two's complement
        assert_eq!(cpu.status.bit_0_is_set(), false); // Carry flag should be clear, indicating a borrow occurred
    }

    #[test]
    fn test_sbc_with_initial_carry() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xA9, 0x10, 0x38, 0xE9, 0x05, 0x00]); // LDA #0x10, SEC (set carry), SBC #0x05
        assert_eq!(cpu.register_a.0, 0x0A); // Result should be 0x0A (0x10 - 0x05 - 1(carry) = 0x0A)
        assert_eq!(cpu.status.bit_1_is_set(), false); // Zero flag should be clear
        assert_eq!(cpu.status.bit_7_is_set(), false); // Negative flag should be clear
        assert_eq!(cpu.status.bit_0_is_set(), true); // Carry flag should be set, as no borrow was required
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
