use crate::riscv::common::{RiscvArgs, Xlen};
use crate::riscv::interpreter::main::{RiscvInt};
pub fn sign_ext_imm(val: u32) -> u64 {
    // For "I" decoded instructions.
    // decoder should take care of this up to 32 bit. For 64 bit, on our own.
    val as i32 as i64 as u64
}
pub fn xor(ri: &mut RiscvInt, args: &RiscvArgs) {
    ri.regs[args.rd as usize] = ri.regs[args.rs1 as usize] ^ ri.regs[args.rs2 as usize];
}
pub fn xori(ri: &mut RiscvInt, args: &RiscvArgs) {
    ri.regs[args.rd as usize] = ri.regs[args.rs1 as usize] ^ sign_ext_imm(args.imm);
}


pub fn or(ri: &mut RiscvInt, args: &RiscvArgs) {
    ri.regs[args.rd as usize] = ri.regs[args.rs1 as usize] | ri.regs[args.rs2 as usize];
}
pub fn ori(ri: &mut RiscvInt, args: &RiscvArgs) {
    ri.regs[args.rd as usize] = ri.regs[args.rs1 as usize] | sign_ext_imm(args.imm);
}

pub fn and(ri: &mut RiscvInt, args: &RiscvArgs) {
    ri.regs[args.rd as usize] = ri.regs[args.rs1 as usize] & ri.regs[args.rs2 as usize];
}

pub fn andi(ri: &mut RiscvInt, args: &RiscvArgs) {
    ri.regs[args.rd as usize] = ri.regs[args.rs1 as usize] & sign_ext_imm(args.imm);
}

pub fn add(ri: &mut RiscvInt, args: &RiscvArgs) {
    let finalval = (ri.regs[args.rs1 as usize])
        .wrapping_add(ri.regs[args.rs2 as usize]);
    ri.regs[args.rd as usize] = ri.sign_ext(finalval);
    
}
pub fn addi(ri: &mut RiscvInt, args: &RiscvArgs) {
    // because rs1 and rd are5 bits, should never go above 32 (uneless issue with decode)
    // overlfow doesnt matter
    let finalval: u64 = (ri.regs[args.rs1 as usize] as u64).wrapping_add(sign_ext_imm(args.imm));
    ri.regs[args.rd as usize] = ri.sign_ext(finalval);
    
}
pub fn addw(ri: &mut RiscvInt, args: &RiscvArgs) {
    ri.regs[args.rd as usize] = ri.regs[args.rs1 as usize].wrapping_add(ri.regs[args.rs2 as usize]) as i32 as i64 as u64;
    
}

pub fn sub(ri: &mut RiscvInt, args: &RiscvArgs) {
    let finalval: u64 = ri.regs[args.rs1 as usize].wrapping_sub(ri.regs[args.rs2 as usize]);
    ri.regs[args.rd as usize] = ri.sign_ext(finalval);
    
}
pub fn subw(ri: &mut RiscvInt, args: &RiscvArgs) {
    ri.regs[args.rd as usize] = ri.regs[args.rs1 as usize].wrapping_sub(ri.regs[args.rs2 as usize]) as i32 as i64 as u64;
    
}

pub fn mul(ri: &mut RiscvInt, args: &RiscvArgs) {
    ri.regs[args.rd as usize] = ri.sign_ext(ri.regs[args.rs1 as usize].wrapping_mul(ri.regs[args.rs2 as usize])) as u64;
    
}

