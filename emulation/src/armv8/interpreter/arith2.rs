use jit::extract::sextract32;
use crate::armv8::decodedefs::ArmInstr;
use crate::armv8::interpreter::arith::{add_sub_carry, add_sub_extended, add_sub_imm, add_sub_shifted, bitfield, BitfieldOp, cond_compare_imm, cond_compare_reg, cond_select, CondSelOps, div_helper, dp_2src_shift_helper, extract, logical_imm, logical_shift, LogicalOps, move_wide_imm, MoveWideOps};
use crate::armv8::interpreter::helpers::{reverse_bytes, ShiftType};
use crate::armv8::interpreter::main::Arm64Cpu;
use crate::armv8::interpreter::vector_ops::bit;
use crate::common::arm_crypto::{CRC32_POLY, crc32checksum};

pub fn udiv(ai: &mut Arm64Cpu, arg: &ArmInstr) {
    if arg.is_64bit_set() {
        div_helper::<u64, i64>(ai, arg, false);
    } else {
        div_helper::<u32, i32>(ai, arg, false);

    }
}
pub fn sdiv(ai: &mut Arm64Cpu, arg: &ArmInstr) {
    if arg.is_64bit_set() {
        div_helper::<u64, i64>(ai, arg, true);
    } else {
        div_helper::<u32, i32>(ai, arg, true);

    }
}
pub fn lslv(ai: &mut Arm64Cpu, arg: &ArmInstr) {
    if arg.is_64bit_set() {
        dp_2src_shift_helper::<u64>(ai, arg, ShiftType::LSL);
    } else {
        dp_2src_shift_helper::<u32>(ai, arg, ShiftType::LSL);

    }
}
pub fn lsrv(ai: &mut Arm64Cpu, arg: &ArmInstr) {
    if arg.is_64bit_set() {
        dp_2src_shift_helper::<u64>(ai, arg, ShiftType::LSR);
    } else {
        dp_2src_shift_helper::<u32>(ai, arg, ShiftType::LSR);

    }
}
pub fn asrv(ai: &mut Arm64Cpu, arg: &ArmInstr) {
    if arg.is_64bit_set() {
        dp_2src_shift_helper::<u64>(ai, arg, ShiftType::ASR);
    } else {
        dp_2src_shift_helper::<u32>(ai, arg, ShiftType::ASR);

    }
}
pub fn rorv(ai: &mut Arm64Cpu, arg: &ArmInstr) {
    if arg.is_64bit_set() {
        dp_2src_shift_helper::<u64>(ai, arg, ShiftType::ROR);
    } else {
        dp_2src_shift_helper::<u32>(ai, arg, ShiftType::ROR);

    }
}
pub fn rbit_int(ai: &mut Arm64Cpu, arg: &ArmInstr) {
    let dst = arg.get_rd();
    let src = arg.get_rn();
    let is64set = arg.is_64bit_set();
    let srcval = ai.get_reg(src, false);
    let val = if is64set {
        srcval.reverse_bits()
    } else {
        (srcval as u32).reverse_bits() as u64
    };
    ai.set_reg(dst, val, false);

}
pub fn rev16_int(ai: &mut Arm64Cpu, arg: &ArmInstr) {
    let dst = arg.get_rd();
    let src = arg.get_rn();
    let is64set = arg.is_64bit_set();
    let srcval = ai.get_reg(src, false);
    let val = if is64set {
        reverse_bytes(srcval, 1)
    } else {
        reverse_bytes(srcval as u32, 1) as u64
    };
    ai.set_reg(dst, val, false);

}
pub fn rev_int(ai: &mut Arm64Cpu, arg: &ArmInstr) {
    let dst = arg.get_rd();
    let src = arg.get_rn();
    let is64set = arg.is_64bit_set();
    let srcval = ai.get_reg(src, false);
    let val = if is64set {
        // if sf == 1, then we know cont_size is  64
        srcval.swap_bytes()
        //reverse_bytes(ai.reg[src], 3)
    } else {
        // if sf == 0, then we know cont_size is  32
        (srcval as u32).swap_bytes() as u64
        // reverse_bytes(ai.reg[src] as u32, 2) as u64

    };
    ai.set_reg(dst, val, false);

}
pub fn rev32_int(ai: &mut Arm64Cpu, arg: &ArmInstr) {
    let dst = arg.get_rd();
    let src = arg.get_rn();
    let is64set = arg.is_64bit_set();
    let srcval = ai.get_reg(src, false);
    let val = if is64set {
        // rev32
        reverse_bytes(srcval, 2)
    } else {
        unreachable!();
    };
    ai.set_reg(dst, val, false);

}
pub fn clz_int(ai: &mut Arm64Cpu, arg: &ArmInstr) {
    let dst = arg.get_rd();
    let src = arg.get_rn();
    let is64set = arg.is_64bit_set();
    let srcval = ai.get_reg(src, false);

    let val = if is64set {
        srcval.leading_zeros() as u64
    } else {
        (srcval as u32).leading_zeros() as u64
    };
    ai.set_reg(dst, val, false);
}
pub fn and_log_shift(ai: &mut Arm64Cpu, arg: &ArmInstr) {
    logical_shift(ai, arg, LogicalOps::And, false, false);
}
pub fn bic_log_shift(ai: &mut Arm64Cpu, arg: &ArmInstr) {
    logical_shift(ai, arg, LogicalOps::And, false, true);
}
pub fn orr_log_shift(ai: &mut Arm64Cpu, arg: &ArmInstr) {
    logical_shift(ai, arg, LogicalOps::Orr,  false, false);
}
pub fn orn_log_shift(ai: &mut Arm64Cpu, arg: &ArmInstr) {
    logical_shift(ai, arg, LogicalOps::Orr, false, true);
}
pub fn eor_log_shift(ai: &mut Arm64Cpu, arg: &ArmInstr) {
    logical_shift(ai, arg, LogicalOps::Eor, false, false);
}
pub fn eon(ai: &mut Arm64Cpu, arg: &ArmInstr) {
    // only reg version
    logical_shift(ai, arg, LogicalOps::Eor, false, true);
}
pub fn ands_log_shift(ai: &mut Arm64Cpu, arg: &ArmInstr) {
    logical_shift(ai, arg, LogicalOps::And, true, false);
}
pub fn bics(ai: &mut Arm64Cpu, arg: &ArmInstr) {
    // only reg version
    logical_shift(ai, arg, LogicalOps::And, true, true);
}
pub fn add_addsub_shift(ai: &mut Arm64Cpu, arg: &ArmInstr) {
    add_sub_shifted(ai, arg, false, false);
}
pub fn adds_addsub_shift(ai: &mut Arm64Cpu, arg: &ArmInstr) {
    add_sub_shifted(ai, arg, false, true);
}
pub fn sub_addsub_shift(ai: &mut Arm64Cpu, arg: &ArmInstr) {
    add_sub_shifted(ai, arg, true, false);
}
pub fn subs_addsub_shift(ai: &mut Arm64Cpu, arg: &ArmInstr) {
    add_sub_shifted(ai, arg, true, true);
}
pub fn add_addsub_ext(ai: &mut Arm64Cpu, arg: &ArmInstr) {
    add_sub_extended(ai, arg, false, false);
}
pub fn adds_addsub_ext(ai: &mut Arm64Cpu, arg: &ArmInstr) {
    add_sub_extended(ai, arg, false, true);
}
pub fn sub_addsub_ext(ai: &mut Arm64Cpu, arg: &ArmInstr) {
    add_sub_extended(ai, arg, true, false);
}
pub fn subs_addsub_ext(ai: &mut Arm64Cpu, arg: &ArmInstr) {
    add_sub_extended(ai, arg, true, true);
}
pub fn adc(ai: &mut Arm64Cpu, arg: &ArmInstr) {
    add_sub_carry(ai, arg, false, false);
}
pub fn adcs(ai: &mut Arm64Cpu, arg: &ArmInstr) {
    add_sub_carry(ai, arg, false, true);
}
pub fn sbc(ai: &mut Arm64Cpu, arg: &ArmInstr) {
    add_sub_carry(ai, arg, true, false);
}
pub fn sbcs(ai: &mut Arm64Cpu, arg: &ArmInstr) {
    add_sub_carry(ai, arg, true, true);
}
pub fn ccmn_reg(ai: &mut Arm64Cpu, arg: &ArmInstr) {
    cond_compare_reg(ai, arg, false);
}
pub fn ccmp_reg(ai: &mut Arm64Cpu, arg: &ArmInstr) {
    cond_compare_reg(ai, arg, true);
}
pub fn ccmn_imm(ai: &mut Arm64Cpu, arg: &ArmInstr) {
    cond_compare_imm(ai, arg, false);
}
pub fn ccmp_imm(ai: &mut Arm64Cpu, arg: &ArmInstr) {
    cond_compare_imm(ai, arg, true);
}

