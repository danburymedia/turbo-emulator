use std::borrow::BorrowMut;
use crate::threaded::x64::global_thread_ptr;

#[inline(always)]
fn set_reg(d: u64, val: u64) {
    unsafe {
        global_thread_ptr.with(|z| {
            //let mut val = z;

            (**z).regs[d as usize] = val;
        });

    }
}
#[inline(always)]
fn get_reg(idx: u64) -> u64 {
    unsafe {
        global_thread_ptr.with(|mut z| {
            (**z).regs[idx as usize]
        })
    }
}
#[inline(always)]
pub fn extsw(d: u64, s: u64) {
    let aval = get_reg(s) as u32 as i32 as i64;
    set_reg(d, aval as u64);
}
#[inline(always)]
pub fn extuw(d: u64, s: u64) {
    let aval = get_reg(s) as u32 as u64;
    set_reg(d, aval);
}
#[inline(always)]
pub fn extsh(d: u64, s: u64) {
    let aval = get_reg(s) as u16 as i16 as i64;
    set_reg(d, aval as u64);
}
#[inline(always)]
pub fn extuh(d: u64, s: u64) {
    let aval = get_reg(s) as u16 as u64;
    set_reg(d, aval);
}
#[inline(always)]
pub fn extsb(d: u64, s: u64) {
    let aval = get_reg(s) as u8 as i8 as i64;
    set_reg(d, aval as u64);
}
#[inline(always)]
pub fn extub(d: u64, s: u64) {
    let aval = get_reg(s) as u8 as u64;
    set_reg(d, aval);
}
#[inline(always)]
pub fn ceq(d: u64, a: u64, b: u64) {
    let aval = get_reg(a);
    let bval = get_reg(b);
    let cval = if aval == bval { 1 } else { 0 };
    set_reg(d, cval);
}
#[inline(always)]
pub fn cne(d: u64, a: u64, b: u64) {
    let aval = get_reg(a);
    let bval = get_reg(b);
    let cval = if aval != bval { 1 } else { 0 };
    set_reg(d, cval);
}
// signed lower, greater
#[inline(always)]
pub fn cslel(d: u64, a: u64, b: u64) {
    let aval = get_reg(a) as i64;
    let bval = get_reg(b) as i64;
    let cval = if aval <= bval { 1 } else { 0 };
    set_reg(d, cval);
}
#[inline(always)]
pub fn cslew(d: u64, a: u64, b: u64) {
    let aval = get_reg(a) as i32;
    let bval = get_reg(b) as i32;
    let cval = if aval <= bval { 1 } else { 0 };
    set_reg(d, cval);

}
#[inline(always)]
pub fn csltl(d: u64, a: u64, b: u64) {
    let aval = get_reg(a) as i64;
    let bval = get_reg(b) as i64;
    let cval = if aval < bval { 1 } else { 0 };
    set_reg(d, cval);
}
#[inline(always)]
pub fn csltw(d: u64, a: u64, b: u64) {
    let aval = get_reg(a) as i32;
    let bval = get_reg(b) as i32;
    let cval = if aval < bval { 1 } else { 0 };
    set_reg(d, cval);
}
#[inline(always)]
pub fn csgel(d: u64, a: u64, b: u64) {
    let aval = get_reg(a) as i64;
    let bval = get_reg(b) as i64;
    let cval = if aval >= bval { 1 } else { 0 };
    set_reg(d, cval);
}
#[inline(always)]
pub fn csgew(d: u64, a: u64, b: u64) {
    let aval = get_reg(a) as i32;
    let bval = get_reg(b) as i32;
    let cval = if aval >= bval { 1 } else { 0 };
    set_reg(d, cval);
}
#[inline(always)]
pub fn csgtl(d: u64, a: u64, b: u64) {
    let aval = get_reg(a) as i64;
    let bval = get_reg(b) as i64;
    let cval = if aval > bval { 1 } else { 0 };
    set_reg(d, cval);
}
#[inline(always)]
pub fn csgtw(d: u64, a: u64, b: u64) {
    let aval = get_reg(a) as i32;
    let bval = get_reg(b) as i32;
    let cval = if aval > bval { 1 } else { 0 };
    set_reg(d, cval);
}
// unsigned lower, greater
#[inline(always)]
pub fn culel(d: u64, a: u64, b: u64) {
    let aval = get_reg(a);
    let bval = get_reg(b);
    let cval = if aval <= bval { 1 } else { 0 };
    set_reg(d, cval);
}
#[inline(always)]
pub fn culew(d: u64, a: u64, b: u64) {
    let aval = get_reg(a) as u32;
    let bval = get_reg(b) as u32;
    let cval = if aval <= bval { 1 } else { 0 };
    set_reg(d, cval);
}
#[inline(always)]
pub fn cultl(d: u64, a: u64, b: u64) {
    let aval = get_reg(a);
    let bval = get_reg(b);
    let cval = if aval < bval { 1 } else { 0 };
    set_reg(d, cval);
}
#[inline(always)]
pub fn cultw(d: u64, a: u64, b: u64) {
    let aval = get_reg(a) as u32;
    let bval = get_reg(b) as u32;
    let cval = if aval < bval { 1 } else { 0 };
    set_reg(d, cval);
}
#[inline(always)]
pub fn cugel(d: u64, a: u64, b: u64) {
    let aval = get_reg(a);
    let bval = get_reg(b);
    let cval = if aval >= bval { 1 } else { 0 };
    set_reg(d, cval);
}
#[inline(always)]
pub fn cugew(d: u64, a: u64, b: u64) {
    let aval = get_reg(a) as u32;
    let bval = get_reg(b) as u32;
    let cval = if aval >= bval { 1 } else { 0 };
    set_reg(d, cval);
}
#[inline(always)]
pub fn cugtl(d: u64, a: u64, b: u64) {
    let aval = get_reg(a);
    let bval = get_reg(b);
    let cval = if aval > bval { 1 } else { 0 };
    set_reg(d, cval);
}
#[inline(always)]
pub fn cugtw(d: u64, a: u64, b: u64) {
    let aval = get_reg(a) as u32;
    let bval = get_reg(b) as u32;
    let cval = if aval > bval { 1 } else { 0 };
    set_reg(d, cval);
}
#[inline(always)]
pub fn load_ext_reg(d: u64, a: u64) {
    unsafe {
        let aval = global_thread_ptr.with(|mut z| {
            (**z).temp_regs[a as usize]
        });
        set_reg(d, aval);
    }
}
#[inline(always)]
pub fn mov_reg(d: u64, s: u64) {
    let aval = get_reg(s);
    set_reg(d, aval);
}
#[inline(always)]
pub fn mov_temp(d: u64, s: u64) {
    unsafe {
        let aval = global_thread_ptr.with(|mut z| {
            (**z).temp_regs[s as usize]
        });
        global_thread_ptr.with(|mut z| {
            (**z).temp_regs[d as usize] = aval;
        });
    }
}
#[inline(always)]
pub fn store_ext_reg(d: u64, a: u64) {
    unsafe {
        let aval = get_reg(a);
        global_thread_ptr.with(|mut z| {
            (**z).temp_regs[d as usize] = aval;
        });
    }
}
#[inline(always)]
pub fn load_imm64(d: u64, imm64: u64) {
    set_reg(d, imm64);
}
#[inline(always)]
pub fn temp_load_imm64(d: u64, imm64: u64) {
    unsafe {
        global_thread_ptr.with(|mut z| {
            (**z).temp_regs[d as usize] = imm64;
        });
    }
}
#[inline(always)]
pub fn add64(d: u64, a: u64, b: u64) {
    let aval = get_reg(a);
    let bval = get_reg(b);
    set_reg(d, aval.wrapping_add(bval));
}
#[inline(always)]
pub fn sub(d: u64, a: u64, b: u64) {
    let aval = get_reg(a);
    let bval = get_reg(b);
    set_reg(d, aval.wrapping_sub(bval));
}
#[inline(always)]
pub fn add32(d: u64, a: u64, b: u64) {
    let aval = get_reg(a) as u32;
    let bval = get_reg(b) as u32;
    set_reg(d, aval.wrapping_add(bval) as u64);
}
#[inline(always)]
pub fn udiv64(d: u64, a: u64, b: u64) {
    let aval = get_reg(a);
    let bval = get_reg(b);
    set_reg(d, aval.wrapping_div(bval));
}
#[inline(always)]
pub fn udiv32(d: u64, a: u64, b: u64) {
    let aval = get_reg(a) as u32;
    let bval = get_reg(b) as u32;
    set_reg(d, aval.wrapping_div(bval) as u64);
}
#[inline(always)]
pub fn sdiv64(d: u64, a: u64, b: u64) {
    let aval = get_reg(a) as i64;
    let bval = get_reg(b) as i64;
    set_reg(d, aval.wrapping_div(bval) as u64);
}
#[inline(always)]
pub fn sdiv32(d: u64, a: u64, b: u64) {
    let aval = get_reg(a) as i32;
    let bval = get_reg(b) as i32;
    set_reg(d, aval.wrapping_div(bval) as u64);
}
#[inline(always)]
pub fn mul64l(d: u64, a: u64, b: u64) {
    let aval = get_reg(a);
    let bval = get_reg(b);
    set_reg(d, aval.wrapping_mul(bval));
}
#[inline(always)]
pub fn umul64h(d: u64, a: u64, b: u64) {
    let aval = get_reg(a) as u128;
    let bval = get_reg(b) as u128;
    let fval = ((aval.wrapping_mul(bval) >> 64) as u64);
    set_reg(d, fval);
}
#[inline(always)]
pub fn smul64h(d: u64, a: u64, b: u64) {
    let aval = get_reg(a) as i64 as i128;
    let bval = get_reg(b) as i64 as i128;
    let fval = ((aval.wrapping_mul(bval) >> 64) as u64);
    set_reg(d, fval);
}
#[inline(always)]
pub fn mul32l(d: u64, a: u64, b: u64) {
    let aval = get_reg(a) as u32;
    let bval = get_reg(b) as u32;
    set_reg(d, aval.wrapping_mul(bval) as u64);
}

