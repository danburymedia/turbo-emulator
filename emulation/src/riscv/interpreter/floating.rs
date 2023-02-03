use std::cmp::Ordering;
use num::ToPrimitive;
use crate::riscv::common::{Xlen, RiscvArgs};
use crate::riscv::interpreter::main::{ExtensionSearchMode, RiscvInt};
use simple_soft_float::{F32, F32Traits, F64, Float, FloatBitsType, FloatClass, FloatTraits, FPState, RoundingMode, Sign, StatusFlags};
use crate::common::floating_wrappers::*;
use crate::riscv::interpreter::consts::*;
use crate::riscv::interpreter::defs::sign_ext_imm;
use crate::riscv::interpreter::floating_helpers::*;
#[derive(Copy, Clone, PartialEq)]
enum FloatingOps {
    Add,
    Sub,
    Mul,
    Div,
    Fmadd,
    FmaddNeg,
    Fmsub,
    FmsubNeg,
}
#[derive(Copy, Clone, PartialEq)]
pub enum CmpOps {
    Equal,
    Less,
    LessThanEqual
}
pub fn float32_gen_cmp(ri: &mut RiscvInt, args: &RiscvArgs, op: CmpOps) {
    let flt1 = F32::from_bits(read_float32(ri, args.rs1 as usize));
    let flt2 = F32::from_bits(read_float32(ri, args.rs2 as usize));
    let mut state = FPState::default();
    let mut value = 0;
    if op == CmpOps::Less || op == CmpOps::Equal {
        let od = match op {
            CmpOps::Less => {
                Ordering::Less
            }
            CmpOps::LessThanEqual => {
                unreachable!()
            }
            CmpOps::Equal => {
                Ordering::Equal
            }
        };
        let res = F32::compare_quiet(&flt1, &flt2, Some(&mut state));
        if res == Some(od) {
            value = 1;
        }
        fps_2_fflags(ri, state);

    } else {
        // lessthanqeual
        let res = F32::compare_quiet(&flt1, &flt2, Some(&mut state));
        if res == Some(Ordering::Equal) || res == Some(Ordering::Less) {
            value = 1;
        }
        fps_2_fflags(ri, state);

    }
    ri.regs[args.rd as usize] = value;
}
pub fn float64_gen_cmp(ri: &mut RiscvInt, args: &RiscvArgs, op: CmpOps) {
    let flt1 = F64::from_bits(read_float64(ri, args.rs1 as usize));
    let flt2 = F64::from_bits(read_float64(ri, args.rs2 as usize));
    let mut state = FPState::default();
    let mut value = 0;
    if op == CmpOps::Less || op == CmpOps::Equal {
        let od = match op {
            CmpOps::Less => {
                Ordering::Less
            }
            CmpOps::LessThanEqual => {
                unreachable!()
            }
            CmpOps::Equal => {
                Ordering::Equal
            }
        };
        let res = F64::compare_quiet(&flt1, &flt2, Some(&mut state));
        if res == Some(od) {
            value = 1;
        }
        fps_2_fflags(ri, state);

    } else {
        // lessthanqeual
        let res = F64::compare_quiet(&flt1, &flt2, Some(&mut state));
        if res == Some(Ordering::Equal) || res == Some(Ordering::Less) {
            value = 1;
        }
        fps_2_fflags(ri, state);

    }
    ri.regs[args.rd as usize] = value;
}

