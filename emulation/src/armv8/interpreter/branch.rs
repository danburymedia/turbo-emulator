use crate::armv8::decodedefs::ArmInstr;
use crate::armv8::interpreter::main::Arm64Cpu;
use crate::common::arm_common::BranchType;
use crate::common::arm_fp_defs::cond_holds;

pub fn addr_top(ai: &mut Arm64Cpu, address: u64, is_instr: bool, el: u8) -> u64 {
    63 // for now
}
pub fn branch_addr(ai: &mut Arm64Cpu, vaddr: u64, el: u8) -> u64 {
    let msb = addr_top(ai, vaddr, true, el);
    assert_eq!(msb, 63); // for now
    vaddr
}
pub fn branch_to(ai: &mut Arm64Cpu, target: u64, btype: BranchType, branch_cond: bool) {
    let el = ai.get_el_level();
    let realaddr = branch_addr(ai, target, el);
    ai.stop_exec = true;
    ai.want_pc = Some(realaddr);

}
pub fn b_uncond(ai: &mut Arm64Cpu, args: &ArmInstr) {
    let offset = args.get_branch_imm26();
    let pc = ai.get_pc();
    branch_to(ai, pc.wrapping_add(offset), BranchType::DIR, false);
}

pub fn b_cond(ai: &mut Arm64Cpu, args: &ArmInstr) {
    let cond = (args.insn & 0xf);
    if cond_holds(cond as u8, ai.flag_status) {
        let pc = ai.get_pc();
        let offset = args.get_branch_imm19();
        branch_to(ai, pc.wrapping_add(offset), BranchType::DIR, true);
    }

}
pub fn bl(ai: &mut Arm64Cpu, args: &ArmInstr) {
    let offset = args.get_branch_imm26();
    let pc = ai.get_pc();
    ai.set_reg(30, pc + 4, false);
    branch_to(ai, pc.wrapping_add(offset), BranchType::DIRCALL, false);
}
pub fn cbz(ai: &mut Arm64Cpu, args: &ArmInstr) {
    let offset = args.get_branch_imm19();
    let t = args.get_rt();
    let val = if args.is_64bit_set() {
        ai.get_reg(t, false)
    } else {
        ai.get_reg(t, false) as u32 as u64
    };
    if val == 0 {
        let pc = ai.get_pc();
        branch_to(ai, pc.wrapping_add(offset), BranchType::DIR, true);
    }
}
pub fn cbnz(ai: &mut Arm64Cpu, args: &ArmInstr) {
    let offset = args.get_branch_imm19();
    let t = args.get_rt();
    let val = if args.is_64bit_set() {
        ai.get_reg(t, false)
    } else {
        ai.get_reg(t, false) as u32 as u64
    };
    if val != 0 {
        let pc = ai.get_pc();
        branch_to(ai, pc.wrapping_add(offset), BranchType::DIR, true);
    }
}
pub fn tbz(ai: &mut Arm64Cpu, args: &ArmInstr) {
    let offset = args.get_branch_imm14();
    let t = args.get_rt();
    let val = if args.is_64bit_set() {
        ai.get_reg(t, false)
    } else {
        ai.get_reg(t, false) as u32 as u64
    };
    let b40 = (args.insn >> 19) & 0x1f;
    let b5 = (args.insn >> 31) & 1;
    let bpos = b40 | (b5 << 5);

    if (val & (1 << bpos)) == 0 {
        let pc = ai.get_pc();
        branch_to(ai, pc.wrapping_add(offset), BranchType::DIR, true);
    }
}

pub fn tbnz(ai: &mut Arm64Cpu, args: &ArmInstr) {
    let offset = args.get_branch_imm14();
    let t = args.get_rt();
    let val = if args.is_64bit_set() {
        ai.get_reg(t, false)
    } else {
        ai.get_reg(t, false) as u32 as u64
    };
    let b40 = (args.insn >> 19) & 0x1f;
    let b5 = (args.insn >> 31) & 1;
    let bpos = b40 | (b5 << 5);

    if (val & (1 << bpos)) != 0 {
        let pc = ai.get_pc();
        branch_to(ai, pc.wrapping_add(offset), BranchType::DIR, true);
    }
}
fn b_reg_gen_nopac(ai: &mut Arm64Cpu, args: &ArmInstr, btype: BranchType) {
    let mut rn = args.get_rn();
    let target = ai.get_reg(rn, false);
    if btype == BranchType::INDCALL {
        let pc = ai.get_pc();
        ai.set_reg(30, pc + 4, false);
    }
    branch_to(ai, target, btype, false);
}
pub fn br(ai: &mut Arm64Cpu, args: &ArmInstr) {
    b_reg_gen_nopac(ai, args, BranchType::INDIR);
}
pub fn blr(ai: &mut Arm64Cpu, args: &ArmInstr) {
    b_reg_gen_nopac(ai, args, BranchType::INDCALL);
}
pub fn ret(ai: &mut Arm64Cpu, args: &ArmInstr) {
    b_reg_gen_nopac(ai, args, BranchType::RET);
}
/*

fn b_reg_gen(ai: &mut Arm64Cpu, args: &ArmInstr,
                z: bool, a: bool, m: bool, btype: BranchType) {
  let z = ((args.insn >> 24) & 1) != 0;
   let a = ((args.insn >> 11) & 1) != 0;
   let m = ((args.insn >> 10) & 1) != 0;
   let rn = args.get_rn();


    let mut rn = args.get_rn();

    let mut use_stack = if z {
        true
    } else {
        false
    };
    let use_pac = a;
    if use_pac {
        unimplemented!();
    }
    let target = ai.get_reg(rn, use_stack);
    if btype == BranchType::INDCALL {
        let pc = ai.get_pc();
        ai.set_reg(30, pc + 4, false);
    }

    let pc = ai.get_pc();
    ai.set_reg(30, pc + 4, false);
    branch_to(ai, pc.wrapping_add(offset), BranchType::DIRCALL, false);
}

 */