#[inline(always)]
pub fn umul32h(d: u64, a: u64, b: u64) {
    let aval = get_reg(a) as u32 as u64;
    let bval = get_reg(b) as u32 as u64;
    let fval = ((aval.wrapping_mul(bval) >> 32) as u32) as u64;
    set_reg(d, fval);
}
#[inline(always)]
pub fn smul32h(d: u64, a: u64, b: u64) {
    let aval = get_reg(a) as i32 as i64;
    let bval = get_reg(b) as i32 as i64;
    let fval = ((aval.wrapping_mul(bval) >> 32) as u32) as u64;
    set_reg(d, fval);
}
#[inline(always)]
pub fn neg64(d: u64, a: u64) {
    let aval = get_reg(a);
    let fval = !aval + 1;
    set_reg(d, fval);
}
#[inline(always)]
pub fn neg32(d: u64, a: u64) {
    let aval = get_reg(a) as u32;
    let fval = !aval + 1;
    set_reg(d, fval as u64);
}
#[inline(always)]
pub fn urem64(d: u64, a: u64, b: u64) {
    let aval = get_reg(a);
    let bval = get_reg(b);
    set_reg(d, aval.wrapping_rem(bval));
}
#[inline(always)]
pub fn urem32(d: u64, a: u64, b: u64) {
    let aval = get_reg(a) as u32;
    let bval = get_reg(b) as u32;
    set_reg(d, aval.wrapping_rem(bval) as u64);
}
#[inline(always)]
pub fn srem64(d: u64, a: u64, b: u64) {
    let aval = get_reg(a) as i64;
    let bval = get_reg(b) as i64;
    set_reg(d, aval.wrapping_rem(bval) as u64);
}
#[inline(always)]
pub fn srem32(d: u64, a: u64, b: u64) {
    let aval = get_reg(a) as i32;
    let bval = get_reg(b) as i32;
    set_reg(d, aval.wrapping_rem(bval) as u64);
}
// for or, xor, and, we can cast if we dont need upper bits
#[inline(always)]
pub fn or(d: u64, a: u64, b: u64) {
    let aval = get_reg(a) as i64;
    let bval = get_reg(b) as i64;
    set_reg(d, (aval | bval) as u64);
}
#[inline(always)]
pub fn and(d: u64, a: u64, b: u64) {
    let aval = get_reg(a) as i64;
    let bval = get_reg(b) as i64;
    set_reg(d, (aval & bval) as u64);
}
#[inline(always)]
pub fn xor(d: u64, a: u64, b: u64) {
    let aval = get_reg(a) as i64;
    let bval = get_reg(b) as i64;
    set_reg(d, (aval ^ bval) as u64);
}
#[inline(always)]
pub fn sar(d: u64, a: u64, b: u64) {
    let aval = get_reg(a) as i64;
    let shft = get_reg(b) as i64;
    set_reg(d, (aval >> shft) as u64);
}
#[inline(always)]
pub fn shr(d: u64, a: u64, b: u64) {
    let aval = get_reg(a) as u64;
    let shft = get_reg(b) as u64;
    set_reg(d, (aval >> shft) as u64);
}
#[inline(always)]
pub fn shl(d: u64, a: u64, b: u64) {
    let aval = get_reg(a) as u64;
    let shft = get_reg(b) as u64;
    set_reg(d, (aval << shft) as u64);
}

