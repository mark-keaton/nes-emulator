use crate::cpu::opscodes::addressing_mode::AddressingMode;
use crate::cpu::Register;
use crate::cpu::CPU;

fn update_zero_and_negative_flags(cpu: &mut CPU, register: Register) -> () {
    cpu.status = cpu.status.set_zero_flag(register.is_zero());
    cpu.status = cpu.status.set_negative_flag(register.bit_7_is_set());
    ()
}

pub fn inx(cpu: &mut CPU) -> () {
    cpu.register_x.increment();
    update_zero_and_negative_flags(cpu, cpu.register_a);
    ()
}

pub fn lda(cpu: &mut CPU, mode: &AddressingMode) -> () {
    let addr = AddressingMode::get_operand_address(cpu, mode);
    let param = cpu.memory.read(addr);
    cpu.register_a = Register::new(param);
    update_zero_and_negative_flags(cpu, cpu.register_a);
    ()
}

pub fn sta(cpu: &mut CPU, mode: &AddressingMode) -> () {
    let addr = AddressingMode::get_operand_address(cpu, mode);
    cpu.memory.write(addr, cpu.register_a.0);
    ()
}

pub fn tax(cpu: &mut CPU) -> () {
    cpu.register_x = cpu.register_a;
    update_zero_and_negative_flags(cpu, cpu.register_x);
    ()
}
