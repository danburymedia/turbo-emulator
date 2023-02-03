#![allow(dead_code, unused_variables)]

use jit::extract::*;
use crate::riscv::common::RiscvArgs;
pub trait DecodeTrait {
    fn c_illegal(&mut self, args: RiscvArgs) -> bool { return false; }
    fn c_addi(&mut self, args: RiscvArgs) -> bool { return false; }
    fn c_lq(&mut self, args: RiscvArgs) -> bool { return false; }
    fn c_fld(&mut self, args: RiscvArgs) -> bool { return false; }
    fn c_lw(&mut self, args: RiscvArgs) -> bool { return false; }
    fn c_sq(&mut self, args: RiscvArgs) -> bool { return false; }
    fn c_fsd(&mut self, args: RiscvArgs) -> bool { return false; }
    fn c_sw(&mut self, args: RiscvArgs) -> bool { return false; }
    fn c_ld(&mut self, args: RiscvArgs) -> bool { return false; }
    fn c_flw(&mut self, args: RiscvArgs) -> bool { return false; }
    fn c_sd(&mut self, args: RiscvArgs) -> bool { return false; }
    fn c_fsw(&mut self, args: RiscvArgs) -> bool { return false; }
    fn c_lui(&mut self, args: RiscvArgs) -> bool { return false; }
    fn c_srli(&mut self, args: RiscvArgs) -> bool { return false; }
    fn c_srai(&mut self, args: RiscvArgs) -> bool { return false; }
    fn c_andi(&mut self, args: RiscvArgs) -> bool { return false; }
    fn c_sub(&mut self, args: RiscvArgs) -> bool { return false; }
    fn c_xor(&mut self, args: RiscvArgs) -> bool { return false; }
    fn c_or(&mut self, args: RiscvArgs) -> bool { return false; }
    fn c_and(&mut self, args: RiscvArgs) -> bool { return false; }
    fn c_jal(&mut self, args: RiscvArgs) -> bool { return false; }
    fn c_beq(&mut self, args: RiscvArgs) -> bool { return false; }
    fn c_bne(&mut self, args: RiscvArgs) -> bool { return false; }
    fn c64_illegal(&mut self, args: RiscvArgs) -> bool { return false; }
    fn c_addiw(&mut self, args: RiscvArgs) -> bool { return false; }
    fn c_subw(&mut self, args: RiscvArgs) -> bool { return false; }
    fn c_addw(&mut self, args: RiscvArgs) -> bool { return false; }
    fn c_slli(&mut self, args: RiscvArgs) -> bool { return false; }
    fn c_jalr(&mut self, args: RiscvArgs) -> bool { return false; }
    fn c_ebreak(&mut self, args: RiscvArgs) -> bool { return false; }
    fn c_add(&mut self, args: RiscvArgs) -> bool { return false; }
    fn is_128_bit(&self) ->bool {return false; }
}
fn ex_rvc_shiftli<T: DecodeTrait>(ctx: &T, imm: u32) -> u32 {
    if ctx.is_128_bit() && imm == 0 {
        64
    } else {
        imm
    }
}
fn ex_rvc_shiftri<T: DecodeTrait>(ctx: &T, imm: u32) -> u32 {
    if ctx.is_128_bit() {
        let imm2 = imm | ((imm & 32) << 1);
        if imm2 == 0 {
            64
        } else {
            imm2
        }
    } else {
        imm
    }
}
fn decode_extract_c_addi16sp<T: DecodeTrait>(ctx: &T, a: &mut RiscvArgs, insn: u16)
{
    a.imm = ex_shift_4(deposit32(deposit32(deposit32(deposit32(extract32(insn as u32, 6, 1), 1, 31, extract32(insn as u32, 2, 1)), 2, 30, extract32(insn as u32, 5, 1)), 3, 29, extract32(insn as u32, 3, 2)), 5, 27, sextract32(insn as u32, 12, 1)));
    a.rs1 = 2;
    a.rd = 2;
}

