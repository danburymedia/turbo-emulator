use std::ops::{Shl, Shr};

pub fn extract32(num: u32, start: u32, length: u32) -> u32 {
    (num >> start) & ((1 << length) - 1)
}
pub fn sextract32(num: u32, start: u32, length: u32) -> u32 {
    // even though result is u32, it will still be sign extended. Up to handler to convert to i32
    let ret: i32 = ((num as i32) << (32 - length - start) as i32) >> (32 - length); // for arith right shift
    return ret as u32;
}
pub fn times_2(num: u32) -> u32 {
    num.wrapping_mul(2)
}
pub fn plus_2(num: u32) -> u32 {
    num + 2
}
pub fn times_4(num: u32) -> u32 {
    num.wrapping_mul(4)
}
pub fn deposit32(num: u32, start: u32, length: u32, val:u32) -> u32 {
    let mask = (1 << length) - 1;

    (num & !(mask << start)) | ((val & mask) << start)
}
pub fn deposit128(num: u128, start: u128, length: u128, val: u128) -> u128 {
    let mask = (1 << length) - 1;

    (num & !(mask << start)) | ((val & mask) << start)

}
pub fn extract128(num: u128, start: u128, length: u128) -> u128 {
    (num >> start) & ((1 << length) - 1)

}
pub fn ex_shift_1(imm: u32) -> u32 {
    imm << 1
}
pub fn ex_shift_3(imm: u32) -> u32 {
    imm << 3
}
pub fn ex_shift_2(imm: u32) -> u32 {
    imm << 2
}
pub fn ex_shift_4(imm: u32) -> u32 {
    imm << 4
}
pub fn ex_shift_12(imm: u32) -> u32 {
    imm << 12
}
pub fn ex_plus_1(imm: u32) -> u32 {
    imm + 1
}
pub fn ex_rvc_register(reg: u32) -> u32 {
    reg + 8
}