use crate::armv8::interpreter::main::Arm64Cpu;
use crate::armv8::interpreter::vect_helper::{get_elem_vect, set_elem_vect, VectorReg};
use crate::common::vect::*;

pub fn set_from_array(vecval: u128, arr: &[u64], vinfo: VectInfo) -> u128 {
    let mut change = vecval;
    for i in 0..arr.len() {
        change = set_elem_vect_fixed(change, arr[i], i, vinfo);
    }
    change
}
pub fn vec_maxmin<T: num::PrimInt>(ai: &mut Arm64Cpu, rn: usize, rd: usize, elemcount: usize, min: bool) {
    let op = ai.vreg[rn];
    let mut maxmin: T = get_elem_vect::<T>(op.vect, 0);
    for i in 1..elemcount {
        let element: T = get_elem_vect::<T>(op.vect, i);
        maxmin = if min {
            maxmin.min(element)
        } else {
            maxmin.max(element)
        }
    }
    ai.vreg[rd].vect = maxmin.to_u128().unwrap();
}
pub fn dup_element(src: &VectorReg, src_idx: usize, vinfo: VectInfo) -> VectorReg {
    let mut vec: VectorReg = Default::default();
    let val = src.get_elem_fixed(src_idx, vinfo);
    for i in 0..vinfo.lane_count {
        vec.set_elem_fixed(val, i, vinfo);
    }
    vec
}
pub fn eor(dst: &mut VectorReg, src1: VectorReg, src2: VectorReg, vinfo: VectInfo)  {
    dst.clear_vect();
    for i in 0..vinfo.lane_count {
        let src1v = src1.get_elem_fixed(i, vinfo);
        let src2v = src2.get_elem_fixed(i, vinfo);
        dst.set_elem_fixed(src1v ^ src2v, i, vinfo)
    }
}
pub fn orn(dst: &mut VectorReg, src1: VectorReg, src2: VectorReg, vinfo: VectInfo)  {
    dst.clear_vect();
    for i in 0..vinfo.lane_count {
        let src1v = src1.get_elem_fixed(i, vinfo);
        let src2v = src2.get_elem_fixed(i, vinfo);
        dst.set_elem_fixed(src1v | !src2v, i, vinfo)
    }
}
pub fn bic_reg(dst: &mut VectorReg, src1: VectorReg, src2: VectorReg, vinfo: VectInfo)  {
    dst.clear_vect();
    for i in 0..vinfo.lane_count {
        let src1v = src1.get_elem_fixed(i, vinfo);
        let src2v = src2.get_elem_fixed(i, vinfo);
        dst.set_elem_fixed(src1v & !src2v, i, vinfo)
    }
}
pub fn bif(dst: &mut VectorReg, src1: VectorReg, src2: VectorReg, vinfo: VectInfo)  {
    let odst = dst.clone();
    dst.clear_vect();
    for i in 0..vinfo.lane_count {
        let op1 = odst.get_elem_fixed(i, vinfo);
        let op2 = !src2.get_elem_fixed(i, vinfo);
        let op3 = src1.get_elem_fixed(i, vinfo);
        let result = op1 ^ ((op1 ^ op3) & op2);
        dst.set_elem_fixed(result, i, vinfo)
    }
}
pub fn bit(dst: &mut VectorReg, src1: &VectorReg, src2: &VectorReg, vinfo: VectInfo)  {
    let odst = dst.clone();
    dst.clear_vect();
    for i in 0..vinfo.lane_count {
        let op1 = odst.get_elem_fixed(i, vinfo);
        let op2 = src2.get_elem_fixed(i, vinfo);
        let op3 = src1.get_elem_fixed(i, vinfo);
        let result = op1 ^ ((op1 ^ op3) & op2);
        dst.set_elem_fixed(result, i, vinfo)
    }
}
pub fn bsl(dst: &mut VectorReg, src1: VectorReg, src2: VectorReg, vinfo: VectInfo)  {
    let odst = dst.clone();
    dst.clear_vect();
    for i in 0..vinfo.lane_count {
        let op1 = src2.get_elem_fixed(i, vinfo);
        let op2 = odst.get_elem_fixed(i, vinfo);
        let op3 = src1.get_elem_fixed(i, vinfo);
        let result = op1 ^ ((op1 ^ op3) & op2);
        dst.set_elem_fixed(result, i, vinfo)
    }
}
pub fn bic_imm(dst: &mut VectorReg, src: &VectorReg, imm: u64, vinfo: VectInfo)  {
    dst.clear_vect();
    let mut result: [u64; 16] = [0; 16];
    let srcval = imm & vinfo.mask();

    for i in 0..vinfo.lane_count {
        let val = src.get_elem_fixed(i, vinfo);
        result[i] = val & !srcval;
    }
    dst.set_from_array(&result, vinfo);
}
pub fn orr_imm(dst: &mut VectorReg, src: VectorReg, imm: u64, vinfo: VectInfo)  {
    let mut result: [u64; 16] = [0; 16];
    let srcval = imm & vinfo.mask();
    for i in 0..vinfo.lane_count {
        let val = src.get_elem_fixed(i, vinfo);
        result[i] = val | srcval;
    }
    dst.set_from_array(&result, vinfo);

}
pub fn uxtl(dst: &mut VectorReg, src: &VectorReg, upper: bool, vinfo: VectInfo)  {
    let half = vinfo.half_width();
    dst.clear_vect();
    let addend = if upper { vinfo.lane_count } else { 0 };
    for i in 0..vinfo.lane_count {
        // both have same number of slements
        let srcval = src.get_elem_fixed(addend + i, half);
        dst.set_elem_fixed(srcval, i, vinfo);
    }

}
pub fn sxtl(dst: &mut VectorReg, src: &VectorReg, upper: bool, vinfo: VectInfo)  {
    let half = vinfo.half_width();
    dst.clear_vect();
    let addend = if upper { vinfo.elem_size } else { 0 };
    for i in 0..vinfo.lane_count {
        // both have same number of slements
        let srcval = src.get_elem_signed_fixed(addend + i, half);
        dst.set_elem_signed_fixed(srcval, i, vinfo);
    }

}
pub fn suqadd(dst: &mut VectorReg, src: VectorReg, src2: VectorReg, vinfo: VectInfo)  {
    dst.clear_vect();
    for i in 0..vinfo.lane_count {
        // both have same number of slements
        let sa = src.get_elem_fixed_justified_left(i, vinfo) as i64;
        let ub = src2.get_elem_fixed_justified_left(i, vinfo);
        let ur = (sa as u64) + ub;
        let sr = ur as i64;
        if sr < sa {
            let max = vinfo.get_max_signed();
            dst.set_elem_signed_fixed(max, i, vinfo);
        } else {
            let val1 = src.get_elem_signed_fixed(i, vinfo);
            let val2 = src2.get_elem_fixed(i, vinfo);
            dst.set_elem_fixed((val1 as u64) + val2, i, vinfo);

        }
    }

}
pub fn usqadd(dst: &mut VectorReg, src: VectorReg, src2: VectorReg, vinfo: VectInfo)  {
    dst.clear_vect();
    for i in 0..vinfo.lane_count {
        // both have same number of slements
        let ua = src.get_elem_fixed_justified_left(i, vinfo) as u64;
        let sb = src2.get_elem_fixed_justified_left(i, vinfo) as i64;
        let ur = (ua as u64) + (sb as u64);
        if (sb > 0) && (ur <= ua) {
            let max = vinfo.get_max();
            dst.set_elem_signed_fixed(max as i64, i, vinfo);
        } else if (sb < 0) && (ur >= ua) {
            dst.set_elem_signed_fixed(0, i, vinfo);
        } else {
            let val1 = src.get_elem_fixed(i, vinfo);
            let val2 = src2.get_elem_signed_fixed(i, vinfo);
            dst.set_elem_fixed(val1 + (val2 as u64), i, vinfo);
        }

    }

}
pub fn cls(dst: &mut VectorReg, src: VectorReg, vinfo: VectInfo)  {
    dst.clear_vect();
    for i in 0..vinfo.lane_count {
        let src1val = src.get_leading_sign(i, vinfo);
        dst.set_elem_fixed(src1val as u64, i, vinfo);
    }
}
pub fn clz(dst: &mut VectorReg, src: VectorReg, vinfo: VectInfo)  {
    dst.clear_vect();
    for i in 0..vinfo.lane_count {
        let src1val = src.get_leading_zeros(i, vinfo);
        dst.set_elem_fixed(src1val as u64, i, vinfo);
    }
}
pub fn cnot(dst: &mut VectorReg, src: VectorReg, vinfo: VectInfo)  {
    dst.clear_vect();
    for i in 0..vinfo.lane_count {
        let src1val = if src.get_elem_fixed(i, vinfo) == 0 {
            1
        } else {
            0
        };
        dst.set_elem_fixed(src1val as u64, i, vinfo);
    }
}
pub fn not(dst: &mut VectorReg, src: VectorReg, vinfo: VectInfo)  {
    dst.clear_unused(vinfo);
    for i in 0..vinfo.lane_count {
        let val = !src.get_elem_fixed(i, vinfo);
        dst.set_elem_fixed(val as u64, i, vinfo);
    }
}
pub fn cnt(dst: &mut VectorReg, src: VectorReg, vinfo: VectInfo)  {
    dst.clear_vect();
    for i in 0..vinfo.lane_count {
        let val = src.get_elem_fixed(i, vinfo).count_ones();
        dst.set_elem_fixed(val as u64, i, vinfo);
    }
}
pub fn mul(dst: &mut VectorReg, src1: &VectorReg, src2: &VectorReg, vinfo: VectInfo) {
    dst.clear_vect();
    for i in 0..vinfo.lane_count {
        let src1val = src1.get_elem_fixed(i, vinfo);
        let src2val = src2.get_elem_fixed(i, vinfo);
        let res = src1val.wrapping_mul(src2val);
        dst.set_elem_fixed(res, i, vinfo);
    }

}
pub fn mul_by_elem(dst: &mut VectorReg, src1: &VectorReg, src2: &VectorReg, idx: usize, vinfo: VectInfo) {
    dst.clear_vect();
    let indexform = vinfo.elem_use_whole_reg();
    let mut temp: VectorReg = dup_element(src2, idx, indexform);
    mul(dst, src1, &temp, vinfo);
}
/*
pub fn mla_by_elem(dst: &mut VectorReg, src1: VectorReg, src2: VectorReg, idx: usize, vinfo: VectInfo) {
    dst.clear_vect();
    let indexform = vinfo.elem_use_whole_reg();
    let mut temp: VectorReg = dup_element(src2, idx, indexform);
    mla(dst, src1, temp, vinfo);
}
pub fn mls_by_elem(dst: &mut VectorReg, src1: VectorReg, src2: VectorReg, idx: usize, vinfo: VectInfo) {
    dst.clear_vect();
    let indexform = vinfo.elem_use_whole_reg();
    let mut temp: VectorReg = dup_element(src2, idx, indexform);
    mls(dst, src1, temp, vinfo);
}

 */
