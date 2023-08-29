use crate::cpu::opscodes::addressing_mode::AddressingMode;
use crate::cpu::Register;
use crate::cpu::CPU;

fn update_zero_and_negative_flags(cpu: &mut CPU, register: Register) -> () {
    cpu.status.set_zero_flag(register.is_zero());
    cpu.status.set_negative_flag(register.bit_7_is_set());
    ()
}

fn update_carry_zero_and_negative_flags(cpu: &mut CPU, comparator: u8, register: Register) -> () {
    let difference = (register.0).wrapping_sub(comparator);
    let result = Register::new(difference);
    cpu.status.set_carry_flag(register.0 >= comparator);
    cpu.status.set_zero_flag(register.0 == comparator);
    cpu.status.set_negative_flag(result.bit_7_is_set());
    ()
}

pub fn adc(cpu: &mut CPU, mode: &AddressingMode) -> () {
    let addr = AddressingMode::get_operand_address(cpu, mode);
    let carry_value = cpu.status.get_carry_flag();
    let param = cpu.memory.read(addr);

    let temp = cpu.register_a.0 as u16 + param as u16 + carry_value as u16;
    let result = (temp & 0xFF) as u8;

    // Setting flags
    cpu.status.set_carry_flag(temp > 0xFF);
    cpu.status.set_zero_flag(result == 0);
    cpu.status.set_overflow_flag(
        // Overflow occurs when the sign bit is different before and after the addition
        // XOR'ing between each transition checks for a sign bit change
        (cpu.register_a.0 ^ result) & (param ^ result) & 0x80 != 0,
    );
    cpu.status.set_negative_flag((result & 0b1000_0000) != 0);

    cpu.register_a.0 = result;
}
