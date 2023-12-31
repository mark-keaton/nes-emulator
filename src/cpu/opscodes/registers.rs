use crate::cpu::addressing_mode::AddressingMode;
use crate::cpu::Register;
use crate::cpu::CPU;
use crate::util::shared::{AdjustBy1, Comparison};
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
    update_zero_and_negative_flags(cpu, cpu.register_x.0);
    ()
}

pub fn dey(cpu: &mut CPU) -> () {
    cpu.register_y.decrement();
    update_zero_and_negative_flags(cpu, cpu.register_y.0);
    ()
}

pub fn inc(cpu: &mut CPU, mode: &AddressingMode) -> () {
    let addr = AddressingMode::get_operand_address(cpu, mode);
    let param = cpu.memory.read(addr);
    let result = param.wrapping_add(1);
    cpu.memory.write(addr, result);
    update_zero_and_negative_flags(cpu, result);
    ()
}

pub fn inx(cpu: &mut CPU) -> () {
    cpu.register_x.increment();
    update_zero_and_negative_flags(cpu, cpu.register_x.0);
    ()
}

pub fn iny(cpu: &mut CPU) -> () {
    cpu.register_y.increment();
    update_zero_and_negative_flags(cpu, cpu.register_y.0);
    ()
}

pub fn lda(cpu: &mut CPU, mode: &AddressingMode) -> () {
    let addr = AddressingMode::get_operand_address(cpu, mode);
    let param = cpu.memory.read(addr);
    cpu.register_a.0 = param;
    update_zero_and_negative_flags(cpu, cpu.register_a.0);
    ()
}

pub fn ldx(cpu: &mut CPU, mode: &AddressingMode) -> () {
    let addr = AddressingMode::get_operand_address(cpu, mode);
    let param = cpu.memory.read(addr);
    cpu.register_x.0 = param;
    update_zero_and_negative_flags(cpu, cpu.register_x.0);
    ()
}

pub fn ldy(cpu: &mut CPU, mode: &AddressingMode) -> () {
    let addr = AddressingMode::get_operand_address(cpu, mode);
    let param = cpu.memory.read(addr);
    cpu.register_y.0 = param;
    update_zero_and_negative_flags(cpu, cpu.register_y.0);
    ()
}

pub fn pha(cpu: &mut CPU) -> () {
    let stack_pointer = cpu.register_s;
    cpu.memory.push_to_stack(stack_pointer.0, cpu.register_a.0);
    cpu.register_s.decrement();
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
    update_zero_and_negative_flags(cpu, cpu.register_x.0);
    ()
}

pub fn tay(cpu: &mut CPU) -> () {
    cpu.register_y = cpu.register_a;
    update_zero_and_negative_flags(cpu, cpu.register_y.0);
    ()
}

pub fn tsx(cpu: &mut CPU) -> () {
    cpu.register_x = cpu.register_s;
    update_zero_and_negative_flags(cpu, cpu.register_x.0);
    ()
}

pub fn txa(cpu: &mut CPU) -> () {
    cpu.register_a = cpu.register_x;
    update_zero_and_negative_flags(cpu, cpu.register_a.0);
    ()
}

pub fn txs(cpu: &mut CPU) -> () {
    cpu.register_s = cpu.register_x;
    update_zero_and_negative_flags(cpu, cpu.register_s.0);
    ()
}

pub fn tya(cpu: &mut CPU) -> () {
    cpu.register_a = cpu.register_y;
    update_zero_and_negative_flags(cpu, cpu.register_a.0);
    ()
}
