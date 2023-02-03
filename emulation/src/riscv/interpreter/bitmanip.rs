// bitmanip extensions, so zb* and zbkc
use crate::riscv::common::{RiscvArgs, Xlen, xlen2bits};
use crate::riscv::interpreter::main::{RiscvInt};


pub fn andn(ri: &mut RiscvInt, args: &RiscvArgs) {
    ri.regs[args.rd as usize] = ri.regs[args.rs1 as usize] & !ri.regs[args.rs2 as usize];
}
pub fn adduw(ri: &mut RiscvInt, args: &RiscvArgs) {
    ri.regs[args.rd as usize] = ri.regs[args.rs2 as usize] + (ri.regs[args.rs1 as usize] as u32 as u64);
}
pub fn bclr(ri: &mut RiscvInt, args: &RiscvArgs) {
    let index = ri.regs[args.rs2 as usize] & (xlen2bits(ri.xlen) - 1);
    ri.regs[args.rd as usize] = ri.sign_ext(ri.regs[args.rs1 as usize] & !(1 << index));
}
pub fn bclri(ri: &mut RiscvInt, args: &RiscvArgs) {
    let index = (args.shamt as u64) & (xlen2bits(ri.xlen) - 1);
    ri.regs[args.rd as usize] = ri.sign_ext(ri.regs[args.rs1 as usize] & !(1 << index));
}
pub fn bext(ri: &mut RiscvInt, args: &RiscvArgs) {
    let index = ri.regs[args.rs2 as usize] & (xlen2bits(ri.xlen) - 1);
    ri.regs[args.rd as usize] = ri.sign_ext((ri.regs[args.rs1 as usize] >> index) & 1);
}
pub fn bexti(ri: &mut RiscvInt, args: &RiscvArgs) {
    let index = (args.shamt as u64) & (xlen2bits(ri.xlen) - 1);
    ri.regs[args.rd as usize] = ri.sign_ext((ri.regs[args.rs1 as usize] >> index) & 1);
}
pub fn binv(ri: &mut RiscvInt, args: &RiscvArgs) {
    let index = ri.regs[args.rs2 as usize] & (xlen2bits(ri.xlen) - 1);
    ri.regs[args.rd as usize] = ri.sign_ext(ri.regs[args.rs1 as usize] ^ (1 << index));
}
pub fn binvi(ri: &mut RiscvInt, args: &RiscvArgs) {
    let index = (args.shamt as u64) & (xlen2bits(ri.xlen) - 1);
    ri.regs[args.rd as usize] = ri.sign_ext(ri.regs[args.rs1 as usize] ^ (1 << index));
}
pub fn bset(ri: &mut RiscvInt, args: &RiscvArgs) {
    let index = ri.regs[args.rs2 as usize] & (xlen2bits(ri.xlen) - 1);
    ri.regs[args.rd as usize] = ri.sign_ext(ri.regs[args.rs1 as usize] | (1 << index));
}
pub fn bseti(ri: &mut RiscvInt, args: &RiscvArgs) {
    let index = (args.shamt as u64) & (xlen2bits(ri.xlen) - 1);
    ri.regs[args.rd as usize] = ri.sign_ext(ri.regs[args.rs1 as usize] | (1 << index));
}
pub fn clmul(ri: &mut RiscvInt, args: &RiscvArgs) {
    let a: u64 = ri.cull_reg(ri.regs[args.rs1 as usize]);
    let b: u64 = ri.cull_reg(ri.regs[args.rs2 as usize]);
    let mut x: u64 = 0;
    for i in 0..xlen2bits(ri.xlen) {
        if (b >> i) & 1 != 0 {
            x ^= a << i;
        }
    }
    ri.regs[args.rd as usize] = ri.sign_ext(x);
}

