use crate::cpu::addressing_mode::AddressingMode;
use crate::cpu::Register;
use crate::cpu::CPU;
use crate::util::shared::Comparison;
use crate::util::u8_ext::BitwiseU8;

fn update_zero_and_negative_flags(cpu: &mut CPU, value: u8) -> () {
    cpu.status.set_zero_flag(value.is_zero());
    cpu.status.set_negative_flag(value.bit_7_is_set());
    ()
}

fn update_carry_zero_and_negative_flags(cpu: &mut CPU, comparator: u8, register: Register) -> () {
    let difference = (register.0).wrapping_sub(comparator);
    let result = difference;
    cpu.status.set_carry_flag(register.0 >= comparator);
    cpu.status.set_zero_flag(register.0 == comparator);
    cpu.status.set_negative_flag(result.bit_7_is_set());
    ()
}

pub fn bcc(cpu: &mut CPU, mode: &AddressingMode) -> () {
    // TODO: Handle cycles
    if cpu.status.get_carry_flag() == 0 {
        let addr = AddressingMode::get_operand_address(cpu, mode);
        // Read memory as signed i8 for negatives before casting to i16
        let relative_displacement = cpu.memory.read(addr) as i8 as i16;

        cpu.program_counter =
            (cpu.program_counter as i16).wrapping_add(relative_displacement) as u16;
    }
}

pub fn bcs(cpu: &mut CPU, mode: &AddressingMode) -> () {
    // TODO: Handle cycles
    if cpu.status.get_carry_flag() != 0 {
        let addr = AddressingMode::get_operand_address(cpu, mode);
        // Read memory as signed i8 for negatives before casting to i16
        let relative_displacement = cpu.memory.read(addr) as i8 as i16;

        cpu.program_counter =
            (cpu.program_counter as i16).wrapping_add(relative_displacement) as u16;
    }
}
