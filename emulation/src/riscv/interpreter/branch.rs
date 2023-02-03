use crate::riscv::common::{Xlen,RiscvArgs};
use crate::riscv::interpreter::defs::sign_ext_imm;
use crate::riscv::interpreter::main::{RiscvInt};

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum BranchOps {
    Beq,
    Bne,
    Blt,
    Bge,
}

fn gen_branch(ri: &mut RiscvInt, args: &RiscvArgs, btype: BranchOps, is_unsigned: bool) {
    let cond: bool = match btype {
        BranchOps::Beq => {
            ri.regs[args.rs1 as usize] == ri.regs[args.rs2 as usize]
        }
        BranchOps::Bne => {
            ri.regs[args.rs1 as usize] != ri.regs[args.rs2 as usize]
        }
        BranchOps::Blt => {
            if is_unsigned {
                ri.cull_reg(ri.regs[args.rs1 as usize]) < ri.cull_reg(ri.regs[args.rs2 as usize])

            } else {
                (ri.sign_ext(ri.regs[args.rs1 as usize]) as i64) < (ri.sign_ext(ri.regs[args.rs2 as usize]) as i64)

            }
        }
        BranchOps::Bge => {
            if is_unsigned {
                ri.cull_reg(ri.regs[args.rs1 as usize]) >= ri.cull_reg(ri.regs[args.rs2 as usize])

            } else {
                (ri.sign_ext(ri.regs[args.rs1 as usize]) as i64) >= (ri.sign_ext(ri.regs[args.rs2 as usize]) as i64)

            }
        }
    };
    if cond {
        ri.want_pc = Some(ri.get_pc_of_current_instr().wrapping_add(sign_ext_imm(args.imm)));
        ri.changed_pc = true;
        ri.stop_exec = true;
        // if stacked ops want to stop, the vec would have run out. We just report change here, if it goes on if no branch is somehwere else decision

    }

}
pub fn beq(ri: &mut RiscvInt, args: &RiscvArgs) {
    gen_branch(ri, args, BranchOps::Beq, false);
}
pub fn bne(ri: &mut RiscvInt, args: &RiscvArgs) {
    gen_branch(ri, args, BranchOps::Bne, false);

}
pub fn blt(ri: &mut RiscvInt, args: &RiscvArgs)  {
    gen_branch(ri, args, BranchOps::Blt, false);

}
pub fn bge(ri: &mut RiscvInt, args: &RiscvArgs) {
    gen_branch(ri, args, BranchOps::Bge, false);

}
pub fn bltu(ri: &mut RiscvInt, args: &RiscvArgs)  {
    gen_branch(ri, args, BranchOps::Blt, true);

}
pub fn bgeu(ri: &mut RiscvInt, args: &RiscvArgs)  {
    gen_branch(ri, args, BranchOps::Bge, true);
}
pub fn auipc(ri: &mut RiscvInt, args: &RiscvArgs) {
    let pc = ri.get_pc_of_current_instr();
    let finalval =  pc.wrapping_add(sign_ext_imm(args.imm));
    ri.regs[args.rd as usize] = ri.sign_ext(finalval);
}
pub fn jal(ri: &mut RiscvInt, args: &RiscvArgs) {
    let newpc = ri.get_pc_of_next_instr();
    let newpc_sext = ri.sign_ext(newpc);
    ri.regs[args.rd as usize] = newpc_sext;
    ri.want_pc = Some(ri.get_pc_of_current_instr().wrapping_add(sign_ext_imm(args.imm)));
    ri.stop_exec = true;

}
pub fn jalr(ri: &mut RiscvInt, args: &RiscvArgs) {
    let newpc = ri.get_pc_of_next_instr();
    let newpc_sext = ri.sign_ext(newpc);
    ri.want_pc = Some(ri.regs[args.rs1 as usize].wrapping_add(sign_ext_imm(args.imm)));
    ri.regs[args.rd as usize] = newpc_sext;
    ri.stop_exec = true;

}