pub fn sub(dst: &mut VectorReg, src1: &VectorReg, src2: &VectorReg, vinfo: VectInfo) {
    dst.clear_vect();
    for i in 0..vinfo.lane_count {
        let src1val = src1.get_elem_fixed_justified_left(i, vinfo);
        let src2val = src2.get_elem_fixed_justified_left(i, vinfo);
        let justval = src1val.wrapping_sub(src2val);
        if src2val > src1val {
            dst.set_unsigned_sat(i, true);
        }
        let bit1 = (src1val >> 63) == 0;
        let bit2 = (src2val >> 63) == 0;
        let bit3 = (justval >> 63) == 0;
        if (bit1 != bit2) && (bit1 != bit3) {
            dst.set_unsigned_sat(i, true);
        }
        dst.set_elem_fixed((justval >> ((64 - vinfo.lane_count) as u64)), i, vinfo);
    }

}
pub fn add(dst: &mut VectorReg, src1: &VectorReg, src2: &VectorReg, vinfo: VectInfo) {
    dst.clear_vect();
    dst.clear_einfo();
    for i in 0..vinfo.lane_count {
        let src1val = src1.get_elem_fixed_justified_left(i, vinfo);
        let src2val = src2.get_elem_fixed_justified_left(i, vinfo);
        let justval = src1val.wrapping_add(src2val);
        if justval < src1val {
            dst.set_unsigned_sat(i, true);
        }
        let bit1 = (src1val >> 63) == 0;
        let bit2 = (src2val >> 63) == 0;
        let bit3 = (justval >> 63) == 0;
        if (bit1 == bit2) && (bit1 != bit3) {
            dst.set_unsigned_sat(i, true);
        }
        dst.set_elem_fixed((justval >> ((64 - vinfo.elem_size) as u64)), i, vinfo);
    }

}


