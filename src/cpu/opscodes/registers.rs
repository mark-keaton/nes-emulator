use crate::cpu::opscodes::addressing_mode::AddressingMode;
use crate::cpu::Register;
use crate::cpu::CPU;

fn update_zero_and_negative_flags(cpu: &mut CPU, register: Register) -> () {
    cpu.status = cpu.status.set_zero_flag(register.is_zero());
    cpu.status = cpu.status.set_negative_flag(register.bit_7_is_set());
    ()
}

fn update_carry_zero_and_negative_flags(cpu: &mut CPU, comparator: u8, register: Register) -> () {
    let difference = (register.0).wrapping_sub(comparator);
    let result = Register::new(difference);
    cpu.status = cpu.status.set_carry_flag(register.0 >= comparator);
    cpu.status = cpu.status.set_zero_flag(register.0 == comparator);
    cpu.status = cpu.status.set_negative_flag(result.bit_7_is_set());
    ()
}

pub fn cpx(cpu: &mut CPU, mode: &AddressingMode) -> () {
    let addr = AddressingMode::get_operand_address(cpu, mode);
    let operand = cpu.memory.read(addr);
    update_carry_zero_and_negative_flags(cpu, operand, cpu.register_x);
    ()
}

pub fn cpy(cpu: &mut CPU, mode: &AddressingMode) -> () {
    let addr = AddressingMode::get_operand_address(cpu, mode);
    let operand = cpu.memory.read(addr);
    update_carry_zero_and_negative_flags(cpu, operand, cpu.register_y);
    ()
}

pub fn dex(cpu: &mut CPU) -> () {
    cpu.register_x.decrement();
    update_zero_and_negative_flags(cpu, cpu.register_x);
    ()
}

pub fn dey(cpu: &mut CPU) -> () {
    cpu.register_y.decrement();
    update_zero_and_negative_flags(cpu, cpu.register_y);
    ()
}

pub fn inc(cpu: &mut CPU, mode: &AddressingMode) -> () {
    let addr = AddressingMode::get_operand_address(cpu, mode);
    let param = cpu.memory.read(addr);
    let result = param.wrapping_add(1);
    cpu.memory.write(addr, result);
    update_zero_and_negative_flags(cpu, Register::new(result));
    ()
}

pub fn inx(cpu: &mut CPU) -> () {
    cpu.register_x.increment();
    update_zero_and_negative_flags(cpu, cpu.register_x);
    ()
}

pub fn iny(cpu: &mut CPU) -> () {
    cpu.register_y.increment();
    update_zero_and_negative_flags(cpu, cpu.register_y);
    ()
}

pub fn lda(cpu: &mut CPU, mode: &AddressingMode) -> () {
    let addr = AddressingMode::get_operand_address(cpu, mode);
    let param = cpu.memory.read(addr);
    cpu.register_a = Register::new(param);
    update_zero_and_negative_flags(cpu, cpu.register_a);
    ()
}

pub fn ldx(cpu: &mut CPU, mode: &AddressingMode) -> () {
    let addr = AddressingMode::get_operand_address(cpu, mode);
    let param = cpu.memory.read(addr);
    cpu.register_x = Register::new(param);
    update_zero_and_negative_flags(cpu, cpu.register_x);
    ()
}

pub fn ldy(cpu: &mut CPU, mode: &AddressingMode) -> () {
    let addr = AddressingMode::get_operand_address(cpu, mode);
    let param = cpu.memory.read(addr);
    cpu.register_y = Register::new(param);
    update_zero_and_negative_flags(cpu, cpu.register_y);
    ()
}

pub fn sta(cpu: &mut CPU, mode: &AddressingMode) -> () {
    let addr = AddressingMode::get_operand_address(cpu, mode);
    cpu.memory.write(addr, cpu.register_a.0);
    ()
}

pub fn stx(cpu: &mut CPU, mode: &AddressingMode) -> () {
    let addr = AddressingMode::get_operand_address(cpu, mode);
    cpu.memory.write(addr, cpu.register_x.0);
    ()
}

pub fn sty(cpu: &mut CPU, mode: &AddressingMode) -> () {
    let addr = AddressingMode::get_operand_address(cpu, mode);
    cpu.memory.write(addr, cpu.register_y.0);
    ()
}

pub fn tax(cpu: &mut CPU) -> () {
    cpu.register_x = cpu.register_a;
    update_zero_and_negative_flags(cpu, cpu.register_x);
    ()
}