pub fn clmulh(ri: &mut RiscvInt, args: &RiscvArgs) {
    let a: u64 = ri.cull_reg(ri.regs[args.rs1 as usize]);
    let b: u64 = ri.cull_reg(ri.regs[args.rs2 as usize]);
    let mut x: u64 = 0;
    for i in 1..xlen2bits(ri.xlen) {
        if (b >> i) & 1 != 0 {
            x ^= a >> (xlen2bits(ri.xlen) - i);
        }
    }
    ri.regs[args.rd as usize] = ri.sign_ext(x);
}
pub fn clmulr(ri: &mut RiscvInt, args: &RiscvArgs) {
    let a: u64 = ri.cull_reg(ri.regs[args.rs1 as usize]);
    let b: u64 = ri.cull_reg(ri.regs[args.rs2 as usize]);
    let mut x: u64 = 0;
    for i in 1..(xlen2bits(ri.xlen) -1) { // mistmatch between this and spike.
        if (b >> i) & 1 != 0 {
            x ^= a >> (xlen2bits(ri.xlen) - i -1);
        }
    }
    ri.regs[args.rd as usize] = ri.sign_ext(x);
}
pub fn clz(ri: &mut RiscvInt, args: &RiscvArgs) {
    let a: u64 = ri.regs[args.rs1 as usize];
    let xlen = xlen2bits(ri.xlen);

    let mut x = xlen;
    for i in 0..xlen {
        if (a >> (xlen - i - 1)) & 1 != 0 {
            x = i;
            break;
        }
    }
    ri.regs[args.rd as usize] = ri.sign_ext(x);
}
pub fn clzw(ri: &mut RiscvInt, args: &RiscvArgs) {
    let a: u64 = ri.regs[args.rs1 as usize];
    let mut x = 32;
    for i in 0..32 {
        if (a >> (31 - i)) & 1 != 0 {
            x = i;
            break;
        }
    }
    ri.regs[args.rd as usize] = ri.sign_ext(x);
}
pub fn cpop(ri: &mut RiscvInt, args: &RiscvArgs) {
    let a: u64 = ri.regs[args.rs1 as usize];
    let xlen = xlen2bits(ri.xlen);
    let mut x = 0;
    for i in 0..xlen {
        if (a >> i) & 1 != 0 {
            x += 1;
        }
    }
    ri.regs[args.rd as usize] = ri.sign_ext(x);
}
pub fn cpopw(ri: &mut RiscvInt, args: &RiscvArgs) {
    let a: u64 = ri.regs[args.rs1 as usize];
    let mut x = 0;
    for i in 0..32 {
        if (a >> i) & 1 != 0 {
            x += 1;
        }
    }
    ri.regs[args.rd as usize] = ri.sign_ext(x);
}
pub fn ctz(ri: &mut RiscvInt, args: &RiscvArgs) {
    let a: u64 = ri.regs[args.rs1 as usize];
    let xlen = xlen2bits(ri.xlen);

    let mut x = xlen;
    for i in 0..xlen {
        if (a >> i) & 1 != 0 {
            x = i;
            break;
        }
    }
    ri.regs[args.rd as usize] = ri.sign_ext(x);
}
pub fn ctzw(ri: &mut RiscvInt, args: &RiscvArgs) {
    let a: u64 = ri.regs[args.rs1 as usize];
    let mut x = 32;
    for i in 0..32 {
        if (a >> i) & 1 != 0 {
            x = i;
            break;
        }
    }
    ri.regs[args.rd as usize] = ri.sign_ext(x);
}
pub fn max(ri: &mut RiscvInt, args: &RiscvArgs) {
    let a = ri.regs[args.rs1 as usize] as i64;
    let b = ri.regs[args.rs2 as usize] as i64;
    let fin = std::cmp::max(a,b);
    ri.regs[args.rd as usize] = ri.sign_ext(fin as u64);
}
pub fn maxu(ri: &mut RiscvInt, args: &RiscvArgs) {
    let a = ri.regs[args.rs1 as usize];
    let b = ri.regs[args.rs2 as usize];
    let fin = std::cmp::max(a,b);
    ri.regs[args.rd as usize] = ri.sign_ext(fin as u64);
}
pub fn min(ri: &mut RiscvInt, args: &RiscvArgs) {
    let a = ri.regs[args.rs1 as usize] as i64;
    let b = ri.regs[args.rs2 as usize] as i64;
    let fin = std::cmp::min(a,b);
    ri.regs[args.rd as usize] = ri.sign_ext(fin as u64);
}
pub fn minu(ri: &mut RiscvInt, args: &RiscvArgs) {
    let a = ri.regs[args.rs1 as usize];
    let b = ri.regs[args.rs2 as usize];
    let fin = std::cmp::min(a,b);
    ri.regs[args.rd as usize] = ri.sign_ext(fin as u64);
}
pub fn orc_b(ri: &mut RiscvInt, args: &RiscvArgs) {
    // was similar to canned gorci instruction, with shift amt being 7
    let a = ri.regs[args.rs1 as usize];
    let shamt: u32 = 7;
    let mut x = a;
    if (shamt & 1) != 0 {
        x |= ((x & 0x5555555555555555) <<  1) | ((x & 0xAAAAAAAAAAAAAAAA) >>  1);
    }
    if (shamt & 2) != 0 {
        x |= ((x & 0x3333333333333333) <<  2) | ((x & 0xCCCCCCCCCCCCCCCC) >>  2);
    }
    if (shamt & 4) != 0 {
        x |= ((x & 0x0F0F0F0F0F0F0F0F) <<  4) | ((x & 0xF0F0F0F0F0F0F0F0) >>  4);
    }
    if (shamt & 8) != 0 {
        x |= ((x & 0x00FF00FF00FF00FF) <<  8) | ((x & 0xFF00FF00FF00FF00) >>  8);
    }
    if (shamt & 16) != 0 {
        x |= ((x & 0x0000FFFF0000FFFF) <<  16) | ((x & 0xFFFF0000FFFF0000) >>  16);
    }
    if (shamt & 32) != 0 {
        x |= ((x & 0x00000000FFFFFFFF) <<  32) | ((x & 0xFFFFFFFF00000000) >>  32);
    }
    ri.regs[args.rd as usize] = ri.sign_ext(x as u64);
}
pub fn orn(ri: &mut RiscvInt, args: &RiscvArgs) {
    ri.regs[args.rd as usize] = ri.regs[args.rs1 as usize] | !ri.regs[args.rs2 as usize];
}
pub fn rot_gen(ri: &mut RiscvInt, args: &RiscvArgs, force31: bool, isleft: bool) {
    let xlen = if force31 {
        32
    } else {
        xlen2bits(ri.xlen)
    };
    let shamt = ri.regs[args.rs2 as usize] & (xlen - 1);
    let rshamt = (-(shamt as i64) as u64) & (xlen - 1);
    let fin = if isleft {
        ((ri.regs[args.rs1 as usize] << shamt) | (ri.cull_reg(ri.regs[args.rs1 as usize]) >> rshamt))
    } else {
        ((ri.regs[args.rs1 as usize] << rshamt) | (ri.cull_reg(ri.regs[args.rs1 as usize]) >> shamt))
    };
    ri.regs[args.rd as usize] = ri.sign_ext(fin as u64);



}
pub fn rol(ri: &mut RiscvInt, args: &RiscvArgs) {
    rot_gen(ri, args, false, true);
}
pub fn rolw(ri: &mut RiscvInt, args: &RiscvArgs) {
    rot_gen(ri, args, true, true);
}
pub fn ror(ri: &mut RiscvInt, args: &RiscvArgs) {
    rot_gen(ri, args, false, false);
}
pub fn rorw(ri: &mut RiscvInt, args: &RiscvArgs) {
    rot_gen(ri, args, true, false);
}
pub fn rori(ri: &mut RiscvInt, args: &RiscvArgs) {
    let xlen = xlen2bits(ri.xlen);

    let shamt = (args.shamt as u64) & (xlen - 1);
    let rshamt = (-(shamt as i64) as u64) & (xlen - 1);

    let fin = ((ri.regs[args.rs1 as usize] << rshamt) | (ri.cull_reg(ri.regs[args.rs1 as usize]) >> shamt));
    ri.regs[args.rd as usize] = ri.sign_ext(fin as u64);
}
pub fn roriw(ri: &mut RiscvInt, args: &RiscvArgs) {
    let shamt = (args.shamt as u64) & 31;
    let rshamt = (-(shamt as i64) as u64) & 31;

    let fin = ((ri.regs[args.rs1 as usize] << rshamt) | (ri.cull_reg(ri.regs[args.rs1 as usize]) >> shamt));
    ri.regs[args.rd as usize] = ri.sign_ext(fin as u64);
}
pub fn sext_b(ri: &mut RiscvInt, args: &RiscvArgs) {
    ri.regs[args.rd as usize] = ri.regs[args.rs1 as usize] as i8 as i64 as u64;
}
pub fn sext_h(ri: &mut RiscvInt, args: &RiscvArgs) {
    ri.regs[args.rd as usize] = ri.regs[args.rs1 as usize] as i16 as i64 as u64;
}
pub fn zext_h(ri: &mut RiscvInt, args: &RiscvArgs) {
    ri.regs[args.rd as usize] = ri.regs[args.rs1 as usize] as u16 as u64;
}
pub fn zext_h_32(ri: &mut RiscvInt, args: &RiscvArgs) {
    zext_h(ri, args)
}
pub fn zext_h_64(ri: &mut RiscvInt, args: &RiscvArgs) {
    zext_h(ri, args)
}