pub fn addp(dst: &mut VectorReg, src1: &VectorReg, src2: &VectorReg, vinfo: VectInfo) {
    let mut temp1: VectorReg = Default::default();
    let mut temp2: VectorReg = Default::default();
    uzp(&mut temp1, src1, src2, false, vinfo);
    uzp(&mut temp2, src1, src2, true, vinfo);
    add(dst, &temp1, &temp2, vinfo);

}
/*
pub fn mla(dst: &mut VectorReg, scra: VectorReg, src1: VectorReg, src2: VectorReg, vinfo: VectInfo) {
    let mut temp1: VectorReg = Default::default();
    mul(&mut temp1, src1, src2, vinfo);
    add(dst, scra, temp1, vinfo);

}

 */
pub fn mls(dst: &mut VectorReg, srca: &VectorReg, src1: &VectorReg, src2: &VectorReg, vinfo: VectInfo) {
    let mut temp1: VectorReg = Default::default();
    mul(&mut temp1, src1, src2, vinfo);
    sub(dst, srca, &temp1, vinfo);
}
pub fn ushl(dst: &mut VectorReg, src1: VectorReg, src2: VectorReg, vinfo: VectInfo) {
    dst.clear_vect();
    for i in 0..vinfo.lane_count {
        let shift_val = src2.get_elem_signed_fixed(i, vinfo) as i8;
        let left_srcval = src1.get_elem_fixed_justified_left(i, vinfo);
        let real_srcval = src1.get_elem_fixed(i, vinfo);
        if (shift_val  > (left_srcval.leading_zeros() as i8)) && (left_srcval != 0) {
            dst.set_unsigned_sat(i, true);
        }
        if (shift_val > 64) || (shift_val < -64) {
            dst.set_elem_fixed(0, i, vinfo);
        } else {
            let mut res: u64 = 0;
            if shift_val < 0 {
                if ((real_srcval >> ((-shift_val - 1 ) as u64)) &1) == 1 {
                    dst.set_rounding(i, true);
                }
                if shift_val == -64 {
                    res = 0;
                } else {
                    res = real_srcval >> ((-shift_val) as u64);
                }
            } else {
                res = real_srcval << (shift_val as u64);
            }
            dst.set_elem_fixed(res, i, vinfo);
        }

    }

}
pub fn neg(dst: &mut VectorReg, src: VectorReg, vinfo: VectInfo) {
    dst.clear_vect();
    for i in 0..vinfo.lane_count {
        let val = src.get_elem_signed_fixed(i, vinfo);
        if val == vinfo.get_min_signed() {
            dst.set_signed_sat(i, true);
        }
        let wval = if val == i64::MIN {
            val
        } else {
            -val
        };
        dst.set_elem_signed_fixed(wval, i, vinfo);
    }
}

pub fn dup_imm(imm: u64, vinfo: VectInfo) -> VectorReg {
    let realval = imm & vinfo.mask();
    let mut vec: VectorReg = Default::default();
    for i in 0..vinfo.lane_count {
        vec.set_elem_fixed(realval, i, vinfo);
    }
    vec
}

pub fn ins_element(src: VectorReg, dst: &mut VectorReg, src_idx: usize, dst_idx: usize, vinfo: VectInfo) {
    let srcval = src.get_elem_fixed(src_idx, vinfo);
    dst.set_elem_fixed(srcval, dst_idx, vinfo);
}
pub fn ins_imm(dst: &mut VectorReg, imm: u64, dst_idx: usize, vinfo: VectInfo)  {
    let srcval = imm & vinfo.mask();
    dst.set_elem_fixed(srcval, dst_idx, vinfo);

}
pub fn movi(dst: &mut VectorReg, imm: u64, vinfo: VectInfo)  {
    let srcval = imm & vinfo.mask();
    dst.clear_vect();
    for i in 0..vinfo.lane_count {
        dst.set_elem_fixed(srcval, i, vinfo);
    }

}
pub fn mvni(dst: &mut VectorReg, imm: u64, vinfo: VectInfo)  {
    let srcval = imm & vinfo.mask();
    dst.clear_vect();
    for i in 0..vinfo.lane_count {
        dst.set_elem_fixed(!srcval, i, vinfo);
    }

}

