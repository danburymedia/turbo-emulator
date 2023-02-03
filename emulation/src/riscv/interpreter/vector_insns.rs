use crate::riscv::common::{Exception, Trap, Xlen, RiscvArgs};
use crate::riscv::decoder::{arg_atomic, arg_b, arg_i, arg_s};
use crate::riscv::interpreter::main::{RiscvInt};
use crate::riscv::vector::{gen_vi_vi_loop, VectorViVars, vm_rd_bugcheck};
use crate::riscv::vector::vector_gen_load;


fn vsetvl(ri: &mut RiscvInt, args: &RiscvArgs) {
    ri.vector_state.set_vec_len(args.rd, args.rs2,
                                ri.regs[args.rd as usize], ri.regs[args.rs2 as usize]);
    ri.regs[args.rd as usize] = ri.vector_state.vl;

}
fn vsetvli(ri: &mut RiscvInt, args: &RiscvArgs) {
    ri.vector_state.set_vec_len(args.rd, args.rs2,
                                ri.regs[args.rd as usize], args.imm as u64);
    ri.regs[args.rd as usize] = ri.vector_state.vl;

}
fn vsetivli(ri: &mut RiscvInt, args: &RiscvArgs) {
    ri.vector_state.set_vec_len(args.rd, 1,
                                args.rs1 as u64, args.imm as u64);
    // rs1idx is not 0, what matters. In vsetivli the rs1 field has another field
    ri.regs[args.rd as usize] = ri.vector_state.vl;

}
fn vle8_v(ri: &mut RiscvInt, args: &RiscvArgs) {
    vector_gen_load(ri, args, 0, true, 1, false);

}
fn vle16_v(ri: &mut RiscvInt, args: &RiscvArgs) {
    vector_gen_load(ri, args, 0, true, 2, false);

}
fn vle32_v(ri: &mut RiscvInt, args: &RiscvArgs) {
    vector_gen_load(ri, args, 0, true, 4, false);
}
fn vle64_v(ri: &mut RiscvInt, args: &RiscvArgs) {
    vector_gen_load(ri, args, 0, true,8, false);
}