use crate::riscv::common::{Exception, Trap, Xlen, RiscvArgs};
use crate::riscv::interpreter::main::{RiscvInt};


pub fn illegal_instr(ri: &mut RiscvInt, args: &RiscvArgs) {
    let current_pc = ri.get_pc_of_current_instr();
    ri.set_trap(Trap {
        ttype: Exception::IllegalInstruction,
        val: current_pc
    });
    ri.stop_exec = true;
}


pub fn nop(ri: &mut RiscvInt, ar: &RiscvArgs) {
    
}
pub fn wfi(ri: &mut RiscvInt, ar: &RiscvArgs) {
    ri.wfi = true;
    ri.stop_exec = true;

}


