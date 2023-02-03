use std::cmp::Ordering;
use simple_soft_float::{DynamicFloat, F16Traits, F16WithPlatformPropertiesTraits, F32, F32Traits, F64, F64Traits, Float, FloatBitsType, FloatTraits, FPState, PlatformProperties, RoundingMode, StatusFlags};
use crate::armv8::decodedefs::*;
use crate::armv8::interpreter::floating::{gen_fp_proc_nans, handle_fpstate};
use crate::armv8::interpreter::floating_helper::*;
use crate::armv8::interpreter::main::Arm64Cpu;
use crate::common::vect::*;
use crate::armv8::interpreter::vect_helper::VectorReg;
use crate::common::floating_wrappers::{fp_abs, fp_add, fp_add_new, fp_cvt_2_16, fp_cvt_2_32, fp_cvt_2_64, fp_div_arm, fp_mul, fp_neg, fp_sub};

pub fn fadd(ai: &mut Arm64Cpu, dst: &mut VectorReg, src1: VectorReg, src2: VectorReg, vinfo: VectInfo, rm: RoundingMode) {
    dst.clear_vect();
    let fpsize = vinfo.elem_size; // 16, 32, or 64
    for i in 0..vinfo.lane_count {
        let mut result: u64 = 0;
        let op1 = src1.get_elem_fixed(i, vinfo);
        let op2 = src2.get_elem_fixed(i, vinfo);
        let (done, nres) = gen_fp_proc_nans(ai, op1, op2, fpsize);
        if done {
            result = nres;
        } else {
            let (res, fpstate) = if fpsize == 16 {
                fp_add_new::<u16, F16Traits>(op1 as u16, op2 as u16, rm)
            } else if fpsize == 32 {
                fp_add_new::<u32, F32Traits>(op1 as u32, op2 as u32, rm)
            } else if fpsize == 64 {
                fp_add_new::<u64, F64Traits>(op1 as u64, op2 as u64, rm)
            } else {
                panic!();
            };
            if handle_fpstate(ai, fpstate) {
                return;
            }
            result = res;
        }
        dst.set_elem_fixed(result, i, vinfo);
    }
}
#[derive(Copy, Clone)]
pub enum FPCmpTypes {
    Eq,
    Ge,
    Gt,
    Le,
    Lt,
    Ne,
    Uo
}
pub fn fcm_(ai: &mut Arm64Cpu, dst: &mut VectorReg, src1: VectorReg, src2: VectorReg, vinfo: VectInfo, cnd: FPCmpTypes) {
    dst.clear_vect();
    let fpsize = vinfo.elem_size; // 16, 32, or 64
    for i in 0..vinfo.lane_count {
        let op1 = src1.get_elem_fixed(i, vinfo);
        let op2 = src2.get_elem_fixed(i, vinfo);
        let (flags, state) = match fpsize {
            16 => FPCompare::<u16, F16Traits>(op1 as u16, op2 as u16, false),
            32 => FPCompare::<u32, F32Traits>(op1 as u32, op2 as u32, false),
            64 => FPCompare::<u64, F64Traits>(op1 as u64, op2 as u64, false),
            _ => panic!()
        };
        let condpass = match cnd {
            FPCmpTypes::Eq => {
                flags.c == true && flags.z == true
            }
            FPCmpTypes::Ge => {
                flags.c == true
            }
            FPCmpTypes::Gt => {
                flags.c == true && flags.z == false

            }
            FPCmpTypes::Le => {
                flags.n == true || (flags.c == true && flags.z == true)
            }
            FPCmpTypes::Lt => {
                flags.n == true
            }
            FPCmpTypes::Ne => {
                flags.z == false
            }
            FPCmpTypes::Uo => {
                panic!();
            }
        };
        let finval = if condpass {
            vinfo.get_max()
        } else {
            0
        };
        dst.set_elem_fixed(finval, i, vinfo);

    }
}
pub fn frecps(ai: &mut Arm64Cpu, dst: &mut VectorReg, src1: VectorReg, src2: VectorReg, vinfo: VectInfo, rm: RoundingMode) {
    dst.clear_vect();
    let fpsize = vinfo.elem_size; // 16, 32, or 64
    for i in 0..vinfo.lane_count {
        let mut result: u64 = 0;
        let op1 = src1.get_elem_fixed(i, vinfo);
        let op2 = src2.get_elem_fixed(i, vinfo);
        let res = if fpsize == 16 {
            fp_frecps::<u16, F16Traits>(ai, op1 as u16, op2 as u16, rm)
        } else if fpsize == 32 {
            fp_frecps::<u32, F32Traits>(ai, op1 as u32, op2 as u32, rm)
        } else if fpsize == 64 {
            fp_frecps::<u64, F64Traits>(ai, op1 as u64, op2 as u64, rm)
        } else {
            panic!();
        };
        result = res;
        dst.set_elem_fixed(result, i, vinfo);
    }
}
pub fn frsqrts(ai: &mut Arm64Cpu, dst: &mut VectorReg, src1: VectorReg, src2: VectorReg, vinfo: VectInfo, rm: RoundingMode) {
    dst.clear_vect();
    let fpsize = vinfo.elem_size; // 16, 32, or 64
    for i in 0..vinfo.lane_count {
        let mut result: u64 = 0;
        let op1 = src1.get_elem_fixed(i, vinfo);
        let op2 = src2.get_elem_fixed(i, vinfo);
        let res = if fpsize == 16 {
            fp_frsqrts::<u16, F16Traits>(ai, op1 as u16, op2 as u16, rm)
        } else if fpsize == 32 {
            fp_frsqrts::<u32, F32Traits>(ai, op1 as u32, op2 as u32, rm)
        } else if fpsize == 64 {
            fp_frsqrts::<u64, F64Traits>(ai, op1 as u64, op2 as u64, rm)
        } else {
            panic!();
        };
        result = res;
        dst.set_elem_fixed(result, i, vinfo);
    }
}
pub fn fsub(ai: &mut Arm64Cpu, dst: &mut VectorReg, src1: VectorReg, src2: VectorReg, vinfo: VectInfo, rm: RoundingMode) {
    dst.clear_vect();
    let fpsize = vinfo.elem_size; // 16, 32, or 64
    for i in 0..vinfo.lane_count {
        let mut result: u64 = 0;
        let op1 = src1.get_elem_fixed(i, vinfo);
        let op2 = src2.get_elem_fixed(i, vinfo);
        let (done, nres) = gen_fp_proc_nans(ai, op1, op2, fpsize);
        if done {
            result = nres;
        } else {
            let (res, fpstate) = if fpsize == 16 {
                fp_sub::<u16, F16Traits>(op1 as u16, op2 as u16, rm)
            } else if fpsize == 32 {
                fp_sub::<u32, F32Traits>(op1 as u32, op2 as u32, rm)
            } else if fpsize == 64 {
                fp_sub::<u64, F64Traits>(op1 as u64, op2 as u64, rm)
            } else {
                panic!();
            };
            if handle_fpstate(ai, fpstate) {
                return;
            }
            result = res;
        }
        dst.set_elem_fixed(result, i, vinfo);
    }
}
pub fn fabd(ai: &mut Arm64Cpu, dst: &mut VectorReg, src1: VectorReg, src2: VectorReg, vinfo: VectInfo, rm: RoundingMode) {
    let mut temp: VectorReg = VectorReg::default();
    fsub(ai, &mut temp, src1, src2, vinfo, rm);
    fabs(dst, temp, vinfo);
}
pub fn fmul(ai: &mut Arm64Cpu, dst: &mut VectorReg, src1: VectorReg, src2: VectorReg, vinfo: VectInfo, rm: RoundingMode) {
    dst.clear_vect();
    let fpsize = vinfo.elem_size; // 16, 32, or 64
    for i in 0..vinfo.lane_count {
        let mut result: u64 = 0;
        let op1 = src1.get_elem_fixed(i, vinfo);
        let op2 = src2.get_elem_fixed(i, vinfo);
        let (done, nres) = gen_fp_proc_nans(ai, op1, op2, fpsize);
        if done {
            result = nres;
        } else {
            let (res, fpstate) = if fpsize == 16 {
                fp_mul::<u16, F16Traits>(op1 as u16, op2 as u16, rm)
            } else if fpsize == 32 {
                fp_mul::<u32, F32Traits>(op1 as u32, op2 as u32, rm)
            } else if fpsize == 64 {
                fp_mul::<u64, F64Traits>(op1 as u64, op2 as u64, rm)
            } else {
                panic!();
            };
            if handle_fpstate(ai, fpstate) {
                return;
            }
            result = res;
        }
        dst.set_elem_fixed(result, i, vinfo);
    }
}
pub fn fmulx(ai: &mut Arm64Cpu, dst: &mut VectorReg, src1: VectorReg, src2: VectorReg, vinfo: VectInfo, rm: RoundingMode) {
    dst.clear_vect();
    let fpsize = vinfo.elem_size; // 16, 32, or 64
    for i in 0..vinfo.lane_count {
        let mut result: u64 = 0;
        let op1 = src1.get_elem_fixed(i, vinfo);
        let op2 = src2.get_elem_fixed(i, vinfo);
        let (done, nres) = gen_fp_proc_nans(ai, op1, op2, fpsize);
        if done {
            result = nres;
        } else {
            let (res, fpstate) = if fpsize == 16 {
                fp_mulx::<u16, F16Traits>(op1 as u16, op2 as u16, rm)
            } else if fpsize == 32 {
                fp_mulx::<u32, F32Traits>(op1 as u32, op2 as u32, rm)
            } else if fpsize == 64 {
                fp_mulx::<u64, F64Traits>(op1 as u64, op2 as u64, rm)
            } else {
                panic!();
            };
            if handle_fpstate(ai, fpstate) {
                return;
            }
            result = res;
        }
        dst.set_elem_fixed(result, i, vinfo);
    }
}
pub fn fdiv(ai: &mut Arm64Cpu, dst: &mut VectorReg, src1: VectorReg, src2: VectorReg, vinfo: VectInfo, rm: RoundingMode) {
    dst.clear_vect();
    let fpsize = vinfo.elem_size; // 16, 32, or 64
    for i in 0..vinfo.lane_count {
        let mut result: u64 = 0;
        let op1 = src1.get_elem_fixed(i, vinfo);
        let op2 = src2.get_elem_fixed(i, vinfo);
        let (done, nres) = gen_fp_proc_nans(ai, op1, op2, fpsize);
        if done {
            result = nres;
        } else {
            let (res, fpstate) = if fpsize == 16 {
                fp_div_arm::<u16, F16Traits>(op1 as u16, op2 as u16, rm)
            } else if fpsize == 32 {
                fp_div_arm::<u32, F32Traits>(op1 as u32, op2 as u32, rm)
            } else if fpsize == 64 {
                fp_div_arm::<u64, F64Traits>(op1 as u64, op2 as u64, rm)
            } else {
                panic!();
            };
            if handle_fpstate(ai, fpstate) {
                return;
            }
            result = res;
        }
        dst.set_elem_fixed(result, i, vinfo);
    }
}
pub fn fmax(ai: &mut Arm64Cpu, dst: &mut VectorReg, src1: VectorReg, src2: VectorReg, vinfo: VectInfo, rm: RoundingMode) {
    dst.clear_vect();
    let fpsize = vinfo.elem_size; // 16, 32, or 64
    for i in 0..vinfo.lane_count {
        let mut result: u64 = 0;
        let op1 = src1.get_elem_fixed(i, vinfo);
        let op2 = src2.get_elem_fixed(i, vinfo);
        let (res, fpstate) = if fpsize == 16 {
            fp_max::<u16, F16Traits>(op1 as u16, op2 as u16)
        } else if fpsize == 32 {
            fp_max::<u32, F32Traits>(op1 as u32, op2 as u32)
        } else if fpsize == 64 {
            fp_max::<u64, F64Traits>(op1 as u64, op2 as u64)
        } else {
            panic!();
        };
        if handle_fpstate(ai, fpstate) {
            return;
        }
        result = res;
        dst.set_elem_fixed(result, i, vinfo);
    }
}
pub fn fmin(ai: &mut Arm64Cpu, dst: &mut VectorReg, src1: VectorReg, src2: VectorReg, vinfo: VectInfo) {
    dst.clear_vect();
    let fpsize = vinfo.elem_size; // 16, 32, or 64
    for i in 0..vinfo.lane_count {
        let mut result: u64 = 0;
        let op1 = src1.get_elem_fixed(i, vinfo);
        let op2 = src2.get_elem_fixed(i, vinfo);
        let (res, fpstate) = if fpsize == 16 {
            fp_min::<u16, F16Traits>(op1 as u16, op2 as u16)
        } else if fpsize == 32 {
            fp_min::<u32, F32Traits>(op1 as u32, op2 as u32)
        } else if fpsize == 64 {
            fp_min::<u64, F64Traits>(op1 as u64, op2 as u64)
        } else {
            panic!();
        };
        if handle_fpstate(ai, fpstate) {
            return;
        }
        result = res;
        dst.set_elem_fixed(result, i, vinfo);
    }
}
pub fn fminnm(ai: &mut Arm64Cpu, dst: &mut VectorReg, src1: VectorReg, src2: VectorReg, vinfo: VectInfo) {
    dst.clear_vect();
    let fpsize = vinfo.elem_size; // 16, 32, or 64
    for i in 0..vinfo.lane_count {
        let mut result: u64 = 0;
        let op1 = src1.get_elem_fixed(i, vinfo);
        let op2 = src2.get_elem_fixed(i, vinfo);
        let (res, fpstate) = if fpsize == 16 {
            fp_minnm::<u16, F16Traits>(op1 as u16, op2 as u16)
        } else if fpsize == 32 {
            fp_minnm::<u32, F32Traits>(op1 as u32, op2 as u32)
        } else if fpsize == 64 {
            fp_minnm::<u64, F64Traits>(op1 as u64, op2 as u64)
        } else {
            panic!();
        };
        if handle_fpstate(ai, fpstate) {
            return;
        }
        result = res;
        dst.set_elem_fixed(result, i, vinfo);
    }
}
pub fn fmaxnm(ai: &mut Arm64Cpu, dst: &mut VectorReg, src1: VectorReg, src2: VectorReg, vinfo: VectInfo) {
    dst.clear_vect();
    let fpsize = vinfo.elem_size; // 16, 32, or 64
    for i in 0..vinfo.lane_count {
        let mut result: u64 = 0;
        let op1 = src1.get_elem_fixed(i, vinfo);
        let op2 = src2.get_elem_fixed(i, vinfo);
        let (res, fpstate) = if fpsize == 16 {
            fp_maxnm::<u16, F16Traits>(op1 as u16, op2 as u16)
        } else if fpsize == 32 {
            fp_maxnm::<u32, F32Traits>(op1 as u32, op2 as u32)
        } else if fpsize == 64 {
            fp_maxnm::<u64, F64Traits>(op1 as u64, op2 as u64)
        } else {
            panic!();
        };
        if handle_fpstate(ai, fpstate) {
            return;
        }
        result = res;
        dst.set_elem_fixed(result, i, vinfo);
    }
}
pub fn fnmul(ai: &mut Arm64Cpu, dst: &mut VectorReg, src1: VectorReg, src2: VectorReg, vinfo: VectInfo, rm: RoundingMode) {
    dst.clear_vect();
    let fpsize = vinfo.elem_size; // 16, 32, or 64
    for i in 0..vinfo.lane_count {
        let mut result: u64 = 0;
        let op1 = src1.get_elem_fixed(i, vinfo);
        let op2 = src2.get_elem_fixed(i, vinfo);
        let (done, nres) = gen_fp_proc_nans(ai, op1, op2, fpsize);
        if done {
            result = nres;
        } else {
            let (res, fpstate) = if fpsize == 16 {
                fp_nmul::<u16, F16Traits>(op1 as u16, op2 as u16, rm)
            } else if fpsize == 32 {
                fp_nmul::<u32, F32Traits>(op1 as u32, op2 as u32, rm)
            } else if fpsize == 64 {
                fp_nmul::<u64, F64Traits>(op1 as u64, op2 as u64, rm)
            } else {
                panic!();
            };
            if handle_fpstate(ai, fpstate) {
                return;
            }
            result = res;
        }
        dst.set_elem_fixed(result, i, vinfo);
    }
}
pub fn fabscmp(ai: &mut Arm64Cpu, dst: &mut VectorReg, src1: VectorReg, src2: VectorReg, vinfo: VectInfo, cnd: FPCmpTypes) {
    let mut temp1: VectorReg = VectorReg::default();
    let mut temp2: VectorReg = VectorReg::default();
    fabs(&mut temp1, src1, vinfo);
    fabs(&mut temp2, src2, vinfo);
    fcm_(ai, dst, temp1, temp2, vinfo, cnd);
}
pub fn fabs(dst: &mut VectorReg, src1: VectorReg, vinfo: VectInfo) {
    dst.clear_vect();
    let fpsize = vinfo.elem_size; // 16, 32, or 64
    for i in 0..vinfo.lane_count {
        let op1 = src1.get_elem_fixed(i, vinfo);
        let result = if fpsize == 16 {
            fp_abs::<u16, F16Traits>(op1 as u16)
        } else if fpsize == 32 {
            fp_abs::<u32, F32Traits>(op1 as u32)
        } else if fpsize == 64 {
            fp_abs::<u64, F64Traits>(op1 as u64)
        } else {
            panic!();
        };
        dst.set_elem_fixed(result, i, vinfo);
    }
}
pub fn fneg(dst: &mut VectorReg, src1: VectorReg, vinfo: VectInfo) {
    dst.clear_vect();
    let fpsize = vinfo.elem_size; // 16, 32, or 64
    for i in 0..vinfo.lane_count {
        let op1 = src1.get_elem_fixed(i, vinfo);
        let result = if fpsize == 16 {
            fp_neg::<u16, F16Traits>(op1 as u16)
        } else if fpsize == 32 {
            fp_neg::<u32, F32Traits>(op1 as u32)
        } else if fpsize == 64 {
            fp_neg::<u64, F64Traits>(op1 as u64)
        } else {
            panic!();
        };
        dst.set_elem_fixed(result, i, vinfo);
    }
}
pub fn fsqrt(ai: &mut Arm64Cpu,dst: &mut VectorReg, src1: VectorReg, vinfo: VectInfo, rm: RoundingMode) {
    dst.clear_vect();
    let fpsize = vinfo.elem_size; // 16, 32, or 64
    for i in 0..vinfo.lane_count {
        let op1 = src1.get_elem_fixed(i, vinfo);
        let (result, fpstate) = if fpsize == 16 {
            fp_sqrt::<u16, F16Traits>(op1 as u16, rm)
        } else if fpsize == 32 {
            fp_sqrt::<u32, F32Traits>(op1 as u32, rm )
        } else if fpsize == 64 {
            fp_sqrt::<u64, F64Traits>(op1 as u64, rm)
        } else {
            panic!();
        };
        if handle_fpstate(ai, fpstate) {
            return;
        }
        dst.set_elem_fixed(result, i, vinfo);
    }
}
/*
pub fn fp_fcvti_simple(ai: &mut Arm64Cpu, dst: &mut VectorReg, src: VectorReg, vinfo: VectInfo, signed: bool, fbits: u64, rm: RoundingMode) {
    dst.clear_vect();
    let fpsize = vinfo.elem_size;
    for i in 0..vinfo.lane_count {
        let op1 = src.get_elem_fixed(i, vinfo);
        let (intval, state) = match fpsize {
            16 => fp_cvt_2_int::<u16, F16Traits>(op1 as u16, fpsize as u64, signed, rm, fbits),
            32 => fp_cvt_2_int::<u32, F32Traits>(op1 as u32, fpsize as u64, signed, rm, fbits),
            64 => fp_cvt_2_int::<u64, F64Traits>(op1 as u64, fpsize as u64, signed, rm, fbits),
            _ => panic!()
        };
        if handle_fpstate(ai, state) {
            return;
        }
        dst.set_elem_fixed(intval, i, vinfo);
    }
}

 */
