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

pub fn adc(cpu: &mut CPU, mode: &AddressingMode) -> () {
    let addr = AddressingMode::get_operand_address(cpu, mode);
    let carry_value = cpu.status.get_carry_flag();
    let param = cpu.memory.read(addr);

    let temp = (cpu.register_a.0 as u16)
        .wrapping_add(param as u16)
        .wrapping_add(carry_value as u16);
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

pub fn and(cpu: &mut CPU, mode: &AddressingMode) -> () {
    let addr = AddressingMode::get_operand_address(cpu, mode);
    let param = cpu.memory.read(addr);
    cpu.register_a.0 = cpu.register_a.0 & param;

    update_zero_and_negative_flags(cpu, cpu.register_a.0);
}

pub fn asl(cpu: &mut CPU, mode: &AddressingMode) -> () {
    let mut new_value = 0;
    let mut bit7 = false;

    match mode {
        AddressingMode::Accumulator => {
            let value = cpu.register_a.0;
            bit7 = value.bit_7_is_set();
            new_value = value << 1;
            cpu.register_a.0 = new_value;
        }
        _ => {
            let addr = AddressingMode::get_operand_address(cpu, mode);
            let value = cpu.memory.read(addr);
            bit7 = value.bit_7_is_set();
            new_value = value << 1;
            cpu.memory.write(addr, new_value);
        }
    }

    update_zero_and_negative_flags(cpu, new_value);
    cpu.status.set_carry_flag(bit7);
}

pub fn bit(cpu: &mut CPU, mode: &AddressingMode) -> () {
    let addr = AddressingMode::get_operand_address(cpu, mode);
    let param = cpu.memory.read(addr);
    let result = cpu.register_a.0 & param;

    cpu.status.set_zero_flag(result == 0);
    cpu.status.set_overflow_flag(param.bit_6_is_set());
    cpu.status.set_negative_flag(param.bit_7_is_set());
}

pub fn cmp(cpu: &mut CPU, mode: &AddressingMode) -> () {
    let addr = AddressingMode::get_operand_address(cpu, mode);
    let param = cpu.memory.read(addr);
    let result = cpu.register_a.0.wrapping_sub(param);

    cpu.status.set_carry_flag(cpu.register_a.0 >= param);
    cpu.status.set_zero_flag(cpu.register_a.0 == param);
    cpu.status.set_negative_flag(result.bit_7_is_set());
}

pub fn dec(cpu: &mut CPU, mode: &AddressingMode) -> () {
    let addr = AddressingMode::get_operand_address(cpu, mode);
    let param = cpu.memory.read(addr);
    let result = param.wrapping_sub(1);
    cpu.memory.write(addr, result);

    cpu.status.set_zero_flag(result == 0);
    cpu.status.set_negative_flag(result.bit_7_is_set());
}

pub fn eor(cpu: &mut CPU, mode: &AddressingMode) -> () {
    let addr = AddressingMode::get_operand_address(cpu, mode);
    let param = cpu.memory.read(addr);
    cpu.register_a.0 = cpu.register_a.0 ^ param;
    cpu.status.set_zero_flag(cpu.register_a.0 == 0);
    cpu.status.set_negative_flag(cpu.register_a.bit_7_is_set());
}

pub fn lsr(cpu: &mut CPU, mode: &AddressingMode) -> () {
    let mut new_value = 0;
    let mut bit0 = false;

    match mode {
        AddressingMode::Accumulator => {
            let value = cpu.register_a.0;
            bit0 = value.bit_0_is_set();
            new_value = value >> 1;
            cpu.register_a.0 = new_value;
        }
        _ => {
            let addr = AddressingMode::get_operand_address(cpu, mode);
            let value = cpu.memory.read(addr);
            bit0 = value.bit_0_is_set();
            new_value = value >> 1;
            cpu.memory.write(addr, new_value);
        }
    }

    update_zero_and_negative_flags(cpu, new_value);
    cpu.status.set_carry_flag(bit0);
}

pub fn ora(cpu: &mut CPU, mode: &AddressingMode) -> () {
    let addr = AddressingMode::get_operand_address(cpu, mode);
    let param = cpu.memory.read(addr);
    cpu.register_a.0 = cpu.register_a.0 | param;
    cpu.status.set_zero_flag(cpu.register_a.0 == 0);
    cpu.status.set_negative_flag(cpu.register_a.bit_7_is_set());
}

pub fn rol(cpu: &mut CPU, mode: &AddressingMode) -> () {
    let mut new_value = 0;
    let mut bit7 = false;
    let carry_flag = cpu.status.get_carry_flag();

    match mode {
        AddressingMode::Accumulator => {
            let value = cpu.register_a.0;
            bit7 = value.bit_7_is_set();
            new_value = value << 1;
            new_value |= carry_flag;
            cpu.register_a.0 = new_value;
        }
        _ => {
            let addr = AddressingMode::get_operand_address(cpu, mode);
            let value = cpu.memory.read(addr);
            bit7 = value.bit_7_is_set();
            new_value = value << 1;
            new_value |= carry_flag;
            cpu.memory.write(addr, new_value);
        }
    }

    update_zero_and_negative_flags(cpu, new_value);
    cpu.status.set_carry_flag(bit7);
}
pub fn ror(cpu: &mut CPU, mode: &AddressingMode) -> () {
    let mut new_value = 0;
    let mut bit0 = false;
    let carry_flag = cpu.status.get_carry_flag();

    match mode {
        AddressingMode::Accumulator => {
            let value = cpu.register_a.0;
            bit0 = value.bit_0_is_set();
            new_value = value >> 1;
            new_value |= carry_flag << 7;
            cpu.register_a.0 = new_value;
        }
        _ => {
            let addr = AddressingMode::get_operand_address(cpu, mode);
            let value = cpu.memory.read(addr);
            bit0 = value.bit_0_is_set();
            new_value = value >> 1;
            new_value |= carry_flag << 7;
            cpu.memory.write(addr, new_value);
        }
    }

    update_zero_and_negative_flags(cpu, new_value);
    cpu.status.set_carry_flag(bit0);
}

pub fn sbc(cpu: &mut CPU, mode: &AddressingMode) -> () {
    let addr = AddressingMode::get_operand_address(cpu, mode);
    let carry_value = cpu.status.get_carry_flag();
    let param = cpu.memory.read(addr);

    let temp = (cpu.register_a.0 as u16)
        .wrapping_sub(param as u16)
        .wrapping_sub(carry_value as u16);
    let result = (temp & 0xFF) as u8;

    // Setting flags
    cpu.status.set_carry_flag(temp <= 0xFF);
    cpu.status.set_zero_flag(result == 0);
    cpu.status.set_overflow_flag(
        ((cpu.register_a.0 ^ param) & 0x80 == 0) && ((cpu.register_a.0 ^ result) & 0x80 != 0),
    );
    cpu.status.set_negative_flag((result & 0b1000_0000) != 0);

    cpu.register_a.0 = result;
}
