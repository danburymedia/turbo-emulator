use std::mem;
use num::traits::WrappingAdd;
use jit::extract::sextract32;
use crate::armv8::interpreter::helpers::{add_with_carry, calc_n_flag, calc_z_flag, count_sign_bits, decode_imm_bitmask, extend_value, ExtendType, reverse_bytes, ShiftType};
use crate::armv8::interpreter::main::Arm64Cpu;
use crate::armv8::decodedefs::*;
use crate::common::arm_crypto::{CRC32_POLY, CRC32C_POLY, crc32checksum};
use crate::common::{BIT32_MASK, BIT64_MASK};
use crate::common::arm_fp_defs::cond_holds;

fn ShiftOperand<T: num::PrimInt>(value: T, shift: ShiftType, amt: u64) -> T {
    if amt == 0 {
        return value;
    }
    match shift {
        ShiftType::LSL => {
            value.shl(amt as usize)
        }
        ShiftType::LSR => {
            value.shr(amt as usize)
        }
        ShiftType::ASR => {
            value.signed_shr(amt as u32)
        }
        ShiftType::ROR => {
            value.rotate_right(amt as u32)
        }
        _ => {
            panic!("invalid shift type")
        }
    }
}
#[derive(Copy, Clone)]
pub enum LogicalOps {
    And,
    Orr,
    Eor
}
fn logic_worker<T: num::PrimInt>(ai: &mut Arm64Cpu, op1: T, op2: T, arg: &ArmInstr, lop: LogicalOps,
                                 is_imm: bool, update_flags: bool) {

    // let mut op1 = T::from(ai.get_reg(arg.get_rn(), false)).unwrap();
    let mut result: T = T::zero();
    match lop{
        LogicalOps::And => {
            result = op1 & op2;
        },
        LogicalOps::Orr => {
            result = op1 | op2;
        },
        LogicalOps::Eor => {
            result = op1 ^ op2;
        },
        _ => panic!("unsupported portion")
    }
    if update_flags {
        let n = calc_n_flag(result);
        let z = calc_z_flag(result);
        ai.set_n_flag(n);
        ai.set_z_flag(z);
        ai.set_c_flag(false);
        ai.set_v_flag(false);

    }
    let rd = arg.get_rd();
    let usestack = if is_imm && !update_flags {
         true
    } else {
         false
    };
    ai.set_reg(rd, result.to_u64().unwrap(), usestack);

}
pub fn logical_shift(ai: &mut Arm64Cpu, arg: &ArmInstr, lop: LogicalOps,
                     update_flags: bool, invert: bool) {
    let shifttyraw =  ((arg.insn >> 22) & 0b11) as u8;
    let shtype = ShiftType::num2type(shifttyraw);
    let shift_amt = ((arg.insn >> 10) & 0b111111) as u64;
    let op1 = ai.get_reg(arg.get_rn(), false);
    let rmval = ai.get_reg(arg.get_rm(), false);
    if arg.is_64bit_set() {
        let mut op2 = ShiftOperand(rmval as u64, shtype, shift_amt);
        if invert {
            op2 = !op2;
        }
        logic_worker(ai, op1, op2, arg, lop, false, update_flags);
    } else {
        let mut op2 = ShiftOperand(rmval as u32, shtype, shift_amt);
        if invert {
            op2 = !op2;
        }
        logic_worker(ai, op1 as u32, op2, arg, lop, false, update_flags);
    }
}
pub fn logical_imm(ai: &mut Arm64Cpu, arg: &ArmInstr, lop: LogicalOps,
                  update_flags: bool) {
    let n_bit = if arg.is_bit_set(22) {1} else {0};
    let regsize = if arg.is_64bit_set() { 64 } else { 32 };

    let imms = (arg.insn >> 10) & 0b111111;
    let immr = (arg.insn >> 16) & 0b111111;
    let imm = decode_imm_bitmask(n_bit, imms, immr, regsize);
    let op1 = ai.get_reg(arg.get_rn(), false);

    if arg.is_64bit_set() {
        logic_worker(ai, op1, imm as u64, arg, lop, true, update_flags);
    } else {
        logic_worker(ai, op1 as u32, imm as u32, arg, lop, true, update_flags);
    }
}
fn add_sub_worker<T: num::PrimInt + WrappingAdd>(ai: &mut Arm64Cpu, arg: &ArmInstr, op2: T,
                                   mut stack: bool, is_sub: bool, set_flags: bool) {
    let sz = mem::size_of::<T>();
    let mut new_val: T = T::zero();
    let mut leftreg = ai.get_reg(arg.get_rn(), stack);
    if sz == 4 {
        // 32 bit
        leftreg = leftreg as u32 as u64;
    }
    let leftreg_t: T = T::from(leftreg).unwrap();
    let (res, flags) = if is_sub {
        add_with_carry(leftreg_t, !op2, set_flags, 1)

    } else {
        add_with_carry(leftreg_t, op2, set_flags, 0)
    };
    new_val = res;
    if set_flags {
        ai.set_flags(flags.unwrap());
        stack = false;
    }
    ai.set_reg(arg.get_rd(), new_val.to_u64().unwrap(), stack);

}
fn add_sub_carry_worker<T: num::PrimInt + WrappingAdd>(ai: &mut Arm64Cpu, arg: &ArmInstr,
                                         is_sub: bool, set_flags: bool) {
    let mut op1 = T::from(ai.get_reg(arg.get_rn(), false)).unwrap();
    let mut op2 = T::from(ai.get_reg(arg.get_rm(), false)).unwrap();
    if is_sub {
        // subtracion op bit
        op2 = !op2;
    }
    let carry_bit = if ai.get_c_flag() { 1 } else { 0 };
    let (result, flags) = add_with_carry(op1, op2, set_flags, carry_bit);
    // no stack reg
    if set_flags {
        ai.set_flags(flags.unwrap());
    }
    ai.set_reg(arg.get_rd(), result.to_u64().unwrap(), false);
}
pub fn add_sub_carry(ai: &mut Arm64Cpu, arg: &ArmInstr, is_sub: bool, set_flags: bool) {
    if arg.is_64bit_set() {
        add_sub_carry_worker::<u64>(ai, arg, is_sub, set_flags);
    } else {
        add_sub_carry_worker::<u32>(ai, arg, is_sub, set_flags);
    }
}
pub fn div_helper<T: num::PrimInt, ST: num::PrimInt>(ai: &mut Arm64Cpu, arg: &ArmInstr, is_signed: bool) {
    let mut result: u64 = 0;
    let rnval = ai.get_reg(arg.get_rn(), false);
    let rmval = ai.get_reg(arg.get_rm(), false);

    if is_signed {
        let rn = ST::from(rnval).unwrap();
        let rm = ST::from(rmval).unwrap();
        let val = if (rn == ST::min_value()) && (rm == ST::from(-1).unwrap()) {
            ST::min_value()
        } else if rm == ST::zero() {
            ST::zero()
        } else {
            rn / rm
        };
        result = val.to_u64().unwrap();
    } else {
        let rn = T::from(rnval).unwrap();
        let rm = T::from(rmval).unwrap();
        let val: T = if rm == T::zero() {
            T::zero()
        } else {
            rn / rm
        };
        result = val.to_u64().unwrap();
    }
    ai.set_reg(arg.get_rd(), result, false);
}
pub fn dp_2src_shift_helper<T: num::PrimInt>(ai: &mut Arm64Cpu, arg: &ArmInstr, shifttype: ShiftType) {
    let mut shift = ai.get_reg(arg.get_rm(), false) as u32;
    let size = mem::size_of::<T>();
    if size == 4 {
        // 32 bit
        shift &= 0x1f;
    } else {
        shift &= 0x3f;
    }
    let result = ShiftOperand(T::from(ai.get_reg(arg.get_rn(), false)).unwrap(), shifttype, shift as u64).to_u64().unwrap();
    ai.set_reg(arg.get_rd(), result, false);

}