fn float32_gen_arith(ri: &mut RiscvInt, args: &RiscvArgs, op: FloatingOps) {
    let flt1 = F32::from_bits(read_float32(ri, args.rs1 as usize));
    let flt2 = F32::from_bits(read_float32(ri, args.rs2 as usize));
    let (res, state) = match op {
        FloatingOps::Add => {
            f32_add(flt1, flt2, insn_2_rm_with_csr(ri, args.rm))
        },
        FloatingOps::Sub => {
            f32_sub(flt1, flt2, insn_2_rm_with_csr(ri, args.rm))
        },
        FloatingOps::Mul => {
            f32_mul(flt1, flt2, insn_2_rm_with_csr(ri, args.rm))
        },
        FloatingOps::Div => {
            f32_div(flt1, flt2, insn_2_rm_with_csr(ri, args.rm))
        },
        FloatingOps::Fmadd => {
            let flt3 = read_float32(ri, args.rs3 as usize);
            f32_fused_mul_add(flt1, flt2, F32::from_bits(flt3), insn_2_rm_with_csr(ri, args.rm))
        }
        FloatingOps::FmaddNeg => {
            let flt3 = read_float32(ri, args.rs3 as usize);
            let mut res = f32_fused_mul_add(flt1, flt2, F32::from_bits(flt3), insn_2_rm_with_csr(ri, args.rm));
            res.0 = res.0.neg();
            res
        }
        FloatingOps::Fmsub => {
            let flt3 = F32::from_bits(read_float32(ri, args.rs3 as usize));
            f32_fused_mul_add(flt1, flt2, flt3.neg(), insn_2_rm_with_csr(ri, args.rm))
        }
        FloatingOps::FmsubNeg => {
            let flt3 = F32::from_bits(read_float32(ri, args.rs3 as usize));
            let mut res = f32_fused_mul_add(flt1, flt2, flt3.neg(), insn_2_rm_with_csr(ri, args.rm));
            res.0 = res.0.neg();
            res
        }
    };
    write_float32(ri, res.into_bits(), args.rd as usize);
    fps_2_fflags(ri, state);
}
fn float64_gen_arith(ri: &mut RiscvInt, args: &RiscvArgs, op: FloatingOps) {
    let flt1 = F64::from_bits(read_float64(ri, args.rs1 as usize));
    let flt2 = F64::from_bits(read_float64(ri, args.rs2 as usize));
    let (res, state) = match op {
        FloatingOps::Add => {
            f64_add(flt1, flt2, insn_2_rm_with_csr(ri, args.rm))
        },
        FloatingOps::Sub => {
            f64_sub(flt1, flt2, insn_2_rm_with_csr(ri, args.rm))
        },
        FloatingOps::Mul => {
            f64_mul(flt1, flt2, insn_2_rm_with_csr(ri, args.rm))
        },
        FloatingOps::Div => {
            f64_div(flt1, flt2, insn_2_rm_with_csr(ri, args.rm))
        },
        FloatingOps::Fmadd => {
            let flt3 = read_float64(ri, args.rs3 as usize);
            f64_fused_mul_add(flt1, flt2, F64::from_bits(flt3), insn_2_rm_with_csr(ri, args.rm))
        }
        FloatingOps::FmaddNeg => {
            let flt3 = read_float64(ri, args.rs3 as usize);
            let mut res = f64_fused_mul_add(flt1, flt2, F64::from_bits(flt3), insn_2_rm_with_csr(ri, args.rm));
            res.0 = res.0.neg();
            res
        }
        FloatingOps::Fmsub => {
            let flt3 = F64::from_bits(read_float64(ri, args.rs3 as usize));
            f64_fused_mul_add(flt1, flt2, flt3.neg(), insn_2_rm_with_csr(ri, args.rm))
        }
        FloatingOps::FmsubNeg => {
            let flt3 = F64::from_bits(read_float64(ri, args.rs3 as usize));
            let mut res = f64_fused_mul_add(flt1, flt2, flt3.neg(), insn_2_rm_with_csr(ri, args.rm));
            res.0 = res.0.neg();
            res
        }
    };
    write_float64(ri, res.into_bits(), args.rd as usize);
    fps_2_fflags(ri, state);
}
pub fn feq_s(ri: &mut RiscvInt, args: &RiscvArgs) {
    float32_gen_cmp(ri, args, CmpOps::Equal);
}
pub fn feq_d(ri: &mut RiscvInt, args: &RiscvArgs) {
    float64_gen_cmp(ri, args, CmpOps::Equal);
}

pub fn fle_s(ri: &mut RiscvInt, args: &RiscvArgs) {
    float32_gen_cmp(ri, args, CmpOps::LessThanEqual);
}
pub fn fle_d(ri: &mut RiscvInt, args: &RiscvArgs) {
    float64_gen_cmp(ri, args, CmpOps::LessThanEqual);
}


pub fn flt_s(ri: &mut RiscvInt, args: &RiscvArgs) {
    float32_gen_cmp(ri, args, CmpOps::Less);
}
pub fn flt_d(ri: &mut RiscvInt, args: &RiscvArgs) {
    float64_gen_cmp(ri, args, CmpOps::Less);
}

