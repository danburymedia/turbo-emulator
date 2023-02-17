use std::sync::atomic::Ordering;
use crate::armv8::decodedefs::ArmInstr;
use crate::armv8::interpreter::main::Arm64Cpu;
pub fn sysreg_write(ai: &mut Arm64Cpu, sys_op0: u32,
                   sys_op1: u32, sys_crn: u32,  sys_crm: u32,
                   sys_op2: u32, val: u64) {
    match (sys_op0, sys_op1, sys_crn, sys_crm, sys_op2) {
        (0b11, 0b011, 0b1101, 0b0, 0b010) => {
            // tpidr_el0
            ai.tpidr[0] = val;
        }
        (_, _, _, _, _) => {
            unimplemented!();
        }
    }
}
pub fn sysreg_read(ai: &mut Arm64Cpu, sys_op0: u32,
                   sys_op1: u32, sys_crn: u32,  sys_crm: u32,
                   sys_op2: u32, t: usize) {
    let value: u64 = match (sys_op0, sys_op1, sys_crn, sys_crm, sys_op2) {
        (0b11, 0b011, 0b0, 0b0, 0b111) => {
            // dczid_el0
            (1 << 4) // dc instructions are prohibited
        }
        (0b11, 0b011, 0b1101, 0b0, 0b010) => {
            // tpidr_el0
            ai.tpidr[0]
        }
        (0b11, 0b011, 0b0100, 0b0100, 0) => {
            // fpcr
            ai.fpcr as u64

        },
        (0b11, 0b011, 0b0100, 0b0100, 0b001) => {
            // fpsr
            ai.fpsr as u64
        }
        (_, _, _, _, _) => {
            unimplemented!();
        }
    };
    ai.set_reg(t, value, false);
}
pub fn mrs(ai: &mut Arm64Cpu, arg: &ArmInstr) {
    let t = arg.get_rt();
    let op2 = (arg.insn >> 5) & 0b111;
    let crm = (arg.insn >> 8) & 0b1111;
    let crn = (arg.insn >> 12) & 0b1111;
    let op1 = (arg.insn >> 16) & 0b111;
    let mut op0 = (arg.insn >> 19) & 1;
    op0 += 2;
    sysreg_read(ai, op0, op1,
                crn, crm, op2, t);

}
pub fn msr_reg(ai: &mut Arm64Cpu, arg: &ArmInstr) {
    let t = arg.get_rt();
    let tval = ai.get_reg(t, false);
    let op2 = (arg.insn >> 5) & 0b111;
    let crm = (arg.insn >> 8) & 0b1111;
    let crn = (arg.insn >> 12) & 0b1111;
    let op1 = (arg.insn >> 16) & 0b111;
    let mut op0 = (arg.insn >> 19) & 1;
    op0 += 2;
    sysreg_write(ai, op0, op1,
                crn, crm, op2, tval);

}
pub fn svc(ai: &mut Arm64Cpu, arg: &ArmInstr) {
    if ai.is_usermode {
        // todo: check to see if 0 (does it matter?)
        ai.stop_exec = true;
        ai.want_syscall = true;
    } else {
        unimplemented!();
    }
}
pub fn hint(ai: &mut Arm64Cpu, arg: &ArmInstr) {
}
pub fn isb(ai: &mut Arm64Cpu, arg: &ArmInstr) {

}
pub fn prfm_imm(ai: &mut Arm64Cpu, arg: &ArmInstr) {
}
pub fn dmb(ai: &mut Arm64Cpu, arg: &ArmInstr) {
    std::sync::atomic::fence(Ordering::SeqCst);
}