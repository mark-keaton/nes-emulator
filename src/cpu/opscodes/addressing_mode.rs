use crate::cpu::CPU;

#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum AddressingMode {
    Accumulator,
    Immediate,
    ZeroPage,
    ZeroPage_X,
    ZeroPage_Y,
    Absolute,
    Absolute_X,
    Absolute_Y,
    Indirect_X,
    Indirect_Y,
    NoneAddressing,
}

impl AddressingMode {
    pub fn get_operand_address(cpu: &CPU, mode: &AddressingMode) -> u16 {
        match mode {
            AddressingMode::Immediate => cpu.program_counter,

            AddressingMode::ZeroPage => cpu.memory.read(cpu.program_counter) as u16,

            AddressingMode::Absolute => cpu.memory.read_u16(cpu.program_counter),

            AddressingMode::ZeroPage_X => {
                let pos = cpu.memory.read(cpu.program_counter);
                let addr = pos.wrapping_add(cpu.register_x.0) as u16;
                addr
            }
            AddressingMode::ZeroPage_Y => {
                let pos = cpu.memory.read(cpu.program_counter);
                let addr = pos.wrapping_add(cpu.register_y.0) as u16;
                addr
            }

            AddressingMode::Absolute_X => {
                let base = cpu.memory.read_u16(cpu.program_counter);
                let addr = base.wrapping_add(cpu.register_x.0 as u16);
                addr
            }
            AddressingMode::Absolute_Y => {
                let base = cpu.memory.read_u16(cpu.program_counter);
                let addr = base.wrapping_add(cpu.register_y.0 as u16);
                addr
            }

            AddressingMode::Indirect_X => {
                let base = cpu.memory.read(cpu.program_counter);

                let ptr: u8 = (base as u8).wrapping_add(cpu.register_x.0);
                let lo = cpu.memory.read(ptr as u16);
                let hi = cpu.memory.read(ptr.wrapping_add(1) as u16);
                (hi as u16) << 8 | (lo as u16)
            }
            AddressingMode::Indirect_Y => {
                let base = cpu.memory.read(cpu.program_counter);

                let lo = cpu.memory.read(base as u16);
                let hi = cpu.memory.read((base as u8).wrapping_add(1) as u16);
                let deref_base = (hi as u16) << 8 | (lo as u16);
                let deref = deref_base.wrapping_add(cpu.register_y.0 as u16);
                deref
            }

            _ => {
                panic!("mode {:?} is not supported", mode);
            }
        }
    }
}