fn decode_extract_c_addi4spn<T: DecodeTrait>(ctx: &T, a: &mut RiscvArgs, insn: u16)
{
    a.imm = ex_shift_2(deposit32(deposit32(deposit32(extract32(insn as u32, 6, 1), 1, 31, extract32(insn as u32, 5, 1)), 2, 30, extract32(insn as u32, 11, 2)), 4, 28, extract32(insn as u32, 7, 4)));
    a.rs1 = 2;
    a.rd = ex_rvc_register(extract32(insn as u32, 2, 3));
}

fn decode_extract_c_andi<T: DecodeTrait>(ctx: &T, a: &mut RiscvArgs, insn: u16)
{
    a.imm = deposit32(extract32(insn as u32, 2, 5), 5, 27, sextract32(insn as u32, 12, 1));
    a.rs1 = ex_rvc_register(extract32(insn as u32, 7, 3));
    a.rd = ex_rvc_register(extract32(insn as u32, 7, 3));
}

fn decode_extract_c_jalr<T: DecodeTrait>(ctx: &T, a: &mut RiscvArgs, insn: u16)
{
    a.imm = 0;
    a.rs1 = extract32(insn as u32, 7, 5);
}

fn decode_extract_c_ldsp<T: DecodeTrait>(ctx: &T, a: &mut RiscvArgs, insn: u16)
{
    a.imm = ex_shift_3(deposit32(deposit32(extract32(insn as u32, 5, 2), 2, 30, extract32(insn as u32, 12, 1)), 3, 29, extract32(insn as u32, 2, 3)));
    a.rs1 = 2;
    a.rd = extract32(insn as u32, 7, 5);
}

fn decode_extract_c_li<T: DecodeTrait>(ctx: &T, a: &mut RiscvArgs, insn: u16)
{
    a.imm = deposit32(extract32(insn as u32, 2, 5), 5, 27, sextract32(insn as u32, 12, 1));
    a.rs1 = 0;
    a.rd = extract32(insn as u32, 7, 5);
}

fn decode_extract_c_lqsp<T: DecodeTrait>(ctx: &T, a: &mut RiscvArgs, insn: u16)
{
    a.imm = ex_shift_4(deposit32(deposit32(extract32(insn as u32, 6, 1), 1, 31, extract32(insn as u32, 12, 1)), 2, 30, extract32(insn as u32, 2, 4)));
    a.rs1 = 2;
    a.rd = extract32(insn as u32, 7, 5);
}

fn decode_extract_c_lui<T: DecodeTrait>(ctx: &T, a: &mut RiscvArgs, insn: u16)
{
    a.imm = ex_shift_12(deposit32(extract32(insn as u32, 2, 5), 5, 27, sextract32(insn as u32, 12, 1)));
    a.rd = extract32(insn as u32, 7, 5);
}

fn decode_extract_c_lwsp<T: DecodeTrait>(ctx: &T, a: &mut RiscvArgs, insn: u16)
{
    a.imm = ex_shift_2(deposit32(deposit32(extract32(insn as u32, 4, 3), 3, 29, extract32(insn as u32, 12, 1)), 4, 28, extract32(insn as u32, 2, 2)));
    a.rs1 = 2;
    a.rd = extract32(insn as u32, 7, 5);
}

fn decode_extract_c_mv<T: DecodeTrait>(ctx: &T, a: &mut RiscvArgs, insn: u16)
{
    a.imm = 0;
    a.rs1 = extract32(insn as u32, 2, 5);
    a.rd = extract32(insn as u32, 7, 5);
}

fn decode_extract_c_sdsp<T: DecodeTrait>(ctx: &T, a: &mut RiscvArgs, insn: u16)
{
    a.imm = ex_shift_3(deposit32(extract32(insn as u32, 10, 3), 3, 29, extract32(insn as u32, 7, 3)));
    a.rs1 = 2;
    a.rs2 = extract32(insn as u32, 2, 5);
}

