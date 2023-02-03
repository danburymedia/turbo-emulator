use std::cmp::Ordering;
use num::ToPrimitive;
use simple_soft_float::{F16Traits, F32, F32Traits, F64, F64Traits, Float, FloatBitsType, FloatTraits, FPState, RoundingMode, StatusFlags};

pub fn f32_add(f1: F32, f2: F32, rm: Option<RoundingMode>) -> (F32, FPState) {
    let mut fpstate: FPState = Default::default();
    let res = f1.add(&f2, rm, Some(&mut fpstate));
    (res, fpstate)

}
pub fn f64_add(f1: F64, f2: F64, rm: Option<RoundingMode>) -> (F64, FPState) {
    let mut fpstate: FPState = Default::default();
    let res = f1.add(&f2, rm, Some(&mut fpstate));
    (res, fpstate)

}
pub fn fp_gen_op<Bits: FloatBitsType + Copy, FT: FloatTraits<Bits = Bits> + Default>(f1: Bits, f2: Bits, f3: Bits) -> (u64, FPState) {
    let fp1 = Float::<FT>::from_bits(f1);
    let fp2 = Float::<FT>::from_bits(f2);
    let fp3 = Float::<FT>::from_bits(f3);
    let (val, state) = fp_add(fp1, fp2, None);
    (val.bits().to_u64().unwrap(), state)
}
pub fn fp_add<Bits: FloatBitsType + Copy, FT: FloatTraits<Bits = Bits> + Default>(f1: Float<FT>, f2: Float<FT>, rm: Option<RoundingMode>) -> (Float<FT>, FPState) {
    let mut fpstate: FPState = Default::default();
    let res = f1.add(&f2, rm, Some(&mut fpstate));
    (res, fpstate)

}
pub fn fp_sub_typed<Bits: FloatBitsType + Copy, FT: FloatTraits<Bits = Bits> + Default>(f1: Float<FT>, f2: Float<FT>, rm: Option<RoundingMode>) -> (Float<FT>, FPState) {
    let mut fpstate: FPState = Default::default();
    let res = f1.sub(&f2, rm, Some(&mut fpstate));
    (res, fpstate)

}
pub fn f32_sub(f1: F32, f2: F32, rm: Option<RoundingMode>) -> (F32, FPState) {
    let mut fpstate: FPState = Default::default();
    let res = f1.sub(&f2, rm, Some(&mut fpstate));
    (res, fpstate)

}
pub fn f64_sub(f1: F64, f2: F64, rm: Option<RoundingMode>) -> (F64, FPState) {
    let mut fpstate: FPState = Default::default();
    let res = f1.sub(&f2, rm, Some(&mut fpstate));
    (res, fpstate)

}
pub fn f32_mul(f1: F32, f2: F32, rm: Option<RoundingMode>) -> (F32, FPState) {
    let mut fpstate: FPState = Default::default();
    let res = f1.mul(&f2, rm, Some(&mut fpstate));
    (res, fpstate)

}
pub fn f64_mul(f1: F64, f2: F64, rm: Option<RoundingMode>) -> (F64, FPState) {
    let mut fpstate: FPState = Default::default();
    let res = f1.mul(&f2, rm, Some(&mut fpstate));
    (res, fpstate)

}
pub fn f32_div(f1: F32, f2: F32, rm: Option<RoundingMode>) -> (F32, FPState) {
    let mut fpstate: FPState = Default::default();
    let res = f1.div(&f2, rm, Some(&mut fpstate));
    (res, fpstate)

}
pub fn f64_div(f1: F64, f2: F64, rm: Option<RoundingMode>) -> (F64, FPState) {
    let mut fpstate: FPState = Default::default();
    let res = f1.div(&f2, rm, Some(&mut fpstate));
    (res, fpstate)

}
pub fn f32_fused_mul_add(f1: F32, f2: F32, f3: F32, rm: Option<RoundingMode>) -> (F32, FPState) {
    let mut fpstate: FPState = Default::default();
    let res = f1.fused_mul_add(&f2, &f3, rm, Some(&mut fpstate));
    (res, fpstate)
}
pub fn f64_fused_mul_add(f1: F64, f2: F64, f3: F64, rm: Option<RoundingMode>) -> (F64, FPState) {
    let mut fpstate: FPState = Default::default();
    let res = f1.fused_mul_add(&f2, &f3, rm, Some(&mut fpstate));
    (res, fpstate)
}
pub fn f32_cmp(f1: F32, f2: F32, is_max: bool) -> (F32, FPState) {
    let omode = if is_max {
        Ordering::Greater
    } else {
        Ordering::Less
    };
    let mut fpstate: FPState = Default::default();
    let finval = if f1.is_nan() && f2.is_nan() {
        F32::quiet_nan()
    } else if (f1.is_negative_zero() && f2.is_zero() && !is_max) || (f1.is_positive_zero() && f2.is_zero() && is_max){
        f1
    } else {
        if f1.compare_quiet(&f2, Some(&mut fpstate)) == Some(omode) {
            f1
        } else {
            f2
        }
    };
    (finval, fpstate)

}