pub fn extract_narrow(dst: &mut VectorReg, dst_signed: bool, src: VectorReg, src_signed: bool, dstinfo: VectInfo) {
    let upperhalf: bool = if (dstinfo.elem_size / 4) * (dstinfo.lane_count) == 128 {
        true
    } else if (dstinfo.elem_size / 4) * (dstinfo.lane_count) == 64 {
        false
    } else {
        panic!();
    };
    let srcinfo_size = dstinfo.elem_size * 2;
    let srcinfo_lc = 64 / srcinfo_size;
    let srcinfo = VectInfo {
        lane_count: srcinfo_lc,
        elem_size: srcinfo_size
    };
    let mut ssrc: [i64; 8] = [0; 8];
    let mut usrc: [u64; 8] = [0; 8];
    for i in 0..srcinfo.lane_count {
        ssrc[i] = src.get_elem_signed_fixed(i, srcinfo);
        usrc[i] = src.get_elem_fixed(i, srcinfo);
    }
    let offset = if upperhalf {
        dstinfo.lane_count / 2
    } else {
        dst.clear_vect();
        0
    };
    for i in 0..srcinfo.lane_count {
        if ssrc[i] > dstinfo.get_max_signed() {
            dst.set_signed_sat(offset + i, true);
        } else if ssrc[i] < dstinfo.get_min_signed() {
            dst.set_signed_sat(offset + i, false);

        }

        if src_signed {
            if ssrc[i] > dstinfo.get_max() as i64 {
                dst.set_unsigned_sat(offset + i, true);
            } else if ssrc[i] < 0 {
                dst.set_unsigned_sat(offset + i, false);
            }
        } else {
            if usrc[i] > dstinfo.get_max() {
                dst.set_unsigned_sat(offset + i, true);
            }
        }
        let result = if src_signed {
            (ssrc[i] as u64) & dstinfo.get_max()
        } else {
            usrc[i] & dstinfo.get_max()
        };
        if dst_signed {
            dst.set_elem_signed_fixed(result as i64, offset + i, dstinfo);
        } else {
            dst.set_elem_fixed(result, offset + i, dstinfo);
        }
    }

}
pub fn xtn(dst: &mut VectorReg, src: VectorReg, vinfo: VectInfo) {
    extract_narrow(dst, true, src, true, vinfo);
}
pub fn sqxtn(dst: &mut VectorReg, src: VectorReg, vinfo: VectInfo) {
    extract_narrow(dst, true, src, true, vinfo);
    dst.signed_saturate(vinfo);
}
pub fn sqxtun(dst: &mut VectorReg, src: VectorReg, vinfo: VectInfo) {
    extract_narrow(dst, false, src, true, vinfo);
    dst.unsigned_saturate(vinfo);
}
pub fn uqxtn(dst: &mut VectorReg, src: VectorReg, vinfo: VectInfo) {
    extract_narrow(dst, false, src, false, vinfo);
    dst.unsigned_saturate(vinfo);
}
pub fn abs(dst: &mut VectorReg, src: VectorReg, vinfo: VectInfo) {
    dst.clear_vect();
    for i in 0..vinfo.lane_count {
        let sa = src.get_elem_signed_fixed(i, vinfo);
        if sa == vinfo.get_min_signed() {
            dst.set_signed_sat(i, true);
        }
        if sa < 0 {
            let val = if sa == i64::MIN {
                sa
            } else {
                -sa
            };
            dst.set_elem_signed_fixed(val, i, vinfo);
        } else {
            dst.set_elem_signed_fixed(sa, i, vinfo);

        }
    }
}
pub fn sshl(dst: &mut VectorReg, src1: VectorReg, src2: VectorReg, vinfo: VectInfo) {
    dst.clear_vect();
    for i in 0..vinfo.lane_count {
        let shift_val = src2.get_elem_signed_fixed(i, vinfo) as i8;
        let left_srcval = src1.get_elem_fixed_justified_left(i, vinfo) as i64;
        let real_srcval = src1.get_elem_signed_fixed(i, vinfo);
        if (shift_val  > (left_srcval.leading_zeros() as i8)) && (left_srcval != 0) {
            dst.set_signed_sat(i, left_srcval >= 0);
        }
        if left_srcval < 0 {
            dst.set_unsigned_sat(1, false);
        } else if (shift_val  > (left_srcval.leading_zeros() as i8)) && (left_srcval != 0) {
            dst.set_unsigned_sat(1, true);
        }
        let src_neg: bool = real_srcval < 0;
        if shift_val > 63 {
            dst.set_elem_fixed(0, i, vinfo);
        } else if shift_val < -64 {
            dst.set_rounding(i, src_neg);
            let finval = if src_neg { -1 } else { 0 };
            dst.set_elem_signed_fixed(finval, i, vinfo);
        } else {
            let mut unval = real_srcval as u64;
            if shift_val < 0 {
                let r_shift_val = -shift_val;
                if ((real_srcval >> ((r_shift_val - 1 ) as u64)) &1) == 1 {
                    dst.set_rounding(i, true);
                }
                unval >>= (shift_val as u64);
                if src_neg {
                    unval |= (!(0 as u64) << (64 - r_shift_val));
                }
            } else {
                unval <<= shift_val;
            }
            dst.set_elem_fixed(unval, i, vinfo);
        }

    }

}
pub fn rbit(dest: &mut VectorReg, src: VectorReg, vinfo: VectInfo) {
    let mut result: [u64; 16] = [0; 16];
    let lanebits = vinfo.elem_size;
    for i in 0..vinfo.lane_count {
        let mut value = src.get_elem_fixed(i, vinfo);
        let mut reversed_value = 0;
        for _ in 0..lanebits {
            reversed_value = (reversed_value << 1) | (value & 1);
            value >>= 1;
        }
        result[i] = reversed_value;

    }
    dest.clear_unused(vinfo);
    for i in 0..vinfo.lane_count {
        dest.set_elem_fixed(result[i],i, vinfo);
    }
}
pub fn rev_byte(dest: &mut VectorReg, src: VectorReg, vinfo: VectInfo, rev_size: usize) {
    let mut result: [u64; 16] = [0; 16];
    let lanebytes = vinfo.elem_size / 8;
    let lanes_per_loop = rev_size / lanebytes;
    let mut i = 0;
    while i < vinfo.lane_count {
        for j in 0..lanes_per_loop {
            result[i + lanes_per_loop - 1 - j] = src.get_elem_fixed(i + j, vinfo);
        }
        i += lanes_per_loop;
    }
    dest.clear_unused(vinfo);
    for i in 0..vinfo.lane_count {
        dest.set_elem_fixed(result[i],i, vinfo);
    }
}
pub fn rev16(dest: &mut VectorReg, src: VectorReg, vinfo: VectInfo) {
    rev_byte(dest, src, vinfo,2);
}
pub fn rev32(dest: &mut VectorReg, src: VectorReg, vinfo: VectInfo) {
    rev_byte(dest, src, vinfo,4);
}
pub fn rev64(dest: &mut VectorReg, src: VectorReg, vinfo: VectInfo) {
    rev_byte(dest, src, vinfo,8);
}
pub fn addlp(dest: &mut VectorReg, src: VectorReg, vinfo: VectInfo, is_signed: bool, accumlate: bool) {
    let mut result: [u64; 16] = [0; 16];
    dest.clear_vect();
    for i in 0..vinfo.lane_count {
        if is_signed {
            let val1 = src.get_elem_signed_fixed(2 * i, vinfo);
            let val2 = src.get_elem_signed_fixed((2 * i) + 1, vinfo);
            result[i] = (val1 + val2) as u64;
        } else {
            let val1 = src.get_elem_fixed(2 * i, vinfo);
            let val2 = src.get_elem_fixed((2 * i) + 1, vinfo);
            result[i] = (val1 + val2) as u64;
        }

    }
    if !accumlate {
        dest.clear_vect();
    }
    for i in 0..vinfo.lane_count {
        if accumlate {
            result[i] += dest.get_elem_fixed(i, vinfo);
        }
        dest.set_elem_fixed(result[i], i, vinfo);
    }
}
pub fn saddlp(dest: &mut VectorReg, src: VectorReg, vinfo: VectInfo) {
    addlp(dest, src, vinfo, true, false)
}
pub fn uaddlp(dest: &mut VectorReg, src: VectorReg, vinfo: VectInfo) {
    addlp(dest, src, vinfo, false, false)
}
pub fn sadalp(dest: &mut VectorReg, src: VectorReg, vinfo: VectInfo) {
    addlp(dest, src, vinfo, true, true)
}
pub fn uadalp(dest: &mut VectorReg, src: VectorReg, vinfo: VectInfo) {
    addlp(dest, src, vinfo, false, true)
}
pub fn uaddw(dest: &mut VectorReg, src1: &VectorReg, src2: &VectorReg, is_two: bool, vinfo: VectInfo) {
    let mut temp = VectorReg::default();
    uxtl(&mut temp, src2, is_two, vinfo);
    add(dest, src1, &temp, vinfo);
}
pub fn saddw(dest: &mut VectorReg, src1: &VectorReg, src2: &VectorReg, is_two: bool, vinfo: VectInfo) {
    let mut temp = VectorReg::default();
    sxtl(&mut temp, src2, is_two, vinfo);
    add(dest, src1, &temp, vinfo);
}
pub fn ssubw(dest: &mut VectorReg, src1: &VectorReg, src2: &VectorReg, is_two: bool, vinfo: VectInfo) {
    let mut temp = VectorReg::default();
    sxtl(&mut temp, src2, is_two, vinfo);
    sub(dest, src1, &temp, vinfo);
}
pub fn usubl(dest: &mut VectorReg, src1: &VectorReg, src2: &VectorReg, is_two: bool, vinfo: VectInfo) {
    let mut temp1 = VectorReg::default();
    let mut temp2 = VectorReg::default();
    uxtl(&mut temp1, src1, is_two, vinfo);
    uxtl(&mut temp2, src2, is_two, vinfo);
    sub(dest, &temp1, &temp2, vinfo)
}
pub fn uminp(dest: &mut VectorReg, src1: &VectorReg, src2: &VectorReg,
             vinfo: VectInfo) {
    uminmaxp(dest, src1, src2, vinfo, false);
}
pub fn umaxp(dest: &mut VectorReg, src1: &VectorReg, src2: &VectorReg,
                vinfo: VectInfo) {
    uminmaxp(dest, src1, src2, vinfo, true);
}
pub fn uminmaxp(dest: &mut VectorReg, src1: &VectorReg, src2: &VectorReg,
             vinfo: VectInfo, max: bool) {
    let mut result: [u64; 16] = [0; 16];
    let mut currentptr = src1;
    let lanes = vinfo.lane_count;
    for j in 0..2 {
        for i in (0..vinfo.lane_count).step_by(2) {
            let val1 = currentptr.get_elem_fixed(i, vinfo);
            let val2 = currentptr.get_elem_fixed(i + 1, vinfo);
            let dst = if max {
                if val1 > val2 {
                    val1
                } else {
                    val2
                }
            } else {
                if val1 < val2 {
                    val1
                } else {
                    val2
                }
            };
            result[(i >> 1) + (j * lanes / 2)] = dst;
        }
        currentptr = src2;
    }
    dest.set_from_array(&result, vinfo);
}
pub fn saddl(dest: &mut VectorReg, src1: &VectorReg, src2: &VectorReg, is_two: bool, vinfo: VectInfo) {
    let mut temp1 = VectorReg::default();
    let mut temp2 = VectorReg::default();
    sxtl(&mut temp1, src1, is_two, vinfo);
    sxtl(&mut temp2, src2, is_two, vinfo);
    add(dest, &temp1, &temp2, vinfo);
}
pub fn ssubl(dest: &mut VectorReg, src1: &VectorReg, src2: &VectorReg, is_two: bool, vinfo: VectInfo) {
    let mut temp1 = VectorReg::default();
    let mut temp2 = VectorReg::default();
    sxtl(&mut temp1, src1, is_two, vinfo);
    sxtl(&mut temp2, src2, is_two, vinfo);
    sub(dest, &temp1, &temp2, vinfo);
}
/*

pub fn zip(ai: &mut Arm64Cpu, rn: usize, rm: usize, rd: usize, part: usize, vinfo: VectInfo) {
    let rnval = ai.vreg[rn];
    let rmval = ai.vreg[rm];
    let mut result: [u64; 16] = [0; 16];
    let pairs = vinfo.lane_count / 2;
    assert!(part == 1 || part == 0); // zip2, part = 1
    let addend = part * pairs;
    for i in 0..pairs {
        result[i] = rnval.get_elem_fixed(addend + i, vinfo);
        result[(2 * i) + 1] = rmval.get_elem_fixed(addend + i, vinfo);
    }
    ai.vreg[rd].set_from_array(&result, vinfo);

}

 */