pub fn mulh(ri: &mut RiscvInt, args: &RiscvArgs) {
    ri.regs[args.rd as usize] = match ri.xlen {
        Xlen::X32 => {
            ri.sign_ext(((ri.regs[args.rs1 as usize].wrapping_mul(ri.regs[args.rs2 as usize])) >> 32)) as u64
        },
        Xlen::X64 => {
            ((ri.regs[args.rs1 as usize] as i64 as i128).wrapping_mul(ri.regs[args.rs2 as usize] as i64 as i128) >> 64) as i64 as u64
        }
    };
    
}
pub fn mulhu(ri: &mut RiscvInt, args: &RiscvArgs) {
    ri.regs[args.rd as usize] = match ri.xlen {
        Xlen::X32 => {
            ri.sign_ext(((ri.regs[args.rs1 as usize] as u32 as u64).wrapping_mul(ri.regs[args.rs2 as usize] as u32 as u64) >> 32) as u64)
        },
        Xlen::X64 => {
            ((ri.regs[args.rs1 as usize] as u64 as u128).wrapping_mul(ri.regs[args.rs2 as usize] as u64 as u128) >> 64) as i64 as u64
        }
    } as u64;

}
pub fn mulhsu(ri: &mut RiscvInt, args: &RiscvArgs) {
    ri.regs[args.rd as usize] = match ri.xlen {
        Xlen::X32 => {
            ri.sign_ext(((ri.regs[args.rs1 as usize] as i64).wrapping_mul(ri.regs[args.rs2 as usize] as u32 as i64) >> 32) as u64)
        },
        Xlen::X64 => {
            ((ri.regs[args.rs1 as usize] as u128).wrapping_mul(ri.regs[args.rs2 as usize] as u64 as u128) >> 64) as i64 as u64
        }
    } as u64;
    
}
pub fn rem(ri: &mut RiscvInt, args: &RiscvArgs) {
    let dividend = ri.sign_ext(ri.regs[args.rs1 as usize]) as i64;
    let divisor = ri.sign_ext(ri.regs[args.rs2 as usize]) as i64;
    if divisor == 0 {
        ri.regs[args.rd as usize] = dividend as u64;
    } else if dividend == ri.most_negative() && divisor == -1 {
        ri.regs[args.rd as usize] = 0;
    } else {
        ri.regs[args.rd as usize] = ri.sign_ext(dividend.wrapping_rem(divisor) as u64) as u64;
    }
    
}

pub fn remu(ri: &mut RiscvInt, args: &RiscvArgs) {
    let dividend = ri.cull_reg(ri.regs[args.rs1 as usize]);
    let divisor = ri.cull_reg(ri.regs[args.rs2 as usize]);
    ri.regs[args.rd as usize] = match divisor {
        0 => ri.sign_ext(dividend),
        _ => ri.sign_ext(dividend.wrapping_rem(divisor))
    } as u64;
    
}

pub fn mulw(ri: &mut RiscvInt, args: &RiscvArgs) {
    ri.regs[args.rd as usize] = ri.sign_ext((ri.regs[args.rs1 as usize] as i32).wrapping_mul(ri.regs[args.rs2 as usize] as i32) as u64) as u64;
    
}
pub fn div(ri: &mut RiscvInt, args: &RiscvArgs) {
    let dividend = ri.regs[args.rs1 as usize] as i64;
    let divisor = ri.regs[args.rs2 as usize] as i64;
    if divisor == 0 {
        ri.regs[args.rd as usize] = -1 as i64 as u64;
    } else if dividend == std::i64::MIN && divisor == -1 {
        ri.regs[args.rd as usize] = dividend  as u64;
    } else {
        ri.regs[args.rd as usize] = dividend.wrapping_div(divisor) as u64;
    }
}