/*
pub fn fp_icvtf_simple(ai: &mut Arm64Cpu, dst: &mut VectorReg, src: VectorReg, vinfo: VectInfo, signed: bool, fbits: u64, rm: RoundingMode) {
    dst.clear_vect();
    let fpsize = vinfo.elem_size;
    for i in 0..vinfo.lane_count {
        let op1 = src.get_elem_fixed(i, vinfo);
        let (intval, state) = match fpsize {
            16 => fp_cvt_2_float_raw::<u16, F16Traits>(op1, fpsize as u64, signed, rm, fbits),
            32 => fp_cvt_2_float_raw::<u32, F32Traits>(op1, fpsize as u64, signed, rm, fbits),
            64 => fp_cvt_2_float_raw::<u64, F64Traits>(op1, fpsize as u64, signed, rm, fbits),
            _ => panic!()
        };
        if handle_fpstate(ai, state) {
            return;
        }
        dst.set_elem_fixed(intval, i, vinfo);
    }
}

 */
/* pub fn frint(ai: &mut Arm64Cpu, dst: &mut VectorReg, src1: VectorReg,
             vinfo: VectInfo, rm: RoundingMode, ) {
    dst.clear_vect();
    let fpsize = vinfo.elem_size; // 16, 32, or 64
    for i in 0..vinfo.lane_count {
        let op1 = src1.get_elem_fixed(i, vinfo);
        let (res, fpstate) = match fpsize {
            16 => fp_cvt_2_16::<u16, F16Traits>(rnval.vect as u16, rm),
            32 => fp_cvt_2_32::<u32, F32Traits>(rnval.vect as u32, rm),
            64 => fp_cvt_2_64::<u64, F64Traits>(rnval.vect as u64, rm),
        };
        if handle_fpstate(ai, fpstate) {
            return;
        }
        dst.set_elem_fixed(res, i, vinfo);
    }
}
 */