pub fn add_sub_shifted(ai: &mut Arm64Cpu, arg: &ArmInstr, is_sub: bool, set_flags: bool) {
    let shifttyraw =  ((arg.insn >> 22) & 0b11) as u8;
    let shtype = ShiftType::num2type(shifttyraw);
    let shift_amt = ((arg.insn >> 10) & 0b111111) as u64;
    if arg.is_64bit_set() {
        let mut op2 = ShiftOperand(ai.get_reg(arg.get_rm(), false) as u64, shtype, shift_amt);
        add_sub_worker(ai, arg, op2, false, is_sub, set_flags);
    } else {
        let mut op2 = ShiftOperand(ai.get_reg(arg.get_rm(), false) as u32, shtype, shift_amt);
        add_sub_worker(ai, arg, op2, false, is_sub, set_flags);
    }
}
pub fn add_sub_imm(ai: &mut Arm64Cpu, arg: &ArmInstr, is_sub: bool, set_flags: bool) {
    let mut op2 = ((arg.insn >> 10) & 0xfff) as u64;
    if arg.is_bit_set(22) { // shift bit
        op2 <<= 12;
    }
    if arg.is_64bit_set() {
        add_sub_worker(ai, arg, op2 as u64, true, is_sub, set_flags);
    } else {
        add_sub_worker(ai, arg, op2 as u32, true, is_sub, set_flags);
    }
}
pub fn add_sub_extended(ai: &mut Arm64Cpu, arg: &ArmInstr, is_sub: bool, set_flags: bool) {
    let extraw =  ((arg.insn >> 13) & 0b111) as u8;

    let ext = ExtendType::num2type(extraw);
    let left_shift = ((arg.insn >> 10) & 0b111) as usize;
    if arg.is_64bit_set() {
        let mut op2 = extend_value(ai.get_reg(arg.get_rm(), false) as u64, ext, left_shift);
        add_sub_worker(ai, arg, op2, true, is_sub, set_flags);
    } else {
        let mut op2: u32 =
            extend_value(ai.get_reg(arg.get_rm(), false) as u32 as u64, ext, left_shift) as u32;
        add_sub_worker(ai, arg, op2, true, is_sub, set_flags);
    }
}
#[derive(Copy, Clone)]
pub enum BitfieldOp {
    Sbfm,
    Bfm,
    Ubfm
}
fn rotate_right_cust(src: u64, rot: u64, reg_size: u64) -> u64 {
    if reg_size == 64 {
        src.rotate_right(rot as u32) as u64
    } else if reg_size == 32 {
        ((src as u32).rotate_right(rot as u32)) as u64
    } else {
        panic!();
    }
}
fn bitfield_worker(ai: &mut Arm64Cpu,
                   args: &ArmInstr, bop: BitfieldOp, is_64: bool) {
    let reg_size: u64 = if is_64 { 64 } else { 32 };
    let reg_mask: u64 = if is_64 { BIT64_MASK } else { BIT32_MASK };
    let imms = ((args.insn >> 10) & 0b111111) as i32;
    let immr = ((args.insn >> 16) & 0b111111) as i32;

    let mut diff = imms - immr;
    let mut mask: u64 = 0;
    if diff >= 0 {
        mask = if (diff as u32 as u64) < (reg_size - 1) {
            u64::MAX >> (64 - ((diff as u64) + 1))
        } else {
            reg_mask
        };
    } else {
        mask = u64::MAX >> (64 - ((imms as u64) + 1));
        mask = if is_64 {
            mask.rotate_right(immr as u32)
        } else {
            ((mask as u32).rotate_right(immr as u32)) as u64
        };
        diff += (reg_size as i32);
    }
    let mut inzero: bool = false;
    let mut extend: bool = false;
    match bop {
        BitfieldOp::Bfm => {

        },
        BitfieldOp::Sbfm => {
            inzero = true;
            extend = true;
        }
        BitfieldOp::Ubfm => {
            inzero = true;
        },
        _ => panic!()
    }
    let dst: u64 = if inzero {
        0
    } else {
        if reg_size == 64 {
            ai.get_reg(args.get_rd(), false)
        } else {
            ai.get_reg(args.get_rd(), false) as u32 as u64
        }
    };
    let src = if reg_size == 64 {
        ai.get_reg(args.get_rn(), false)
    } else {
        ai.get_reg(args.get_rn(), false) as u32 as u64
    };
    let mut result = rotate_right_cust(src, immr as u32 as u64, reg_size);
    let topbits: u64 = if diff == 63 {
        0
    } else {
        u64::MAX << ((diff + 1) as u32 as u64)
    };
    let signbits = if extend && (((src >> (imms as u32 as u64) ) & 1) != 0) {
        topbits
    } else {
        0
    };
    result = signbits | (result & mask) | (dst & !mask); // todo: trunc just to be sure
    ai.set_reg(args.get_rd(), result, false);

}
pub fn bitfield(ai: &mut Arm64Cpu, arg: &ArmInstr, bop: BitfieldOp) {
    if arg.is_64bit_set() {
        bitfield_worker(ai, arg, bop, true);
    } else {
        bitfield_worker(ai, arg, bop, false);
    }
}
fn extract_worker<T: num::PrimInt>(ai: &mut Arm64Cpu, args: &ArmInstr) {
    let imms = ((args.insn >> 10) & 0b111_111) as u64;

    let lsb = T::from(imms).unwrap();
    let size = mem::size_of::<T>();
    // let reg_size = (size * 8) as T;
    let op2 = T::from(ai.get_reg(args.get_rm(), false)).unwrap();
    let mut result = if lsb != T::zero() {
        let op1 = ai.get_reg(args.get_rn(), false) as u32;
        op2.rotate_right(op1)
    } else {
        op2
    };
    ai.set_reg(args.get_rd(), result.to_u64().unwrap(), false);
}
pub fn extract(ai: &mut Arm64Cpu, arg: &ArmInstr) {
    let lsb = ((arg.insn >> 10) & 0b111_111) as u64;
    let reg_size = if arg.is_64bit_set() { 64 } else { 32 };
    let low = ai.get_reg(arg.get_rm(), false) >> lsb;
    let high = if lsb == 0 {
        0
    } else {
        ai.get_reg(arg.get_rn(), false) << (reg_size - lsb)
    };
    let mut finalv = low | high;
    if arg.is_64bit_set() {
        //extract_worker::<u64>(ai, arg);
    } else {
        finalv &= BIT32_MASK;
       // extract_worker::<u32>(ai, arg);
    }
    ai.set_reg(arg.get_rd(), finalv, false);

}

