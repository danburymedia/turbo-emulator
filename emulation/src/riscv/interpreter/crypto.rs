use crate::riscv::common::{RiscvArgs, Xlen};
use crate::riscv::interpreter::main::{RiscvInt};
const AES_ENC_SANDBOX: [u8; 256] = [
    0x63, 0x7C, 0x77, 0x7B, 0xF2, 0x6B, 0x6F, 0xC5,
    0x30, 0x01, 0x67, 0x2B, 0xFE, 0xD7, 0xAB, 0x76,
    0xCA, 0x82, 0xC9, 0x7D, 0xFA, 0x59, 0x47, 0xF0,
    0xAD, 0xD4, 0xA2, 0xAF, 0x9C, 0xA4, 0x72, 0xC0,
    0xB7, 0xFD, 0x93, 0x26, 0x36, 0x3F, 0xF7, 0xCC,
    0x34, 0xA5, 0xE5, 0xF1, 0x71, 0xD8, 0x31, 0x15,
    0x04, 0xC7, 0x23, 0xC3, 0x18, 0x96, 0x05, 0x9A,
    0x07, 0x12, 0x80, 0xE2, 0xEB, 0x27, 0xB2, 0x75,
    0x09, 0x83, 0x2C, 0x1A, 0x1B, 0x6E, 0x5A, 0xA0,
    0x52, 0x3B, 0xD6, 0xB3, 0x29, 0xE3, 0x2F, 0x84,
    0x53, 0xD1, 0x00, 0xED, 0x20, 0xFC, 0xB1, 0x5B,
    0x6A, 0xCB, 0xBE, 0x39, 0x4A, 0x4C, 0x58, 0xCF,
    0xD0, 0xEF, 0xAA, 0xFB, 0x43, 0x4D, 0x33, 0x85,
    0x45, 0xF9, 0x02, 0x7F, 0x50, 0x3C, 0x9F, 0xA8,
    0x51, 0xA3, 0x40, 0x8F, 0x92, 0x9D, 0x38, 0xF5,
    0xBC, 0xB6, 0xDA, 0x21, 0x10, 0xFF, 0xF3, 0xD2,
    0xCD, 0x0C, 0x13, 0xEC, 0x5F, 0x97, 0x44, 0x17,
    0xC4, 0xA7, 0x7E, 0x3D, 0x64, 0x5D, 0x19, 0x73,
    0x60, 0x81, 0x4F, 0xDC, 0x22, 0x2A, 0x90, 0x88,
    0x46, 0xEE, 0xB8, 0x14, 0xDE, 0x5E, 0x0B, 0xDB,
    0xE0, 0x32, 0x3A, 0x0A, 0x49, 0x06, 0x24, 0x5C,
    0xC2, 0xD3, 0xAC, 0x62, 0x91, 0x95, 0xE4, 0x79,
    0xE7, 0xC8, 0x37, 0x6D, 0x8D, 0xD5, 0x4E, 0xA9,
    0x6C, 0x56, 0xF4, 0xEA, 0x65, 0x7A, 0xAE, 0x08,
    0xBA, 0x78, 0x25, 0x2E, 0x1C, 0xA6, 0xB4, 0xC6,
    0xE8, 0xDD, 0x74, 0x1F, 0x4B, 0xBD, 0x8B, 0x8A,
    0x70, 0x3E, 0xB5, 0x66, 0x48, 0x03, 0xF6, 0x0E,
    0x61, 0x35, 0x57, 0xB9, 0x86, 0xC1, 0x1D, 0x9E,
    0xE1, 0xF8, 0x98, 0x11, 0x69, 0xD9, 0x8E, 0x94,
    0x9B, 0x1E, 0x87, 0xE9, 0xCE, 0x55, 0x28, 0xDF,
    0x8C, 0xA1, 0x89, 0x0D, 0xBF, 0xE6, 0x42, 0x68,
    0x41, 0x99, 0x2D, 0x0F, 0xB0, 0x54, 0xBB, 0x16
];
const AES_DEC_SANDBOX: [u8; 256] = [
    0x52, 0x09, 0x6A, 0xD5, 0x30, 0x36, 0xA5, 0x38,
    0xBF, 0x40, 0xA3, 0x9E, 0x81, 0xF3, 0xD7, 0xFB,
    0x7C, 0xE3, 0x39, 0x82, 0x9B, 0x2F, 0xFF, 0x87,
    0x34, 0x8E, 0x43, 0x44, 0xC4, 0xDE, 0xE9, 0xCB,
    0x54, 0x7B, 0x94, 0x32, 0xA6, 0xC2, 0x23, 0x3D,
    0xEE, 0x4C, 0x95, 0x0B, 0x42, 0xFA, 0xC3, 0x4E,
    0x08, 0x2E, 0xA1, 0x66, 0x28, 0xD9, 0x24, 0xB2,
    0x76, 0x5B, 0xA2, 0x49, 0x6D, 0x8B, 0xD1, 0x25,
    0x72, 0xF8, 0xF6, 0x64, 0x86, 0x68, 0x98, 0x16,
    0xD4, 0xA4, 0x5C, 0xCC, 0x5D, 0x65, 0xB6, 0x92,
    0x6C, 0x70, 0x48, 0x50, 0xFD, 0xED, 0xB9, 0xDA,
    0x5E, 0x15, 0x46, 0x57, 0xA7, 0x8D, 0x9D, 0x84,
    0x90, 0xD8, 0xAB, 0x00, 0x8C, 0xBC, 0xD3, 0x0A,
    0xF7, 0xE4, 0x58, 0x05, 0xB8, 0xB3, 0x45, 0x06,
    0xD0, 0x2C, 0x1E, 0x8F, 0xCA, 0x3F, 0x0F, 0x02,
    0xC1, 0xAF, 0xBD, 0x03, 0x01, 0x13, 0x8A, 0x6B,
    0x3A, 0x91, 0x11, 0x41, 0x4F, 0x67, 0xDC, 0xEA,
    0x97, 0xF2, 0xCF, 0xCE, 0xF0, 0xB4, 0xE6, 0x73,
    0x96, 0xAC, 0x74, 0x22, 0xE7, 0xAD, 0x35, 0x85,
    0xE2, 0xF9, 0x37, 0xE8, 0x1C, 0x75, 0xDF, 0x6E,
    0x47, 0xF1, 0x1A, 0x71, 0x1D, 0x29, 0xC5, 0x89,
    0x6F, 0xB7, 0x62, 0x0E, 0xAA, 0x18, 0xBE, 0x1B,
    0xFC, 0x56, 0x3E, 0x4B, 0xC6, 0xD2, 0x79, 0x20,
    0x9A, 0xDB, 0xC0, 0xFE, 0x78, 0xCD, 0x5A, 0xF4,
    0x1F, 0xDD, 0xA8, 0x33, 0x88, 0x07, 0xC7, 0x31,
    0xB1, 0x12, 0x10, 0x59, 0x27, 0x80, 0xEC, 0x5F,
    0x60, 0x51, 0x7F, 0xA9, 0x19, 0xB5, 0x4A, 0x0D,
    0x2D, 0xE5, 0x7A, 0x9F, 0x93, 0xC9, 0x9C, 0xEF,
    0xA0, 0xE0, 0x3B, 0x4D, 0xAE, 0x2A, 0xF5, 0xB0,
    0xC8, 0xEB, 0xBB, 0x3C, 0x83, 0x53, 0x99, 0x61,
    0x17, 0x2B, 0x04, 0x7E, 0xBA, 0x77, 0xD6, 0x26,
    0xE1, 0x69, 0x14, 0x63, 0x55, 0x21, 0x0C, 0x7D
];
const sm4_sbox: [u8; 256] = [
    0xD6, 0x90, 0xE9, 0xFE, 0xCC, 0xE1, 0x3D, 0xB7, 0x16, 0xB6, 0x14, 0xC2,
    0x28, 0xFB, 0x2C, 0x05, 0x2B, 0x67, 0x9A, 0x76, 0x2A, 0xBE, 0x04, 0xC3,
    0xAA, 0x44, 0x13, 0x26, 0x49, 0x86, 0x06, 0x99, 0x9C, 0x42, 0x50, 0xF4,
    0x91, 0xEF, 0x98, 0x7A, 0x33, 0x54, 0x0B, 0x43, 0xED, 0xCF, 0xAC, 0x62,
    0xE4, 0xB3, 0x1C, 0xA9, 0xC9, 0x08, 0xE8, 0x95, 0x80, 0xDF, 0x94, 0xFA,
    0x75, 0x8F, 0x3F, 0xA6, 0x47, 0x07, 0xA7, 0xFC, 0xF3, 0x73, 0x17, 0xBA,
    0x83, 0x59, 0x3C, 0x19, 0xE6, 0x85, 0x4F, 0xA8, 0x68, 0x6B, 0x81, 0xB2,
    0x71, 0x64, 0xDA, 0x8B, 0xF8, 0xEB, 0x0F, 0x4B, 0x70, 0x56, 0x9D, 0x35,
    0x1E, 0x24, 0x0E, 0x5E, 0x63, 0x58, 0xD1, 0xA2, 0x25, 0x22, 0x7C, 0x3B,
    0x01, 0x21, 0x78, 0x87, 0xD4, 0x00, 0x46, 0x57, 0x9F, 0xD3, 0x27, 0x52,
    0x4C, 0x36, 0x02, 0xE7, 0xA0, 0xC4, 0xC8, 0x9E, 0xEA, 0xBF, 0x8A, 0xD2,
    0x40, 0xC7, 0x38, 0xB5, 0xA3, 0xF7, 0xF2, 0xCE, 0xF9, 0x61, 0x15, 0xA1,
    0xE0, 0xAE, 0x5D, 0xA4, 0x9B, 0x34, 0x1A, 0x55, 0xAD, 0x93, 0x32, 0x30,
    0xF5, 0x8C, 0xB1, 0xE3, 0x1D, 0xF6, 0xE2, 0x2E, 0x82, 0x66, 0xCA, 0x60,
    0xC0, 0x29, 0x23, 0xAB, 0x0D, 0x53, 0x4E, 0x6F, 0xD5, 0xDB, 0x37, 0x45,
    0xDE, 0xFD, 0x8E, 0x2F, 0x03, 0xFF, 0x6A, 0x72, 0x6D, 0x6C, 0x5B, 0x51,
    0x8D, 0x1B, 0xAF, 0x92, 0xBB, 0xDD, 0xBC, 0x7F, 0x11, 0xD9, 0x5C, 0x41,
    0x1F, 0x10, 0x5A, 0xD8, 0x0A, 0xC1, 0x31, 0x88, 0xA5, 0xCD, 0x7B, 0xBD,
    0x2D, 0x74, 0xD0, 0x12, 0xB8, 0xE5, 0xB4, 0xB0, 0x89, 0x69, 0x97, 0x4A,
    0x0C, 0x96, 0x77, 0x7E, 0x65, 0xB9, 0xF1, 0x09, 0xC5, 0x6E, 0xC6, 0x84,
    0x18, 0xF0, 0x7D, 0xEC, 0x3A, 0xDC, 0x4D, 0x20, 0x79, 0xEE, 0x5F, 0x3E,
    0xD7, 0xCB, 0x39, 0x48
];
fn aes_unpack_bytes(rs1: u32, rs2: u32) -> (u8,u8,u8,u8) {
    let v1: u8 = (rs1 & 0xff) as u8;
    let v2: u8 = ((rs2 >> 8) & 0xff) as u8;
    let v3: u8 = ((rs1 >> 16) & 0xff) as u8;
    let v4: u8 = ((rs2 >> 24) & 0xff) as u8;
    (v1,v2,v3,v4)

}
fn aes_xtime(a: u8) -> u8 {
    let xorval: u8 = if (a & 0x80) != 0 {
        0x1b
    } else {
        0
    };
    (a << 1) ^ xorval
}
fn aes_byte(x: u64, i: u64) -> u8 {
    ((x >> (8 * i)) & 0xFF) as u8

}
fn invmixbyte(col: u64, b0: u8, b1: u8, b2: u8, b3: u8) -> u8 {
    aes_gfmul(aes_byte(col, b3 as u64),0x9) ^
        aes_gfmul(aes_byte(col, b2 as u64),0xd) ^
        aes_gfmul(aes_byte(col, b1 as u64),0xb) ^
        aes_gfmul(aes_byte(col, b0 as u64),0xe)

}
fn invmixcolumn(col: u64) -> u32 {
    ((invmixbyte(col,3,0,1,2) as u32) << 24)
        | ((invmixbyte(col,2,3,0,1) as u32) << 16)
        | ((invmixbyte(col,1,2,3,0) as u32) << 8)
        | (invmixbyte(col,0,1,2,3) as u32)

}
fn aes_gfmul(a: u8, b: u8) -> u8 {
    let x1: u8 = if b & 0x1 != 0 {
        a
    } else {
        0
    };
    let x2: u8 = if b & 0x2 != 0 {
        aes_xtime(a)
    } else {
        0
    };
    let x3: u8 = if b & 0x4 != 0 {
        aes_xtime(aes_xtime(a))
    } else {
        0
    };
    let x4: u8 = if b & 0x8 != 0 {
        aes_xtime(aes_xtime(aes_xtime(a)))
    } else {
        0
    };
    (x1 ^ x2 ^ x3 ^ x4) & 0xff
}
fn aes_invshifrows_lo(r1: u64, r2: u64) -> u64 {
    (((r2 >> 24) & 0xFF) << 56) |
    (((r2 >> 48) & 0xFF) << 48) |
    (((r1 >>  8) & 0xFF) << 40) |
    (((r1 >> 32) & 0xFF) << 32) |
    (((r1 >> 56) & 0xFF) << 24) |
    (((r2 >> 16) & 0xFF) << 16) |
    (((r2 >> 40) & 0xFF) <<  8) |
    (((r1 >>  0) & 0xFF) <<  0)
}