pub fn divw(ri: &mut RiscvInt, args: &RiscvArgs) {
    let dividend = ri.regs[args.rs1 as usize] as i32;
    let divisor = ri.regs[args.rs2 as usize] as i32;
    if divisor == 0 {
        ri.regs[args.rd as usize] = -1 as i64 as u64;
    } else if dividend == std::i32::MIN && divisor == -1 {
        ri.regs[args.rd as usize] = dividend as i32 as i64 as u64;
    } else {
        ri.regs[args.rd as usize] = dividend.wrapping_div(divisor) as i32 as i64 as u64
    }
    
}
pub fn divu(ri: &mut RiscvInt, args: &RiscvArgs) {
    let dividend = ri.cull_reg(ri.regs[args.rs1 as usize]);
    let divisor = ri.cull_reg(ri.regs[args.rs2 as usize]);
    if divisor == 0 {
        ri.regs[args.rd as usize] = -1 as i64 as u64;
    } else {
        ri.regs[args.rd as usize] = ri.sign_ext(dividend.wrapping_div(divisor));
    }

}
pub fn divuw(ri: &mut RiscvInt, args: &RiscvArgs) {
    let dividend = ri.cull_reg(ri.regs[args.rs1 as usize]) as u32;
    let divisor = ri.cull_reg(ri.regs[args.rs2 as usize]) as u32;
    if divisor == 0 {
        ri.regs[args.rd as usize] = -1 as i64 as u64;
    } else {
        ri.regs[args.rd as usize] = dividend.wrapping_div(divisor) as i32 as i64 as u64;
    }
    
}

pub fn remw(ri: &mut RiscvInt, args: &RiscvArgs) {
    let dividend = ri.regs[args.rs1 as usize] as i32;
    let divisor = ri.regs[args.rs2 as usize] as i32;
    if divisor == 0 {
        ri.regs[args.rd as usize] = dividend as i64 as u64;
    } else if dividend == std::i32::MIN && divisor == -1 {
        ri.regs[args.rd as usize] = 0;
    } else {
        ri.regs[args.rd as usize] = dividend.wrapping_rem(divisor) as i64 as u64;
    }
    
}
pub fn remuw(ri: &mut RiscvInt, args: &RiscvArgs) {
    let dividend = ri.regs[args.rs1 as usize] as u32;
    let divisor = ri.regs[args.rs2 as usize] as u32;
    if divisor == 0 {
        ri.regs[args.rd as usize] = dividend as i32 as i64 as u64;
    }  else {
        ri.regs[args.rd as usize] = dividend.wrapping_rem(divisor) as i32 as i64 as u64;
    }

}
pub fn slli(ri: &mut RiscvInt, args: &RiscvArgs) {
    let shiftby: u64 = match ri.xlen {
        Xlen::X32 => {
            args.shamt & 0x1f // lower 5 bits
        },
        Xlen::X64 => {
            args.shamt & 0x3f // lower 6 bits
        }
    } as u64;
    ri.regs[args.rd as usize] = ri.sign_ext(ri.regs[args.rs1 as usize] << shiftby);
    
}

pub fn srli(ri: &mut RiscvInt, args: &RiscvArgs) {
    let shiftby: u64 = match ri.xlen {
        Xlen::X32 => {
            args.shamt & 0x1f // lower 5 bits
        },
        Xlen::X64 => {
            args.shamt & 0x3f // lower 6 bits
        }
    } as u64;
    ri.regs[args.rd as usize] = ri.sign_ext(ri.cull_reg(ri.regs[args.rs1 as usize]) >> shiftby);
    
}

pub fn srai(ri: &mut RiscvInt, args: &RiscvArgs) {
    let shiftby: u32 = match ri.xlen {
        Xlen::X32 => {
            args.shamt & 0x1f // lower 5 bits
        },
        Xlen::X64 => {
            args.shamt & 0x3f // lower 6 bits
        }
    };
    ri.regs[args.rd as usize] = match ri.xlen {
        Xlen::X64 => {
            let mut val: i64 = ri.regs[args.rs1 as usize] as i64;
            val = val >> (shiftby as i64);
            val as u64

        },
        Xlen::X32 => {
            let mut val: i32 = ri.regs[args.rs1 as usize] as i32;
            val = val >> (shiftby as i32);
            (val as i64) as u64 // we want it to extend all the way, just in case
        }
    };
    
}