fn decode_extract_c_shift<T: DecodeTrait>(ctx: &T, a: &mut RiscvArgs, insn: u16)
{
    a.rd = ex_rvc_register(extract32(insn as u32, 7, 3));
    a.rs1 = ex_rvc_register(extract32(insn as u32, 7, 3));
    a.shamt = ex_rvc_shiftri(ctx, deposit32(extract32(insn as u32, 2, 5), 5, 27, extract32(insn as u32, 12, 1)));
}

fn decode_extract_c_shift2<T: DecodeTrait>(ctx: &T, a: &mut RiscvArgs, insn: u16)
{
    a.rd = extract32(insn as u32, 7, 5);
    a.rs1 = extract32(insn as u32, 7, 5);
    a.shamt = ex_rvc_shiftli(ctx, deposit32(extract32(insn as u32, 2, 5), 5, 27, extract32(insn as u32, 12, 1)));
}

fn decode_extract_c_sqsp<T: DecodeTrait>(ctx: &T, a: &mut RiscvArgs, insn: u16)
{
    a.imm = ex_shift_4(deposit32(extract32(insn as u32, 11, 2), 2, 30, extract32(insn as u32, 7, 4)));
    a.rs1 = 2;
    a.rs2 = extract32(insn as u32, 2, 5);
}

fn decode_extract_c_swsp<T: DecodeTrait>(ctx: &T, a: &mut RiscvArgs, insn: u16)
{
    a.imm = ex_shift_2(deposit32(extract32(insn as u32, 9, 4), 4, 28, extract32(insn as u32, 7, 2)));
    a.rs1 = 2;
    a.rs2 = extract32(insn as u32, 2, 5);
}

fn decode_extract_cb_z<T: DecodeTrait>(ctx: &T, a: &mut RiscvArgs, insn: u16)
{
    a.imm = ex_shift_1(deposit32(deposit32(deposit32(deposit32(extract32(insn as u32, 3, 2), 2, 30, extract32(insn as u32, 10, 2)), 4, 28, extract32(insn as u32, 2, 1)), 5, 27, extract32(insn as u32, 5, 2)), 7, 25, sextract32(insn as u32, 12, 1)));
    a.rs1 = ex_rvc_register(extract32(insn as u32, 7, 3));
    a.rs2 = 0;
}

fn decode_extract_ci<T: DecodeTrait>(ctx: &T, a: &mut RiscvArgs, insn: u16)
{
    a.imm = deposit32(extract32(insn as u32, 2, 5), 5, 27, sextract32(insn as u32, 12, 1));
    a.rs1 = extract32(insn as u32, 7, 5);
    a.rd = extract32(insn as u32, 7, 5);
}

fn decode_extract_cj<T: DecodeTrait>(ctx: &T, a: &mut RiscvArgs, insn: u16)
{
    a.imm = ex_shift_1(deposit32(deposit32(deposit32(deposit32(deposit32(deposit32(deposit32(extract32(insn as u32, 3, 3), 3, 29, extract32(insn as u32, 11, 1)), 4, 28, extract32(insn as u32, 2, 1)), 5, 27, extract32(insn as u32, 7, 1)), 6, 26, extract32(insn as u32, 6, 1)), 7, 25, extract32(insn as u32, 9, 2)), 9, 23, extract32(insn as u32, 8, 1)), 10, 22, sextract32(insn as u32, 12, 1)));
}

fn decode_extract_cl_d<T: DecodeTrait>(ctx: &T, a: &mut RiscvArgs, insn: u16)
{
    a.imm = ex_shift_3(deposit32(extract32(insn as u32, 10, 3), 3, 29, extract32(insn as u32, 5, 2)));
    a.rs1 = ex_rvc_register(extract32(insn as u32, 7, 3));
    a.rd = ex_rvc_register(extract32(insn as u32, 2, 3));
}

