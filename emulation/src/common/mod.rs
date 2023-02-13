use std::ops::{BitAnd, BitOr, BitXor};
use jit::main::JitBackend;
use crate::common::memory::MemEndian;

pub mod memory;
pub mod genfunc;
pub mod floating_wrappers;
pub mod arm_crypto;
pub mod vect;
pub mod arm_fp_defs;
mod arm_fp_ops;
pub mod arm_common;

#[cfg(target_endian = "little")]
pub const IS_LITTLE_ENDIAN: bool = true;
#[cfg(target_endian = "big")]
pub const IS_LITTLE_ENDIAN: bool = false;
#[cfg(target_endian = "little")]
pub fn host_guest_endian_mismatch(me: MemEndian) -> bool {
    if me == MemEndian::Big {
        true
    } else {
        false
    }
}
#[cfg(target_endian = "big")]
pub fn host_guest_endian_mismatch(me: MemEndian) -> bool {
    if me == MemEndian::Big {
        false
    } else {
        true
    }
}
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum GenericCompare {
    LessThan,
    LessThanEqual,
    Equal,
    GreaterThan,
    GreaterThanEqual,
}
pub fn generic_cmp_fn<T: num::PrimInt>(op1: T, op2: T, gencmp: GenericCompare) -> bool {
    match gencmp {
        GenericCompare::LessThan => op1 < op2,
        GenericCompare::LessThanEqual => op1 <= op2,
        GenericCompare::Equal => op1 == op2,
        GenericCompare::GreaterThan => op1 > op2,
        GenericCompare::GreaterThanEqual => op1 >= op2
    }
}
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum GenericLogical {
    Or,
    And,
    Xor,
}

pub fn generic_logic_fn<T: BitAnd<Output = T> + BitOr<Output = T> + BitXor<Output = T>>(op1: T, op2: T, genlog: GenericLogical) -> T {
    match genlog {
        GenericLogical::Or => op1 | op2,
        GenericLogical::And => op1 & op2,
        GenericLogical::Xor => op1 ^ op2
    }
}
pub fn signext_arbpos(val: u64, bitpos: u64) -> u64 {
    let mut mask: u64 = 0xffffffffffffffff << bitpos; // bitpos use "natural" counting
    if val & (1 << (bitpos - 1)) != 0 {
        val | mask
    } else {
        val
    }

}
pub fn val_cutoff(val: u64, bitpos: u64) -> u64 {
    let mut mask: u64 = 0xffffffffffffffff << bitpos;
    val & !mask

}
pub const BIT64_MASK: u64 = 0xffffffffffffffff;
pub const BIT32_MASK: u64 = 0xffffffff;
pub fn place_variable_guest_fmt_64(val: u64, dst: &mut u64, end: MemEndian) {
    let mut useval = val;
    if (IS_LITTLE_ENDIAN && end == MemEndian::Big)
        || (!IS_LITTLE_ENDIAN && end == MemEndian::Little) {
        useval = useval.swap_bytes();
    };
    *dst = useval;

}
pub fn place_variable_guest_fmt_32(val: u32, dst: &mut u32, end: MemEndian) {
    let mut useval = val;
    if (IS_LITTLE_ENDIAN && end == MemEndian::Big)
        || (!IS_LITTLE_ENDIAN && end == MemEndian::Little) {
        useval = useval.swap_bytes();
    };
    *dst = useval;
}
pub fn place_variable_guest_fmt_16(val: u32, dst: &mut u32, end: MemEndian) {
    let mut useval = val;
    if (IS_LITTLE_ENDIAN && end == MemEndian::Big)
        || (!IS_LITTLE_ENDIAN && end == MemEndian::Little) {
        useval = useval.swap_bytes();
    };
    *dst = useval;
}
pub fn get_variable_guest_fmt_32(dst: u32, end: MemEndian) -> u32 {
    let mut useval = dst;
    if (IS_LITTLE_ENDIAN && end == MemEndian::Big)
        || (!IS_LITTLE_ENDIAN && end == MemEndian::Little) {
        useval = useval.swap_bytes();
    };
    useval
}
pub fn get_variable_guest_fmt_64(dst: u64, end: MemEndian) -> u64 {
    let mut useval = dst;
    if (IS_LITTLE_ENDIAN && end == MemEndian::Big)
        || (!IS_LITTLE_ENDIAN && end == MemEndian::Little) {
        useval = useval.swap_bytes();
    };
    useval
}
pub fn get_variable_guest_fmt_16(dst: u16, end: MemEndian) -> u16 {
    let mut useval = dst;
    if (IS_LITTLE_ENDIAN && end == MemEndian::Big)
        || (!IS_LITTLE_ENDIAN && end == MemEndian::Little) {
        useval = useval.swap_bytes();
    };
    useval
}
// passthrough struct between frontends and the jit (which uses thrad local storage)
/*pub struct TLSJitVar {

}
impl JitBackend for TLSJitVar {

}

 */