pub fn fadd_s(ri: &mut RiscvInt, args: &RiscvArgs) {
    float32_gen_arith(ri, args, FloatingOps::Add);

}
pub fn fmadd_s(ri: &mut RiscvInt, args: &RiscvArgs) {
    float32_gen_arith(ri, args, FloatingOps::Fmadd);

}
pub fn fnmadd_s(ri: &mut RiscvInt, args: &RiscvArgs) {
    float32_gen_arith(ri, args, FloatingOps::FmaddNeg);

}
pub fn fmsub_s(ri: &mut RiscvInt, args: &RiscvArgs) {
    float32_gen_arith(ri, args, FloatingOps::Fmsub);

}
pub fn fnmsub_s(ri: &mut RiscvInt, args: &RiscvArgs) {
    float32_gen_arith(ri, args, FloatingOps::FmsubNeg);

}
pub fn fsub_s(ri: &mut RiscvInt, args: &RiscvArgs) {
    float32_gen_arith(ri, args, FloatingOps::Sub);

}
pub fn fmul_s(ri: &mut RiscvInt, args: &RiscvArgs) {
    float32_gen_arith(ri, args, FloatingOps::Mul);

}
pub fn fdiv_s(ri: &mut RiscvInt, args: &RiscvArgs) {
    float32_gen_arith(ri, args, FloatingOps::Div);

}
pub fn fadd_d(ri: &mut RiscvInt, args: &RiscvArgs) {

    let flt1 = read_float64(ri, args.rs1 as usize);
    let flt2 = read_float64(ri, args.rs2 as usize);
    let (res, state) = f64_add(F64::from_bits(flt1), F64::from_bits(flt2), insn_2_rm_with_csr(ri, args.rm));
    write_float64(ri, res.into_bits(), args.rd as usize);
    fps_2_fflags(ri, state);

}
pub fn fmul_d(ri: &mut RiscvInt, args: &RiscvArgs) {
    float64_gen_arith(ri, args, FloatingOps::Mul);
}
pub fn fdiv_d(ri: &mut RiscvInt, args: &RiscvArgs) {
    float64_gen_arith(ri, args, FloatingOps::Div);
}
pub fn flw(ri: &mut RiscvInt, args: &RiscvArgs) {
    let addr = ri.regs[args.rs1 as usize].wrapping_add(sign_ext_imm(args.imm)) as u64;
    let load_value = match ri.read32(addr, false, true) {
        Err(_) => {
            return;
        },
        Ok(d) => d
    };
    write_float32(ri, load_value, args.rd as usize);
}
pub fn fsw(ri: &mut RiscvInt, args: &RiscvArgs) {
    let addr = ri.regs[args.rs1 as usize].wrapping_add(sign_ext_imm(args.imm)) as u64; // this is 0 extendd
    let store_value = read_float32_raw(ri, args.rs2 as usize);
    match ri.write32(addr, store_value, true) {
        Err(_) => {
            return;
        },
        Ok(_) => { }
    };
}
pub fn fsd(ri: &mut RiscvInt, args: &RiscvArgs) {
    let addr = ri.regs[args.rs1 as usize].wrapping_add(sign_ext_imm(args.imm)) as u64;
    let store_value = read_float64_raw(ri, args.rs2 as usize);
    match ri.write64(addr, store_value, true) {
        Err(_) => {
            return;
        },
        Ok(_) => { }
    };
}
pub fn fld(ri: &mut RiscvInt, args: &RiscvArgs) {
    let addr = ri.regs[args.rs1 as usize].wrapping_add(sign_ext_imm(args.imm)) as u64;
    let load_value = match ri.read64(addr, false, true) {
        Err(_) => {
            return;
        },
        Ok(d) => d
    };
    write_float64(ri, load_value, args.rd as usize);
}
pub fn fsgnj_s(ri: &mut RiscvInt, args: &RiscvArgs) {
    let rs1 = read_float32(ri, args.rs1 as usize);
    let rs2 = read_float32(ri, args.rs2 as usize);
    let res = rs1 & ((1 << 31) - 1) | rs2 & (1 << 31);
    write_float32(ri, res, args.rd as usize);
}

