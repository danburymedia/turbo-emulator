use std::mem;
use num::traits::WrappingAdd;
use crate::common::arm_fp_defs::Flags;
use crate::armv8::interpreter::main::Arm64Cpu;

#[derive(Clone, Copy, Debug)]
pub enum ExtendType {
    UXTB = 0,
    UXTH,
    UXTW,
    UXTX,
    SXTB, 
    SXTH, 
    SXTW, 
    SXTX,

}
impl ExtendType {
    pub fn num2type(num: u8) -> Self {
        match num {
            0 => ExtendType::UXTB,
            1 => ExtendType::UXTH,
            2 => ExtendType::UXTW,
            3 => ExtendType::UXTX,
            4 => ExtendType::SXTB,
            5 => ExtendType::SXTH,
            6 => ExtendType::SXTW,
            7 => ExtendType::SXTX,
            _ => panic!()
        }
    }
    pub fn type2num(&self) -> u8 {
        *self as u8
    }
}
/* #[derive(Clone, Copy, Debug)]
pub enum Condition {
    Eq = 0,
    Ne = 1,
    CsHs = 2,
    CcLo = 3,
    Mi = 4,
    Pl = 5,
    Vs = 6,
    Vc = 7,
    Hi = 8,
    Ls = 9,
    Ge = 10,
    Lt = 11,
    Gt = 12,
    Le = 13,
    Al = 14,
    Nv = 15  // Behaves as always/al.

}
impl Condition {
    // if decoder works properly, cannot fail because max value is 15
    pub fn num2type(num: u8) -> Self {
        todo!();
    }
    pub fn type2num(&self) -> u8 {
        *self as u8
    }
}
pub fn condition_pass(ai: &mut Arm64Cpu, cond: Condition) -> bool {

    match cond {
        Condition::Eq => ai.get_z_flag(),
        Condition::Ne => !ai.get_z_flag(),
        Condition::CsHs => ai.get_c_flag(),
        Condition::CcLo => !ai.get_c_flag(),
        Condition::Mi => ai.get_n_flag(),
        Condition::Pl => !ai.get_n_flag(),
        Condition::Vs => ai.get_v_flag(),
        Condition::Vc => !ai.get_v_flag(),
        Condition::Hi => ai.get_c_flag() && !ai.get_z_flag(),
        Condition::Ls => !(ai.get_c_flag() && !ai.get_z_flag()),
        Condition::Ge => ai.get_n_flag() == ai.get_v_flag(),
        Condition::Lt => ai.get_n_flag() != ai.get_v_flag(),
        Condition::Gt => !ai.get_z_flag() && (ai.get_n_flag() == ai.get_v_flag()),
        Condition::Le => !(!ai.get_z_flag() && (ai.get_n_flag() == ai.get_v_flag())),
        Condition::Al => true,
        Condition::Nv => true
    }
}


 */
pub fn get_info_from_type(exttype: ExtendType) -> (bool, u64) {
    match exttype {
        ExtendType::SXTB => (false, 8 as u64),
        ExtendType::SXTH => (false, 16 as u64),
        ExtendType::SXTW => (false, 32 as u64),
        ExtendType::SXTX => (false, 64 as u64),
        ExtendType::UXTB => (true, 8 as u64),
        ExtendType::UXTH => (true, 16 as u64),
        ExtendType::UXTW => (true, 32 as u64),
        ExtendType::UXTX => (true, 64 as u64),
    }
}
#[derive(Clone, Copy, Debug)]
pub enum ShiftType {
    LSL = 0,
    LSR,
    ASR,
    ROR,
}
impl ShiftType {
    pub fn num2type(num: u8) -> Self {
        match num {
            0 => ShiftType::LSL,
            1 => ShiftType::LSR,
            2 => ShiftType::ASR,
            3 => ShiftType::ROR,
            _ => panic!()
        }
    }
    pub fn type2num(&self) -> u8 {
        *self as u8
    }
}


pub fn highestbitset(val: u64, len: u64) -> Option<u64> {
    let mut bit: u64 = 1 << (len - 1);
    for i in (0..len).rev() {
        if (bit & val) != 0 {
            return Some(i);
        }
        bit >>= 1;
    }
    return None;
}
pub fn ones(amt: u64) -> u64 {
    let mut retthis: u64 = 0;
    for _ in 0..amt {
        retthis <<= 1;
        retthis |= 1;
    }
    retthis
}
/*pub fn replicate(val: u64, val_size: u64, full_len: u64) -> u64 {
    let mut repval: u64 = 0;
    for _ in 0..(full_len/val_size) {
        repval = (repval << val_size) | (val & ones(val_size));
    }
    repval
}

 */
