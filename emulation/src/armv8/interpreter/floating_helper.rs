use std::cmp::Ordering;
use simple_soft_float::{Sign, StatusFlags};
use num::{pow, ToPrimitive};
use simple_soft_float::{DynamicFloat, F32, FloatBitsType, FloatTraits, Float, FPState, RoundingMode, F16Traits, F32Traits, F64Traits, F64, F16};
use crate::armv8::interpreter::floating::handle_fpstate;
use crate::armv8::interpreter::main::Arm64Cpu;
use crate::common::arm_fp_defs::*;

#[derive(Copy, Clone)]
pub enum  FloatMode {
    Half,
    Single,
    Double
}
pub fn fp_process_nans_gen<Bits: FloatBitsType + Copy,
    FT: FloatTraits<Bits = Bits> + Default>(ai: &mut Arm64Cpu, af: Float<FT>, bf: Float<FT>, state: &mut FPSR) -> (bool, u64) {
    let mut done = true;
    // order is important, don't cascade
    let result = if af.is_signaling_nan() {
        fp_process_nan_gen(ai, af, state)
    } else if bf.is_signaling_nan() {
        fp_process_nan_gen(ai, bf, state)
    } else if af.is_nan() {
        fp_process_nan_gen(ai, af, state)
    } else if bf.is_nan() {
        fp_process_nan_gen(ai, bf, state)
    } else {
        done = false;
        0
    };
    return (done, result);

}
pub fn fp32_process_NaNs(ai: &mut Arm64Cpu, af: F32, bf: F32, state: &mut FPSR) -> u32 {
    if af.is_signaling_nan() {
        return fp32_process_nan(ai, af, state);
    }
    if bf.is_signaling_nan() {
        return fp32_process_nan(ai, bf, state);
    }
    if af.is_nan() {
        return fp32_process_nan(ai, af, state);
    }
    if bf.is_nan() {
        return fp32_process_nan(ai, bf, state);
    }
    return 0;

}
pub fn fp_nmul<Bits: FloatBitsType + Copy,
    FT: FloatTraits<Bits = Bits> + Default>(a: Bits, b: Bits, rm: RoundingMode) -> (u64, FPState) {
    let af = Float::<FT>::from_bits(a);
    let bf = Float::<FT>::from_bits(b);
    let mut fpstate: FPState = Default::default();
    let mut res = af.mul(&bf, Some(rm), Some(&mut fpstate));
    res = res.neg();
    (res.bits().to_u64().unwrap(), fpstate)
}
pub fn fp_mulx<Bits: FloatBitsType + Copy,
    FT: FloatTraits<Bits = Bits> + Default>(a: Bits, b: Bits, rm: RoundingMode) -> (u64, FPState) {
    let af = Float::<FT>::from_bits(a);
    let bf = Float::<FT>::from_bits(b);
    let mut c2 = Float::<FT>::from_u8(2, None, None);

    let mut fpstate: FPState = Default::default();
    if (af.is_infinity() && bf.is_zero())
        || (bf.is_infinity()  && af.is_zero() ) {
        if (af.sign() == Sign::Negative && bf.sign() != Sign::Negative) || (bf.sign() == Sign::Negative && af.sign() != Sign::Negative) {
            c2 = c2.neg();
        }
        return (c2.bits().to_u64().unwrap(), fpstate);
    }
    let  res = af.mul(&bf, Some(rm), Some(&mut fpstate));
    (res.bits().to_u64().unwrap(), fpstate)
}
pub fn fp_max<Bits: FloatBitsType + Copy,
    FT: FloatTraits<Bits = Bits> + Default>(a: Bits, b: Bits) -> (u64, FPState) {
    let af = Float::<FT>::from_bits(a);
    let bf = Float::<FT>::from_bits(b);
    let mut fpstate: FPState = Default::default();
    let mut res = if af.is_zero() && bf.is_zero() && (af.sign() != bf.sign()) {
        Float::<FT>::positive_zero()
    } else {
        if af.compare_quiet(&bf, Some(&mut fpstate)) == Some(Ordering::Greater) {
            af
        } else {
            bf
        }
    };
    (res.bits().to_u64().unwrap(), fpstate)
}
pub fn fp_min<Bits: FloatBitsType + Copy,
    FT: FloatTraits<Bits = Bits> + Default>(a: Bits, b: Bits) -> (u64, FPState) {
    let af = Float::<FT>::from_bits(a);
    let bf = Float::<FT>::from_bits(b);
    let mut fpstate: FPState = Default::default();
    let mut res = if af.is_zero() && bf.is_zero() && (af.sign() != bf.sign()) {
        Float::<FT>::negative_zero()
    } else {
        if af.compare_quiet(&bf, Some(&mut fpstate)) == Some(Ordering::Less) {
            af
        } else {
            bf
        }
    };
    (res.bits().to_u64().unwrap(), fpstate)
}