pub fn fsgnj_d(ri: &mut RiscvInt, args: &RiscvArgs) {
    let rs1 = read_float64(ri, args.rs1 as usize);
    let rs2 = read_float64(ri, args.rs2 as usize);
    let res = rs1 & ((1 << 63) - 1) | rs2 & (1 << 63);
    write_float64(ri, res, args.rd as usize);
}
pub fn fsgnjn_s(ri: &mut RiscvInt, args: &RiscvArgs) {
    let rs1 = read_float32(ri, args.rs1 as usize);
    let rs2 = read_float32(ri, args.rs2 as usize);
    let res = rs1 & ((1 << 31) - 1) | !rs2 & (1 << 31);
    write_float32(ri, res, args.rd as usize);
}
pub fn fsgnjx_s(ri: &mut RiscvInt, args: &RiscvArgs) {
    let rs1 = read_float32(ri, args.rs1 as usize);
    let rs2 = read_float32(ri, args.rs2 as usize);
    let res = rs1 & ((1 << 31) - 1) | (rs1 ^ rs2) & (1 << 31);
    write_float32(ri, res, args.rd as usize);
}
pub fn fsgnjx_d(ri: &mut RiscvInt, args: &RiscvArgs) {
    let rs1 = read_float64(ri, args.rs1 as usize);
    let rs2 = read_float64(ri, args.rs2 as usize);
    let res = rs1 & ((1 << 63) - 1) | (rs1 ^ rs2) & (1 << 63);
    write_float64(ri, res, args.rd as usize);
}
pub fn fmin_s(ri: &mut RiscvInt, args: &RiscvArgs) {
    let fs1 = F32::from_bits(read_float32(ri, args.rs1 as usize));
    let fs2 = F32::from_bits(read_float32(ri, args.rs2 as usize));
    let (res, state)  = f32_cmp(fs1,fs2, false);
    write_float32(ri, res.into_bits(), args.rd as usize);
    fps_2_fflags(ri, state);
}
pub fn fsqrt_s(ri: &mut RiscvInt, args: &RiscvArgs) {
    let fs1 = F32::from_bits(read_float32(ri, args.rs1 as usize));
    let (res, state)  = f32_sqrt(fs1,insn_2_rm_with_csr(ri, args.rm));
    write_float32(ri, res.into_bits(), args.rd as usize);
    fps_2_fflags(ri, state);
}
pub fn fmax_s(ri: &mut RiscvInt, args: &RiscvArgs) {
    let fs1 = F32::from_bits(read_float32(ri, args.rs1 as usize));
    let fs2 = F32::from_bits(read_float32(ri, args.rs2 as usize));
    let (res, state)  = f32_cmp(fs1,fs2, true);
    write_float32(ri, res.into_bits(), args.rd as usize);
    fps_2_fflags(ri, state);
}
pub fn fcvt_w_s(ri: &mut RiscvInt, args: &RiscvArgs) {
    let mut fpstate: FPState = Default::default();

    let fs1 = F32::from_bits(read_float32(ri, args.rs1 as usize));
    let val: i32 = if let Some(v) = fs1.to_i32(true, insn_2_rm_with_csr(ri, args.rm), Some(&mut fpstate)) {
        v
    } else {
        if fs1.is_nan() || fs1.sign() == Sign::Positive {
            ((1u32 << 31) - 1) as i32
        } else {
            (1u32 << 31) as i32
        }
    };
    ri.regs[args.rd as usize] = val as i32 as i64 as u64;
    fps_2_fflags(ri, fpstate);
}
pub fn fcvt_d_s(ri: &mut RiscvInt, args: &RiscvArgs) {
    let mut fpstate: FPState = Default::default();

    let fs1 = F32::from_bits(read_float32(ri, args.rs1 as usize));
    let f64val = F64::convert_from_float::<F32Traits>(&fs1, insn_2_rm_with_csr(ri, args.rm), Some(&mut fpstate));
    write_float64(ri, f64val.into_bits().to_u64().unwrap(), args.rd as usize);
    fps_2_fflags(ri, fpstate);
}
pub fn fcvt_l_s(ri: &mut RiscvInt, args: &RiscvArgs) {
    let mut fpstate: FPState = Default::default();

    let fs1 = F32::from_bits(read_float32(ri, args.rs1 as usize));
    let val: i64 = if let Some(v) = fs1.to_i64(true, insn_2_rm_with_csr(ri, args.rm), Some(&mut fpstate)) {
        v
    } else {
        if fs1.is_nan() || fs1.sign() == Sign::Positive {
            ((1u64 << 63) - 1) as i64
        } else {
            (1u64 << 63) as i64
        }
    };
    ri.regs[args.rd as usize] = val as u64;
    fps_2_fflags(ri, fpstate);
}
pub fn fcvt_w_d(ri: &mut RiscvInt, args: &RiscvArgs) {
    let mut fpstate: FPState = Default::default();

    let fs1 = F64::from_bits(read_float64(ri, args.rs1 as usize));
    let val: i32 = if let Some(v) = fs1.to_i32(true, insn_2_rm_with_csr(ri, args.rm), Some(&mut fpstate)) {
        v
    } else {
        if fs1.is_nan() || fs1.sign() == Sign::Positive {
            ((1u32 << 31) - 1) as i32
        } else {
            (1u32 << 31) as i32
        }
    };
    ri.regs[args.rd as usize] = ri.cull_reg(val as i64 as u64);
    fps_2_fflags(ri, fpstate);
}
pub fn fcvt_l_d(ri: &mut RiscvInt, args: &RiscvArgs) {
    let mut fpstate: FPState = Default::default();

    let fs1 = F64::from_bits(read_float64(ri, args.rs1 as usize));
    let val: i64 = if let Some(v) = fs1.to_i64(true, insn_2_rm_with_csr(ri, args.rm), Some(&mut fpstate)) {
        v
    } else {
        if fs1.is_nan() || fs1.sign() == Sign::Positive {
            ((1u64 << 63) - 1) as i64
        } else {
            (1u64 << 63) as i64
        }
    };
    ri.regs[args.rd as usize] = val as u64;
    fps_2_fflags(ri, fpstate);
}
pub fn fcvt_wu_s(ri: &mut RiscvInt, args: &RiscvArgs) {
    let mut fpstate: FPState = Default::default();

    let fs1 = F32::from_bits(read_float32(ri, args.rs1 as usize));
    let val: u32 = if let Some(v) = fs1.to_u32(true, insn_2_rm_with_csr(ri, args.rm)
                                               , Some(&mut fpstate)) {
        v
    } else {
        if fs1.is_nan() || fs1.sign() == Sign::Positive {
            -1 as i32 as u32
        } else {
            0
        }
    };
    ri.regs[args.rd as usize] = val as i32 as i64 as u64;
    fps_2_fflags(ri, fpstate);
}
pub fn fmv_x_w(ri: &mut RiscvInt, args: &RiscvArgs) {
    let val = read_float32_raw(ri, args.rs1 as usize) as u64;
    let val_se = val as i32 as i64 as u64;
    ri.regs[args.rd as usize] = val_se;

}
pub fn fmv_x_d(ri: &mut RiscvInt, args: &RiscvArgs) {
    let val = read_float64_raw(ri, args.rs1 as usize) as u64;
    ri.regs[args.rd as usize] = val;

}
pub fn fmv_d_x(ri: &mut RiscvInt, args: &RiscvArgs) {
    write_float64(ri, ri.regs[args.rs1 as usize] as u64, args.rd as usize);
}
pub fn fclass_s(ri: &mut RiscvInt, args: &RiscvArgs) {
    let fs1 = F32::from_bits(read_float32(ri, args.rs1 as usize));
    ri.regs[args.rd as usize] = class_f32(fs1);
}
pub fn fcvt_s_w(ri: &mut RiscvInt, args: &RiscvArgs) {
    let mut fpstate: FPState = Default::default();
    let fs1 = F32::from_i32(ri.regs[args.rs1 as usize] as i32, insn_2_rm_with_csr(ri, args.rm), Some(&mut fpstate));
    write_float32(ri, fs1.into_bits(), args.rd as usize);
    fps_2_fflags(ri, fpstate);
}
pub fn fcvt_s_wu(ri: &mut RiscvInt, args: &RiscvArgs) {
    let mut fpstate: FPState = Default::default();
    let fs1 = F32::from_u32(ri.regs[args.rs1 as usize] as u32, insn_2_rm_with_csr(ri, args.rm), Some(&mut fpstate));
    write_float32(ri, fs1.into_bits(), args.rd as usize);
    fps_2_fflags(ri, fpstate);
}
pub fn fmv_w_x(ri: &mut RiscvInt, args: &RiscvArgs) {
    write_float32(ri, ri.regs[args.rs1 as usize] as u32, args.rd as usize);

}