pub fn uzp(dst: &mut VectorReg, src1: &VectorReg, src2: &VectorReg, is_uzp2: bool, vinfo: VectInfo) {
    //
    let mut result: [u64; 64] = [0; 64];
    let part = if is_uzp2 { 1 } else { 0 };
    for i in 0..vinfo.lane_count {
        result[i] = src1.get_elem_fixed(i, vinfo);
        result[vinfo.lane_count + i] = src2.get_elem_fixed(i, vinfo);
    }
    dst.clear_vect();
    for i in 0..vinfo.lane_count {
        dst.set_elem_fixed(result[(2 * i) + part], i, vinfo);
    }

}
/*
pub fn uaba(dst: &mut VectorReg, src1: VectorReg, src2: VectorReg, vinfo: VectInfo) {
    let mut temp = VectorReg::default();
    dst.clear_unused(vinfo);
    absdiff(&mut temp, src1, src2, false, vinfo);
    let edst = dst.clone();
    add(dst, edst, temp, vinfo);
}
pub fn uabdl(dst: &mut VectorReg, src1: VectorReg, src2: VectorReg, is_upper: bool, vinfo: VectInfo) {
    let mut temp1 = VectorReg::default();
    let mut temp2 = VectorReg::default();
    uxtl(&mut temp1, src1, is_upper, vinfo);
    uxtl(&mut temp2, src2, is_upper, vinfo);
    absdiff(dst, temp1, temp2, false, vinfo);
}
pub fn sabdl(dst: &mut VectorReg, src1: VectorReg, src2: VectorReg, is_upper: bool, vinfo: VectInfo) {
    let mut temp1 = VectorReg::default();
    let mut temp2 = VectorReg::default();
    sxtl(&mut temp1, src1, is_upper, vinfo);
    sxtl(&mut temp2, src2, is_upper, vinfo);
    absdiff(dst, temp1, temp2, true, vinfo);
}
pub fn uabal(dst: &mut VectorReg, src1: VectorReg, src2: VectorReg, is_upper: bool, vinfo: VectInfo) {
    let mut temp1 = VectorReg::default();
    let mut temp2 = VectorReg::default();
    uxtl(&mut temp1, src1, is_upper, vinfo);
    uxtl(&mut temp2, src2, is_upper, vinfo);
    uaba(dst, temp1, temp2, vinfo);
}
pub fn sabal(dst: &mut VectorReg, src1: VectorReg, src2: VectorReg, is_upper: bool, vinfo: VectInfo) {
    let mut temp1 = VectorReg::default();
    let mut temp2 = VectorReg::default();
    sxtl(&mut temp1, src1, is_upper, vinfo);
    sxtl(&mut temp2, src2, is_upper, vinfo);
    saba(dst, temp1, temp2, vinfo);
}
pub fn saba(dst: &mut VectorReg, src1: VectorReg, src2: VectorReg, vinfo: VectInfo) {
    let mut temp = VectorReg::default();
    dst.clear_unused(vinfo);
    absdiff(&mut temp, src1, src2, true, vinfo);
    let edst = dst.clone();
    add(dst, edst, temp, vinfo);
}
pub fn umull(dst: &mut VectorReg, src1: VectorReg, src2: VectorReg, is_upper: bool, vinfo: VectInfo) {
    let mut temp1 = VectorReg::default();
    let mut temp2 = VectorReg::default();
    uxtl(&mut temp1, src1, is_upper, vinfo);
    uxtl(&mut temp2, src2, is_upper, vinfo);
    mul(dst, temp1, temp2, vinfo);
}
pub fn smull(dst: &mut VectorReg, src1: VectorReg, src2: VectorReg, is_upper: bool, vinfo: VectInfo) {
    let mut temp1 = VectorReg::default();
    let mut temp2 = VectorReg::default();
    sxtl(&mut temp1, src1, is_upper, vinfo);
    sxtl(&mut temp2, src2, is_upper, vinfo);
    mul(dst, temp1, temp2, vinfo);
}
pub fn umlsl(dst: &mut VectorReg, src1: VectorReg, src2: VectorReg, is_upper: bool, vinfo: VectInfo) {
    let mut temp1 = VectorReg::default();
    let mut temp2 = VectorReg::default();
    uxtl(&mut temp1, src1, is_upper, vinfo);
    uxtl(&mut temp2, src2, is_upper, vinfo);
    mls(dst, dst.clone(), temp1, temp2, vinfo);
}
pub fn smlsl(dst: &mut VectorReg, src1: VectorReg, src2: VectorReg, is_upper: bool, vinfo: VectInfo) {
    let mut temp1 = VectorReg::default();
    let mut temp2 = VectorReg::default();
    uxtl(&mut temp1, src1, is_upper, vinfo);
    uxtl(&mut temp2, src2, is_upper, vinfo);
    mls(dst, dst.clone(), temp1, temp2, vinfo);
}
pub fn umlal(dst: &mut VectorReg, src1: VectorReg, src2: VectorReg, is_upper: bool, vinfo: VectInfo) {
    let mut temp1 = VectorReg::default();
    let mut temp2 = VectorReg::default();
    uxtl(&mut temp1, src1, is_upper, vinfo);
    uxtl(&mut temp2, src2, is_upper, vinfo);
    mla(dst, dst.clone(), temp1, temp2, vinfo);
}
pub fn smlal(dst: &mut VectorReg, src1: VectorReg, src2: VectorReg, is_upper: bool, vinfo: VectInfo) {
    let mut temp1 = VectorReg::default();
    let mut temp2 = VectorReg::default();
    sxtl(&mut temp1, src1, is_upper, vinfo);
    sxtl(&mut temp2, src2, is_upper, vinfo);
    mla(dst, dst.clone(), temp1, temp2, vinfo);
}
pub fn sqdmull(dst: &mut VectorReg, src1: VectorReg, src2: VectorReg, is_upper: bool, vinfo: VectInfo) {
    let mut temp1 = VectorReg::default();
    smull(&mut temp1, src1, src2, is_upper, vinfo);
    add(dst, temp1, temp1, vinfo);
    dst.signed_saturate(vinfo);
}
pub fn sqdmlal(dst: &mut VectorReg, src1: VectorReg, src2: VectorReg, is_upper: bool, vinfo: VectInfo) {
    let mut temp1 = VectorReg::default();
    sqdmull(&mut temp1, src1, src2, is_upper, vinfo);
    add(dst, dst.clone(), temp1, vinfo);
    dst.signed_saturate(vinfo);
}
pub fn sqdmlsl(dst: &mut VectorReg, src1: VectorReg, src2: VectorReg, is_upper: bool, vinfo: VectInfo) {
    let mut temp1 = VectorReg::default();
    sqdmull(&mut temp1, src1, src2, is_upper, vinfo);
    sub(dst, dst.clone(), temp1, vinfo);
    dst.signed_saturate(vinfo);
}
pub fn smulh(dst: &mut VectorReg, src1: VectorReg, src2: VectorReg, vinfo: VectInfo) {
    for i in 0..vinfo.lane_count {
        let mut finval: u64 = 0;
        let val1 = src1.get_elem_signed_fixed(i, vinfo);
        let val2 = src2.get_elem_signed_fixed(i, vinfo);
        match vinfo.elem_size {
            8 => {
                let mut v = (val1 as i8 as i16) * (val2 as i8 as i16);
                finval = ((v >> 8) & 0xff) as u64;
            }
            16 => {
                let mut v = (val1 as i16 as i32) * (val2 as i16 as i32);
                finval = ((v >> 16) & 0xffff) as u64;
            }
            32 => {
                let mut v = (val1 as i32 as i64) * (val2 as i32 as i64);
                finval = ((v >> 32) & 0xffffffff) as u64;
            }
            64 => {
                let mut v = (val1 as i64 as i128) * (val2 as i64 as i128);
                finval = (v >> 64) as u64;
            },
            _ => panic!()
        }
        dst.set_elem_fixed(finval, i, vinfo);

    }
}
pub fn shl(dst: &mut VectorReg, src: VectorReg, shift: u64, vinfo: VectInfo) {
    let shiftreg = dup_imm(shift, vinfo);
    ushl(dst, src, shiftreg, vinfo);
}
pub fn sshll(dst: &mut VectorReg, src: VectorReg, shift: u64, is_upper: bool, vinfo: VectInfo) {
    let shiftreg = dup_imm(shift, vinfo);
    let mut extendedreg = VectorReg::default();
    sxtl(&mut extendedreg, src, is_upper, vinfo);
    sshl(dst, extendedreg, shiftreg, vinfo);
}
pub fn shll(dst: &mut VectorReg, src: VectorReg, is_upper: bool, vinfo: VectInfo) {
    let shift = vinfo.elem_size / 2;
    sshll(dst, src, shift as u64, is_upper, vinfo);
}
pub fn ushll(dst: &mut VectorReg, src: VectorReg, shift: u64, is_upper: bool, vinfo: VectInfo) {
    let shiftreg = dup_imm(shift, vinfo);
    let mut extendedreg = VectorReg::default();
    uxtl(&mut extendedreg, src, is_upper, vinfo);
    ushl(dst, extendedreg, shiftreg, vinfo);
}
/*
pub fn usra(dst: &mut VectorReg, src: VectorReg, shift: u64, vinfo: VectInfo) {
    let mut shiftreg = VectorReg::default();
    ushr_imm(&mut shiftreg, src, shift, vinfo);
    ushl(dst, extendedreg, shiftreg, vinfo);
}

 */
