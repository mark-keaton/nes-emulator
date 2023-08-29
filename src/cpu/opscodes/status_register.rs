use crate::cpu::CPU;

pub fn sec(cpu: &mut CPU) -> () {
    cpu.status.set_carry_flag(true);
    ()
}