pub fn sha256sig0(ri: &mut RiscvInt, args: &RiscvArgs) {
    let rs1 = ri.regs[args.rs1 as usize] as u32;
    let writetoreg: u32 = rs1.rotate_right(7) ^ rs1.rotate_right(18) ^ (rs1 >> 3);
    ri.regs[args.rd as usize] = writetoreg as i32 as i64 as u64;

}
pub fn sha256sig1(ri: &mut RiscvInt, args: &RiscvArgs) {
    let rs1 = ri.regs[args.rs1 as usize] as u32;
    let writetoreg: u32 = rs1.rotate_right(17) ^ rs1.rotate_right(19) ^ (rs1 >> 10);
    ri.regs[args.rd as usize] = writetoreg as i32 as i64 as u64;

}
pub fn sha256sum0(ri: &mut RiscvInt, args: &RiscvArgs) {
    let rs1 = ri.regs[args.rs1 as usize] as u32;
    let writetoreg: u32 = rs1.rotate_right(2) ^ rs1.rotate_right(13) ^ rs1.rotate_right(22);
    ri.regs[args.rd as usize] = writetoreg as i32 as i64 as u64;

}
pub fn sha256sum1(ri: &mut RiscvInt, args: &RiscvArgs) {
    let rs1 = ri.regs[args.rs1 as usize] as u32;
    let writetoreg: u32 = rs1.rotate_right(6) ^ rs1.rotate_right(11) ^ rs1.rotate_right(25);
    ri.regs[args.rd as usize] = writetoreg as i32 as i64 as u64;

}
pub fn sha512sig0(ri: &mut RiscvInt, args: &RiscvArgs) {
    let rs1 = ri.regs[args.rs1 as usize] as u64;
    let writetoreg: u64 = rs1.rotate_right(1) ^ rs1.rotate_right(8) ^ (rs1 >> 7);
    ri.regs[args.rd as usize] = writetoreg;

}
pub fn sha512sig0h(ri: &mut RiscvInt, args: &RiscvArgs) {
    let rs1 = ri.regs[args.rs1 as usize] as u32 as u64;
    let rs2 = ri.regs[args.rs2 as usize] as u32 as u64;

    let writetoreg: u64 = (rs1 >> 1)
        ^ (rs1 >> 7)
        ^ (rs1 >> 8)
        ^ (rs2 << 31)
        ^ (rs2 << 24);
    ri.regs[args.rd as usize] = ri.sign_ext(writetoreg);

}
pub fn sha512sig0l(ri: &mut RiscvInt, args: &RiscvArgs) {
    let rs1 = ri.regs[args.rs1 as usize] as u32 as u64;
    let rs2 = ri.regs[args.rs2 as usize] as u32 as u64;

    let writetoreg: u64 = (rs1 >> 1)
        ^ (rs1 >> 7)
        ^ (rs1 >> 8)
        ^ (rs2 << 31)
        ^ (rs2 << 25)
        ^ (rs2 << 24);
    ri.regs[args.rd as usize] = ri.sign_ext(writetoreg);

}
pub fn sha512sig1(ri: &mut RiscvInt, args: &RiscvArgs) {
    let rs1 = ri.regs[args.rs1 as usize] as u64;
    let writetoreg: u64 = rs1.rotate_right(19) ^ rs1.rotate_right(61) ^ (rs1 >> 6);
    ri.regs[args.rd as usize] = writetoreg;

}
pub fn sha512sig1h(ri: &mut RiscvInt, args: &RiscvArgs) {
    let rs1 = ri.regs[args.rs1 as usize] as u32 as u64;
    let rs2 = ri.regs[args.rs2 as usize] as u32 as u64;

    let writetoreg: u64 = (rs1 << 3)
        ^ (rs1 >> 6)
        ^ (rs1 >> 19)
        ^ (rs2 >> 29)
        ^ (rs2 << 13);
    ri.regs[args.rd as usize] = ri.sign_ext(writetoreg);

}
pub fn sha512sig1l(ri: &mut RiscvInt, args: &RiscvArgs) {
    let rs1 = ri.regs[args.rs1 as usize] as u32 as u64;
    let rs2 = ri.regs[args.rs2 as usize] as u32 as u64;

    let writetoreg: u64 = (rs1 << 3)
        ^ (rs1 >> 6)
        ^ (rs1 >> 19)
        ^ (rs2 << 26)
        ^ (rs2 >> 29)
        ^ (rs2 << 13);
    ri.regs[args.rd as usize] = ri.sign_ext(writetoreg);

}
pub fn sha512sum0(ri: &mut RiscvInt, args: &RiscvArgs) {
    let rs1 = ri.regs[args.rs1 as usize] as u64;
    let writetoreg: u64 = rs1.rotate_right(28) ^ rs1.rotate_right(34) ^ rs1.rotate_right(39);
    ri.regs[args.rd as usize] = writetoreg;

}
pub fn sha512sum0r(ri: &mut RiscvInt, args: &RiscvArgs) {
    let rs1 = ri.regs[args.rs1 as usize] as u32 as u64;
    let rs2 = ri.regs[args.rs2 as usize] as u32 as u64;

    let writetoreg: u64 = (rs1 << 25)
        ^ (rs1 << 30)
        ^ (rs1 >> 28)
        ^ (rs2 >> 7)
        ^ (rs2 >> 2)
        ^ (rs2 << 4);
    ri.regs[args.rd as usize] = ri.sign_ext(writetoreg);

}
pub fn sha512sum1(ri: &mut RiscvInt, args: &RiscvArgs) {
    let rs1 = ri.regs[args.rs1 as usize] as u64;
    let writetoreg: u64 = rs1.rotate_right(14) ^ rs1.rotate_right(18) ^ rs1.rotate_right(41);
    ri.regs[args.rd as usize] = writetoreg;

}
pub fn sha512sum1r(ri: &mut RiscvInt, args: &RiscvArgs) {
    let rs1 = ri.regs[args.rs1 as usize] as u32 as u64;
    let rs2 = ri.regs[args.rs2 as usize] as u32 as u64;

    let writetoreg: u64 = (rs1 << 23)
        ^ (rs1 >> 14)
        ^ (rs1 >> 18)
        ^ (rs2 >> 9)
        ^ (rs2 << 18)
        ^ (rs2 << 14);
    ri.regs[args.rd as usize] = ri.sign_ext(writetoreg);

}
pub fn aes32dsi(ri: &mut RiscvInt, args: &RiscvArgs) {
    // note that "bs" is "shamt" and it is already multipled by 3
    // todo: check if decodetree does anything similar. we know it does for some other insteuctions
    let t: usize = (ri.regs[args.rs2 as usize] >> (args.shamt as u64)) as usize;
    let x: u8 = AES_DEC_SANDBOX[t];
    let mut u: u32 = x as u32;
    u = (u << args.shamt) | (u >> (32 - args.shamt));
    ri.regs[args.rd as usize] = ri.sign_ext(((u as u32) ^ (ri.regs[args.rs1 as usize] as u32)) as u64);

}
pub fn aes32dsmi(ri: &mut RiscvInt, args: &RiscvArgs) {
    let t: usize = (ri.regs[args.rs2 as usize] >> (args.shamt as u64)) as usize;
    let x: u8 = AES_DEC_SANDBOX[t];
    let mut u: u32 = ((aes_gfmul(x, 0xb) as u32) << 24)
        | ((aes_gfmul(x, 0xd) as u32) << 16)
        | ((aes_gfmul(x, 0x9) as u32) << 8)
        | (aes_gfmul(x, 0xe) as u32);
    u = (u << args.shamt) | (u >> (32 - args.shamt));
    ri.regs[args.rd as usize] = ri.sign_ext(((u as u32) ^ (ri.regs[args.rs1 as usize] as u32)) as u64);
}
pub fn aes32esi(ri: &mut RiscvInt, args: &RiscvArgs) {
    let t: usize = (ri.regs[args.rs2 as usize] >> (args.shamt as u64)) as usize;
    let x: u8 = AES_ENC_SANDBOX[t];
    let mut u: u32 = x as u32;
    u = (u << args.shamt) | (u >> (32 - args.shamt));
    ri.regs[args.rd as usize] = ri.sign_ext(((u as u32) ^ (ri.regs[args.rs1 as usize] as u32)) as u64);
}
pub fn aes32esmi(ri: &mut RiscvInt, args: &RiscvArgs) {
    let t: usize = (ri.regs[args.rs2 as usize] >> (args.shamt as u64)) as usize;
    let x: u32 = AES_ENC_SANDBOX[t] as u32;
    let mut u: u32 = ((aes_gfmul(x as u8, 3) as u32) << 24)
        | (x << 16)
        | (x << 8)
        | (aes_gfmul(x as u8, 2) as u32);
    u = (u << args.shamt) | (u >> (32 - args.shamt));
    ri.regs[args.rd as usize] = ri.sign_ext(((u as u32) ^ (ri.regs[args.rs1 as usize] as u32)) as u64);
}
pub fn aes64ds(ri: &mut RiscvInt, args: &RiscvArgs) {
    let mut temp = aes_invshifrows_lo(ri.regs[args.rs1 as usize], ri.regs[args.rs2 as usize]);
    temp = (
        ((AES_DEC_SANDBOX[((temp >>  0) & 0xFF) as usize] as u64) <<  0) |
        ((AES_DEC_SANDBOX[((temp >>  8) & 0xFF) as usize] as u64) <<  8) |
        ((AES_DEC_SANDBOX[((temp >>  16) & 0xFF) as usize] as u64) << 16) |
        ((AES_DEC_SANDBOX[((temp >>  24) & 0xFF) as usize] as u64) << 24) |
        ((AES_DEC_SANDBOX[((temp >>  32) & 0xFF) as usize] as u64) << 32) |
        ((AES_DEC_SANDBOX[((temp >>  40) & 0xFF) as usize] as u64) << 40) |
        ((AES_DEC_SANDBOX[((temp >>  48) & 0xFF) as usize] as u64) << 48) |
    ((AES_DEC_SANDBOX[((temp >>  56) & 0xFF) as usize] as u64) << 56)
    );
    
    ri.regs[args.rd as usize] = temp;
}
pub fn aes64dsm(ri: &mut RiscvInt, args: &RiscvArgs) {
    let mut temp = aes_invshifrows_lo(ri.regs[args.rs1 as usize], ri.regs[args.rs2 as usize]);
    temp = (
        ((AES_DEC_SANDBOX[((temp >>  0) & 0xFF) as usize] as u64) <<  0) |
            ((AES_DEC_SANDBOX[((temp >>  8) & 0xFF) as usize] as u64) <<  8) |
            ((AES_DEC_SANDBOX[((temp >>  16) & 0xFF) as usize] as u64) << 16) |
            ((AES_DEC_SANDBOX[((temp >>  24) & 0xFF) as usize] as u64) << 24) |
            ((AES_DEC_SANDBOX[((temp >>  32) & 0xFF) as usize] as u64) << 32) |
            ((AES_DEC_SANDBOX[((temp >>  40) & 0xFF) as usize] as u64) << 40) |
            ((AES_DEC_SANDBOX[((temp >>  48) & 0xFF) as usize] as u64) << 48) |
            ((AES_DEC_SANDBOX[((temp >>  56) & 0xFF) as usize] as u64) << 56)
    );
    let mut col_0: u32 = (temp & 0xFFFFFFFF) as u32;
    let mut col_1: u32 = (temp >> 32) as u32;
    col_0 = invmixcolumn(col_0 as u64);
    col_1 = invmixcolumn(col_1 as u64);
    let result: u64 = ((col_1 as u64) << 32) | (col_0 as u64);
    ri.regs[args.rd as usize] = result;
}
pub fn aes64im(ri: &mut RiscvInt, args: &RiscvArgs) {
    let rs1 = ri.regs[args.rs1 as usize];
    let mut col_0: u32 = (rs1 & 0xFFFFFFFF) as u32;
    let mut col_1: u32 = (rs1 >> 32) as u32;

    col_0 = invmixcolumn(col_0 as u64);
    col_1 = invmixcolumn(col_1 as u64);
    let result: u64 = ((col_1 as u64) << 32) | (col_0 as u64);
    ri.regs[args.rd as usize] = result;
}
pub fn aes64ks1i(ri: &mut RiscvInt, args: &RiscvArgs) {
    // imm is rcon
    let round_consts: [u8; 10] =[
        0x01, 0x02, 0x04, 0x08, 0x10, 0x20, 0x40, 0x80, 0x1b, 0x36
    ];
    let enc_rcon = args.imm;
    if enc_rcon > 0xa {
        panic!(); // todo: illegal instruction trap
    }
    let rs1 = ri.regs[args.rs1 as usize];
    let mut temp: u32 = ((rs1 >> 32) & 0xFFFFFFFF) as u32;
    let mut rcon: u8 = 0;
    if enc_rcon != 0xa {
        temp    = (temp >> 8) | (temp << 24);
        rcon    = round_consts[enc_rcon as usize];


    }
    temp = (
        ((AES_ENC_SANDBOX[((temp >>  24) & 0xFF) as usize] as u32) <<  24) |
            ((AES_ENC_SANDBOX[((temp >>  16) & 0xFF) as usize] as u32) <<  16) |
            ((AES_ENC_SANDBOX[((temp >>  8) & 0xFF) as usize] as u32) << 8) |
            (AES_ENC_SANDBOX[(temp & 0xFF) as usize] as u32));
    temp ^= (rcon as u32);
    let result: u64 = ((temp as u64) << 32) | (temp as u64);
    ri.regs[args.rd as usize] = result;
}
pub fn aes64ks2(ri: &mut RiscvInt, args: &RiscvArgs) {
    let rs1 = ri.regs[args.rs1 as usize];
    let rs2 = ri.regs[args.rs2 as usize];

    let rs1_hi  =  (rs1 >> 32) as u32;
    let rs2_lo  =  (rs2 & 0xffffffff) as u32;
    let rs2_hi  =  (rs2 >> 32) as u32;

    let r_lo = rs1_hi ^ rs2_lo;
    let r_hi = rs1_hi ^ rs2_lo ^ rs2_hi;
    let result: u64 = ((r_hi as u64) << 32) | (r_lo as u64);
    ri.regs[args.rd as usize] = result;


}
pub fn sm3p0(ri: &mut RiscvInt, args: &RiscvArgs) {
    let rs1 = ri.regs[args.rs1 as usize] as u32;
    let retval  = rs1 ^ rs1.rotate_left(9) ^ rs1.rotate_left(17);
    ri.regs[args.rd as usize] = retval as i32 as i64 as u64;
}
pub fn sm3p1(ri: &mut RiscvInt, args: &RiscvArgs) {
    let rs1 = ri.regs[args.rs1 as usize] as u32;
    let retval  = rs1 ^ rs1.rotate_left(15) ^ rs1.rotate_left(23);
    ri.regs[args.rd as usize] = retval as i32 as i64 as u64;
}
pub fn sm4ed(ri: &mut RiscvInt, args: &RiscvArgs) {
    let rs1 = ri.regs[args.rs1 as usize];
    let rs2 = ri.regs[args.rs2 as usize];

    let sb_in: usize = ((rs2 >> (args.shamt as u64)) & 0xff) as usize;
    let sb_out: u32 = sm4_sbox[sb_in] as u32;
    let linear: u32 = sb_out ^ (sb_out <<  8) ^
        (sb_out <<  2) ^
        (sb_out  << 18) ^
        ((sb_out & 0x3f) << 26) ^
        ((sb_out & 0xC0) << 10);
    let rotl: u32 = (linear << (args.shamt as u32)) | (linear >> ((32-args.shamt) as u32));
    let result: u32 = rotl ^ (rs1 as u32);
    ri.regs[args.rd as usize] = result as i32 as i64 as u64;

}
pub fn sm4ks(ri: &mut RiscvInt, args: &RiscvArgs) {
    let rs1 = ri.regs[args.rs1 as usize];
    let rs2 = ri.regs[args.rs2 as usize];

    let sb_in: usize = ((rs2 >> (args.shamt as u64)) & 0xff) as usize;
    let sb_out: u32 = sm4_sbox[sb_in] as u32;
    let x: u32 = sb_out ^
        ((sb_out & 0x07) << 29) ^ ((sb_out & 0xFE) <<  7) ^
        ((sb_out & 0x01) << 23) ^ ((sb_out & 0xF8) << 13);

    let rotl: u32 = (x << (args.shamt as u32)) | (x >> ((32-args.shamt) as u32));
    let result: u32 = rotl ^ (rs1 as u32);
    ri.regs[args.rd as usize] = result as i32 as i64 as u64;

}