pub fn sll(ri: &mut RiscvInt, args: &RiscvArgs) {
    let shiftby: u64 = match ri.xlen {
        Xlen::X32 => {
            ri.regs[args.rs2 as usize] & 0x1f // lower 5 bits
        },
        Xlen::X64 => {
            ri.regs[args.rs2 as usize] & 0x3f // lower 6 bits
        }
    };
    ri.regs[args.rd as usize] = ri.sign_ext(ri.regs[args.rs1 as usize].wrapping_shl(shiftby as u32));
    

}

pub fn slt(ri: &mut RiscvInt, args: &RiscvArgs) {
    let condition: bool = match ri.xlen {
        Xlen::X32 => {
            (ri.regs[args.rs1 as usize] as i32) < (ri.regs[args.rs2 as usize] as i32)
        }
        Xlen::X64 => {
            (ri.regs[args.rs1 as usize] as i64) < (ri.regs[args.rs2 as usize] as i64)
        }
    };
    ri.regs[args.rd as usize] = if condition {
        1
    } else {
        0
    };
    
}

pub fn sltu(ri: &mut RiscvInt, args: &RiscvArgs) {
    let condition: bool = match ri.xlen {
        Xlen::X32 => {
            (ri.regs[args.rs1 as usize] as u32) < (ri.regs[args.rs2 as usize] as u32)
        }
        Xlen::X64 => {
            (ri.regs[args.rs1 as usize] as u64) < (ri.regs[args.rs2 as usize] as u64)
        }
    };
    ri.regs[args.rd as usize] = if condition {
        1
    } else {
        0
    };
    
}


pub fn srl(ri: &mut RiscvInt, args: &RiscvArgs) {
    let shiftby: u64 = match ri.xlen {
        Xlen::X32 => {
            ri.regs[args.rs2 as usize] & 0x1f // lower 5 bits
        },
        Xlen::X64 => {
            ri.regs[args.rs2 as usize] & 0x3f // lower 6 bits
        }
    };
    ri.regs[args.rd as usize] = ri.sign_ext(ri.cull_reg(ri.regs[args.rs1 as usize]).wrapping_shr(shiftby as u32));
    
}

pub fn sra(ri: &mut RiscvInt, args: &RiscvArgs) {
    let shiftby: u64 = match ri.xlen {
        Xlen::X32 => {
            ri.regs[args.rs2 as usize] & 0x1f // lower 5 bits
        },
        Xlen::X64 => {
            ri.regs[args.rs2 as usize] & 0x3f // lower 6 bits
        }
    };
    ri.regs[args.rd as usize] = match ri.xlen {
        Xlen::X64 => {
            let mut val: i64 = ri.regs[args.rs1 as usize] as i64;
            val = val >> (shiftby as i64);
            val as u64

        },
        Xlen::X32 => {
            let mut val: i32 = ri.regs[args.rs1 as usize] as i32;
            val = val >> (shiftby as i32);
            (val as i64) as u64 // we want it to extend all the way, just in case
        }
    };
    
}
pub fn addiw(ri: &mut RiscvInt, args: &RiscvArgs) {
    ri.regs[args.rd as usize] = ri.regs[args.rs1 as usize].wrapping_add(sign_ext_imm(args.imm)) as i32 as i64 as u64;
    
}

pub fn slliw(ri: &mut RiscvInt, args: &RiscvArgs) {
    ri.regs[args.rd as usize] = (ri.regs[args.rs1 as usize].wrapping_shl(args.shamt)) as i32 as u64;
    
}

pub fn srliw(ri: &mut RiscvInt, args: &RiscvArgs) {
    ri.regs[args.rd as usize] = ((ri.regs[args.rs1 as usize] as u32) >> args.shamt) as i32 as i64 as u64;
    
}

pub fn sraiw(ri: &mut RiscvInt, args: &RiscvArgs) {
    ri.regs[args.rd as usize] = ((ri.regs[args.rs1 as usize] as i32) >> args.shamt) as i64 as u64;
    
}
pub fn sllw(ri: &mut RiscvInt, args: &RiscvArgs) {
    ri.regs[args.rd as usize] = (ri.regs[args.rs1 as usize] as u32).wrapping_shl(ri.regs[args.rs2 as usize] as u32) as i32 as i64 as u64;
    
}