pub fn replicate(val: u64, times: u64, width: u64) -> u64 {
    let mut realtimes = times;
    let mut realval = val;
    let mut realwidth = width;
    if realtimes == 64 {
        realval |= realval << realwidth;
        realwidth <<= 1;
        realtimes >>= 1;
    }
    if realtimes == 32 {
        realval |= realval << realwidth;
        realwidth <<= 1;
        realtimes >>= 1;
    }
    if realtimes == 16 {
        realval |= realval << realwidth;
        realwidth <<= 1;
        realtimes >>= 1;
    }
    if realtimes == 8 {
        realval |= realval << realwidth;
        realwidth <<= 1;
        realtimes >>= 1;
    }
    if realtimes == 4 {
        realval |= realval << realwidth;
        realwidth <<= 1;
        realtimes >>= 1;
    }
    if realtimes == 2 {
        realval |= realval << realwidth;
        realtimes >>= 1;
    }
    if realtimes == 1 {
        return realval;
    }
    if realtimes == 0 {
        return 0;
    }
    realval = val;
    for _ in 0..times {
        realval <<= width;
        realval |= val;
    }
    realval
}
pub fn zeroextend(val: u64, len: u64) -> u64 {
    val & ones(len)
}
pub fn ror(val: u64, actualsize: u64, rotate: u64) -> u64 {
    let mut rval = val;
    for _ in 0..rotate {
        rval = ((rval & 1) << (actualsize - 1)) | (rval >> 1);
    }
    rval
}
pub fn count_sign_bits<T: num::PrimInt>(val: T) -> u64 {
    let size = mem::size_of::<T>();
    if ((val.to_u64().unwrap() >> (size * 8 - 1)) & 1) != 0 {
        val.leading_ones() as u64
    } else {
        val.leading_zeros() as u64
    }
}
fn repeat_bits_reg(regsize: u64, val: u64, width: u64) -> u64 {
    let mut result = val & ((1 << width) - 1);
    let mut i = width;
    while i < regsize {
        result |= (result - 1);
        i *= 2;
    }
    result
}
pub fn decode_imm_bitmask(n: u32, imms: u32, immr: u32, size: u32) -> u64 {
    if n == 1 {
        if imms == 0x3f {
            return 0;
        }
        let bits = ((1 as u64) << ((imms + 1) as u64)) - 1;
        return bits.rotate_right(immr);
    } else {
        if (imms >> 1) == 0x1f {
            return 0;
        }
        let mut width: i32 = 0x20;
        while width >= 0x2 {
            let uwidth = width as u32;
            if (imms & uwidth) == 0 {
                let mask = uwidth - 1;
                if (imms & mask) == mask {
                    return 0;
                }
                let bits = ((1 as u64) << (((imms & mask) + 1) as u64)) - 1;
                let bror = rotate_right_cust(bits, immr & mask, uwidth);
                return repeat_bits_reg(size as u64, bror, uwidth as u64);
            }
            width >>= 1;
        }
    }
    unreachable!();
}
/*
pub fn decodebitmasks(immn: u8, imms: u8, immr: u8, immed: bool, data_size: u64) -> (u64, u64) {

  //  immn.hig
    // todo: handle properly (no wunrap and panic)
    let len = highestbitset((immn << 6 | (!real_imms & 0b111111) ) as u64, 6).unwrap();
    if 32 < (1 << len) {
        panic!();
    }
    if len > 6 {
        panic!(); // just be careful
    }
    let  levels: u64 = ones(len);
    if immed && ((imms as u64) & levels) == levels {
        panic!();
    }
    let s: u64 = (imms as u64) & levels;
    let r: u64 = (immr as u64) & levels;
    let diff = (s - r) & 0x3f;
    let esize = 1 << len;
    let d = diff & levels;
    let welem = zeroextend(ones(s + 1), esize);
    let telem = zeroextend(ones(d + 1), esize);
    let wmask = replicate(ror(welem, esize, r), esize, data_size);
    let tmask = replicate(telem, esize, data_size);
    (wmask, tmask)

}

 */
pub fn crc32_gen(exist_crc: u32, val: u64, len: u8, poly: u32) -> u32 {
    let mut rcrc = exist_crc.reverse_bits();
    let arr = val.to_le_bytes();
    for i in 0..len {
        let mut realb = arr[i as usize].reverse_bits();
        for _ in 0..7 {
            if (rcrc ^ (realb as u32)) & (1 << 31) != 0 {
                rcrc = (rcrc << 1) ^ poly;
            } else {
                rcrc = rcrc << 1;
            }
            realb = realb << 1;

        }
    }
    rcrc.reverse_bits()

}