fn decode_extract_cl_q<T: DecodeTrait>(ctx: &T, a: &mut RiscvArgs, insn: u16)
{
    a.imm = ex_shift_4(deposit32(deposit32(extract32(insn as u32, 11, 2), 2, 30, extract32(insn as u32, 5, 2)), 4, 28, extract32(insn as u32, 10, 1)));
    a.rs1 = ex_rvc_register(extract32(insn as u32, 7, 3));
    a.rd = ex_rvc_register(extract32(insn as u32, 2, 3));
}

fn decode_extract_cl_w<T: DecodeTrait>(ctx: &T, a: &mut RiscvArgs, insn: u16)
{
    a.imm = ex_shift_2(deposit32(deposit32(extract32(insn as u32, 6, 1), 1, 31, extract32(insn as u32, 10, 3)), 4, 28, extract32(insn as u32, 5, 1)));
    a.rs1 = ex_rvc_register(extract32(insn as u32, 7, 3));
    a.rd = ex_rvc_register(extract32(insn as u32, 2, 3));
}

fn decode_extract_cr<T: DecodeTrait>(ctx: &T, a: &mut RiscvArgs, insn: u16)
{
    a.rs2 = extract32(insn as u32, 2, 5);
    a.rs1 = extract32(insn as u32, 7, 5);
    a.rd = extract32(insn as u32, 7, 5);
}

fn decode_extract_cs_2<T: DecodeTrait>(ctx: &T, a: &mut RiscvArgs, insn: u16)
{
    a.rs2 = ex_rvc_register(extract32(insn as u32, 2, 3));
    a.rs1 = ex_rvc_register(extract32(insn as u32, 7, 3));
    a.rd = ex_rvc_register(extract32(insn as u32, 7, 3));
}

fn decode_extract_cs_d<T: DecodeTrait>(ctx: &T, a: &mut RiscvArgs, insn: u16)
{
    a.imm = ex_shift_3(deposit32(extract32(insn as u32, 10, 3), 3, 29, extract32(insn as u32, 5, 2)));
    a.rs1 = ex_rvc_register(extract32(insn as u32, 7, 3));
    a.rs2 = ex_rvc_register(extract32(insn as u32, 2, 3));
}

fn decode_extract_cs_q<T: DecodeTrait>(ctx: &T, a: &mut RiscvArgs, insn: u16)
{
    a.imm = ex_shift_4(deposit32(deposit32(extract32(insn as u32, 11, 2), 2, 30, extract32(insn as u32, 5, 2)), 4, 28, extract32(insn as u32, 10, 1)));
    a.rs1 = ex_rvc_register(extract32(insn as u32, 7, 3));
    a.rs2 = ex_rvc_register(extract32(insn as u32, 2, 3));
}

fn decode_extract_cs_w<T: DecodeTrait>(ctx: &T, a: &mut RiscvArgs, insn: u16)
{
    a.imm = ex_shift_2(deposit32(deposit32(extract32(insn as u32, 6, 1), 1, 31, extract32(insn as u32, 10, 3)), 4, 28, extract32(insn as u32, 5, 1)));
    a.rs1 = ex_rvc_register(extract32(insn as u32, 7, 3));
    a.rs2 = ex_rvc_register(extract32(insn as u32, 2, 3));
}