pub fn fp_maxnm<Bits: FloatBitsType + Copy,
    FT: FloatTraits<Bits = Bits> + Default>(a: Bits, b: Bits) -> (u64, FPState) {
    let mut af = Float::<FT>::from_bits(a);
    let mut bf = Float::<FT>::from_bits(b);
    let mut fpstate: FPState = Default::default();
    if !(af.is_nan() && bf.is_nan()) {
        if af.is_quiet_nan() == true && bf.is_quiet_nan() != false {
            af = Float::<FT>::negative_infinity();
        } else if af.is_quiet_nan() != true && bf.is_quiet_nan() == false {
            bf = Float::<FT>::negative_infinity();
        }
    }
    let mut res = if af.is_zero() && bf.is_zero() && (af.sign() != bf.sign()) {
        Float::<FT>::positive_zero()
    } else {
        if af.compare_quiet(&bf, Some(&mut fpstate)) == Some(Ordering::Greater) {
            af
        } else {
            bf
        }
    };
    (res.bits().to_u64().unwrap(), fpstate)
}
pub fn fp_minnm<Bits: FloatBitsType + Copy,
    FT: FloatTraits<Bits = Bits> + Default>(a: Bits, b: Bits) -> (u64, FPState) {
    let mut af = Float::<FT>::from_bits(a);
    let mut bf = Float::<FT>::from_bits(b);
    let mut fpstate: FPState = Default::default();
    if !(af.is_nan() && bf.is_nan()) {
        if af.is_quiet_nan() == true && bf.is_quiet_nan() != false {
            af = Float::<FT>::negative_infinity();
        } else if af.is_quiet_nan() != true && bf.is_quiet_nan() == false {
            bf = Float::<FT>::negative_infinity();
        }
    }
    let mut res = if af.is_zero() && bf.is_zero() && (af.sign() != bf.sign()) {
        Float::<FT>::positive_zero()
    } else {
        if af.compare_quiet(&bf, Some(&mut fpstate)) == Some(Ordering::Less) {
            af
        } else {
            bf
        }
    };
    (res.bits().to_u64().unwrap(), fpstate)
}
pub fn fp_sqrt<Bits: FloatBitsType + Copy,
    FT: FloatTraits<Bits = Bits> + Default>(a: Bits, rm: RoundingMode) -> (u64, FPState) {
    let af = Float::<FT>::from_bits(a);
    let mut fpstate: FPState = Default::default();

    let res = af.sqrt(Some(rm), Some(&mut fpstate));
    (res.bits().to_u64().unwrap(), fpstate)
}
pub fn fp_process_nan_gen<Bits: FloatBitsType + Copy,
    FT: FloatTraits<Bits = Bits> + Default>(ai: &mut Arm64Cpu, val: Float<FT>, state: &mut FPSR) -> u64 {
    if val.is_signaling_nan()  {
        state.ioc = true;
        return val.into_quiet_nan().bits().to_u64().unwrap();
    }
    return if ai.get_fpcr_dn_flag() {
        Float::<FT>::quiet_nan().bits().to_u64().unwrap()
    } else {
        val.bits().to_u64().unwrap()
    }
}
pub fn fp32_process_nan(ai: &mut Arm64Cpu, val: F32, state: &mut FPSR) -> u32 {
    if val.is_signaling_nan()  {
        state.ioc = true;
        return val.into_quiet_nan().bits().to_u32().unwrap();
    }
    return  if ai.get_fpcr_dn_flag() {
        F32::quiet_nan().bits().to_u32().unwrap()
    } else {
        val.bits().to_u32().unwrap()
    }
}
// nzcv
pub fn FPCompare<Bits: FloatBitsType + Copy,
    FT: FloatTraits<Bits = Bits> + Default>(a: Bits, b: Bits, sig_nans: bool) -> (Flags, FPState) {
    let af = Float::<FT>::from_bits(a);
    let bf = Float::<FT>::from_bits(b);
    let mut fpstate: FPState = Default::default();
    let mut retflags: Flags = Default::default();
    if af.is_nan() || bf.is_nan() {
        retflags.v = true;
        retflags.c = true;
        if af.is_signaling_nan() || bf.is_signaling_nan() || sig_nans {
            fpstate.status_flags.insert(StatusFlags::INVALID_OPERATION);
        }
        return (retflags, fpstate);

    }
    let cmp =  af.compare(&bf, false, None).unwrap();
    if cmp == Ordering::Equal {
        retflags.c = true;
        retflags.z = true;
    } else if cmp == Ordering::Less {
        retflags.n = true;
    } else {
        retflags.c = true;
    }
    (retflags, fpstate)
}
pub fn fp_cvt_2_int<Bits: FloatBitsType + Copy,
    FT: FloatTraits<Bits = Bits> + Default>(a: Bits, intsize: u64, signed: bool, rm: RoundingMode, fbits: u64) -> (u64, FPState) {
    let mut af = Float::<FT>::from_bits(a);
    let mut fpstate: FPState = Default::default();
    if fbits != 0 {
        let mulend = Float::<FT>::from_u32(pow(2, fbits as usize), None, None);
        af = af.mul(&mulend, None, None);
    }

    let res = if intsize == 64 {
        if signed {
            af.to_i64(false, Some(rm), Some(&mut fpstate)).unwrap() as u64
        } else {
            af.to_u64(false, Some(rm), Some(&mut fpstate)).unwrap()
        }
    } else if intsize == 32 {
        if signed {
            af.to_i32(false, Some(rm), Some(&mut fpstate)).unwrap() as i64 as u64
        } else {
            af.to_u32(false, Some(rm), Some(&mut fpstate)).unwrap() as u64
        }
    } else {
        panic!();
    };
    (res, fpstate)
}
pub fn fp_cvt_2_float_raw<Bits: FloatBitsType + Copy,
    FT: FloatTraits<Bits = Bits> + Default>(val: u64, intsize: u64, signed: bool, rm: RoundingMode, fbits: u64) -> (u64, FPState) {
    let mut fpstate: FPState = Default::default();
    let divend = Float::<FT>::from_u32(pow(2, fbits as usize), None, None);
    let res = if intsize == 32 {
        if signed {
            Float::<FT>::from_i32(val as i32, Some(rm), Some(&mut fpstate))
        } else {
            Float::<FT>::from_u32(val as u32, Some(rm), Some(&mut fpstate))
        }
    } else if intsize == 64 {
        if signed {
            Float::<FT>::from_i64(val as i64, Some(rm), Some(&mut fpstate))
        } else {
            Float::<FT>::from_u64(val, Some(rm), Some(&mut fpstate))
        }
    } else {
        panic!();
    };
    let newres = res.div(&divend, None, None);
    (newres.bits().to_u64().unwrap(), fpstate)
}
pub fn imm8_to_fp16(imm8: u32) -> u16 {
    let bit7 = (imm8 >> 7) & 0x1;
    let bit6 = (imm8 >> 6) & 0x1;
    let bits_5_0 = imm8 & 0x3f;
    let result = ((bit7 << 15) | ((4 - bit6) << 12) | (bits_5_0 << 6)) as u16;
    return result;
}
pub fn imm8_to_fp32(imm8: u32) -> u32 {
    let bit7 = (imm8 >> 7) & 0x1;
    let bit6 = (imm8 >> 6) & 0x1;
    let bits_5_0 = imm8 & 0x3f;
    let result = (bit7 << 31) | ((32 - bit6) << 25) | (bits_5_0 << 19);
    return result;
}
pub fn imm8_to_fp64(imm8: u32) -> u64 {
    let bit7 = ((imm8 >> 7) & 0x1) as u64;
    let bit6 = ((imm8 >> 6) & 0x1) as u64;
    let bits_5_0 = (imm8 & 0x3f) as u64;
    let result = (bit7 << 63) | ((256 - bit6) << 54) | (bits_5_0 << 48);
    return result;
}
pub fn fp_frecps<Bits: FloatBitsType + Copy,
    FT: FloatTraits<Bits = Bits> + Default>(ai: &mut Arm64Cpu, a: Bits, b: Bits, rm: RoundingMode) -> u64 {
    let mut af = Float::<FT>::from_bits(a);
    let bf = Float::<FT>::from_bits(b);
    let mut state: FPSR = FPSR::default();
    af = af.neg();
    let (isnan, val) = fp_process_nans_gen(ai, af.clone(), bf.clone(), &mut state);
    ai.accumlate_fpsr_errors(state);
    if isnan {
        return val;
    }
    if (af.is_infinity() && bf.is_zero()) || (af.is_zero() && bf.is_infinity()) {
        Float::<FT>::from_u32(2, None, None).bits().to_u64().unwrap()
    } else if af.is_infinity() || bf.is_infinity() {
        if af.sign() == bf.sign() {
            Float::<FT>::positive_infinity().bits().to_u64().unwrap()
        } else {
            Float::<FT>::negative_infinity().bits().to_u64().unwrap()
        }
    } else {
        let mut fpstate: FPState = Default::default();
        let twoval = Float::<FT>::from_u32(2, None, None);
        let newval = af.fused_mul_add(&bf, &twoval, Some(rm), Some(&mut fpstate));
        handle_fpstate(ai, fpstate);
        newval.bits().to_u64().unwrap()
    }
}
pub fn fp_frsqrts<Bits: FloatBitsType + Copy,
    FT: FloatTraits<Bits = Bits> + Default>(ai: &mut Arm64Cpu, a: Bits,
                                            b: Bits, rm: RoundingMode) -> u64 {
    let mut af = Float::<FT>::from_bits(a);
    let bf = Float::<FT>::from_bits(b);
    let mut state: FPSR = FPSR::default();
    af = af.neg();
    let (isnan, val) = fp_process_nans_gen(ai, af.clone(), bf.clone(), &mut state);
    ai.accumlate_fpsr_errors(state);
    if isnan {
        return val;
    }
    let onepointfive: f64 = 1.5;
    let rawbits = onepointfive.to_bits();
    let opf_native_f64 = Float::<F64Traits>::from_bits(rawbits);
    let opf_native = Float::<FT>::convert_from_float::<F64Traits>(&opf_native_f64, None, None);

    if (af.is_infinity() && bf.is_zero()) || (af.is_zero() && bf.is_infinity()) {
        opf_native.bits().to_u64().unwrap()
    } else if af.is_infinity() || bf.is_infinity() {
        if af.sign() == bf.sign() {
            Float::<FT>::positive_infinity().bits().to_u64().unwrap()
        } else {
            Float::<FT>::negative_infinity().bits().to_u64().unwrap()
        }
    } else {
        let mut fpstate: FPState = Default::default();
        let twoval = Float::<FT>::from_u32(2, None, None);
        let try1 = af.div(&twoval, None, None);
        let try2 = bf.div(&twoval, None, None);
        // to avoid rounding
        let newval = if try1.is_normal() {
            try1.fused_mul_add(&bf, &opf_native, Some(rm), Some(&mut fpstate))
        } else if try2.is_normal() {
            af.fused_mul_add(&try2, &opf_native, Some(rm), Some(&mut fpstate))
        } else {
            opf_native
        };
        handle_fpstate(ai, fpstate);
        newval.bits().to_u64().unwrap()
    }
}