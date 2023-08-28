use crate::cpu::memory::Memory;
use crate::cpu::processor_status::ProcessorStatus;
use crate::cpu::register::Register;

mod memory;
mod opscodes;
mod processor_status;
mod register;

pub struct CPU {
    pub register_a: Register,
    pub register_x: Register,
    pub status: ProcessorStatus,
    pub program_counter: u16,
    memory: Memory,
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
        loop {
            let opscode = self.memory.read(self.program_counter);
            self.program_counter += 1;

            match opscode {
                0x00 => {
                    return;
                }
                0xA9 => {
                    opscodes::registers::lda(self);
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
        cpu.load_and_run(vec![0xa9, 0x0a, 0xaa, 0x00]);

        assert_eq!(cpu.register_x.value(), 10)
    }

    #[test]
    fn test_0xa9_lda_immediate_load_data() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x05, 0x00]);
        assert_eq!(cpu.register_a.value(), 0x05);
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
        assert_eq!(cpu.register_x.value(), 2);
    }

    #[test]
    fn test_5_ops_working_together() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0xc0, 0xaa, 0xe8, 0x00]);

        assert_eq!(cpu.register_x.value(), 0xc1)
    }

    #[test]
    fn test_inx_overflow() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0xff, 0xaa, 0xe8, 0xe8, 0x00]);

        assert_eq!(cpu.register_x.value(), 1)
    }
}