pub fn srlw(ri: &mut RiscvInt, args: &RiscvArgs) {
    ri.regs[args.rd as usize] = (ri.regs[args.rs1 as usize] as u32).wrapping_shr(ri.regs[args.rs2 as usize] as u32) as i32 as i64 as u64;
    
}

pub fn sraw(ri: &mut RiscvInt, args: &RiscvArgs) {
    ri.regs[args.rd as usize] = (ri.regs[args.rs1 as usize] as i32).wrapping_shr(ri.regs[args.rs2 as usize] as u32) as i64 as u64;
    
}


pub fn lui(ri: &mut RiscvInt, args: &RiscvArgs) {
    // will overwite lower 12 with 0s, and dtree does this already, so just assign
    ri.regs[args.rd as usize] = sign_ext_imm(args.imm); // see riscv atlas
    
}
pub fn slti(ri: &mut RiscvInt, args: &RiscvArgs) {
    let condition: bool = match ri.xlen {
        Xlen::X32 => {
            (ri.regs[args.rs1 as usize] as i32) < (args.imm as i32)
        }
        Xlen::X64 => {
            (ri.regs[args.rs1 as usize] as i64) < (sign_ext_imm(args.imm) as i64)
        }
    };
    ri.regs[args.rd as usize] = if condition {
        1
    } else {
        0
    };
    
}

pub fn sltiu(ri: &mut RiscvInt, args: &RiscvArgs) {
    let condition: bool = match ri.xlen {
        Xlen::X32 => {
            (ri.regs[args.rs1 as usize] as u32) < (args.imm as u32)
        }
        Xlen::X64 => {
            (ri.regs[args.rs1 as usize] as u64) < (sign_ext_imm(args.imm) as u64)
        }
    };
    ri.regs[args.rd as usize] = if condition {
        1
    } else {
        0
    };

}
// technically part of crypto extension, but put it here because it fits better
pub fn gen_sh_add(ri: &mut RiscvInt, args: &RiscvArgs, amt: u64, is_uw: bool) {
    let mut rs1 = if is_uw {
        ri.regs[args.rs1 as usize] as u32 as u64
    } else {
        ri.regs[args.rs1 as usize]
    };
    if is_uw && ri.xlen != Xlen::X64 {
        panic!(); // todo: illegal instr
    }
    rs1 <<= amt;
    let newval = rs1 + ri.regs[args.rs2 as usize];
    ri.regs[args.rd as usize] = ri.sign_ext(newval);

}
pub fn sh1add(ri: &mut RiscvInt, args: &RiscvArgs) {
    gen_sh_add(ri, args, 1, false);

}
pub fn sh1add_uw(ri: &mut RiscvInt, args: &RiscvArgs) {
    gen_sh_add(ri, args, 1, true);

}

pub fn sh2add(ri: &mut RiscvInt, args: &RiscvArgs) {
    gen_sh_add(ri, args, 2, false);

}
pub fn sh2add_uw(ri: &mut RiscvInt, args: &RiscvArgs) {
    gen_sh_add(ri, args, 2, true);

}

pub fn sh3add(ri: &mut RiscvInt, args: &RiscvArgs) {
    gen_sh_add(ri, args, 3, false);

}
pub fn sh3add_uw(ri: &mut RiscvInt, args: &RiscvArgs) {
    gen_sh_add(ri, args, 3, true);

}

pub fn add_uw(ri: &mut RiscvInt, args: &RiscvArgs) {
    gen_sh_add(ri, args, 0, true);

}
pub fn xnor(ri: &mut RiscvInt, args: &RiscvArgs) {
    ri.regs[args.rd as usize] = ri.regs[args.rs1 as usize] ^ (!ri.regs[args.rs2 as usize]);


}