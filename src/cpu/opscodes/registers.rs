use crate::cpu::Register;
use crate::cpu::CPU;

fn update_zero_and_negative_flags(cpu: &mut CPU, register: Register) -> () {
    cpu.status = cpu.status.set_zero_flag(register.is_zero());
    cpu.status = cpu.status.set_negative_flag(register.bit_7_is_set());
    ()
}

pub fn lda(cpu: &mut CPU, program: &Vec<u8>) -> () {
    let param = program[cpu.program_counter as usize];
    cpu.program_counter += 1;
    cpu.register_a = Register::new(param);

    update_zero_and_negative_flags(cpu, cpu.register_a);
    ()
}

pub fn tax(cpu: &mut CPU) -> () {
    cpu.register_x = cpu.register_a;
    update_zero_and_negative_flags(cpu, cpu.register_x);
    ()
}