pub fn perform_saturate_add(op1: u128, op2: u128, actualsize: u8) -> u128 {
    match actualsize {
        8 => (op1 as u8).saturating_add(op2 as u8) as u128,
        16 => (op1 as u16).saturating_add(op2 as u16) as u128,
        32 => (op1 as u32).saturating_add(op2 as u32) as u128,
        64 => (op1 as u64).saturating_add(op2 as u64) as u128,
        128 => (op1 as u128).saturating_add(op2 as u128) as u128,
        _ => panic!()
    }
}
pub fn perform_signed_saturate_add(op1: u128, op2: u128, actualsize: u8) -> u128 {
    match actualsize {
        8 => (op1 as i8).saturating_add(op2 as i8) as u128,
        16 => (op1 as i16).saturating_add(op2 as i16) as u128,
        32 => (op1 as i32).saturating_add(op2 as i32) as u128,
        64 => (op1 as i64).saturating_add(op2 as i64) as u128,
        128 => (op1 as i128).saturating_add(op2 as i128) as u128,
        _ => panic!()
    }
}
pub fn calc_z_flag<T: num::PrimInt>(val: T) -> bool {
    if val.is_zero() {
        true
    } else {
        false
    }
}
pub fn calc_n_flag<T: num::PrimInt>(val: T) -> bool {
    let size = mem::size_of::<T>();
    let realval = val.to_u64().unwrap();
    if ((realval >> (size * 8 - 1)) & 1) != 0 {
        true
    } else {
        false
    }
}
pub fn extend_value(value: u64,
                     extend_type: ExtendType,
                    left_shift: usize) -> u64 {
    let mut newval: u64 = value;
    match extend_type {
        ExtendType::UXTB => {
            newval &= 0xff;
        }
        ExtendType::UXTH => {
            newval &= 0xffff;

        }
        ExtendType::UXTW => {
            newval &= 0xffffffff;

        }
        ExtendType::SXTB => {
            newval &= 0xff;
            newval = newval as i8 as i64 as u64;

        }
        ExtendType::SXTH => {
            newval &= 0xffff;
            newval = newval as i16 as i64 as u64;

        }
        ExtendType::SXTW => {
            newval &= 0xffffffff;
            newval = newval as i32 as i64 as u64;
        }
        ExtendType::SXTX | ExtendType::UXTX => {}
    }
    (newval << left_shift)

}
/* pub fn extend_value<T: num::PrimInt, ST: num::PrimInt>(left: T, extend_type: ExtendType, left_shift: usize) -> T {
    let size = mem::size_of::<T>();
    let b_shift = ((size - 1) * 8);
    let h_shift = ((size - 2) * 8);
    let w_shift = ((size - 4) * 8);
    let mut newval: T = left;
    match extend_type {
        ExtendType::UXTB => {
            newval = newval.bitand(T::from(0xff).unwrap());
        }
        ExtendType::UXTH => {
            newval = newval.bitand(T::from(0xffff).unwrap());
        }
        ExtendType::UXTW => {
            newval = newval.bitand(T::from(0xffffffffu32).unwrap());
        }
        ExtendType::SXTB => {
            newval = newval.shl(b_shift).signed_shr(b_shift as u32);
        }
        ExtendType::SXTH => {
            newval = newval.shl(h_shift).signed_shr(h_shift as u32);
        }
        ExtendType::SXTW => {
            newval = newval.shl(w_shift).signed_shr(w_shift as u32);
        }
        ExtendType::UXTX | ExtendType::SXTX => {}

    }
    newval.shl(left_shift)

}

 */
pub fn add_with_carry<T: num::PrimInt + WrappingAdd>(left: T, right: T, set_flags: bool, carry_in: u8) -> (T, Option<Flags>)
{
    let mut result: T = left.wrapping_add(&right);
    let carry_t = T::from(carry_in).unwrap();
    result = result.wrapping_add(&carry_t);
    let size = mem::size_of::<T>();
    let mut retflags: Option<Flags> = None;
    if set_flags {
        let nflag = calc_n_flag(result);
        let zflag = calc_z_flag(result);
        let max_uint_2op = T::max_value() - carry_t;
        let cflag = if (left > max_uint_2op) || ((max_uint_2op - left) < right) {
            true
        } else {
            false
        };
        let sign_mask = (T::from(1).unwrap()) << (size * 8 - 1);
        let left_sign = left & sign_mask;
        let right_sign = right & sign_mask;
        let result_sign = result & sign_mask;
        let vflag = if (left_sign == right_sign) && (left_sign != result_sign) {
            true
        } else {
            false
        };
        retflags = Some(Flags {
            n: nflag,
            z: zflag,
            c: cflag,
            v: vflag
        })

    }
    (result, retflags)
}
pub fn reverse_bytes<T: num::PrimInt>(val: T, gran: usize) -> T{
    let size =  mem::size_of::<T>();
    assert!(size == 4 || size == 8);
    let mut bytes: [u8; 8] = [0; 8];
    let mut mask: u64 = 0xff00000000000000;
    for i in (0..=7).rev() {
        bytes[i] = (((val.to_u64().unwrap()) & mask) >> ((i as u64) * 8)) as u8;
        mask >>= 8;
    }
    let perm_table: [[usize; 8]; 3] = [
        [6, 7, 4, 5, 2, 3, 0, 1],
        [4, 5, 6, 7, 0, 1, 2, 3],
        [0, 1, 2, 3, 4, 5, 6, 7],

    ];
    let mut result: T = T::zero();
    for i in 0..8 {
        result = result.shl(8);
        let orby = T::from(bytes[perm_table[gran - 1][i]]).unwrap();
        result = result.bitor(orby);
    }
    return result;

}
pub fn rotate_right_cust(val: u64, rot: u32, width: u32) -> u64 {
    let realrot = rot & 63;
    let mut realval = val;
    let width_mask = (!(0 as u64)) >> (64 - width);
    if rot > 0 {
        realval &= width_mask;
        realval = (realval << (width - rot)) | (realval >> realrot);
    }
    realval & width_mask
}