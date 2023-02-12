use crate::threaded::ops_impl::*;


pub extern "win64" fn x64_extsw(a: u64, b: u64) { extsw(a, b); }
pub extern "win64" fn x64_extuw(a: u64, b: u64) { extuw(a, b); }
pub extern "win64" fn x64_extsh(a: u64, b: u64) { extsh(a, b); }
pub extern "win64" fn x64_extuh(a: u64, b: u64) { extuh(a, b); }
pub extern "win64" fn x64_extsb(a: u64, b: u64) { extsb(a, b); }
pub extern "win64" fn x64_extub(a: u64, b: u64) { extub(a, b); }
pub extern "win64" fn x64_load_ext_reg(a: u64, b: u64) { load_ext_reg(a, b); }
pub extern "win64" fn x64_store_ext_reg(a: u64, b: u64) { store_ext_reg(a, b); }
pub extern "win64" fn x64_mov_temp(a: u64, b: u64) { mov_temp(a, b); }
pub extern "win64" fn x64_sub(a: u64, b: u64, c: u64) { sub(a, b, c); }

pub extern "win64" fn x64_load_imm64(a: u64, b: u64) { load_imm64(a, b); }
pub extern "win64" fn x64_temp_load_imm64(a: u64, b: u64) { temp_load_imm64(a, b); }
pub extern "win64" fn x64_neg64(a: u64, b: u64) { neg64(a, b); }
pub extern "win64" fn x64_neg32(a: u64, b: u64) { neg32(a, b); }
pub extern "win64" fn x64_mov_reg(a: u64, b: u64) { mov_reg(a, b); }

pub extern "win64" fn x64_ceq(a: u64, b: u64, c: u64) { ceq(a, b, c); }
pub extern "win64" fn x64_cne(a: u64, b: u64, c: u64) { cne(a, b, c); }
pub extern "win64" fn x64_cslel(a: u64, b: u64, c: u64) { cslel(a, b, c); }
pub extern "win64" fn x64_cslew(a: u64, b: u64, c: u64) { cslew(a, b, c); }
pub extern "win64" fn x64_csltl(a: u64, b: u64, c: u64) { csltl(a, b, c); }
pub extern "win64" fn x64_csltw(a: u64, b: u64, c: u64) { csltw(a, b, c); }
pub extern "win64" fn x64_csgel(a: u64, b: u64, c: u64) { csgel(a, b, c); }
pub extern "win64" fn x64_csgew(a: u64, b: u64, c: u64) { csgew(a, b, c); }
pub extern "win64" fn x64_csgtl(a: u64, b: u64, c: u64) { csgtl(a, b, c); }
pub extern "win64" fn x64_csgtw(a: u64, b: u64, c: u64) { csgtw(a, b, c); }
pub extern "win64" fn x64_culel(a: u64, b: u64, c: u64) { culel(a, b, c); }
pub extern "win64" fn x64_culew(a: u64, b: u64, c: u64) { culew(a, b, c); }
pub extern "win64" fn x64_cultl(a: u64, b: u64, c: u64) { cultl(a, b, c); }
pub extern "win64" fn x64_cultw(a: u64, b: u64, c: u64) { cultw(a, b, c); }
pub extern "win64" fn x64_cugel(a: u64, b: u64, c: u64) { cugel(a, b, c); }
pub extern "win64" fn x64_cugew(a: u64, b: u64, c: u64) { cugew(a, b, c); }
pub extern "win64" fn x64_cugtl(a: u64, b: u64, c: u64) { cugtl(a, b, c); }
pub extern "win64" fn x64_cugtw(a: u64, b: u64, c: u64) { cugtw(a, b, c); }
pub extern "win64" fn x64_add64(a: u64, b: u64, c: u64) { add64(a, b, c); }
pub extern "win64" fn x64_add32(a: u64, b: u64, c: u64) { add32(a, b, c); }
pub extern "win64" fn x64_udiv64(a: u64, b: u64, c: u64) { udiv64(a, b, c); }
pub extern "win64" fn x64_udiv32(a: u64, b: u64, c: u64) { udiv32(a, b, c); }
pub extern "win64" fn x64_sdiv64(a: u64, b: u64, c: u64) { sdiv64(a, b, c); }
pub extern "win64" fn x64_sdiv32(a: u64, b: u64, c: u64) { sdiv32(a, b, c); }
pub extern "win64" fn x64_mul64l(a: u64, b: u64, c: u64) { mul64l(a, b, c); }
pub extern "win64" fn x64_umul64h(a: u64, b: u64, c: u64) { umul64h(a, b, c); }
pub extern "win64" fn x64_smul64h(a: u64, b: u64, c: u64) { smul64h(a, b, c); }
pub extern "win64" fn x64_mul32l(a: u64, b: u64, c: u64) { mul32l(a, b, c); }
pub extern "win64" fn x64_umul32h(a: u64, b: u64, c: u64) { umul32h(a, b, c); }
pub extern "win64" fn x64_smul32h(a: u64, b: u64, c: u64) { smul32h(a, b, c); }
pub extern "win64" fn x64_urem64(a: u64, b: u64, c: u64) { urem64(a, b, c); }
pub extern "win64" fn x64_urem32(a: u64, b: u64, c: u64) { urem32(a, b, c); }
pub extern "win64" fn x64_srem64(a: u64, b: u64, c: u64) { srem64(a, b, c); }
pub extern "win64" fn x64_srem32(a: u64, b: u64, c: u64) { srem32(a, b, c); }
pub extern "win64" fn x64_or(a: u64, b: u64, c: u64) { or(a, b, c); }
pub extern "win64" fn x64_and(a: u64, b: u64, c: u64) { and(a, b, c); }
pub extern "win64" fn x64_xor(a: u64, b: u64, c: u64) { xor(a, b, c); }
pub extern "win64" fn x64_sar(a: u64, b: u64, c: u64) { sar(a, b, c); }
pub extern "win64" fn x64_shr(a: u64, b: u64, c: u64) { shr(a, b, c); }
pub extern "win64" fn x64_shl(a: u64, b: u64, c: u64) { shl(a, b, c); }