pub fn csel(ai: &mut Arm64Cpu, arg: &ArmInstr) {
    cond_select(ai, arg, CondSelOps::Csel);
}
pub fn csinc(ai: &mut Arm64Cpu, arg: &ArmInstr) {
    cond_select(ai, arg, CondSelOps::Csinc);
}
pub fn csinv(ai: &mut Arm64Cpu, arg: &ArmInstr) {
    cond_select(ai, arg, CondSelOps::Csinv);
}
pub fn csneg(ai: &mut Arm64Cpu, arg: &ArmInstr) {
    cond_select(ai, arg, CondSelOps::Csneg);
}
pub fn madd(ai: &mut Arm64Cpu, arg: &ArmInstr) {
    let is64bit = arg.is_64bit_set();
    ai.get_reg(arg.get_rn(), false);
    let rn = arg.get_rn();
    let rm = arg.get_rm();
    let ra = arg.get_ra();
    let (op1, op2, op3) = if is64bit {
        (
            ai.get_reg(rn, false),
            ai.get_reg(rm, false),
            ai.get_reg(ra, false),
        )
    } else {
        (
            ai.get_reg(rn, false) as u32 as u64,
            ai.get_reg(rm, false) as u32 as u64,
            ai.get_reg(ra, false) as u32 as u64,
        )
    };
    // madd
    let mulres = op1.wrapping_mul(op2);
    let mut result = op3.wrapping_add(mulres);
    if !arg.is_64bit_set()  {
        result &= 0xffffffff;
    }
    ai.set_reg(arg.get_rd(), result, false);
}
pub fn msub(ai: &mut Arm64Cpu, arg: &ArmInstr) {
    let is64bit = arg.is_64bit_set();
    let rn = arg.get_rn();
    let rm = arg.get_rm();
    let ra = arg.get_ra();

    let (op1, op2, op3) = if is64bit {
        (
            ai.get_reg(rn, false),
            ai.get_reg(rm, false),
            ai.get_reg(ra, false),
        )
    } else {
        (
            ai.get_reg(rn, false) as u32 as u64,
            ai.get_reg(rm, false) as u32 as u64,
            ai.get_reg(ra, false) as u32 as u64,
        )
    };
    let mulres = op1.wrapping_mul(op2);
    let mut result = op3.wrapping_sub(mulres);
    if !arg.is_64bit_set()  {
        result &= 0xffffffff;
    }
    ai.set_reg(arg.get_rd(), result, false);
}
pub fn smaddl(ai: &mut Arm64Cpu, arg: &ArmInstr) {
    let mulres = (ai.get_reg(arg.get_rn(), false) as i32 as i64).wrapping_mul(ai.get_reg(arg.get_rm(), false) as i32 as i64);
    let mut result = ai.get_reg(arg.get_ra(), false).wrapping_add(mulres as u64);
    if !arg.is_64bit_set()  {
        result &= 0xffffffff;
    }
    ai.set_reg(arg.get_rd(), result, false);
}
pub fn smsubl(ai: &mut Arm64Cpu, arg: &ArmInstr) {
    let mulres = (ai.get_reg(arg.get_rn(), false) as i32 as i64).wrapping_mul(ai.get_reg(arg.get_rm(), false) as i32 as i64);
    let mut result = ai.get_reg(arg.get_ra(), false).wrapping_sub(mulres as u64);
    if !arg.is_64bit_set()  {
        result &= 0xffffffff;
    }
    ai.set_reg(arg.get_rd(), result, false);
}
pub fn smulh(ai: &mut Arm64Cpu, arg: &ArmInstr) {
    let mut result = ((ai.get_reg(arg.get_rn(), false) as i64 as i128).wrapping_mul(ai.get_reg(arg.get_rm(), false) as i64 as i128) >> 64) as u64;
    if !arg.is_64bit_set() {
        result &= 0xffffffff;
    }
    ai.set_reg(arg.get_rd(), result, false);
}
pub fn umaddl(ai: &mut Arm64Cpu, arg: &ArmInstr) {
    let rnval = ai.get_reg(arg.get_rn(), false);
    let rmval = ai.get_reg(arg.get_rm(), false);
    let raval = ai.get_reg(arg.get_ra(), false);
    let mulres = (rnval as u32 as u64).wrapping_mul(rmval as u32 as u64);
    let mut result = raval.wrapping_add(mulres);
    if !arg.is_64bit_set() {
        result &= 0xffffffff;
    }
    ai.set_reg(arg.get_rd(), result, false);
}
pub fn umsubl(ai: &mut Arm64Cpu, arg: &ArmInstr) {
    let rnval = ai.get_reg(arg.get_rn(), false);
    let rmval = ai.get_reg(arg.get_rm(), false);
    let raval = ai.get_reg(arg.get_ra(), false);

    let mulres = (rnval as u32 as u64).wrapping_mul(rmval as u32 as u64);
    let mut result = raval.wrapping_sub(mulres);
    if !arg.is_64bit_set() {
        result &= 0xffffffff;
    }
    ai.set_reg(arg.get_rd(), result, false);
}
pub fn umulh(ai: &mut Arm64Cpu, arg: &ArmInstr) {
    let rnval = ai.get_reg(arg.get_rn(), false);
    let rmval = ai.get_reg(arg.get_rm(), false);

    let mut result = ((rnval as u64 as u128)
        .wrapping_mul(rmval as u64 as u128) >> 64) as u64;
    if !arg.is_64bit_set() {
        result &= 0xffffffff;
    }
    ai.set_reg(arg.get_rd(), result, false);
}
pub fn adr(ai: &mut Arm64Cpu, arg: &ArmInstr) {
    let immhi = sextract32(arg.insn, 5, 19) as i32 as i64 as u64;
    let immlo = ((arg.insn >> 29) & 0b11) as u64;
    let fval = ((immhi << 2) | (immlo as u64));
    let imm = fval;
    let pc = ai.get_pc();
    let result = pc + imm;
    ai.set_reg(arg.get_rd(), result, false);
}
pub fn adrp(ai: &mut Arm64Cpu, arg: &ArmInstr) {
    let immhi = sextract32(arg.insn, 5, 19) as i32 as i64 as u64;
    let immlo = ((arg.insn >> 29) & 0b11) as u64;
    let fval = ((immhi << 2) | (immlo as u64));
    let imm = fval << 12;
    let pc =  ai.get_pc() & !(0xfff); // clear lower 12 bits
    let result = pc.wrapping_add(imm);
    ai.set_reg(arg.get_rd(), result, false);
}