pub fn decode<T: DecodeTrait>(transimpl: &mut T, insn: u16) -> bool
{

    let mut args: RiscvArgs = Default::default();
    match insn & 0xe003 {
        0x0000 => {
            /* 000..... ......00 */
            if (insn & 0x00001fe0) == 0x00000000 {
                if transimpl.c_illegal(args) { return true; }
            }
            decode_extract_c_addi4spn(transimpl, &mut args, insn);
            if transimpl.c_addi(args) { return true; }
        },
        0x0001 => {
            /* 000..... ......01 */
            decode_extract_ci(transimpl, &mut args, insn);
            if transimpl.c_addi(args) { return true; }
        },
        0x0002 => {
            /* 000..... ......10 */
            decode_extract_c_shift2(transimpl, &mut args, insn);
            if transimpl.c_slli(args) { return true; }
        },
        0x2000 => {
            /* 001..... ......00 */
            decode_extract_cl_q(transimpl, &mut args, insn);
            if transimpl.c_lq(args) { return true; }
            decode_extract_cl_d(transimpl, &mut args, insn);
            if transimpl.c_fld(args) { return true; }
        },
        0x2001 => {
            /* 001..... ......01 */
            if (insn & 0x00000f80) == 0x00000000 {
                if transimpl.c64_illegal(args) { return true; }
            }
            decode_extract_ci(transimpl, &mut args, insn);
            if transimpl.c_addiw(args) { return true; }
            decode_extract_cj(transimpl, &mut args, insn);
            args.rd = 1;
            if transimpl.c_jal(args) { return true; }
        },
        0x2002 => {
            /* 001..... ......10 */
            decode_extract_c_lqsp(transimpl, &mut args, insn);
            if transimpl.c_lq(args) { return true; }
            decode_extract_c_ldsp(transimpl, &mut args, insn);
            if transimpl.c_fld(args) { return true; }
        },
        0x4000 => {
            /* 010..... ......00 */
            decode_extract_cl_w(transimpl, &mut args, insn);
            if transimpl.c_lw(args) { return true; }
        },
        0x4001 => {
            /* 010..... ......01 */
            decode_extract_c_li(transimpl, &mut args, insn);
            if transimpl.c_addi(args) { return true; }
        },
        0x4002 => {
            /* 010..... ......10 */
            if (insn & 0x00000f80) == 0x00000000 {
                if transimpl.c_illegal(args) { return true; }
            }
            decode_extract_c_lwsp(transimpl, &mut args, insn);
            if transimpl.c_lw(args) { return true; }
        },
        0x6000 => {
            /* 011..... ......00 */
            decode_extract_cl_d(transimpl, &mut args, insn);
            if transimpl.c_ld(args) { return true; }
            decode_extract_cl_w(transimpl, &mut args, insn);
            if transimpl.c_flw(args) { return true; }
        },
        0x6001 => {
            /* 011..... ......01 */
            if (insn & 0x0000107c) == 0x00000000 {
                if transimpl.c_illegal(args) { return true; }
            }
            if (insn & 0x00000f80) == 0x00000100 {
                decode_extract_c_addi16sp(transimpl, &mut args, insn);
                if transimpl.c_addi(args) { return true; }
            }
            decode_extract_c_lui(transimpl, &mut args, insn);
            if transimpl.c_lui(args) { return true; }
        },
        0x6002 => {
            /* 011..... ......10 */
            if (insn & 0x00000f80) == 0x00000000 {
                if transimpl.c64_illegal(args) { return true; }
            }
            decode_extract_c_ldsp(transimpl, &mut args, insn);
            if transimpl.c_ld(args) { return true; }
            decode_extract_c_lwsp(transimpl, &mut args, insn);
            if transimpl.c_flw(args) { return true; }
        },
        0x8001 => {
            /* 100..... ......01 */
            match (insn >> 10) & 0x3 {
                0x0 => {
                    /* 100.00.. ......01 */
                    decode_extract_c_shift(transimpl, &mut args, insn);
                    if transimpl.c_srli(args) { return true; }
                },
                0x1 => {
                    /* 100.01.. ......01 */
                    decode_extract_c_shift(transimpl, &mut args, insn);
                    if transimpl.c_srai(args) { return true; }
                },
                0x2 => {
                    /* 100.10.. ......01 */
                    decode_extract_c_andi(transimpl, &mut args, insn);
                    if transimpl.c_andi(args) { return true; }
                },
                0x3 => {
                    /* 100.11.. ......01 */
                    decode_extract_cs_2(transimpl, &mut args, insn);
                    match insn & 0x1060 {
                        0x0000 => {
                            /* 100011.. .00...01 */
                            if transimpl.c_sub(args) { return true; }
                        },
                        0x0020 => {
                            /* 100011.. .01...01 */
                            if transimpl.c_xor(args) { return true; }
                        },
                        0x0040 => {
                            /* 100011.. .10...01 */
                            if transimpl.c_or(args) { return true; }
                        },
                        0x0060 => {
                            /* 100011.. .11...01 */
                            if transimpl.c_and(args) { return true; }
                        },
                        0x1000 => {
                            /* 100111.. .00...01 */
                            if transimpl.c_subw(args) { return true; }
                        },
                        0x1020 => {
                            /* 100111.. .01...01 */
                            if transimpl.c_addw(args) { return true; }
                        },
                        _ => { },
                    };
                },
                _ => { },
            };
        },
        0x8002 => {
            /* 100..... ......10 */
            match (insn >> 12) & 0x1 {
                0x0 => {
                    /* 1000.... ......10 */
                    if (insn & 0x00000ffc) == 0x00000000 {
                        if transimpl.c_illegal(args) { return true; }
                    }
                    if (insn & 0x0000007c) == 0x00000000 {
                        decode_extract_c_jalr(transimpl, &mut args, insn);
                        args.rd = 0;
                        if transimpl.c_jalr(args) { return true; }
                    }
                    decode_extract_c_mv(transimpl, &mut args, insn);
                    if transimpl.c_addi(args) { return true; }
                },
                0x1 => {
                    /* 1001.... ......10 */
                    if (insn & 0x00000ffc) == 0x00000000 {
                        if transimpl.c_ebreak(args) { return true; }
                    }
                    if (insn & 0x0000007c) == 0x00000000 {
                        decode_extract_c_jalr(transimpl, &mut args, insn);
                        args.rd = 1;
                        if transimpl.c_jalr(args) { return true; }
                    }
                    decode_extract_cr(transimpl, &mut args, insn);
                    if transimpl.c_add(args) { return true; }
                },
                _ => { },
            };
        },
        0xa000 => {
            /* 101..... ......00 */
            decode_extract_cs_q(transimpl, &mut args, insn);
            if transimpl.c_sq(args) { return true; }
            decode_extract_cs_d(transimpl, &mut args, insn);
            if transimpl.c_fsd(args) { return true; }
        },
        0xa001 => {
            /* 101..... ......01 */
            decode_extract_cj(transimpl, &mut args, insn);
            args.rd = 0;
            if transimpl.c_jal(args) { return true; }
        },
        0xa002 => {
            /* 101..... ......10 */
            decode_extract_c_sqsp(transimpl, &mut args, insn);
            if transimpl.c_sq(args) { return true; }
            decode_extract_c_sdsp(transimpl, &mut args, insn);
            if transimpl.c_fsd(args) { return true; }
        },
        0xc000 => {
            /* 110..... ......00 */
            decode_extract_cs_w(transimpl, &mut args, insn);
            if transimpl.c_sw(args) { return true; }
        },
        0xc001 => {
            /* 110..... ......01 */
            decode_extract_cb_z(transimpl, &mut args, insn);
            if transimpl.c_beq(args) { return true; }
        },
        0xc002 => {
            /* 110..... ......10 */
            decode_extract_c_swsp(transimpl, &mut args, insn);
            if transimpl.c_sw(args) { return true; }
        },
        0xe000 => {
            /* 111..... ......00 */
            decode_extract_cs_d(transimpl, &mut args, insn);
            if transimpl.c_sd(args) { return true; }
            decode_extract_cs_w(transimpl, &mut args, insn);
            if transimpl.c_fsw(args) { return true; }
        },
        0xe001 => {
            /* 111..... ......01 */
            decode_extract_cb_z(transimpl, &mut args, insn);
            if transimpl.c_bne(args) { return true; }
        },
        0xe002 => {
            /* 111..... ......10 */
            decode_extract_c_sdsp(transimpl, &mut args, insn);
            if transimpl.c_sd(args) { return true; }
            decode_extract_c_swsp(transimpl, &mut args, insn);
            if transimpl.c_fsw(args) { return true; }
        },
        _ => { },
    };
    return false;
}