pub fn f32_sqrt(f1: F32, rm: Option<RoundingMode>) -> (F32, FPState) {
    let mut fpstate: FPState = Default::default();
    let res = f1.sqrt( rm, Some(&mut fpstate));
    (res, fpstate)

}
pub fn f64_sqrt(f1: F64, rm: Option<RoundingMode>) -> (F64, FPState) {
    let mut fpstate: FPState = Default::default();
    let res = f1.sqrt( rm, Some(&mut fpstate));
    (res, fpstate)

}
pub fn f32_rsqrt(f1: F32, rm: Option<RoundingMode>) -> (F32, FPState) {
    let mut fpstate: FPState = Default::default();
    let res = f1.rsqrt( rm, Some(&mut fpstate));
    (res, fpstate)

}
pub fn f64_rsqrt(f1: F64, rm: Option<RoundingMode>) -> (F64, FPState) {
    let mut fpstate: FPState = Default::default();
    let res = f1.rsqrt( rm, Some(&mut fpstate));
    (res, fpstate)

}
pub fn f64_cmp(f1: F64, f2: F64, is_max: bool) -> (F64, FPState) {
    let omode = if is_max {
        Ordering::Greater
    } else {
        Ordering::Less
    };
    let mut fpstate: FPState = Default::default();
    let finval = if f1.is_nan() && f2.is_nan() {
        F64::quiet_nan()
    } else if (f1.is_negative_zero() && f2.is_zero() && !is_max) || (f1.is_positive_zero() && f2.is_zero() && is_max){
        f1
    } else {
        if f1.compare_quiet(&f2, Some(&mut fpstate)) == Some(omode) {
            f1
        } else {
            f2
        }
    };
    (finval, fpstate)

}
pub fn fp_abs<Bits: FloatBitsType + Copy,
    FT: FloatTraits<Bits = Bits> + Default>(a: Bits) -> u64 {
    let af = Float::<FT>::from_bits(a);
    let res = af.abs();
    res.bits().to_u64().unwrap()
}
pub fn fp_neg<Bits: FloatBitsType + Copy,
    FT: FloatTraits<Bits = Bits> + Default>(a: Bits) -> u64 {
    let af = Float::<FT>::from_bits(a);
    let res = af.neg();
    res.bits().to_u64().unwrap()
}
pub fn fp_muladd<Bits: FloatBitsType + Copy,
    FT: FloatTraits<Bits = Bits> + Default>(addend: Bits, op1: Bits, op2: Bits, a_neg: bool, o1_neg: bool, o2_neg: bool, rm: RoundingMode) -> (u64, FPState) {
    let mut addf = Float::<FT>::from_bits(addend);
    let mut op1f = Float::<FT>::from_bits(op1);
    let mut op2f = Float::<FT>::from_bits(op2);
    if a_neg {
        addf = addf.neg();
    }
    if o1_neg {
        op1f = op1f.neg();
    }
    if o2_neg {
        op2f = op2f.neg();
    }
    let mut fpstate: FPState = Default::default();
    let mut res = op1f.fused_mul_add(&op2f, &addf, Some(rm), Some(&mut fpstate));
    (res.bits().to_u64().unwrap(), fpstate)
}
pub fn fp_mul_native<Bits: FloatBitsType + Copy,
    FT: FloatTraits<Bits = Bits> + Default>(af: Float<FT>, bf: Float<FT>, rm: RoundingMode)
    -> (u64, FPState) {
    let mut fpstate: FPState = Default::default();
    let mut res = af.mul(&bf, Some(rm), Some(&mut fpstate));
    (res.bits().to_u64().unwrap(), fpstate)
}
pub fn fp_mul<Bits: FloatBitsType + Copy,
    FT: FloatTraits<Bits = Bits> + Default>(a: Bits, b: Bits, rm: RoundingMode) -> (u64, FPState) {
    let af = Float::<FT>::from_bits(a);
    let bf = Float::<FT>::from_bits(b);
    let mut fpstate: FPState = Default::default();
    let mut res = af.mul(&bf, Some(rm), Some(&mut fpstate));
    (res.bits().to_u64().unwrap(), fpstate)
}
pub fn fp_div_arm<Bits: FloatBitsType + Copy,
    FT: FloatTraits<Bits = Bits> + Default>(a: Bits, b: Bits, rm: RoundingMode) -> (u64, FPState) {
    let af = Float::<FT>::from_bits(a);
    let bf = Float::<FT>::from_bits(b);
    let mut fpstate: FPState = Default::default();
    if (af.is_infinity() && bf.is_infinity()) || (af.is_zero() && bf.is_zero()) {
        fpstate.status_flags.insert(StatusFlags::INVALID_OPERATION);
        return (Float::<FT>::quiet_nan().bits().to_u64().unwrap(), fpstate);
    }
    let mut res = af.div(&bf, Some(rm), Some(&mut fpstate));
    (res.bits().to_u64().unwrap(), fpstate)
}
pub fn fp_sub<Bits: FloatBitsType + Copy,
    FT: FloatTraits<Bits = Bits> + Default>(a: Bits, b: Bits, rm: RoundingMode) -> (u64, FPState) {
    let af = Float::<FT>::from_bits(a);
    let bf = Float::<FT>::from_bits(b);
    let mut fpstate: FPState = Default::default();
    let mut res = af.sub(&bf, Some(rm), Some(&mut fpstate));
    (res.bits().to_u64().unwrap(), fpstate)
}
pub fn fp_add_new<Bits: FloatBitsType + Copy,
    FT: FloatTraits<Bits = Bits> + Default>(a: Bits, b: Bits, rm: RoundingMode) -> (u64, FPState) {
    let af = Float::<FT>::from_bits(a);
    let bf = Float::<FT>::from_bits(b);
    let mut fpstate: FPState = Default::default();
    let mut res = af.add(&bf, Some(rm), Some(&mut fpstate));
    (res.bits().to_u64().unwrap(), fpstate)
}
pub fn fp_to_u32<Bits: FloatBitsType + Copy,
    FT: FloatTraits<Bits = Bits> + Default>(a: Bits, rm: RoundingMode) -> (u64, FPState) {
    let af = Float::<FT>::from_bits(a);
    let mut fpstate: FPState = Default::default();
    let mut res = af.to_u32(false, Some(rm),
                            Some(&mut fpstate)).unwrap() as u32;
    (res as u64, fpstate)
}
pub fn fp_to_i32<Bits: FloatBitsType + Copy,
    FT: FloatTraits<Bits = Bits> + Default>(a: Bits, rm: RoundingMode) -> (u64, FPState) {
    let af = Float::<FT>::from_bits(a);
    let mut fpstate: FPState = Default::default();
    let mut res = af.to_i32(false, Some(rm),
                            Some(&mut fpstate)).unwrap();
    (res as u64, fpstate)
}
pub fn fp_from_i32<Bits: FloatBitsType + Copy,
    FT: FloatTraits<Bits = Bits> + Default>(a: i32, rm: RoundingMode) -> (u64, FPState) {
    let mut fpstate: FPState = Default::default();
    let af = Float::<FT>::from_i32(a, Some(rm),
                                   Some(&mut fpstate));
    (af.bits().to_u64().unwrap(), fpstate)
}
pub fn fp_from_u32<Bits: FloatBitsType + Copy,
    FT: FloatTraits<Bits = Bits> + Default>(a: u32, rm: RoundingMode) -> (u64, FPState) {
    let mut fpstate: FPState = Default::default();
    let af = Float::<FT>::from_u32(a, Some(rm),
                                   Some(&mut fpstate));
    (af.bits().to_u64().unwrap(), fpstate)
}
pub fn fp_cvt_2_16<Bits: FloatBitsType + Copy,
    FT: FloatTraits<Bits = Bits> + Default>(a: Bits, rm: RoundingMode) -> (u64, FPState) {
    let af = Float::<FT>::from_bits(a);
    let mut fpstate: FPState = Default::default();
    let mut res = af.convert_to_float::<F16Traits>(Some(rm), Some(&mut fpstate));
    (res.bits().to_u64().unwrap(), fpstate)
}
pub fn fp_cvt_2_32<Bits: FloatBitsType + Copy,
    FT: FloatTraits<Bits = Bits> + Default>(a: Bits, rm: RoundingMode) -> (u64, FPState) {
    let af = Float::<FT>::from_bits(a);
    let mut fpstate: FPState = Default::default();
    let mut res =
        af.convert_to_float::<F32Traits>(Some(rm), Some(&mut fpstate));
    (res.bits().to_u64().unwrap(), fpstate)
}
pub fn fp_cvt_2_64<Bits: FloatBitsType + Copy,
    FT: FloatTraits<Bits = Bits> + Default>(a: Bits, rm: RoundingMode) -> (u64, FPState) {
    let af = Float::<FT>::from_bits(a);
    let mut fpstate: FPState = Default::default();
    let mut res = af.convert_to_float::<F64Traits>(Some(rm), Some(&mut fpstate));
    (res.bits().to_u64().unwrap(), fpstate)
}
pub fn fp_cvt_2_integral<Bits: FloatBitsType + Copy,
    FT: FloatTraits<Bits = Bits> + Default>(a: Bits, rm: RoundingMode) -> (u64, FPState) {
    let af = Float::<FT>::from_bits(a);
    let mut fpstate: FPState = Default::default();
    let mut res =
        af.round_to_integral(false, Some(rm), Some(&mut fpstate));
    (res.bits().to_u64().unwrap(), fpstate)
}