pub fn umulh(dst: &mut VectorReg, src1: VectorReg, src2: VectorReg, vinfo: VectInfo) {
    for i in 0..vinfo.lane_count {
        let mut finval: u64 = 0;
        let val1 = src1.get_elem_fixed(i, vinfo);
        let val2 = src2.get_elem_fixed(i, vinfo);
        match vinfo.elem_size {
            8 => {
                let mut v = (val1 as u8 as u16) * (val2 as u8 as u16);
                finval = ((v >> 8) & 0xff) as u64;
            }
            16 => {
                let mut v = (val1 as u16 as u32) * (val2 as u16 as u32);
                finval = ((v >> 16) & 0xffff) as u64;
            }
            32 => {
                let mut v = (val1 as u32 as u64) * (val2 as u32 as u64);
                finval = ((v >> 32) & 0xffffffff) as u64;
            }
            64 => {
                let mut v = (val1 as u64 as u128) * (val2 as u64 as u128);
                finval = (v >> 64) as u64;
            },
            _ => panic!()
        }
        dst.set_elem_fixed(finval, i, vinfo);

    }
}
pub fn sshr(dst: &mut VectorReg, src1: VectorReg, src2: VectorReg, vinfo: VectInfo) {
    let mut temp = VectorReg::default();
    neg(&mut temp, src2, vinfo);
    temp.signed_saturate(vinfo);
    sshl(dst, src1, temp, vinfo);

}
pub fn ushr_imm(dst: &mut VectorReg, src1: VectorReg, shift: u64, vinfo: VectInfo) {
    let mut temp= dup_imm((-(shift as i64) as u64), vinfo);
    temp.signed_saturate(vinfo);
    ushl(dst, src1, temp, vinfo);

}
pub fn ushr_tworeg(dst: &mut VectorReg, src1: VectorReg, src2: VectorReg, vinfo: VectInfo) {
    let mut temp = VectorReg::default();
    neg(&mut temp, src2, vinfo);
    temp.signed_saturate(vinfo);
    ushl(dst, src1, temp, vinfo);

}
pub fn sqrdmulh(dst: &mut VectorReg, src1: VectorReg, src2: VectorReg, round: bool, vinfo: VectInfo) {
    let mut esize = vinfo.elem_size;
    let round_const = if round {
        (1 << (esize - 2))
    } else {
        0
    };
    dst.clear_unused(vinfo);
    for i in 0..vinfo.lane_count {
        let mut product = src1.get_elem_signed_fixed(i, vinfo)
            * src2.get_elem_signed_fixed(i, vinfo);
        product += round_const;
        product = product >> (esize - 1);
        if product > vinfo.get_max_signed() {
            product = vinfo.get_max_signed();
        } else if product < vinfo.get_min_signed() {
            product = vinfo.get_min_signed();
        }
        dst.set_elem_signed_fixed(product, i, vinfo);
    }
}
pub fn sqdmulh(dst: &mut VectorReg, src1: VectorReg, src2: VectorReg, vinfo: VectInfo) {
    sqrdmulh(dst, src1, src2, false, vinfo)
}
pub fn absdiff(dst: &mut VectorReg, src1: VectorReg, src2: VectorReg, is_signed: bool, vinfo: VectInfo) {
    dst.clear_unused(vinfo);
    for i in 0..vinfo.lane_count {
        let src1_gt_src2 = if is_signed {
            src1.get_elem_signed_fixed(i,vinfo) > src2.get_elem_signed_fixed(i, vinfo)
        } else {
            src1.get_elem_fixed(i,vinfo) > src2.get_elem_fixed(i, vinfo)
        };
        let fval = if src1_gt_src2 {
            src1.get_elem_fixed(i,vinfo) - src2.get_elem_fixed(i, vinfo)
        } else {
            src2.get_elem_fixed(i,vinfo) - src1.get_elem_fixed(i, vinfo)
        };
        dst.set_elem_fixed(fval, i, vinfo);
    }
}
pub fn trn<T: num::PrimInt>(ai: &mut Arm64Cpu, rn: usize, rm: usize, rd: usize, op: usize, vinfo: VectInfo) {
    let rnval = ai.vreg[rn];
    let rmval = ai.vreg[rm];
    let mut result: [u64; 16] = [0; 16];
    let pairs = vinfo.lane_count / 2;
    for i in 0..pairs {
        result[2 * i] = rnval.get_elem_fixed((2 * i) + op, vinfo);
        result[(2 * i) + 1] = rmval.get_elem_fixed((2 * i) + op, vinfo);
    }
    ai.vreg[rd].set_from_array(&mut result, vinfo);
}

 */