#[derive(Copy, Clone)]
pub enum MoveWideOps {
    Movn,
    Movz,
    Movk
}
pub fn move_wide_imm(ai: &mut Arm64Cpu, arg: &ArmInstr, mop: MoveWideOps) {
    let shift = (((arg.insn >> 21) & 0b11) << 4) as u64;
    let imm16 = ((arg.insn >> 5) & 0xffff);
    let shift_imm16 = (imm16 as u64) << shift;
    let mut new_xn_val: u64 = 0;
    let rd = arg.get_rd();
    match mop {
        MoveWideOps::Movn => {
            new_xn_val = !shift_imm16;
            if !arg.is_64bit_set() {
                new_xn_val &= 0xffffffff;
            }
        }
        MoveWideOps::Movz => {
            new_xn_val = shift_imm16;
        }
        MoveWideOps::Movk => {
            let prev_xn_val = if arg.is_64bit_set() {
                ai.get_reg(rd, false)
            } else {
                ai.get_reg(rd, false) & 0xffffffff
            };
            new_xn_val = (prev_xn_val & !((0xffff as u64) << shift)) | shift_imm16;
        }
    }
    ai.set_reg(rd, new_xn_val, false);

}
#[derive(Copy, Clone)]
pub enum CondSelOps {
    Csel,
    Csinc,
    Csinv,
    Csneg
}
pub fn cond_select(ai: &mut Arm64Cpu, arg: &ArmInstr, cop: CondSelOps) {
    let mut new_val = ai.get_reg(arg.get_rn(), false);
    let condraw = (arg.insn >> 12) & 0b1111;
    if !cond_holds(condraw as u8, ai.flag_status) {
        new_val = ai.get_reg(arg.get_rm(), false);
        match cop {
            CondSelOps::Csel => {}
            CondSelOps::Csinc => {
                new_val += 1;
            }
            CondSelOps::Csinv => {
                new_val = !new_val;
            }
            CondSelOps::Csneg => {
                new_val = (!new_val) + 1;
            }
        }
    }
    let rdidx = arg.get_rd();
    if arg.is_64bit_set() {
        ai.set_reg(rdidx, new_val, false);
    } else {
        ai.set_reg(rdidx, new_val & BIT32_MASK, false);
    }
}
fn cond_compare_worker<T: num::PrimInt + WrappingAdd>(ai: &mut Arm64Cpu, args: &ArmInstr, op2: T, is_sub: bool) {
    let op1 = T::from(ai.get_reg(args.get_rn(), false)).unwrap();
    let condraw = (args.insn >> 12) & 0b1111;

    if cond_holds(condraw as u8, ai.flag_status) {
        if is_sub {
            let (_, flags) = add_with_carry(op1, !op2, true, 1);
            ai.set_flags(flags.unwrap());
        } else {
            let (_, flags) = add_with_carry(op1, op2, true, 0);
            ai.set_flags(flags.unwrap());
        }
    } else {
        let flags = args.get_nzcv();
        ai.set_flags(flags);
    }
}
pub fn cond_compare_reg(ai: &mut Arm64Cpu, args: &ArmInstr, is_sub: bool) {
    let rmval = ai.get_reg(args.get_rm(), false);
    if args.is_64bit_set() {
        cond_compare_worker(ai, args, rmval as u64, is_sub);
    } else {
        cond_compare_worker(ai, args, rmval as u32, is_sub);
    }
}
pub fn cond_compare_imm(ai: &mut Arm64Cpu, args: &ArmInstr, is_sub: bool) {
    let imm5 = (args.insn >> 16) & 0b11111;
    if args.is_64bit_set() {
        cond_compare_worker(ai, args, imm5 as u64, is_sub);
    } else {
        cond_compare_worker(ai, args, imm5 as u32, is_sub);
    }
}
pub fn pc_rel_addr(ai: &mut Arm64Cpu, args: &ArmInstr) {
    let page = (args.insn & (1 << 31)) != 0;
    let pc = if page {
        ai.get_pc() & !(0xfff) // clear lower 12 bits
    } else {
        ai.get_pc()
    };
    let immhi = sextract32(args.insn, 5, 19) as i32 as i64 as u64;
    let immlo = ((args.insn >> 29) & 0b11) as u64;
    let fval = ((immhi << 2) | (immlo as u64));
    let imm = if page {
        fval << 12
    } else {
        fval
    };
    let result = pc + imm;
    ai.set_reg(args.get_rd(), result, false);
}