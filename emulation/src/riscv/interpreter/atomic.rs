use std::sync::atomic::Ordering;
use crate::riscv::common::{Xlen, RiscvArgs, Trap};
use crate::riscv::interpreter::main::{RiscvInt};
use crate::riscv::mem::MemAccessCircumstances;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum AtomicOps {
    Swap,
    Add,
    And,
    Or,
    Xor,
    Max,
    Min
}
fn gen_atomic_32(ri: &mut RiscvInt, op: AtomicOps, gg: &RiscvArgs) {
    /*
    todo: to make true atomic, here is an idea
     first, load value, then do op, then write only if mem value matches original.
     if it doesn't, restart process with new value, or report failure

     */
    let addr = ri.regs[gg.rs1 as usize];
    let dat1 = match ri.read32(addr, false, true) {
        Err(z) => {
            return;
        },
        Ok(res) => {
            res
        }
    };
    let dat2 = ri.regs[gg.rs2 as usize] as u32;
    let res = match op {
        AtomicOps::Swap => {
            dat2
        }
        AtomicOps::Add => {
            dat1.wrapping_add(dat2)
        }
        AtomicOps::And => {
            dat1 & dat2
        }
        AtomicOps::Or => {
            dat1 | dat2
        }
        AtomicOps::Xor => {
            dat1 ^ dat2

        }
        AtomicOps::Max => {
            match dat2 as u32 >= dat1 as u32 {
                true => dat2,
                false => dat1
            }
        }
        AtomicOps::Min => {
            match dat2 as u32 >= dat1 as u32 {
                true => dat1,
                false => dat2
            }
        }
    };
    match ri.write32(addr, res, true) {
        Err(_) => {
            return;
        },
        Ok(_) => { }
    };
    ri.regs[gg.rd as usize] = ri.sign_ext(dat1 as u64);
}
fn gen_atomic_64(ri: &mut RiscvInt, op: AtomicOps, gg: &RiscvArgs) {
    /*
    todo: to make true atomic, here is an idea
     first, load value, then do op, then write only if mem value matches original.
     if it doesn't, restart process with new value, or report failure

     */
    let addr = ri.regs[gg.rs1 as usize];
    let dat1 = match ri.read64(addr, false, true) {
        Err(z) => {
            return;
        },
        Ok(res) => {
            res
        }
    };
    let dat2 = ri.regs[gg.rs2 as usize];
    let res = match op {
        AtomicOps::Swap => {
            ri.regs[gg.rs2 as usize]
        }
        AtomicOps::Add => {
            dat1.wrapping_add(dat2)
        }
        AtomicOps::And => {
            dat1 & dat2
        }
        AtomicOps::Or => {
            dat1 | dat2
        }
        AtomicOps::Xor => {
            dat1 ^ dat2

        }
        AtomicOps::Max => {
            match dat2 >= dat1 {
                true => dat2,
                false => dat1
            }
        }
        AtomicOps::Min => {
            match dat2 >= dat1 {
                true => dat1,
                false => dat2
            }
        }
    };
    match ri.write64(addr, res, true) {
        Err(_) => {
            return;
        },
        Ok(_) => { }
    };
    ri.regs[gg.rd as usize] = dat1;
}
pub fn amoadd_d(ri: &mut RiscvInt, args: &RiscvArgs) {
    gen_atomic_64(ri, AtomicOps::Add, args);
}
pub fn amoor_d(ri: &mut RiscvInt, args: &RiscvArgs) {
    gen_atomic_64(ri, AtomicOps::Or, args);
}
pub fn amoadd_w(ri: &mut RiscvInt, args: &RiscvArgs) {
    gen_atomic_32(ri, AtomicOps::Add, args);
}
pub fn sc_w(ri: &mut RiscvInt, args: &RiscvArgs) {
    let addr = ri.regs[args.rs1 as usize];
    let val = ri.regs[args.rs2 as usize] as u32;

    if ri.is_reservation && (ri.res_len == 4) && (addr == ri.res_val) {
        ri.is_reservation = false;
        match ri.write32(addr, val, true) {
            Err(_) => {
                ri.regs[args.rd as usize] = 1;
                return;
            },
            Ok(_) => { }
        };
        ri.regs[args.rd as usize] = 0;
    } else {
        ri.regs[args.rd as usize] = 1;
    }
}

pub fn sc_d(ri: &mut RiscvInt, args: &RiscvArgs) {
    let addr = ri.regs[args.rs1 as usize];
    let val = ri.regs[args.rs2 as usize];

    if ri.is_reservation && (ri.res_len == 8) && (addr == ri.res_val) {
        ri.is_reservation = false;
        match ri.write64(addr, val, true) {
            Err(_) => {
                ri.regs[args.rd as usize] = 1;
                return;
            },
            Ok(_) => { }
        };
        ri.regs[args.rd as usize] = 0;
    } else {
        ri.is_reservation = false;
        ri.regs[args.rd as usize] = 1;
    }
}
pub fn lr_w(ri: &mut RiscvInt, args: &RiscvArgs) {
    let addr = ri.regs[args.rs1 as usize];
    if let Ok(data) = ri.read32(addr, false, true) {
        ri.is_reservation = true;
        ri.res_len = 4;
        ri.res_val = addr;
        ri.regs[args.rd as usize] = data as i32 as i64 as u64;
    }
}
pub fn lr_d(ri: &mut RiscvInt, args: &RiscvArgs) {
    let addr = ri.regs[args.rs1 as usize];
    if let Ok(data) = ri.read64(addr, false, true) {
        ri.is_reservation = true;
        ri.res_len = 8;
        ri.res_val = addr;
        ri.regs[args.rd as usize] = data as i64 as u64;
    }
}
pub fn amoswap_w(ri: &mut RiscvInt, args: &RiscvArgs) {
    // todo: actually do correctly with atomptr
    let addr = ri.regs[args.rs1 as usize];
    let val = ri.regs[args.rs2 as usize] as u32;
    /* let getval = match ri.swap32imm(addr, val, Ordering::SeqCst, false, true) {
        Ok(z) => {
            z
        }
        Err(_) => {
            return;
        }
    };
    ri.regs[args.rd as usize] = getval as i32 as i64 as u64;

     */
    let tmp = match ri.read32(addr, false, true) {
        Ok(z) => z,
        Err(_) => {
            return;
        }
    };
    match ri.write32(addr, val,false) {
        Ok(_) => {},
        Err(_) => {
            return;
        }
    };
    ri.regs[args.rd as usize] = tmp as i32 as i64 as u64;
}