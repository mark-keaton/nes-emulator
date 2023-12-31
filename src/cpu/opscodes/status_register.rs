use crate::cpu::CPU;

pub fn clc(cpu: &mut CPU) -> () {
    cpu.status.set_carry_flag(false);
    ()
}

pub fn sec(cpu: &mut CPU) -> () {
    cpu.status.set_carry_flag(true);
    ()
}
