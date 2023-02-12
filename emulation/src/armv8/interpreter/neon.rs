use std::mem;
use num::pow;
use crate::armv8::decodedefs::ArmInstr;
use crate::armv8::interpreter::floating_jumpers::{fabd, fabscmp, fcm_, fmulx, FPCmpTypes, frecps, frsqrts};
use crate::armv8::interpreter::helpers::replicate;
use crate::armv8::interpreter::main::Arm64Cpu;
use crate::armv8::interpreter::vect_helper::{get_elem_vect, set_elem_vect, VectorReg};
use crate::common::vect::*;
use crate::armv8::interpreter::vector_ops::{addp, bit, cmp, dup_element, dup_imm, GenCmpOps, saddl, saddw, ssubl, ssubw, umaxp, uminp};
use crate::common::signext_arbpos;

pub fn cvt_size_to_vecinfo(siz: u8, q: u8) -> VectInfo {
    let elemsize = 8 << (siz as usize);
    let lanecount = if q != 0 {
        128 / elemsize
    } else {
        64 / elemsize
    };
    VectInfo {
        lane_count: lanecount,
        elem_size: elemsize
    }
}
fn cvt_imm_to_vecinfo(imm: u8, q: u8) -> (u8, VectInfo){
    // let leftimm = (imm as u32) << (32 - 5);
    let leading = imm.trailing_zeros();
    //
    //let elemsize = pow(2, leading as usize) * 8;
    let elemsize = 8 << (leading as usize);
    let lanecount = if q != 0 {
        128 / elemsize
    } else {
        64 / elemsize
    };
    let idx = imm >> ((leading + 1) as u8);
    (idx, VectInfo {
        lane_count: lanecount,
        elem_size: elemsize
    })
}
pub fn saddl_advsimd(ai: &mut Arm64Cpu, arg: &ArmInstr) {
    let nval = ai.vreg[arg.get_rn() as usize];
    let mval = ai.vreg[arg.get_rm() as usize];
    let mut destval = VectorReg::default();
    let size = 8 << arg.get_simd_size();
    let vinfo = VectInfo::new_128bits(size * 2);
    let upper = arg.is_bit_set(30);
    saddl(&mut destval, &nval, &mval, upper, vinfo);
    ai.vreg[arg.get_rd() as usize] = destval;
}
pub fn saddw_advsimd(ai: &mut Arm64Cpu, arg: &ArmInstr) {
    let nval = ai.vreg[arg.get_rn() as usize];
    let mval = ai.vreg[arg.get_rm() as usize];
    let mut destval = VectorReg::default();
    let size = 8 << arg.get_simd_size();
    let vinfo = VectInfo::new_128bits(size * 2);
    let upper = arg.is_bit_set(30);
    saddw(&mut destval, &nval, &mval, upper, vinfo);
    ai.vreg[arg.get_rd() as usize] = destval;
}
pub fn dup_advsimd_gen(ai: &mut Arm64Cpu, arg: &ArmInstr) {
    let rd = arg.get_rd();
    let rn = arg.get_rn();
    let imm5 = ((arg.insn >> 16) & 0x1f);
    let tz = imm5.trailing_zeros();
    let esize = 8 << tz;
    let datasize = if ((arg.insn >> 30) & 1) != 0 { 128 } else { 64 };
    let vinfo = if datasize == 128 {
        VectInfo::new_128bits(esize)
    } else {
        VectInfo::new_64bits(esize)
    };
    let elem = ai.get_reg(rn, false); // dup_imm will truncate for us
    let newvect = dup_imm(elem, vinfo);
    ai.vreg[rd] = newvect;

}
fn advsimd_expand_imm(op: u64, cmode: u64, imm8: u64) -> u64 {
    let mut imm64: u64 = 0;
    let cmode31 = (cmode >> 1) & 0b111;
    match cmode31 {
        0 | 1 | 2 | 3 => {
            replicate(imm8 << (8 * cmode31), 2, 32)
        }
        4 => {
            replicate(imm8, 4, 16)
        }
        5 => {
            replicate(imm8 << 8, 4, 16)
        }
        6 => {
            replicate(imm8 << (8 << (cmode & 1)), 2, 32)
        }
        7 => {
            match ((cmode & 1) << 1) | op {
                0 => {
                    replicate(imm8, 8, 8)
                },
                1 => {
                    let mut immr =  imm8 | (imm8 << (0x08 - 1))
                        | (imm8 << (0x10 - 2)) | (imm8 << (0x18 - 3)) |
                        (imm8 << (0x20 - 4)) | (imm8 << (0x28 - 5))
                        | (imm8 << (0x30 - 6)) | (imm8 << (0x38 - 7));
                    immr &= 0x0101010101010101;
                    replicate(immr, 8, 1)
                },
                2 => {
                    let imm = (((imm8 & 0xc0) ^ 0x80) << 24) |
                        (replicate((imm8 >> 6) & 0b1, 5, 1) << 25)
                        | ((imm8 & 0x3f) << 19);
                    replicate(imm, 2, 32)
                }
                3 => {
                    (((imm8 & 0xc0) ^ 0x80) << 56)
                        | (replicate((imm8 >> 6) & 0b1, 8, 1) << 54)
                        | ((imm8 & 0x3f) << 48)
                },
                _ => panic!()
            }
        }
        _ => panic!()
    }
}
fn get_imm_modified(arg: &ArmInstr) -> u128 {
    let lower = ((arg.insn >> 5) & 0b11111) as u64; // defgh
    let upper = ((arg.insn >> 16) & 0b111) as u64; // abc
    let combined = lower | (upper << 5);
    let cmode = ((arg.insn >> 12) & 0xf) as u64;
    let data = advsimd_expand_imm(((arg.insn >> 29) & 1) as u64,
                                  cmode, combined) as u128;
    if (arg.insn & (1 << 30)) != 0 {
        data | (data << 64)
    } else {
        data
    }


}
pub fn movi_advsimd(ai: &mut Arm64Cpu, arg: &ArmInstr) {
    ai.vreg[arg.get_rd()].vect = get_imm_modified(arg);
}
pub fn bic_advsimd_imm(ai: &mut Arm64Cpu, arg: &ArmInstr) {
    let vval = ai.vreg[arg.get_rd()].vect;
    let imm = get_imm_modified(arg);
    ai.vreg[arg.get_rd()].vect = vval & !imm;
}
pub fn orr_advsimd_reg(ai: &mut Arm64Cpu, arg: &ArmInstr) {
    let is128 = if (arg.insn & (1 << 30)) != 0 { true } else { false };
    let r = ai.vreg[arg.get_rn()].vect;
    let m = ai.vreg[arg.get_rm()].vect;
    let res = r | m;
    ai.vreg[arg.get_rd()].vect = if is128 {
        res
    } else {
        res as u64 as u128
    };
}
pub fn umov_advsimd(ai: &mut Arm64Cpu, arg: &ArmInstr) {
    let imm5 = (arg.insn >> 16) & 0x1f;
    let q = (arg.insn >> 30) & 1;
    let (idx, vinfo) = cvt_imm_to_vecinfo(imm5 as u8, q as u8);
    let mut rn = ai.vreg[arg.get_rn()].get_elem_fixed(idx as usize, vinfo);
    rn &= vinfo.mask();
    ai.set_reg(arg.get_rd(), rn, false);
}
pub fn cmeq_advsimd_zero(ai: &mut Arm64Cpu, arg: &ArmInstr) {
    let is_scalar = ((arg.insn >> 28) & 1) != 0;
    let siz = arg.get_simd_size();

    let elemsize = 8 << (siz);
    let vinfo = if is_scalar {
        VectInfo {
            lane_count: 1,
            elem_size: elemsize
        }
    } else {
        let q = ((arg.insn >> 30) & 1) != 0;
        if q {
            VectInfo::new_128bits(elemsize)
        } else {
            VectInfo::new_64bits(elemsize)
        }
    };
    let mut rd = ai.vreg[arg.get_rd()];
    let mut rn = ai.vreg[arg.get_rn()];
    let zero = VectorReg::new(0);
    cmp(&mut rd, &rn, &zero, vinfo, false, GenCmpOps::Eq);
    ai.vreg[arg.get_rd()] = rd;

}
fn cmp_gen(ai: &mut Arm64Cpu, arg: &ArmInstr, op: GenCmpOps, is_signed: bool, is_zero: bool) {
    let is_scalar = ((arg.insn >> 28) & 1) != 0;
    let siz = arg.get_simd_size();
    let elemsize = 8 << (siz);
    let vinfo = if is_scalar {
        VectInfo {
            lane_count: 1,
            elem_size: elemsize
        }
    } else {
        let q = ((arg.insn >> 30) & 1) != 0;
        if q {
            VectInfo::new_128bits(elemsize)
        } else {
            VectInfo::new_64bits(elemsize)
        }
    };
    let mut rd = ai.vreg[arg.get_rd()];
    let rn = ai.vreg[arg.get_rn()];
    let rm = if is_zero {
        VectorReg::new(0)
    } else {
        ai.vreg[arg.get_rm()]
    };
    cmp(&mut rd, &rn, &rm, vinfo, is_signed, op);
    ai.vreg[arg.get_rd()] = rd;
}
pub fn cmhs_advsimd(ai: &mut Arm64Cpu, arg: &ArmInstr) {
    cmp_gen(ai, arg, GenCmpOps::Ge, false, false);
}
pub fn cmeq_advsimd_reg(ai: &mut Arm64Cpu, arg: &ArmInstr) {
    let is_scalar = ((arg.insn >> 28) & 1) != 0;
    let siz = arg.get_simd_size();
    let elemsize = 8 << (siz);
    let vinfo = if is_scalar {
        VectInfo {
            lane_count: 1,
            elem_size: elemsize
        }
    } else {
        let q = ((arg.insn >> 30) & 1) != 0;
        if q {
            VectInfo::new_128bits(elemsize)
        } else {
            VectInfo::new_64bits(elemsize)
        }
    };
    let mut rd = ai.vreg[arg.get_rd()];
    let rn = ai.vreg[arg.get_rn()];
    let rm = ai.vreg[arg.get_rm()];
    cmp(&mut rd, &rn, &rm, vinfo, false, GenCmpOps::Eq);
    ai.vreg[arg.get_rd()] = rd;

}
pub fn and_advsimd(ai: &mut Arm64Cpu, arg: &ArmInstr) {
    let mut rd = ai.vreg[arg.get_rd()];
    let mut rn = ai.vreg[arg.get_rn()];
    let mut rm = ai.vreg[arg.get_rm()];
    rd.vect = rn.vect & rm.vect;
    let q = ((arg.insn >> 30) & 1) != 0;
    if !q {
        rd.vect = rd.vect as u64 as u128;
    }
    ai.vreg[arg.get_rd()] = rd;

}
pub fn addp_advsimd_vec(ai: &mut Arm64Cpu, arg: &ArmInstr) {
    let q = ((arg.insn >> 30) & 1) != 0;
    let sz = arg.get_simd_size();
    let vinfo = cvt_size_to_vecinfo(sz as u8, q as u8);
    let mut rd = ai.vreg[arg.get_rd()];
    let mut rn = ai.vreg[arg.get_rn()];
    let mut rm = ai.vreg[arg.get_rm()];
    addp(&mut rd, &rn, &rm, vinfo);
    ai.vreg[arg.get_rd()] = rd;
}
pub fn umaxp_advsimd(ai: &mut Arm64Cpu, arg: &ArmInstr) {
    let mut rd = ai.vreg[arg.get_rd()];
    let mut rn = ai.vreg[arg.get_rn()];
    let mut rm = ai.vreg[arg.get_rm()];
    let q = ((arg.insn >> 30) & 1) != 0;
    let sz = arg.get_simd_size();
    let vinfo = cvt_size_to_vecinfo(sz as u8, q as u8);
    umaxp(&mut rd, &rn, &rm, vinfo);
    ai.vreg[arg.get_rd()] = rd;
}
pub fn uminp_advsimd(ai: &mut Arm64Cpu, arg: &ArmInstr) {
    let mut rd = ai.vreg[arg.get_rd()];
    let mut rn = ai.vreg[arg.get_rn()];
    let mut rm = ai.vreg[arg.get_rm()];
    let q = ((arg.insn >> 30) & 1) != 0;
    let sz = arg.get_simd_size();
    let vinfo = cvt_size_to_vecinfo(sz as u8, q as u8);
    uminp(&mut rd, &rn, &rm, vinfo);
    ai.vreg[arg.get_rd()] = rd;
}
pub fn bit_advsimd(ai: &mut Arm64Cpu, arg: &ArmInstr) {
    /*let mut rd = ai.vreg[arg.get_rd()];
    let mut rn = ai.vreg[arg.get_rn()];
    let mut rm = ai.vreg[arg.get_rm()];
    let q = ((arg.insn >> 30) & 1) != 0;
    let vinfo = if q {
        VectInfo::new_128bits(8)
    } else {
        VectInfo::new_64bits(8)
    };
    bit(&mut rd, &rn, &rm, vinfo);
        ai.vreg[arg.get_rd()] = rd;

     */
    let mut op1 = ai.vreg[arg.get_rd()];
    let mut op2 = op1;
    let mut op3 = ai.vreg[arg.get_rm()];
    let mut op4 = ai.vreg[arg.get_rn()];

    let mut res = op1.vect ^ ((op2.vect ^ op4.vect) & op3.vect);
    let q = ((arg.insn >> 30) & 1) != 0;
    if !q {
        res = res as u64 as u128;
    }
    ai.vreg[arg.get_rd()].vect = res;
}