#[derive(Copy, Clone)]
pub enum GenCmpOps {
    Eq,
    Ge,
    Gt,
    Le,
    Lt,
    Ne,
}
pub fn cmp(dst: &mut VectorReg, src1: &VectorReg, src2: &VectorReg, vinfo: VectInfo,
           is_signed: bool, cnd: GenCmpOps) {
    dst.clear_vect();
    for i in 0..vinfo.lane_count {
        let a = src1.get_elem_fixed(i, vinfo);
        let b = src2.get_elem_fixed(i, vinfo);
        let sa = src1.get_elem_signed_fixed(i, vinfo);
        let sb = src2.get_elem_signed_fixed(i, vinfo);
        let condpass = match cnd {
            GenCmpOps::Eq => {
                a == b
            }
            GenCmpOps::Ge => {
                if is_signed {
                    sa >= sb
                } else {
                    a >= b
                }
            }
            GenCmpOps::Gt => {
                if is_signed {
                    sa > sb
                } else {
                    a > b
                }
            }
            GenCmpOps::Le => {
                if is_signed {
                    sa <= sb
                } else {
                    a <= b
                }            }
            GenCmpOps::Lt => {
                if is_signed {
                    sa < sb
                } else {
                    a < b
                }
            }
            GenCmpOps::Ne => {
                a != b
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
