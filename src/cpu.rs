use crate::cpu::processor_status::ProcessorStatus;
use crate::cpu::register::Register;

mod opscodes;
mod processor_status;
mod register;

pub struct CPU {
    pub register_a: Register,
    pub register_x: Register,
    pub status: ProcessorStatus,
    pub program_counter: u8,
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            register_a: Register::new(0),
            register_x: Register::new(0),
            status: ProcessorStatus::new(0),
            program_counter: 0,
        }
    }

    pub fn interpret(&mut self, program: Vec<u8>) {
        loop {
            let opscode = program[self.program_counter as usize];
            self.program_counter += 1;

            match opscode {
                0x00 => {
                    return;
                }
                0xA9 => {
                    opscodes::registers::lda(self, &program);
                }
                0xAA => {
                    opscodes::registers::tax(self);
                }
                0xE8 => {
                    opscodes::registers::inx(self);
                }
                _ => todo!(),
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
        cpu.register_a = Register::new(10);
        cpu.interpret(vec![0xaa, 0x00]);

        assert_eq!(cpu.register_x.value(), 10)
    }

    #[test]
    fn test_0xa9_lda_immediate_load_data() {
        let mut cpu = CPU::new();
        cpu.interpret(vec![0xa9, 0x05, 0x00]);
        assert_eq!(cpu.register_a.value(), 0x05);
        assert_eq!(cpu.status.bit_1_is_set(), false);
        assert_eq!(cpu.status.bit_7_is_set(), false);
    }

    #[test]
    fn test_0xa9_lda_zero_flag() {
        let mut cpu = CPU::new();
        cpu.interpret(vec![0xa9, 0x00, 0x00]);
        assert_eq!(cpu.status.bit_1_is_set(), true);
    }

    #[test]
    fn test_0xe8_inx_increments_x_register() {
        let mut cpu = CPU::new();
        cpu.interpret(vec![0xE8, 0xE8, 0x00]);
        assert_eq!(cpu.register_x.value(), 2);
    }

    #[test]
    fn test_5_ops_working_together() {
        let mut cpu = CPU::new();
        cpu.interpret(vec![0xa9, 0xc0, 0xaa, 0xe8, 0x00]);

        assert_eq!(cpu.register_x.value(), 0xc1)
    }

    #[test]
    fn test_inx_overflow() {
        let mut cpu = CPU::new();
        cpu.register_x = Register::new(0xff);
        cpu.interpret(vec![0xe8, 0xe8, 0x00]);

        assert_eq!(cpu.register_x.value(), 1)
    }
}