pub fn add_addsub_imm(ai: &mut Arm64Cpu, arg: &ArmInstr) {
    add_sub_imm(ai, arg, false, false);
}
pub fn adds_addsub_imm(ai: &mut Arm64Cpu, arg: &ArmInstr) {
    add_sub_imm(ai, arg, false, true);
}
pub fn sub_addsub_imm(ai: &mut Arm64Cpu, arg: &ArmInstr) {
    add_sub_imm(ai, arg, true, false);
}
pub fn subs_addsub_imm(ai: &mut Arm64Cpu, arg: &ArmInstr) {
    add_sub_imm(ai, arg, true, true);
}
pub fn and_log_imm(ai: &mut Arm64Cpu, arg: &ArmInstr) {
    logical_imm(ai, arg, LogicalOps::And, false);
}
pub fn ands_log_imm(ai: &mut Arm64Cpu, arg: &ArmInstr) {
    logical_imm(ai, arg, LogicalOps::And, true);
}
pub fn orr_log_imm(ai: &mut Arm64Cpu, arg: &ArmInstr) {
    logical_imm(ai, arg, LogicalOps::Orr,  false);
}
pub fn eor_log_imm(ai: &mut Arm64Cpu, arg: &ArmInstr) {
    logical_imm(ai, arg, LogicalOps::Eor, false);
}
pub fn movn(ai: &mut Arm64Cpu, arg: &ArmInstr) {
    move_wide_imm(ai, arg, MoveWideOps::Movn);
}
pub fn movz(ai: &mut Arm64Cpu, arg: &ArmInstr) {
    move_wide_imm(ai, arg, MoveWideOps::Movz);
}
pub fn movk(ai: &mut Arm64Cpu, arg: &ArmInstr) {
    move_wide_imm(ai, arg, MoveWideOps::Movk);
}
pub fn sbfm(ai: &mut Arm64Cpu, arg: &ArmInstr) {
    bitfield(ai, arg, BitfieldOp::Sbfm);
}
pub fn bfm(ai: &mut Arm64Cpu, arg: &ArmInstr) {
    bitfield(ai, arg, BitfieldOp::Bfm);
}
pub fn ubfm(ai: &mut Arm64Cpu, arg: &ArmInstr) {
    bitfield(ai, arg, BitfieldOp::Ubfm);
}
pub fn extr(ai: &mut Arm64Cpu, arg: &ArmInstr) {
    extract(ai, arg);
}




