use crate::cpu::processor_status::ProcessorStatus;
use crate::cpu::register::Register;

mod processor_status;
mod register;

pub struct CPU {
    pub register_a: Register,
    pub status: ProcessorStatus,
    pub program_counter: u8,
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            register_a: Register::new(0),
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
                    let param = program[self.program_counter as usize];
                    self.program_counter += 1;
                    self.register_a = Register::new(param);

                    self.status = self.status.set_zero_flag(self.register_a.value() == 0);
                    self.status = self
                        .status
                        .set_negative_flag(self.register_a.bit_7_is_set());
                    println!("");
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
}
