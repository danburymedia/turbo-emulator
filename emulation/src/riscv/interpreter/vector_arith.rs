use crate::riscv::common::{Exception, Trap, Xlen, RiscvArgs};
use crate::riscv::decoder::{arg_atomic, arg_b, arg_i, arg_s};
use crate::riscv::interpreter::main::{RiscvInt};
use crate::riscv::vector::{gen_vi_vi_loop, VectorViVars, vm_rd_bugcheck};


pub fn vadd_vi(ri: &mut RiscvInt, args: &RiscvArgs) {
    fn vecop(p: &mut VectorViVars) {
        p.vd = p.simm5 + p.vs2;
    }
    gen_vi_vi_loop(ri, args, vecop);
}
pub fn vrsub_vi(ri: &mut RiscvInt, args: &RiscvArgs) {
    fn vecop(p: &mut VectorViVars) {
        p.vd = p.simm5 - p.vs2;
    }
    gen_vi_vi_loop(ri, args, vecop);
}
pub fn vand_vi(ri: &mut RiscvInt, args: &RiscvArgs) {
    fn vecop(p: &mut VectorViVars) {
        p.vd = p.simm5 & p.vs2;
    }
    gen_vi_vi_loop(ri, args, vecop);
}
pub fn vor_vi(ri: &mut RiscvInt, args: &RiscvArgs) {
    fn vecop(p: &mut VectorViVars) {
        p.vd = p.simm5 | p.vs2;
    }
    gen_vi_vi_loop(ri, args, vecop);
}
pub fn vsra_vi(ri: &mut RiscvInt, args: &RiscvArgs) {
    fn vecop(p: &mut VectorViVars) {
        p.vd = p.vs2 >> (p.simm5 & (p.sew) & 0x1f);
    }
    gen_vi_vi_loop(ri, args, vecop);
}
pub fn vssra_vi(ri: &mut RiscvInt, args: &RiscvArgs) {
    panic!(); // todo
    fn vecop(p: &mut VectorViVars) {
        p.vd = p.vs2 >> (p.simm5 & (p.sew) & 0x1f);
    }
    gen_vi_vi_loop(ri, args, vecop);
}
pub fn vsll_vi(ri: &mut RiscvInt, args: &RiscvArgs) {
    fn vecop(p: &mut VectorViVars) {
        p.vd = p.vs2 << (p.simm5 & (p.sew) & 0x1f);
    }
    gen_vi_vi_loop(ri, args, vecop);
}

