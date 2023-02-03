
use crate::riscv::common::{Xlen, RiscvArgs};
use crate::riscv::interpreter::defs::sign_ext_imm;
use crate::riscv::interpreter::main::{RiscvInt};

pub fn common_s(ri: &mut RiscvInt, arg: &RiscvArgs, length: u8) {
    // if error than we can short circuit here, callers reutrn after this
    // major todo
    let addr = ri.cull_reg(ri.regs[arg.rs1 as usize].wrapping_add(sign_ext_imm(arg.imm)));
    let data = ri.regs[arg.rs2 as usize];
    let err = match length {
        1 => ri.write8(addr, data as u8, true),
        2 => ri.write16(addr, data as u16, true),
        4 => ri.write32(addr, data as u32, true),
        8 => ri.write64(addr, data as u64, true),
        _ => panic!("error in common_s")
    };


}
pub fn common_l(ri: &mut RiscvInt, args: &RiscvArgs, length: u8, sign_ext: bool) {
    let addr = ri.cull_reg(ri.regs[args.rs1 as usize].wrapping_add(sign_ext_imm(args.imm)));
    let data: u64 = match length {
        1 => {
            let mut fin = match ri.read8(addr, false, true) {
                Ok(t) => t,
                Err(z) => return
            };
            if sign_ext {
                fin as i8 as i64 as u64
            } else {
                fin as u64
            }

        },
        2 => {
            let mut fin = match ri.read16(addr, false, true) {
                Ok(t) => t,
                Err(z) => return
            };
            if sign_ext {
                fin as i16 as i64 as u64
            } else {
                fin as u64
            }

        },
        4 => {
            let mut fin = match ri.read32(addr, false, true) {
                Ok(t) => t,
                Err(z) => return
            };
            if sign_ext {
                fin as i32 as i64 as u64
            } else {
                fin as u64
            }

        },
        8 => {
            let mut fin = match ri.read64(addr, false, true) {
                Ok(t) => t,
                Err(z) => return
            };
            fin

        },
        _ => panic!("how we get here?")

    };
    ri.regs[args.rd as usize] = data;
}
pub fn lb(ri: &mut RiscvInt, args: &RiscvArgs) {
    common_l(ri, args, 1, true);
    
}

pub fn lh(ri: &mut RiscvInt, args: &RiscvArgs) {
    common_l(ri, args, 2, true);
    

}

pub fn lw(ri: &mut RiscvInt, args: &RiscvArgs) {
    common_l(ri, args, 4, true);
    
}

pub fn lbu(ri: &mut RiscvInt, args: &RiscvArgs) {
    common_l(ri, args, 1, false);
    
}

pub fn lhu(ri: &mut RiscvInt, args: &RiscvArgs) {
    common_l(ri, args, 2, false);
    

}

pub fn sb(ri: &mut RiscvInt, args: &RiscvArgs) {
    common_s(ri, args, 1);
    
}

pub fn sh(ri: &mut RiscvInt, args: &RiscvArgs) {
    common_s(ri, args, 2);
}

pub fn sw(ri: &mut RiscvInt, args: &RiscvArgs) {
    common_s(ri, args, 4);
    
}

pub fn lwu(ri: &mut RiscvInt, args: &RiscvArgs) {
    common_l(ri, args, 4, false);
    
}

pub fn ld(ri: &mut RiscvInt, args: &RiscvArgs) {
    common_l(ri, args, 8, false);
    
}

pub fn sd(ri: &mut RiscvInt, args: &RiscvArgs) {
    common_s(ri, args, 8);
    
}