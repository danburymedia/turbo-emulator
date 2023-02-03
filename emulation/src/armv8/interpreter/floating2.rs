use num::ToPrimitive;
use simple_soft_float::{F16, F16Traits, F32, F32Traits, F64, F64Traits, RoundingMode};
use crate::armv8::decodedefs::ArmInstr;
use crate::armv8::interpreter::floating::{fp_data_processing_3, handle_fpstate};
use crate::armv8::interpreter::floating_helper::{FloatMode, fp_cvt_2_float_raw, fp_cvt_2_int, FPCompare, imm8_to_fp16, imm8_to_fp32, imm8_to_fp64};
use crate::armv8::interpreter::floating_jumpers::*;
use crate::armv8::interpreter::main::Arm64Cpu;
use crate::armv8::interpreter::vect_helper::VectorReg;
use crate::common::arm_fp_defs::cond_holds;
use crate::common::{BIT32_MASK, BIT64_MASK};
use crate::common::floating_wrappers::{fp_cvt_2_16, fp_cvt_2_32, fp_cvt_2_64, fp_cvt_2_integral};
use crate::common::vect::VectInfo;
pub fn get_float_info(arg: &ArmInstr) -> (FloatMode, usize, VectInfo) {
    let fpsize = arg.get_float_type(); // ftype
    let fpsize_num = match fpsize {
        FloatMode::Half => 16,
        FloatMode::Single => 32,
        FloatMode::Double => 64
    };
    let vinfo = VectInfo {
        lane_count: 1,
        elem_size: fpsize_num
    };
    (fpsize, fpsize_num, vinfo)
}
pub fn fmov_float(ai: &mut Arm64Cpu, arg: &ArmInstr) {
    let (fpsize, fpsize_num, vinfo) = get_float_info(arg);
    let rnval = ai.vreg[arg.get_rn()];
    ai.vreg[arg.get_rd()].vect = match fpsize {
        FloatMode::Half => rnval.vect as u16 as u128,
        FloatMode::Single => rnval.vect as u32 as u128,
        FloatMode::Double => rnval.vect as u64 as u128,
    };

}
pub fn fabs_float(ai: &mut Arm64Cpu, arg: &ArmInstr) {
    let (_, _, vinfo) = get_float_info(arg);
    let rnval = ai.vreg[arg.get_rn()];
    let mut finalval = VectorReg::default();
    fabs(&mut finalval, rnval, vinfo);
    ai.vreg[arg.get_rd()] = finalval;
}
pub fn fneg_float(ai: &mut Arm64Cpu, arg: &ArmInstr) {
    let (_, _, vinfo) = get_float_info(arg);
    let rnval = ai.vreg[arg.get_rn()];
    let mut finalval = VectorReg::default();
    fneg(&mut finalval, rnval, vinfo);
    ai.vreg[arg.get_rd()] = finalval;
}
pub fn fsqrt_float(ai: &mut Arm64Cpu, arg: &ArmInstr) {
    let (_, _, vinfo) = get_float_info(arg);
    let rnval = ai.vreg[arg.get_rn()];
    let rm = ai.get_fpscr_rounding_mode();
    let mut finalval = VectorReg::default();
    fsqrt(ai, &mut finalval, rnval, vinfo, rm);
    ai.vreg[arg.get_rd()] = finalval;
}
pub fn frint_gen(ai: &mut Arm64Cpu, arg: &ArmInstr, round_mode: RoundingMode) {
    let rnval = ai.vreg[arg.get_rn()];
    let fpsize = arg.get_float_type();
    let mut newdst = VectorReg::default();
    let (res, fpstate) = match fpsize {
        FloatMode::Half => fp_cvt_2_integral::<u16, F16Traits>(rnval.vect as u16,
                                                               round_mode),
        FloatMode::Single => fp_cvt_2_integral::<u32, F32Traits>(rnval.vect as u32,
                                                                 round_mode),
        FloatMode::Double => fp_cvt_2_integral::<u64, F64Traits>(rnval.vect as u64,
                                                                 round_mode),
    };
    if handle_fpstate(ai, fpstate) {
        return;
    }
    //if is_exact {
     //   panic!(); // todo: setup fp except
   // }
    newdst.vect = res as u128;
    ai.vreg[arg.get_rd()] = newdst;
}
pub fn fcvt_float(ai: &mut Arm64Cpu, arg: &ArmInstr) {
    let opc = (arg.insn >> 15) & 0b11;
    let fpsize = arg.get_float_type(); // ftype
    let rm = ai.get_fpscr_rounding_mode();
    let rnval = ai.vreg[arg.get_rn()];
    let mut finalval = VectorReg::default();
    match opc {
        0 => {
            // single
            let (res, fpstate) = match fpsize {
                FloatMode::Half => fp_cvt_2_32::<u16, F16Traits>(rnval.vect as u16, rm),
                FloatMode::Single => fp_cvt_2_32::<u32, F32Traits>(rnval.vect as u32, rm),
                FloatMode::Double => fp_cvt_2_32::<u64, F64Traits>(rnval.vect as u64, rm),
            };
            if handle_fpstate(ai, fpstate) {
                return;
            }
            finalval.vect = res as u128;
        },
        1 => {
            // double
            let (res, fpstate) = match fpsize {
                FloatMode::Half => fp_cvt_2_64::<u16, F16Traits>(rnval.vect as u16, rm),
                FloatMode::Single => fp_cvt_2_64::<u32, F32Traits>(rnval.vect as u32, rm),
                FloatMode::Double => fp_cvt_2_64::<u64, F64Traits>(rnval.vect as u64, rm),
            };
            if handle_fpstate(ai, fpstate) {
                return;
            }
            finalval.vect = res as u128;
        },
        3 => {
            // half
            let (res, fpstate) = match fpsize {
                FloatMode::Half => fp_cvt_2_16::<u16, F16Traits>(rnval.vect as u16, rm),
                FloatMode::Single => fp_cvt_2_16::<u32, F32Traits>(rnval.vect as u32, rm),
                FloatMode::Double => fp_cvt_2_16::<u64, F64Traits>(rnval.vect as u64, rm),
            };
            if handle_fpstate(ai, fpstate) {
                return;
            }
            finalval.vect = res as u128;

        },
        _ => panic!()
    }
    ai.vreg[arg.get_rd()] = finalval;
}
pub fn frintn_float(ai: &mut Arm64Cpu, arg: &ArmInstr) {
    frint_gen(ai, arg, RoundingMode::TiesToEven);
}
pub fn frintp_float(ai: &mut Arm64Cpu, arg: &ArmInstr) {
    frint_gen(ai, arg, RoundingMode::TowardPositive);
}
pub fn frintm_float(ai: &mut Arm64Cpu, arg: &ArmInstr) {
    frint_gen(ai, arg, RoundingMode::TowardNegative);
}
pub fn frintz_float(ai: &mut Arm64Cpu, arg: &ArmInstr) {
    frint_gen(ai, arg, RoundingMode::TowardZero);
}
pub fn frinta_float(ai: &mut Arm64Cpu, arg: &ArmInstr) {
    frint_gen(ai, arg, RoundingMode::TiesToAway);
}
pub fn frinti_float(ai: &mut Arm64Cpu, arg: &ArmInstr) {
    let rm = ai.get_fpscr_rounding_mode();
    frint_gen(ai, arg, rm);
}
pub fn frintx_float(ai: &mut Arm64Cpu, arg: &ArmInstr) {
    panic!();
    let rm = ai.get_fpscr_rounding_mode();
    frint_gen(ai, arg, rm);
}
pub fn fmov_float_imm(ai: &mut Arm64Cpu, arg: &ArmInstr) {
    let fpsize = arg.get_float_type();
    let imm8 = (arg.insn >> 13) & 0xff;
    match fpsize {
        FloatMode::Half => {
            ai.vreg[arg.get_rd()].vect = imm8_to_fp16(imm8) as u128;
        }
        FloatMode::Single => {
            ai.vreg[arg.get_rd()].vect = imm8_to_fp32(imm8) as u128;
        }
        FloatMode::Double => {
            ai.vreg[arg.get_rd()].vect = imm8_to_fp64(imm8) as u128;
        }
    }
}
pub fn fcmp_float(ai: &mut Arm64Cpu, arg: &ArmInstr) {
    let op1 = ai.vreg[arg.get_rn()].vect as u64;
    let opc = (arg.insn >> 3) & 0b11;
    let cmp_with_zero = (opc & 1) == 1;
    let sig_nans = (opc & 0b10) == 0b10;
    let fpsize = arg.get_float_type(); // ftype
    let op2 = if cmp_with_zero {
        match arg.get_float_type() {
            FloatMode::Half => F16::positive_zero().bits().to_u64().unwrap(),
            FloatMode::Single => F32::positive_zero().bits().to_u64().unwrap(),
            FloatMode::Double => F64::positive_zero().bits().to_u64().unwrap()
        }
    } else {
        ai.vreg[arg.get_rm()].vect as u64
    };
    let (flags, state) = match fpsize {
        FloatMode::Half => FPCompare::<u16, F16Traits>(op1 as u16, op2 as u16, sig_nans),
        FloatMode::Single => FPCompare::<u32, F32Traits>(op1 as u32, op2 as u32, sig_nans),
        FloatMode::Double => FPCompare::<u64, F64Traits>(op1 as u64, op2 as u64, sig_nans),
    };
    if handle_fpstate(ai, state) {
        return;
    }
    ai.set_flags(flags);
}
pub fn fcmpe_float(ai: &mut Arm64Cpu, arg: &ArmInstr) {
    fcmp_float(ai, arg); // difference will be caught by this function
}
pub fn fccmp_float(ai: &mut Arm64Cpu, arg: &ArmInstr) {
    let signal_nans = ((arg.insn >> 4) & 1) == 1;
    let cond = arg.get_condition();
    let rnval = ai.vreg[arg.get_rn()].vect;
    let rmval = ai.vreg[arg.get_rm()].vect;
    if cond_holds(cond as u8, ai.flag_status) {
        let (flags, fpstate) = match arg.get_float_type() {
            FloatMode::Half => {
                FPCompare::<u16, F16Traits>(rnval as u16, rmval as u16,signal_nans)
            }
            FloatMode::Single => {
                FPCompare::<u32, F32Traits>(rnval as u32, rmval as u32,signal_nans)
            }
            FloatMode::Double => {
                FPCompare::<u64, F64Traits>(rnval as u64, rmval as u64,signal_nans)
            }
        };
        if handle_fpstate(ai, fpstate) {
            return;
        }
        ai.set_flags(flags);
    }
}
pub fn fccmpe_float(ai: &mut Arm64Cpu, arg: &ArmInstr) {
    fccmp_float(ai, arg);
}
#[derive(Copy, Clone)]
pub enum Dp2Op {
    Fmul,
    Fdiv,
    Fadd,
    Fsub,
    Fmax,
    Fmin,
    Fmaxnm,
    Fminnm,
    Fnmul

}
fn floatdp2_gen(ai: &mut Arm64Cpu, arg: &ArmInstr, dp2: Dp2Op) {
    let rnval = ai.vreg[arg.get_rn()];
    let rmval = ai.vreg[arg.get_rm()];
    let (_, _, vinfo) = get_float_info(arg);
    let roundm = ai.get_fpscr_rounding_mode();
    let mut finalval = VectorReg::default();
    match dp2 {
        Dp2Op::Fmul => {
            fmul(ai, &mut finalval, rnval, rmval, vinfo, roundm);
        }
        Dp2Op::Fdiv => {
            fdiv(ai, &mut finalval, rnval, rmval, vinfo, roundm);
        }
        Dp2Op::Fadd => {
            fadd(ai, &mut finalval, rnval, rmval, vinfo, roundm);
        }
        Dp2Op::Fsub => {
            fsub(ai, &mut finalval, rnval, rmval, vinfo, roundm);
        }
        Dp2Op::Fmax => {
            fmax(ai, &mut finalval, rnval, rmval, vinfo, roundm);
        }
        Dp2Op::Fmin => {
            fmin(ai, &mut finalval, rnval, rmval, vinfo);
        }
        Dp2Op::Fmaxnm => {
            fmaxnm(ai, &mut finalval, rnval, rmval, vinfo);
        }
        Dp2Op::Fminnm => {
            fminnm(ai, &mut finalval, rnval, rmval, vinfo);
        }
        Dp2Op::Fnmul => {
            fnmul(ai, &mut finalval, rnval, rmval, vinfo, roundm);
        }
    }
    ai.vreg[arg.get_rd()] = finalval;
}
pub fn fmul_float(ai: &mut Arm64Cpu, arg: &ArmInstr) {
    floatdp2_gen(ai, arg, Dp2Op::Fmul);
}
pub fn fdiv_float(ai: &mut Arm64Cpu, arg: &ArmInstr) {
    floatdp2_gen(ai, arg, Dp2Op::Fdiv);
}
pub fn fadd_float(ai: &mut Arm64Cpu, arg: &ArmInstr) {
    floatdp2_gen(ai, arg, Dp2Op::Fadd);
}
pub fn fsub_float(ai: &mut Arm64Cpu, arg: &ArmInstr) {
    floatdp2_gen(ai, arg, Dp2Op::Fsub);
}
pub fn fmax_float(ai: &mut Arm64Cpu, arg: &ArmInstr) {
    floatdp2_gen(ai, arg, Dp2Op::Fmax);
}
pub fn fmin_float(ai: &mut Arm64Cpu, arg: &ArmInstr) {
    floatdp2_gen(ai, arg, Dp2Op::Fmin);
}
pub fn fmaxnm_float(ai: &mut Arm64Cpu, arg: &ArmInstr) {
    floatdp2_gen(ai, arg, Dp2Op::Fmaxnm);
}
pub fn fminnm_float(ai: &mut Arm64Cpu, arg: &ArmInstr) {
    floatdp2_gen(ai, arg, Dp2Op::Fminnm);
}
pub fn fnmul_float(ai: &mut Arm64Cpu, arg: &ArmInstr) {
    floatdp2_gen(ai, arg, Dp2Op::Fnmul);
}
pub fn fcsel_float(ai: &mut Arm64Cpu, arg: &ArmInstr) {
    let cond = arg.get_condition();
    let selidx = if cond_holds(cond as u8, ai.flag_status) {
        arg.get_rn()
    } else {
        arg.get_rm()
    };
    let rdidx = arg.get_rd();
    ai.vreg[rdidx].einfo = Default::default();
    match arg.get_float_type() {
        FloatMode::Half => {
            ai.vreg[rdidx].vect = ai.vreg[selidx].vect as u16 as u128;
        },
        FloatMode::Single => {
            ai.vreg[rdidx].vect = ai.vreg[selidx].vect as u32 as u128;
        },
        FloatMode::Double => {
            ai.vreg[rdidx].vect = ai.vreg[selidx].vect as u64 as u128;
        }
    };
}
pub fn fmadd_float(ai: &mut Arm64Cpu, arg: &ArmInstr) {
    fp_data_processing_3(ai, arg);
}
pub fn fmsub_float(ai: &mut Arm64Cpu, arg: &ArmInstr) {
    fp_data_processing_3(ai, arg);
}
pub fn fnmadd_float(ai: &mut Arm64Cpu, arg: &ArmInstr) {
    fp_data_processing_3(ai, arg);
}
pub fn fnmsub_float(ai: &mut Arm64Cpu, arg: &ArmInstr) {
    fp_data_processing_3(ai, arg);
}
fn fcvt_gen(ai: &mut Arm64Cpu, arg: &ArmInstr, rm: RoundingMode, signed: bool) {
    let (fpsize, fpsize_num, vinfo) = get_float_info(arg);
    let intsize = if arg.is_64bit_set() { 64 } else { 32 };
    let fltval = ai.vreg[arg.get_rn()].vect;
    let (intval, state) = match fpsize_num {
        16 => fp_cvt_2_int::<u16, F16Traits>(fltval as u16, intsize, signed, rm, 0),
        32 => fp_cvt_2_int::<u32, F32Traits>(fltval as u32, intsize, signed, rm, 0),
        64 => fp_cvt_2_int::<u64, F64Traits>(fltval as u64, intsize, signed, rm, 0),
        _ => panic!()
    };
    if handle_fpstate(ai, state) {
        return;
    }
    ai.set_reg(arg.get_rd(), intval, false);
}
fn fcvt_vector_gen(ai: &mut Arm64Cpu, arg: &ArmInstr, rm: RoundingMode, signed: bool) {
    let is_vector = if ((arg.insn >> 28) & 1) == 0 { true } else { false };
    let is_fp16 = if ((arg.insn >> 19) & 1) != 0 { true } else { false };
    let mut esize = if !is_fp16 {
        if ((arg.insn >> 22) & 1) != 0 {
            64
        }  else {
            32
        }
    } else {
        16
    };
    let is128 = if ((arg.insn >> 30) & 1) != 0 {
        true
    }  else {
        false
    };
    let vinfo = if is_vector {
        if is128 {
            VectInfo::new_128bits(esize)
        } else {
            VectInfo::new_64bits(esize)
        }
    } else {
        VectInfo {
            lane_count: 1,
            elem_size: esize
        }
    };
    let mut newvect = VectorReg::default();
    let fltval = ai.vreg[arg.get_rn()];
    for i in 0..vinfo.lane_count {
        let val = fltval.get_elem_fixed(i, vinfo);
        let (intval, state) = match vinfo.elem_size {
            16 => fp_cvt_2_int::<u16, F16Traits>(val as u16,
                                                 16, signed, rm, 0),
            32 => fp_cvt_2_int::<u32, F32Traits>(val as u32,
                                                 32, signed, rm, 0),
            64 => fp_cvt_2_int::<u64, F64Traits>(val as u64,
                                                 64, signed, rm, 0),
            _ => panic!()

        };
        if handle_fpstate(ai, state) {
            return;
        }
        newvect.set_elem_fixed(intval, i, vinfo);
    }
    ai.vreg[arg.get_rd()] = newvect;
}
fn cvtf_gen(ai: &mut Arm64Cpu, arg: &ArmInstr, rm: RoundingMode, signed: bool) {
    let (fpsize, fpsize_num, vinfo) = get_float_info(arg);
    let intsize = if arg.is_64bit_set() { 64 } else { 32 };
    let intval = ai.get_reg(arg.get_rn(), false);
    let (fval, state) = match fpsize_num {
        16 => fp_cvt_2_float_raw::<u16, F16Traits>(intval, intsize, signed, rm, 0),
        32 => fp_cvt_2_float_raw::<u32, F32Traits>(intval, intsize, signed, rm, 0),
        64 => fp_cvt_2_float_raw::<u64, F64Traits>(intval, intsize, signed, rm, 0),
        _ => panic!()
    };
    if handle_fpstate(ai, state) {
        return;
    }
    ai.vreg[arg.get_rd()].vect = fval as u128;
}
pub fn fcvtns_float(ai: &mut Arm64Cpu, arg: &ArmInstr) {
    fcvt_gen(ai, arg, RoundingMode::TiesToEven, true);
}
pub fn fcvtnu_float(ai: &mut Arm64Cpu, arg: &ArmInstr) {
    fcvt_gen(ai, arg, RoundingMode::TiesToEven, false);
}
pub fn fcvtas_float(ai: &mut Arm64Cpu, arg: &ArmInstr) {
    fcvt_gen(ai, arg, RoundingMode::TiesToAway, true);
}
pub fn fcvtau_float(ai: &mut Arm64Cpu, arg: &ArmInstr) {
    fcvt_gen(ai, arg, RoundingMode::TiesToAway, false);
}
pub fn fcvtps_float(ai: &mut Arm64Cpu, arg: &ArmInstr) {
    fcvt_gen(ai, arg, RoundingMode::TowardPositive, true);
}
pub fn fcvtpu_float(ai: &mut Arm64Cpu, arg: &ArmInstr) {
    fcvt_gen(ai, arg, RoundingMode::TowardPositive, false);
}
pub fn fcvtms_float(ai: &mut Arm64Cpu, arg: &ArmInstr) {
    fcvt_gen(ai, arg, RoundingMode::TowardNegative, true);
}
pub fn fcvtzs_float_int(ai: &mut Arm64Cpu, arg: &ArmInstr) {
    fcvt_gen(ai, arg, RoundingMode::TowardZero, true);
}
pub fn fcvtzu_float_int(ai: &mut Arm64Cpu, arg: &ArmInstr) {
    fcvt_gen(ai, arg, RoundingMode::TowardZero, false);
}
pub fn fcvtmu_float(ai: &mut Arm64Cpu, arg: &ArmInstr) {
    fcvt_gen(ai, arg, RoundingMode::TowardNegative, false);
}
pub fn scvtf_float_int(ai: &mut Arm64Cpu, arg: &ArmInstr) {
    let rm = ai.get_fpscr_rounding_mode();
    cvtf_gen(ai, arg, rm, true);
}
pub fn ucvtf_float_int(ai: &mut Arm64Cpu, arg: &ArmInstr) {
    let rm = ai.get_fpscr_rounding_mode();
    cvtf_gen(ai, arg, rm, false);
}
pub fn fcvtzs_advsimd_int(ai: &mut Arm64Cpu, arg: &ArmInstr) {
    fcvt_vector_gen(ai, arg, RoundingMode::TowardZero, true);
}
pub fn fcvtzu_advsimd_int(ai: &mut Arm64Cpu, arg: &ArmInstr) {
    fcvt_vector_gen(ai, arg, RoundingMode::TowardZero, false);
}
pub fn fmov_float_gen(ai: &mut Arm64Cpu, arg: &ArmInstr) {
    let fpsize_num = (arg.insn >> 22) & 0b11;
    let to_vreg = (arg.insn & (1 << 16)) != 0;
    let part = if (arg.insn & (1 << 19)) != 0 { 1 } else { 0 };
    let size = if arg.is_64bit_set() { 64 } else { 32 };
    let fpsize = match fpsize_num {
        0 => 32,
        1 => 64,
        2 => 128,
        3 => 16,
        _ => panic!()
    };
    if to_vreg {
        let mut intval = ai.get_reg(arg.get_rn(), false);
        if fpsize == 16 {
            intval &= 0xffff;
        } else if fpsize == 32 {
            intval &= BIT32_MASK;
        } else if fpsize == 64 || fpsize == 128 {
            intval &= BIT64_MASK;
        } else {
            panic!();
        }
        let rd = arg.get_rd();
        if part == 0 {
            ai.vreg[rd].vect = intval as u128;
        } else {
            ai.vreg[rd].vect &= (BIT64_MASK as u128);
            ai.vreg[rd].vect |= (intval as u128) << 64;

        }
    } else {
        // let vi = VectInfo::new_128bits(fromsize);
        let rn = arg.get_rn();
        let mut intval = if part == 0 {
            ai.vreg[rn].vect as u64
        } else {
            (ai.vreg[rn].vect >> 64) as u64
        };
        if size == 16 {
            intval &= 0xffff;
        } else if size == 32 {
            intval &= BIT32_MASK;
        } else if size == 64 {
            intval &= BIT64_MASK;
        } else {
            panic!();
        }
       // let fval = ai.vreg[arg.get_rn()].get_elem_fixed(part as usize, vi);
        ai.set_reg(arg.get_rd(), intval, false);
    }
}


