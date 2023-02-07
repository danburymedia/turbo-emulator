use crate::armv8::decodedefs::{ArmInstr};
use crate::armv8::decoder64::Arm64DecodeTrait;

pub fn decode_undef<T: Arm64DecodeTrait>(trans: &mut T, insn: u32) -> bool {
    return false;
}
pub fn decode_pcreladdr<T: Arm64DecodeTrait>(trans: &mut T, insn: u32) -> bool {
    let op = ((insn >> 31) & 1) != 0;
    let args: ArmInstr = ArmInstr {
        insn
    };
    if !op {
        return trans.adr(args);
    }
    if op {
        return trans.adrp(args);
    }
    return false;
}
pub fn decode_condbranch<T: Arm64DecodeTrait>(trans: &mut T, insn: u32) -> bool {
    let args: ArmInstr = ArmInstr {
        insn
    };
    let o0 = ((insn >> 4) & 1) != 0;
    let o1 = ((insn >> 24) & 1) != 0;
    if(!o1 && !o0) { return trans.b_cond(args); } // -> b_only_condbranch
    return false;
}
pub fn decode_barriers<T: Arm64DecodeTrait>(trans: &mut T, insn: u32) -> bool {
    let args: ArmInstr = ArmInstr {
        insn
    };
    let rt = insn & 0x1f;
    let crm = ((insn >> 8) & 15);
    let op2 = ((insn >> 5) & 7);
    if((crm == 0) && op2==3 && rt==0x1f && trans.has_tme_feat()) { return trans.tcommit(args); } // -> tcommit_only_barriers
    if((crm&3)==2 && op2==1 && rt==0x1f && trans.has_sxs_feat()) { return trans.dsb(args); } // -> dsb_bon_barriers
    if(op2==2 && rt==0x1f) { return trans.clrex(args); } // -> clrex_bn_barriers
    if(op2==4 && rt==0x1f) { return trans.dsb(args); } // -> dsb_bo_barriers
    if(op2==5 && rt==0x1f) { return trans.dmb(args); } // -> dmb_bo_barriers
    if(op2==6 && rt==0x1f) { return trans.isb(args); } // -> isb_bi_barriers
    if(op2==7 && rt==0x1f) { return trans.sb(args); } // -> sb_only_barriers
    return false;
}
pub fn decode_compbranch<T: Arm64DecodeTrait>(trans: &mut T, insn: u32) -> bool {
    let args: ArmInstr = ArmInstr {
        insn
    };
    let sf = ((insn >> 31) & 1) != 0;
    let op = ((insn >> 24) & 1) != 0;
    if(!sf && !op) { return trans.cbz(args); } // -> cbz_32_compbranch
    if(!sf && op) { return trans.cbnz(args); } // -> cbnz_32_compbranch
    if(sf && !op) { return trans.cbz(args); } // -> cbz_64_compbranch
    if(sf && op) { return trans.cbnz(args); } // -> cbnz_64_compbranch
    return false;
}
pub fn decode_exception<T: Arm64DecodeTrait>(trans: &mut T, insn: u32) -> bool {
    let args: ArmInstr = ArmInstr {
        insn
    };
    let ll = insn & 3;
    let opc = ((insn >> 21) & 7);
    let op2 = ((insn >> 2) & 7);

    if((opc == 0) && (op2 == 0) && ll==1) { return trans.svc(args); } // -> svc_ex_exception
    if((opc == 0) && (op2 == 0) && ll==2) { return trans.hvc(args); } // -> hvc_ex_exception
    if((opc == 0) && (op2 == 0) && ll==3) { return trans.smc(args); } // -> smc_ex_exception
    if(opc==1 && (op2 == 0) && (ll == 0)) { return trans.brk(args); } // -> brk_ex_exception
    if(opc==2 && (op2 == 0) && (ll == 0)) { return trans.hlt(args); } // -> hlt_ex_exception
    if(opc==3 && (op2 == 0) && (ll == 0) && trans.has_tme_feat()) { return trans.tcancel(args); } // -> tcancel_ex_exception
    if(opc==5 && (op2 == 0) && ll==1) { return trans.dcps1(args); } // -> dcps1_dc_exception
    if(opc==5 && (op2 == 0) && ll==2) { return trans.dcps2(args); } // -> dcps2_dc_exception
    if(opc==5 && (op2 == 0) && ll==3) { return trans.dcps3(args); } // -> dcps3_dc_exception
    return false;
}
pub fn decode_hints<T: Arm64DecodeTrait>(trans: &mut T, insn: u32) -> bool {
    let crm = (insn >> 8) & 15;
    let op2 = (insn >> 5) & 7;
    let args: ArmInstr = ArmInstr {
        insn
    };
    if((crm == 0) && (op2 == 0)) { return trans.nop(args); } // -> nop_hi_hints
    /* if((crm == 0) && op2==1) { return trans.yield_insn(args); } // -> yield_hi_hints
    if((crm == 0) && op2==2) { return trans.wfe(args); } // -> wfe_hi_hints
    if((crm == 0) && op2==3) { return trans.wfi(args); } // -> wfi_hi_hints
    if((crm == 0) && op2==4) { return trans.sev(args); } // -> sev_hi_hints
    if((crm == 0) && op2==5) { return trans.sevl(args); } // -> sevl_hi_hints
    if((crm == 0) && op2==6 && trans.has_dgh()) { return trans.dgh(args); } // -> dgh_hi_hints
    if((crm == 0) && op2==7 && trans.has_pauth_feat()) { return trans.xpac(args); } // -> xpaclri_hi_hints
    if(crm==1 && (op2 == 0) && trans.has_pauth_feat()) { return trans.pacia(args); } // -> pacia1716_hi_hints
    if(crm==1 && op2==2 && trans.has_pauth_feat()) { return trans.pacib(args); } // -> pacib1716_hi_hints
    if(crm==1 && op2==4 && trans.has_pauth_feat()) { return trans.autia(args); } // -> autia1716_hi_hints
    if(crm==1 && op2==6 && trans.has_pauth_feat()) { return trans.autib(args); } // -> autib1716_hi_hints
    if(crm==2 && (op2 == 0) && trans.has_ras()) { return trans.esb(args); } // -> esb_hi_hints
    if(crm==2 && op2==1 && trans.has_spe()) { return trans.psb(args); } // -> psb_hc_hints
    if(crm==2 && op2==2 && trans.has_trf()) { return trans.tsb(args); } // -> tsb_hc_hints
    if(crm==2 && op2==4) { return trans.csdb(args); } // -> csdb_hi_hints
    if(crm==3 && (op2 == 0) && haspauth()) { return trans.pacia(args); } // -> paciaz_hi_hints
    if(crm==3 && op2==1 && haspauth()) { return trans.pacia(args); } // -> paciasp_hi_hints
    if(crm==3 && op2==2 && haspauth()) { return trans.pacib(args); } // -> pacibz_hi_hints
    if(crm==3 && op2==3 && haspauth()) { return trans.pacib(args); } // -> pacibsp_hi_hints
    if(crm==3 && op2==4 && haspauth()) { return trans.autia(args); } // -> autiaz_hi_hints
    if(crm==3 && op2==5 && haspauth()) { return trans.autia(args); } // -> autiasp_hi_hints
    if(crm==3 && op2==6 && haspauth()) { return trans.autib(args); } // -> autibz_hi_hints
    if(crm==3 && op2==7 && haspauth()) { return trans.autib(args); } // -> autibsp_hi_hints
    if(crm==4 && ((op2&1) == 0) && hasbti()) { return trans.bti(args); } // -> bti_hb_hints
    if(1) { return trans.hint(args); } // -> hint_hm_hints

     */
    return false;
}
pub fn decode_pstate<T: Arm64DecodeTrait>(trans: &mut T, insn: u32) -> bool {
    let args: ArmInstr = ArmInstr {
        insn
    };
    let rt = insn & 0x1f;
    let op1 = ((insn >> 16) & 7);
    let op2 = ((insn >> 5) & 7);
    if((op1 == 0) && (op2 == 0) && rt==0x1f && trans.has_flagm()) { return trans.cfinv(args); } // -> cfinv_m_pstate
    if((op1 == 0) && op2==1 && rt==0x1f && trans.has_flagm2()) { return trans.xaflag(args); } // -> xaflag_m_pstate
    if((op1 == 0) && op2==2 && rt==0x1f && trans.has_flagm2()) { return trans.axflag(args); } // -> axflag_m_pstate
    if(rt==0x1f) { return trans.msr_imm(args); } // -> msr_si_pstate
    return false;
}
pub fn decode_systeminstrs<T: Arm64DecodeTrait>(trans: &mut T, insn: u32) -> bool {
    return false;
}
pub fn decode_systeminstrswithreg<T: Arm64DecodeTrait>(trans: &mut T, insn: u32) -> bool {
    return false;
}
pub fn decode_systemmove<T: Arm64DecodeTrait>(trans: &mut T, insn: u32) -> bool {
    let l = ((insn >> 21) & 1) != 0;
    let args: ArmInstr = ArmInstr {
        insn
    };
    if (!l) { return trans.msr_reg(args); }
    if (l) { return trans.mrs(args); }

    return false;
}
pub fn decode_systemresult<T: Arm64DecodeTrait>(trans: &mut T, insn: u32) -> bool {
    return false;
}
pub fn decode_testbranch<T: Arm64DecodeTrait>(trans: &mut T, insn: u32) -> bool {
    let op = ((insn >> 24) & 1) != 0;
    let args: ArmInstr = ArmInstr {
        insn
    };
    if !op {
        return trans.tbz(args);
    }
    if op {
        return trans.tbnz(args);
    }
    return false;
    return false;
}
pub fn decode_branch_imm<T: Arm64DecodeTrait>(trans: &mut T, insn: u32) -> bool {
    let op = ((insn >> 31) & 1) != 0;
    let args: ArmInstr = ArmInstr {
        insn
     };
    if !op {
        return trans.b_uncond(args);
    }
    if op {
        return trans.bl(args);
    }
    return false;
}
pub fn decode_branch_reg<T: Arm64DecodeTrait>(trans: &mut T, insn: u32) -> bool {
    let args: ArmInstr = ArmInstr {
        insn
    };
    let opc = ((insn >> 21) & 15);
    let op2 = ((insn >> 16) & 0x1f);
    let op3 = ((insn >> 10) & 0x3f);
    let rn = ((insn >> 5) & 0x1f);
    let op4 = (insn & 0x1f);
    if(opc==2 && op2==0x1f && op3==2 && rn==0x1f && op4==0x1f && trans.has_pauth_feat()) { return trans.reta(args); } // -> retaa_64e_branch_reg
    if(opc==2 && op2==0x1f && op3==3 && rn==0x1f && op4==0x1f && trans.has_pauth_feat()) { return trans.reta(args); } // -> retab_64e_branch_reg
    if(opc==4 && op2==0x1f && (op3 == 0) && rn==0x1f && (op4 == 0)) { return trans.eret(args); } // -> eret_64e_branch_reg
    if(opc==4 && op2==0x1f && op3==2 && rn==0x1f && op4==0x1f && trans.has_pauth_feat()) { return trans.ereta(args); } // -> eretaa_64e_branch_reg
    if(opc==4 && op2==0x1f && op3==3 && rn==0x1f && op4==0x1f && trans.has_pauth_feat()) { return trans.ereta(args); } // -> eretab_64e_branch_reg
    if(opc==5 && op2==0x1f && (op3 == 0) && rn==0x1f && (op4 == 0)) { return trans.drps(args); } // -> drps_64e_branch_reg
    if((opc == 0) && op2==0x1f && (op3 == 0) && (op4 == 0)) { return trans.br(args); } // -> br_64_branch_reg
    if((opc == 0) && op2==0x1f && op3==2 && op4==0x1f && trans.has_pauth_feat()) { return trans.bra(args); } // -> braaz_64_branch_reg
    if((opc == 0) && op2==0x1f && op3==3 && op4==0x1f && trans.has_pauth_feat()) { return trans.bra(args); } // -> brabz_64_branch_reg
    if(opc==1 && op2==0x1f && (op3 == 0) && (op4 == 0)) { return trans.blr(args); } // -> blr_64_branch_reg
    if(opc==1 && op2==0x1f && op3==2 && op4==0x1f && trans.has_pauth_feat()) { return trans.blra(args); } // -> blraaz_64_branch_reg
    if(opc==1 && op2==0x1f && op3==3 && op4==0x1f && trans.has_pauth_feat()) { return trans.blra(args); } // -> blrabz_64_branch_reg
    if(opc==2 && op2==0x1f && (op3 == 0) && (op4 == 0)) { return trans.ret(args); } // -> ret_64r_branch_reg
    if(opc==8 && op2==0x1f && op3==2 && trans.has_pauth_feat()) { return trans.bra(args); } // -> braa_64p_branch_reg
    if(opc==8 && op2==0x1f && op3==3 && trans.has_pauth_feat()) { return trans.bra(args); } // -> brab_64p_branch_reg
    if(opc==9 && op2==0x1f && op3==2 && trans.has_pauth_feat()) { return trans.blra(args); } // -> blraa_64p_branch_reg
    if(opc==9 && op2==0x1f && op3==3 && trans.has_pauth_feat()) { return trans.blra(args); } // -> blrab_64p_branch_reg

    return false;
}
pub fn decode_asisdlse<T: Arm64DecodeTrait>(trans: &mut T, insn: u32) -> bool {
    let args: ArmInstr = ArmInstr {
        insn
    };
    let l = ((insn >> 22) & 1) != 0;
    let opcode = ((insn >> 12) & 15);
    if(!l && (opcode == 0)) { return trans.st4_advsimd_mult(args); } // -> st4_asisdlse_r4
    if(!l && opcode==2) { return trans.st1_advsimd_mult(args); } // -> st1_asisdlse_r4_4v
    if(!l && opcode==4) { return trans.st3_advsimd_mult(args); } // -> st3_asisdlse_r3
    if(!l && opcode==6) { return trans.st1_advsimd_mult(args); } // -> st1_asisdlse_r3_3v
    if(!l && opcode==7) { return trans.st1_advsimd_mult(args); } // -> st1_asisdlse_r1_1v
    if(!l && opcode==8) { return trans.st2_advsimd_mult(args); } // -> st2_asisdlse_r2
    if(!l && opcode==10) { return trans.st1_advsimd_mult(args); } // -> st1_asisdlse_r2_2v
    if(l && (opcode == 0)) { return trans.ld4_advsimd_mult(args); } // -> ld4_asisdlse_r4
    if(l && opcode==2) { return trans.ld1_advsimd_mult(args); } // -> ld1_asisdlse_r4_4v
    if(l && opcode==4) { return trans.ld3_advsimd_mult(args); } // -> ld3_asisdlse_r3
    if(l && opcode==6) { return trans.ld1_advsimd_mult(args); } // -> ld1_asisdlse_r3_3v
    if(l && opcode==7) { return trans.ld1_advsimd_mult(args); } // -> ld1_asisdlse_r1_1v
    if(l && opcode==8) { return trans.ld2_advsimd_mult(args); } // -> ld2_asisdlse_r2
    if(l && opcode==10) { return trans.ld1_advsimd_mult(args); } // -> ld1_asisdlse_r2_2v

    return false;
}
pub fn decode_asisdlsep<T: Arm64DecodeTrait>(trans: &mut T, insn: u32) -> bool {
    let args: ArmInstr = ArmInstr {
        insn
    };
    let l = ((insn >> 22) & 1) != 0;
    let opcode = ((insn >> 12) & 15);
    let rm = ((insn >> 16) & 0x1f);
    if(!l && rm!=0x1f && (opcode == 0)) { return trans.st4_advsimd_mult(args); } // -> st4_asisdlsep_r4_r
    if(!l && rm!=0x1f && opcode==2) { return trans.st1_advsimd_mult(args); } // -> st1_asisdlsep_r4_r4
    if(!l && rm!=0x1f && opcode==4) { return trans.st3_advsimd_mult(args); } // -> st3_asisdlsep_r3_r
    if(!l && rm!=0x1f && opcode==6) { return trans.st1_advsimd_mult(args); } // -> st1_asisdlsep_r3_r3
    if(!l && rm!=0x1f && opcode==7) { return trans.st1_advsimd_mult(args); } // -> st1_asisdlsep_r1_r1
    if(!l && rm!=0x1f && opcode==8) { return trans.st2_advsimd_mult(args); } // -> st2_asisdlsep_r2_r
    if(!l && rm!=0x1f && opcode==10) { return trans.st1_advsimd_mult(args); } // -> st1_asisdlsep_r2_r2
    if(!l && rm==0x1f && (opcode == 0)) { return trans.st4_advsimd_mult(args); } // -> st4_asisdlsep_i4_i
    if(!l && rm==0x1f && opcode==2) { return trans.st1_advsimd_mult(args); } // -> st1_asisdlsep_i4_i4
    if(!l && rm==0x1f && opcode==4) { return trans.st3_advsimd_mult(args); } // -> st3_asisdlsep_i3_i
    if(!l && rm==0x1f && opcode==6) { return trans.st1_advsimd_mult(args); } // -> st1_asisdlsep_i3_i3
    if(!l && rm==0x1f && opcode==7) { return trans.st1_advsimd_mult(args); } // -> st1_asisdlsep_i1_i1
    if(!l && rm==0x1f && opcode==8) { return trans.st2_advsimd_mult(args); } // -> st2_asisdlsep_i2_i
    if(!l && rm==0x1f && opcode==10) { return trans.st1_advsimd_mult(args); } // -> st1_asisdlsep_i2_i2
    if(l && rm!=0x1f && (opcode == 0)) { return trans.ld4_advsimd_mult(args); } // -> ld4_asisdlsep_r4_r
    if(l && rm!=0x1f && opcode==2) { return trans.ld1_advsimd_mult(args); } // -> ld1_asisdlsep_r4_r4
    if(l && rm!=0x1f && opcode==4) { return trans.ld3_advsimd_mult(args); } // -> ld3_asisdlsep_r3_r
    if(l && rm!=0x1f && opcode==6) { return trans.ld1_advsimd_mult(args); } // -> ld1_asisdlsep_r3_r3
    if(l && rm!=0x1f && opcode==7) { return trans.ld1_advsimd_mult(args); } // -> ld1_asisdlsep_r1_r1
    if(l && rm!=0x1f && opcode==8) { return trans.ld2_advsimd_mult(args); } // -> ld2_asisdlsep_r2_r
    if(l && rm!=0x1f && opcode==10) { return trans.ld1_advsimd_mult(args); } // -> ld1_asisdlsep_r2_r2
    if(l && rm==0x1f && (opcode == 0)) { return trans.ld4_advsimd_mult(args); } // -> ld4_asisdlsep_i4_i
    if(l && rm==0x1f && opcode==2) { return trans.ld1_advsimd_mult(args); } // -> ld1_asisdlsep_i4_i4
    if(l && rm==0x1f && opcode==4) { return trans.ld3_advsimd_mult(args); } // -> ld3_asisdlsep_i3_i
    if(l && rm==0x1f && opcode==6) { return trans.ld1_advsimd_mult(args); } // -> ld1_asisdlsep_i3_i3
    if(l && rm==0x1f && opcode==7) { return trans.ld1_advsimd_mult(args); } // -> ld1_asisdlsep_i1_i1
    if(l && rm==0x1f && opcode==8) { return trans.ld2_advsimd_mult(args); } // -> ld2_asisdlsep_i2_i
    if(l && rm==0x1f && opcode==10) { return trans.ld1_advsimd_mult(args); } // -> ld1_asisdlsep_i2_i2
    return false;
}
pub fn decode_asisdlso<T: Arm64DecodeTrait>(trans: &mut T, insn: u32) -> bool {
    let args: ArmInstr = ArmInstr {
        insn
    };
    let l = ((insn >> 22) & 1) != 0;
    let r = ((insn >> 21) & 1) != 0;
    let s = ((insn >> 12) & 1) != 0;
    let size = ((insn >> 10) & 3);
    let opcode = ((insn >> 13) & 7);
    if(!l && !r && opcode==4 && !s && size==1) { return trans.st1_advsimd_sngl(args); } // -> st1_asisdlso_d1_1d
    if(!l && !r && opcode==5 && !s && size==1) { return trans.st3_advsimd_sngl(args); } // -> st3_asisdlso_d3_3d
    if(!l && r && opcode==4 && !s && size==1) { return trans.st2_advsimd_sngl(args); } // -> st2_asisdlso_d2_2d
    if(!l && r && opcode==5 && !s && size==1) { return trans.st4_advsimd_sngl(args); } // -> st4_asisdlso_d4_4d
    if(l && !r && opcode==4 && !s && size==1) { return trans.ld1_advsimd_sngl(args); } // -> ld1_asisdlso_d1_1d
    if(l && !r && opcode==5 && !s && size==1) { return trans.ld3_advsimd_sngl(args); } // -> ld3_asisdlso_d3_3d
    if(l && r && opcode==4 && !s && size==1) { return trans.ld2_advsimd_sngl(args); } // -> ld2_asisdlso_d2_2d
    if(l && r && opcode==5 && !s && size==1) { return trans.ld4_advsimd_sngl(args); } // -> ld4_asisdlso_d4_4d
    if(!l && !r && opcode==4 && (size == 0)) { return trans.st1_advsimd_sngl(args); } // -> st1_asisdlso_s1_1s
    if(!l && !r && opcode==5 && (size == 0)) { return trans.st3_advsimd_sngl(args); } // -> st3_asisdlso_s3_3s
    if(!l && r && opcode==4 && (size == 0)) { return trans.st2_advsimd_sngl(args); } // -> st2_asisdlso_s2_2s
    if(!l && r && opcode==5 && (size == 0)) { return trans.st4_advsimd_sngl(args); } // -> st4_asisdlso_s4_4s
    if(l && !r && opcode==4 && (size == 0)) { return trans.ld1_advsimd_sngl(args); } // -> ld1_asisdlso_s1_1s
    if(l && !r && opcode==5 && (size == 0)) { return trans.ld3_advsimd_sngl(args); } // -> ld3_asisdlso_s3_3s
    if(l && r && opcode==4 && (size == 0)) { return trans.ld2_advsimd_sngl(args); } // -> ld2_asisdlso_s2_2s
    if(l && r && opcode==5 && (size == 0)) { return trans.ld4_advsimd_sngl(args); } // -> ld4_asisdlso_s4_4s
    if(!l && !r && opcode==2 && ((size&1) == 0)) { return trans.st1_advsimd_sngl(args); } // -> st1_asisdlso_h1_1h
    if(!l && !r && opcode==3 && ((size&1) == 0)) { return trans.st3_advsimd_sngl(args); } // -> st3_asisdlso_h3_3h
    if(!l && r && opcode==2 && ((size&1) == 0)) { return trans.st2_advsimd_sngl(args); } // -> st2_asisdlso_h2_2h
    if(!l && r && opcode==3 && ((size&1) == 0)) { return trans.st4_advsimd_sngl(args); } // -> st4_asisdlso_h4_4h
    if(l && !r && opcode==2 && ((size&1) == 0)) { return trans.ld1_advsimd_sngl(args); } // -> ld1_asisdlso_h1_1h
    if(l && !r && opcode==3 && ((size&1) == 0)) { return trans.ld3_advsimd_sngl(args); } // -> ld3_asisdlso_h3_3h
    if(l && !r && opcode==6 && !s) { return trans.ld1r_advsimd(args); } // -> ld1r_asisdlso_r1
    if(l && !r && opcode==7 && !s) { return trans.ld3r_advsimd(args); } // -> ld3r_asisdlso_r3
    if(l && r && opcode==2 && ((size&1) == 0)) { return trans.ld2_advsimd_sngl(args); } // -> ld2_asisdlso_h2_2h
    if(l && r && opcode==3 && ((size&1) == 0)) { return trans.ld4_advsimd_sngl(args); } // -> ld4_asisdlso_h4_4h
    if(l && r && opcode==6 && !s) { return trans.ld2r_advsimd(args); } // -> ld2r_asisdlso_r2
    if(l && r && opcode==7 && !s) { return trans.ld4r_advsimd(args); } // -> ld4r_asisdlso_r4
    if(!l && !r && (opcode == 0)) { return trans.st1_advsimd_sngl(args); } // -> st1_asisdlso_b1_1b
    if(!l && !r && opcode==1) { return trans.st3_advsimd_sngl(args); } // -> st3_asisdlso_b3_3b
    if(!l && r && (opcode == 0)) { return trans.st2_advsimd_sngl(args); } // -> st2_asisdlso_b2_2b
    if(!l && r && opcode==1) { return trans.st4_advsimd_sngl(args); } // -> st4_asisdlso_b4_4b
    if(l && !r && (opcode == 0)) { return trans.ld1_advsimd_sngl(args); } // -> ld1_asisdlso_b1_1b
    if(l && !r && opcode==1) { return trans.ld3_advsimd_sngl(args); } // -> ld3_asisdlso_b3_3b
    if(l && r && (opcode == 0)) { return trans.ld2_advsimd_sngl(args); } // -> ld2_asisdlso_b2_2b
    if(l && r && opcode==1) { return trans.ld4_advsimd_sngl(args); } // -> ld4_asisdlso_b4_4b
    return false;
}
pub fn decode_asisdlsop<T: Arm64DecodeTrait>(trans: &mut T, insn: u32) -> bool {
    let args: ArmInstr = ArmInstr {
        insn
    };
    let l = ((insn >> 22) & 1) != 0;
    let r = ((insn >> 21) & 1) != 0;
    let s = ((insn >> 12) & 1) != 0;
    let size = ((insn >> 10) & 3);
    let opcode = ((insn >> 13) & 7);
    let rm = ((insn >> 16) & 0x1f);
    if(!l && !r && rm!=0x1f && opcode==4 && !s && size==1) { return trans.st1_advsimd_sngl(args); } // -> st1_asisdlsop_dx1_r1d
    if(!l && !r && rm!=0x1f && opcode==5 && !s && size==1) { return trans.st3_advsimd_sngl(args); } // -> st3_asisdlsop_dx3_r3d
    if(!l && !r && rm==0x1f && opcode==4 && !s && size==1) { return trans.st1_advsimd_sngl(args); } // -> st1_asisdlsop_d1_i1d
    if(!l && !r && rm==0x1f && opcode==5 && !s && size==1) { return trans.st3_advsimd_sngl(args); } // -> st3_asisdlsop_d3_i3d
    if(!l && r && rm!=0x1f && opcode==4 && !s && size==1) { return trans.st2_advsimd_sngl(args); } // -> st2_asisdlsop_dx2_r2d
    if(!l && r && rm!=0x1f && opcode==5 && !s && size==1) { return trans.st4_advsimd_sngl(args); } // -> st4_asisdlsop_dx4_r4d
    if(!l && r && rm==0x1f && opcode==4 && !s && size==1) { return trans.st2_advsimd_sngl(args); } // -> st2_asisdlsop_d2_i2d
    if(!l && r && rm==0x1f && opcode==5 && !s && size==1) { return trans.st4_advsimd_sngl(args); } // -> st4_asisdlsop_d4_i4d
    if(l && !r && rm!=0x1f && opcode==4 && !s && size==1) { return trans.ld1_advsimd_sngl(args); } // -> ld1_asisdlsop_dx1_r1d
    if(l && !r && rm!=0x1f && opcode==5 && !s && size==1) { return trans.ld3_advsimd_sngl(args); } // -> ld3_asisdlsop_dx3_r3d
    if(l && !r && rm==0x1f && opcode==4 && !s && size==1) { return trans.ld1_advsimd_sngl(args); } // -> ld1_asisdlsop_d1_i1d
    if(l && !r && rm==0x1f && opcode==5 && !s && size==1) { return trans.ld3_advsimd_sngl(args); } // -> ld3_asisdlsop_d3_i3d
    if(l && r && rm!=0x1f && opcode==4 && !s && size==1) { return trans.ld2_advsimd_sngl(args); } // -> ld2_asisdlsop_dx2_r2d
    if(l && r && rm!=0x1f && opcode==5 && !s && size==1) { return trans.ld4_advsimd_sngl(args); } // -> ld4_asisdlsop_dx4_r4d
    if(l && r && rm==0x1f && opcode==4 && !s && size==1) { return trans.ld2_advsimd_sngl(args); } // -> ld2_asisdlsop_d2_i2d
    if(l && r && rm==0x1f && opcode==5 && !s && size==1) { return trans.ld4_advsimd_sngl(args); } // -> ld4_asisdlsop_d4_i4d
    if(!l && !r && rm!=0x1f && opcode==4 && (size == 0)) { return trans.st1_advsimd_sngl(args); } // -> st1_asisdlsop_sx1_r1s
    if(!l && !r && rm!=0x1f && opcode==5 && (size == 0)) { return trans.st3_advsimd_sngl(args); } // -> st3_asisdlsop_sx3_r3s
    if(!l && !r && rm==0x1f && opcode==4 && (size == 0)) { return trans.st1_advsimd_sngl(args); } // -> st1_asisdlsop_s1_i1s
    if(!l && !r && rm==0x1f && opcode==5 && (size == 0)) { return trans.st3_advsimd_sngl(args); } // -> st3_asisdlsop_s3_i3s
    if(!l && r && rm!=0x1f && opcode==4 && (size == 0)) { return trans.st2_advsimd_sngl(args); } // -> st2_asisdlsop_sx2_r2s
    if(!l && r && rm!=0x1f && opcode==5 && (size == 0)) { return trans.st4_advsimd_sngl(args); } // -> st4_asisdlsop_sx4_r4s
    if(!l && r && rm==0x1f && opcode==4 && (size == 0)) { return trans.st2_advsimd_sngl(args); } // -> st2_asisdlsop_s2_i2s
    if(!l && r && rm==0x1f && opcode==5 && (size == 0)) { return trans.st4_advsimd_sngl(args); } // -> st4_asisdlsop_s4_i4s
    if(l && !r && rm!=0x1f && opcode==4 && (size == 0)) { return trans.ld1_advsimd_sngl(args); } // -> ld1_asisdlsop_sx1_r1s
    if(l && !r && rm!=0x1f && opcode==5 && (size == 0)) { return trans.ld3_advsimd_sngl(args); } // -> ld3_asisdlsop_sx3_r3s
    if(l && !r && rm==0x1f && opcode==4 && (size == 0)) { return trans.ld1_advsimd_sngl(args); } // -> ld1_asisdlsop_s1_i1s
    if(l && !r && rm==0x1f && opcode==5 && (size == 0)) { return trans.ld3_advsimd_sngl(args); } // -> ld3_asisdlsop_s3_i3s
    if(l && r && rm!=0x1f && opcode==4 && (size == 0)) { return trans.ld2_advsimd_sngl(args); } // -> ld2_asisdlsop_sx2_r2s
    if(l && r && rm!=0x1f && opcode==5 && (size == 0)) { return trans.ld4_advsimd_sngl(args); } // -> ld4_asisdlsop_sx4_r4s
    if(l && r && rm==0x1f && opcode==4 && (size == 0)) { return trans.ld2_advsimd_sngl(args); } // -> ld2_asisdlsop_s2_i2s
    if(l && r && rm==0x1f && opcode==5 && (size == 0)) { return trans.ld4_advsimd_sngl(args); } // -> ld4_asisdlsop_s4_i4s
    if(!l && !r && rm!=0x1f && opcode==2 && ((size&1) == 0)) { return trans.st1_advsimd_sngl(args); } // -> st1_asisdlsop_hx1_r1h
    if(!l && !r && rm!=0x1f && opcode==3 && ((size&1) == 0)) { return trans.st3_advsimd_sngl(args); } // -> st3_asisdlsop_hx3_r3h
    if(!l && !r && rm==0x1f && opcode==2 && ((size&1) == 0)) { return trans.st1_advsimd_sngl(args); } // -> st1_asisdlsop_h1_i1h
    if(!l && !r && rm==0x1f && opcode==3 && ((size&1) == 0)) { return trans.st3_advsimd_sngl(args); } // -> st3_asisdlsop_h3_i3h
    if(!l && r && rm!=0x1f && opcode==2 && ((size&1) == 0)) { return trans.st2_advsimd_sngl(args); } // -> st2_asisdlsop_hx2_r2h
    if(!l && r && rm!=0x1f && opcode==3 && ((size&1) == 0)) { return trans.st4_advsimd_sngl(args); } // -> st4_asisdlsop_hx4_r4h
    if(!l && r && rm==0x1f && opcode==2 && ((size&1) == 0)) { return trans.st2_advsimd_sngl(args); } // -> st2_asisdlsop_h2_i2h
    if(!l && r && rm==0x1f && opcode==3 && ((size&1) == 0)) { return trans.st4_advsimd_sngl(args); } // -> st4_asisdlsop_h4_i4h
    if(l && !r && rm!=0x1f && opcode==2 && ((size&1) == 0)) { return trans.ld1_advsimd_sngl(args); } // -> ld1_asisdlsop_hx1_r1h
    if(l && !r && rm!=0x1f && opcode==3 && ((size&1) == 0)) { return trans.ld3_advsimd_sngl(args); } // -> ld3_asisdlsop_hx3_r3h
    if(l && !r && rm!=0x1f && opcode==6 && !s) { return trans.ld1r_advsimd(args); } // -> ld1r_asisdlsop_rx1_r
    if(l && !r && rm!=0x1f && opcode==7 && !s) { return trans.ld3r_advsimd(args); } // -> ld3r_asisdlsop_rx3_r
    if(l && !r && rm==0x1f && opcode==2 && ((size&1) == 0)) { return trans.ld1_advsimd_sngl(args); } // -> ld1_asisdlsop_h1_i1h
    if(l && !r && rm==0x1f && opcode==3 && ((size&1) == 0)) { return trans.ld3_advsimd_sngl(args); } // -> ld3_asisdlsop_h3_i3h
    if(l && !r && rm==0x1f && opcode==6 && !s) { return trans.ld1r_advsimd(args); } // -> ld1r_asisdlsop_r1_i
    if(l && !r && rm==0x1f && opcode==7 && !s) { return trans.ld3r_advsimd(args); } // -> ld3r_asisdlsop_r3_i
    if(l && r && rm!=0x1f && opcode==2 && ((size&1) == 0)) { return trans.ld2_advsimd_sngl(args); } // -> ld2_asisdlsop_hx2_r2h
    if(l && r && rm!=0x1f && opcode==3 && ((size&1) == 0)) { return trans.ld4_advsimd_sngl(args); } // -> ld4_asisdlsop_hx4_r4h
    if(l && r && rm!=0x1f && opcode==6 && !s) { return trans.ld2r_advsimd(args); } // -> ld2r_asisdlsop_rx2_r
    if(l && r && rm!=0x1f && opcode==7 && !s) { return trans.ld4r_advsimd(args); } // -> ld4r_asisdlsop_rx4_r
    if(l && r && rm==0x1f && opcode==2 && ((size&1) == 0)) { return trans.ld2_advsimd_sngl(args); } // -> ld2_asisdlsop_h2_i2h
    if(l && r && rm==0x1f && opcode==3 && ((size&1) == 0)) { return trans.ld4_advsimd_sngl(args); } // -> ld4_asisdlsop_h4_i4h
    if(l && r && rm==0x1f && opcode==6 && !s) { return trans.ld2r_advsimd(args); } // -> ld2r_asisdlsop_r2_i
    if(l && r && rm==0x1f && opcode==7 && !s) { return trans.ld4r_advsimd(args); } // -> ld4r_asisdlsop_r4_i
    if(!l && !r && rm!=0x1f && (opcode == 0)) { return trans.st1_advsimd_sngl(args); } // -> st1_asisdlsop_bx1_r1b
    if(!l && !r && rm!=0x1f && opcode==1) { return trans.st3_advsimd_sngl(args); } // -> st3_asisdlsop_bx3_r3b
    if(!l && !r && rm==0x1f && (opcode == 0)) { return trans.st1_advsimd_sngl(args); } // -> st1_asisdlsop_b1_i1b
    if(!l && !r && rm==0x1f && opcode==1) { return trans.st3_advsimd_sngl(args); } // -> st3_asisdlsop_b3_i3b
    if(!l && r && rm!=0x1f && (opcode == 0)) { return trans.st2_advsimd_sngl(args); } // -> st2_asisdlsop_bx2_r2b
    if(!l && r && rm!=0x1f && opcode==1) { return trans.st4_advsimd_sngl(args); } // -> st4_asisdlsop_bx4_r4b
    if(!l && r && rm==0x1f && (opcode == 0)) { return trans.st2_advsimd_sngl(args); } // -> st2_asisdlsop_b2_i2b
    if(!l && r && rm==0x1f && opcode==1) { return trans.st4_advsimd_sngl(args); } // -> st4_asisdlsop_b4_i4b
    if(l && !r && rm!=0x1f && (opcode == 0)) { return trans.ld1_advsimd_sngl(args); } // -> ld1_asisdlsop_bx1_r1b
    if(l && !r && rm!=0x1f && opcode==1) { return trans.ld3_advsimd_sngl(args); } // -> ld3_asisdlsop_bx3_r3b
    if(l && !r && rm==0x1f && (opcode == 0)) { return trans.ld1_advsimd_sngl(args); } // -> ld1_asisdlsop_b1_i1b
    if(l && !r && rm==0x1f && opcode==1) { return trans.ld3_advsimd_sngl(args); } // -> ld3_asisdlsop_b3_i3b
    if(l && r && rm!=0x1f && (opcode == 0)) { return trans.ld2_advsimd_sngl(args); } // -> ld2_asisdlsop_bx2_r2b
    if(l && r && rm!=0x1f && opcode==1) { return trans.ld4_advsimd_sngl(args); } // -> ld4_asisdlsop_bx4_r4b
    if(l && r && rm==0x1f && (opcode == 0)) { return trans.ld2_advsimd_sngl(args); } // -> ld2_asisdlsop_b2_i2b
    if(l && r && rm==0x1f && opcode==1) { return trans.ld4_advsimd_sngl(args); } // -> ld4_asisdlsop_b4_i4b
    return false;
}
pub fn decode_memop<T: Arm64DecodeTrait>(trans: &mut T, insn: u32) -> bool {
    let args: ArmInstr = ArmInstr {
        insn
    };
    let v = ((insn >> 26) & 1) != 0;
    let a = ((insn >> 23) & 1) != 0;
    let r = ((insn >> 22) & 1) != 0;
    let rs = ((insn >> 16) & 0x1f);
    let o3 = ((insn >> 15) & 1) != 0;
    let opc = ((insn >> 12) & 7);
    /*
    if(size==3 && !v && !a && !r && rs==0x1f && o3 && opc==1 && trans.has_ls64()) { return trans.st64b(args); } // -> st64b_64l_memop
    if(size==3 && !v && !a && !r && rs==0x1f && o3 && opc==5 && trans.has_ls64()) { return trans.ld64b(args); } // -> ld64b_64l_memop
    if((size == 0) && !v && !a && !r && !o3 && (opc == 0) && trans.has_lse()) { return trans.ldaddb(args); } // -> ldaddb_32_memop
    if((size == 0) && !v && !a && !r && !o3 && opc==1 && trans.has_lse()) { return trans.ldclrb(args); } // -> ldclrb_32_memop
    if((size == 0) && !v && !a && !r && !o3 && opc==2 && trans.has_lse()) { return trans.ldeorb(args); } // -> ldeorb_32_memop
    if((size == 0) && !v && !a && !r && !o3 && opc==3 && trans.has_lse()) { return trans.ldsetb(args); } // -> ldsetb_32_memop
    if((size == 0) && !v && !a && !r && !o3 && opc==4 && trans.has_lse()) { return trans.ldsmaxb(args); } // -> ldsmaxb_32_memop
    if((size == 0) && !v && !a && !r && !o3 && opc==5 && trans.has_lse()) { return trans.ldsminb(args); } // -> ldsminb_32_memop
    if((size == 0) && !v && !a && !r && !o3 && opc==6 && trans.has_lse()) { return trans.ldumaxb(args); } // -> ldumaxb_32_memop
    if((size == 0) && !v && !a && !r && !o3 && opc==7 && trans.has_lse()) { return trans.lduminb(args); } // -> lduminb_32_memop
    if((size == 0) && !v && !a && !r && o3 && (opc == 0) && trans.has_lse()) { return trans.swpb(args); } // -> swpb_32_memop
    if((size == 0) && !v && !a && r && !o3 && (opc == 0) && trans.has_lse()) { return trans.ldaddb(args); } // -> ldaddlb_32_memop
    if((size == 0) && !v && !a && r && !o3 && opc==1 && trans.has_lse()) { return trans.ldclrb(args); } // -> ldclrlb_32_memop
    if((size == 0) && !v && !a && r && !o3 && opc==2 && trans.has_lse()) { return trans.ldeorb(args); } // -> ldeorlb_32_memop
    if((size == 0) && !v && !a && r && !o3 && opc==3 && trans.has_lse()) { return trans.ldsetb(args); } // -> ldsetlb_32_memop
    if((size == 0) && !v && !a && r && !o3 && opc==4 && trans.has_lse()) { return trans.ldsmaxb(args); } // -> ldsmaxlb_32_memop
    if((size == 0) && !v && !a && r && !o3 && opc==5 && trans.has_lse()) { return trans.ldsminb(args); } // -> ldsminlb_32_memop
    if((size == 0) && !v && !a && r && !o3 && opc==6 && trans.has_lse()) { return trans.ldumaxb(args); } // -> ldumaxlb_32_memop
    if((size == 0) && !v && !a && r && !o3 && opc==7 && trans.has_lse()) { return trans.lduminb(args); } // -> lduminlb_32_memop
    if((size == 0) && !v && !a && r && o3 && (opc == 0) && trans.has_lse()) { return trans.swpb(args); } // -> swplb_32_memop
    if((size == 0) && !v && a && !r && !o3 && (opc == 0) && trans.has_lse()) { return trans.ldaddb(args); } // -> ldaddab_32_memop
    if((size == 0) && !v && a && !r && !o3 && opc==1 && trans.has_lse()) { return trans.ldclrb(args); } // -> ldclrab_32_memop
    if((size == 0) && !v && a && !r && !o3 && opc==2 && trans.has_lse()) { return trans.ldeorb(args); } // -> ldeorab_32_memop
    if((size == 0) && !v && a && !r && !o3 && opc==3 && trans.has_lse()) { return trans.ldsetb(args); } // -> ldsetab_32_memop
    if((size == 0) && !v && a && !r && !o3 && opc==4 && trans.has_lse()) { return trans.ldsmaxb(args); } // -> ldsmaxab_32_memop
    if((size == 0) && !v && a && !r && !o3 && opc==5 && trans.has_lse()) { return trans.ldsminb(args); } // -> ldsminab_32_memop
    if((size == 0) && !v && a && !r && !o3 && opc==6 && trans.has_lse()) { return trans.ldumaxb(args); } // -> ldumaxab_32_memop
    if((size == 0) && !v && a && !r && !o3 && opc==7 && trans.has_lse()) { return trans.lduminb(args); } // -> lduminab_32_memop
    if((size == 0) && !v && a && !r && o3 && (opc == 0) && trans.has_lse()) { return trans.swpb(args); } // -> swpab_32_memop
    if((size == 0) && !v && a && !r && o3 && opc==4 && haslrcpc()) { return trans.ldaprb(args); } // -> ldaprb_32l_memop
    if((size == 0) && !v && a && r && !o3 && (opc == 0) && trans.has_lse()) { return trans.ldaddb(args); } // -> ldaddalb_32_memop
    if((size == 0) && !v && a && r && !o3 && opc==1 && trans.has_lse()) { return trans.ldclrb(args); } // -> ldclralb_32_memop
    if((size == 0) && !v && a && r && !o3 && opc==2 && trans.has_lse()) { return trans.ldeorb(args); } // -> ldeoralb_32_memop
    if((size == 0) && !v && a && r && !o3 && opc==3 && trans.has_lse()) { return trans.ldsetb(args); } // -> ldsetalb_32_memop
    if((size == 0) && !v && a && r && !o3 && opc==4 && trans.has_lse()) { return trans.ldsmaxb(args); } // -> ldsmaxalb_32_memop
    if((size == 0) && !v && a && r && !o3 && opc==5 && trans.has_lse()) { return trans.ldsminb(args); } // -> ldsminalb_32_memop
    if((size == 0) && !v && a && r && !o3 && opc==6 && trans.has_lse()) { return trans.ldumaxb(args); } // -> ldumaxalb_32_memop
    if((size == 0) && !v && a && r && !o3 && opc==7 && trans.has_lse()) { return trans.lduminb(args); } // -> lduminalb_32_memop
    if((size == 0) && !v && a && r && o3 && (opc == 0) && trans.has_lse()) { return trans.swpb(args); } // -> swpalb_32_memop
    if(size==1 && !v && !a && !r && !o3 && (opc == 0) && trans.has_lse()) { return trans.ldaddh(args); } // -> ldaddh_32_memop
    if(size==1 && !v && !a && !r && !o3 && opc==1 && trans.has_lse()) { return trans.ldclrh(args); } // -> ldclrh_32_memop
    if(size==1 && !v && !a && !r && !o3 && opc==2 && trans.has_lse()) { return trans.ldeorh(args); } // -> ldeorh_32_memop
    if(size==1 && !v && !a && !r && !o3 && opc==3 && trans.has_lse()) { return trans.ldseth(args); } // -> ldseth_32_memop
    if(size==1 && !v && !a && !r && !o3 && opc==4 && trans.has_lse()) { return trans.ldsmaxh(args); } // -> ldsmaxh_32_memop
    if(size==1 && !v && !a && !r && !o3 && opc==5 && trans.has_lse()) { return trans.ldsminh(args); } // -> ldsminh_32_memop
    if(size==1 && !v && !a && !r && !o3 && opc==6 && trans.has_lse()) { return trans.ldumaxh(args); } // -> ldumaxh_32_memop
    if(size==1 && !v && !a && !r && !o3 && opc==7 && trans.has_lse()) { return trans.lduminh(args); } // -> lduminh_32_memop
    if(size==1 && !v && !a && !r && o3 && (opc == 0) && trans.has_lse()) { return trans.swph(args); } // -> swph_32_memop
    if(size==1 && !v && !a && r && !o3 && (opc == 0) && trans.has_lse()) { return trans.ldaddh(args); } // -> ldaddlh_32_memop
    if(size==1 && !v && !a && r && !o3 && opc==1 && trans.has_lse()) { return trans.ldclrh(args); } // -> ldclrlh_32_memop
    if(size==1 && !v && !a && r && !o3 && opc==2 && trans.has_lse()) { return trans.ldeorh(args); } // -> ldeorlh_32_memop
    if(size==1 && !v && !a && r && !o3 && opc==3 && trans.has_lse()) { return trans.ldseth(args); } // -> ldsetlh_32_memop
    if(size==1 && !v && !a && r && !o3 && opc==4 && trans.has_lse()) { return trans.ldsmaxh(args); } // -> ldsmaxlh_32_memop
    if(size==1 && !v && !a && r && !o3 && opc==5 && trans.has_lse()) { return trans.ldsminh(args); } // -> ldsminlh_32_memop
    if(size==1 && !v && !a && r && !o3 && opc==6 && trans.has_lse()) { return trans.ldumaxh(args); } // -> ldumaxlh_32_memop
    if(size==1 && !v && !a && r && !o3 && opc==7 && trans.has_lse()) { return trans.lduminh(args); } // -> lduminlh_32_memop
    if(size==1 && !v && !a && r && o3 && (opc == 0) && trans.has_lse()) { return trans.swph(args); } // -> swplh_32_memop
    if(size==1 && !v && a && !r && !o3 && (opc == 0) && trans.has_lse()) { return trans.ldaddh(args); } // -> ldaddah_32_memop
    if(size==1 && !v && a && !r && !o3 && opc==1 && trans.has_lse()) { return trans.ldclrh(args); } // -> ldclrah_32_memop
    if(size==1 && !v && a && !r && !o3 && opc==2 && trans.has_lse()) { return trans.ldeorh(args); } // -> ldeorah_32_memop
    if(size==1 && !v && a && !r && !o3 && opc==3 && trans.has_lse()) { return trans.ldseth(args); } // -> ldsetah_32_memop
    if(size==1 && !v && a && !r && !o3 && opc==4 && trans.has_lse()) { return trans.ldsmaxh(args); } // -> ldsmaxah_32_memop
    if(size==1 && !v && a && !r && !o3 && opc==5 && trans.has_lse()) { return trans.ldsminh(args); } // -> ldsminah_32_memop
    if(size==1 && !v && a && !r && !o3 && opc==6 && trans.has_lse()) { return trans.ldumaxh(args); } // -> ldumaxah_32_memop
    if(size==1 && !v && a && !r && !o3 && opc==7 && trans.has_lse()) { return trans.lduminh(args); } // -> lduminah_32_memop
    if(size==1 && !v && a && !r && o3 && (opc == 0) && trans.has_lse()) { return trans.swph(args); } // -> swpah_32_memop
    if(size==1 && !v && a && !r && o3 && opc==4 && haslrcpc()) { return trans.ldaprh(args); } // -> ldaprh_32l_memop
    if(size==1 && !v && a && r && !o3 && (opc == 0) && trans.has_lse()) { return trans.ldaddh(args); } // -> ldaddalh_32_memop
    if(size==1 && !v && a && r && !o3 && opc==1 && trans.has_lse()) { return trans.ldclrh(args); } // -> ldclralh_32_memop
    if(size==1 && !v && a && r && !o3 && opc==2 && trans.has_lse()) { return trans.ldeorh(args); } // -> ldeoralh_32_memop
    if(size==1 && !v && a && r && !o3 && opc==3 && trans.has_lse()) { return trans.ldseth(args); } // -> ldsetalh_32_memop
    if(size==1 && !v && a && r && !o3 && opc==4 && trans.has_lse()) { return trans.ldsmaxh(args); } // -> ldsmaxalh_32_memop
    if(size==1 && !v && a && r && !o3 && opc==5 && trans.has_lse()) { return trans.ldsminh(args); } // -> ldsminalh_32_memop
    if(size==1 && !v && a && r && !o3 && opc==6 && trans.has_lse()) { return trans.ldumaxh(args); } // -> ldumaxalh_32_memop
    if(size==1 && !v && a && r && !o3 && opc==7 && trans.has_lse()) { return trans.lduminh(args); } // -> lduminalh_32_memop
    if(size==1 && !v && a && r && o3 && (opc == 0) && trans.has_lse()) { return trans.swph(args); } // -> swpalh_32_memop
    if(size==2 && !v && !a && !r && !o3 && (opc == 0) && trans.has_lse()) { return trans.ldadd(args); } // -> ldadd_32_memop
    if(size==2 && !v && !a && !r && !o3 && opc==1 && trans.has_lse()) { return trans.ldclr(args); } // -> ldclr_32_memop
    if(size==2 && !v && !a && !r && !o3 && opc==2 && trans.has_lse()) { return trans.ldeor(args); } // -> ldeor_32_memop
    if(size==2 && !v && !a && !r && !o3 && opc==3 && trans.has_lse()) { return trans.ldset(args); } // -> ldset_32_memop
    if(size==2 && !v && !a && !r && !o3 && opc==4 && trans.has_lse()) { return trans.ldsmax(args); } // -> ldsmax_32_memop
    if(size==2 && !v && !a && !r && !o3 && opc==5 && trans.has_lse()) { return trans.ldsmin(args); } // -> ldsmin_32_memop
    if(size==2 && !v && !a && !r && !o3 && opc==6 && trans.has_lse()) { return trans.ldumax(args); } // -> ldumax_32_memop
    if(size==2 && !v && !a && !r && !o3 && opc==7 && trans.has_lse()) { return trans.ldumin(args); } // -> ldumin_32_memop
    if(size==2 && !v && !a && !r && o3 && (opc == 0) && trans.has_lse()) { return trans.swp(args); } // -> swp_32_memop
    if(size==2 && !v && !a && r && !o3 && (opc == 0) && trans.has_lse()) { return trans.ldadd(args); } // -> ldaddl_32_memop
    if(size==2 && !v && !a && r && !o3 && opc==1 && trans.has_lse()) { return trans.ldclr(args); } // -> ldclrl_32_memop
    if(size==2 && !v && !a && r && !o3 && opc==2 && trans.has_lse()) { return trans.ldeor(args); } // -> ldeorl_32_memop
    if(size==2 && !v && !a && r && !o3 && opc==3 && trans.has_lse()) { return trans.ldset(args); } // -> ldsetl_32_memop
    if(size==2 && !v && !a && r && !o3 && opc==4 && trans.has_lse()) { return trans.ldsmax(args); } // -> ldsmaxl_32_memop
    if(size==2 && !v && !a && r && !o3 && opc==5 && trans.has_lse()) { return trans.ldsmin(args); } // -> ldsminl_32_memop
    if(size==2 && !v && !a && r && !o3 && opc==6 && trans.has_lse()) { return trans.ldumax(args); } // -> ldumaxl_32_memop
    if(size==2 && !v && !a && r && !o3 && opc==7 && trans.has_lse()) { return trans.ldumin(args); } // -> lduminl_32_memop
    if(size==2 && !v && !a && r && o3 && (opc == 0) && trans.has_lse()) { return trans.swp(args); } // -> swpl_32_memop
    if(size==2 && !v && a && !r && !o3 && (opc == 0) && trans.has_lse()) { return trans.ldadd(args); } // -> ldadda_32_memop
    if(size==2 && !v && a && !r && !o3 && opc==1 && trans.has_lse()) { return trans.ldclr(args); } // -> ldclra_32_memop
    if(size==2 && !v && a && !r && !o3 && opc==2 && trans.has_lse()) { return trans.ldeor(args); } // -> ldeora_32_memop
    if(size==2 && !v && a && !r && !o3 && opc==3 && trans.has_lse()) { return trans.ldset(args); } // -> ldseta_32_memop
    if(size==2 && !v && a && !r && !o3 && opc==4 && trans.has_lse()) { return trans.ldsmax(args); } // -> ldsmaxa_32_memop
    if(size==2 && !v && a && !r && !o3 && opc==5 && trans.has_lse()) { return trans.ldsmin(args); } // -> ldsmina_32_memop
    if(size==2 && !v && a && !r && !o3 && opc==6 && trans.has_lse()) { return trans.ldumax(args); } // -> ldumaxa_32_memop
    if(size==2 && !v && a && !r && !o3 && opc==7 && trans.has_lse()) { return trans.ldumin(args); } // -> ldumina_32_memop
    if(size==2 && !v && a && !r && o3 && (opc == 0) && trans.has_lse()) { return trans.swp(args); } // -> swpa_32_memop
    if(size==2 && !v && a && !r && o3 && opc==4 && haslrcpc()) { return trans.ldapr(args); } // -> ldapr_32l_memop
    if(size==2 && !v && a && r && !o3 && (opc == 0) && trans.has_lse()) { return trans.ldadd(args); } // -> ldaddal_32_memop
    if(size==2 && !v && a && r && !o3 && opc==1 && trans.has_lse()) { return trans.ldclr(args); } // -> ldclral_32_memop
    if(size==2 && !v && a && r && !o3 && opc==2 && trans.has_lse()) { return trans.ldeor(args); } // -> ldeoral_32_memop
    if(size==2 && !v && a && r && !o3 && opc==3 && trans.has_lse()) { return trans.ldset(args); } // -> ldsetal_32_memop
    if(size==2 && !v && a && r && !o3 && opc==4 && trans.has_lse()) { return trans.ldsmax(args); } // -> ldsmaxal_32_memop
    if(size==2 && !v && a && r && !o3 && opc==5 && trans.has_lse()) { return trans.ldsmin(args); } // -> ldsminal_32_memop
    if(size==2 && !v && a && r && !o3 && opc==6 && trans.has_lse()) { return trans.ldumax(args); } // -> ldumaxal_32_memop
    if(size==2 && !v && a && r && !o3 && opc==7 && trans.has_lse()) { return trans.ldumin(args); } // -> lduminal_32_memop
    if(size==2 && !v && a && r && o3 && (opc == 0) && trans.has_lse()) { return trans.swp(args); } // -> swpal_32_memop
    if(size==3 && !v && !a && !r && !o3 && (opc == 0) && trans.has_lse()) { return trans.ldadd(args); } // -> ldadd_64_memop
    if(size==3 && !v && !a && !r && !o3 && opc==1 && trans.has_lse()) { return trans.ldclr(args); } // -> ldclr_64_memop
    if(size==3 && !v && !a && !r && !o3 && opc==2 && trans.has_lse()) { return trans.ldeor(args); } // -> ldeor_64_memop
    if(size==3 && !v && !a && !r && !o3 && opc==3 && trans.has_lse()) { return trans.ldset(args); } // -> ldset_64_memop
    if(size==3 && !v && !a && !r && !o3 && opc==4 && trans.has_lse()) { return trans.ldsmax(args); } // -> ldsmax_64_memop
    if(size==3 && !v && !a && !r && !o3 && opc==5 && trans.has_lse()) { return trans.ldsmin(args); } // -> ldsmin_64_memop
    if(size==3 && !v && !a && !r && !o3 && opc==6 && trans.has_lse()) { return trans.ldumax(args); } // -> ldumax_64_memop
    if(size==3 && !v && !a && !r && !o3 && opc==7 && trans.has_lse()) { return trans.ldumin(args); } // -> ldumin_64_memop
    if(size==3 && !v && !a && !r && o3 && (opc == 0) && trans.has_lse()) { return trans.swp(args); } // -> swp_64_memop
    if(size==3 && !v && !a && !r && o3 && opc==2 && hasls64_v()) { return trans.st64bv0(args); } // -> st64bv0_64_memop
    if(size==3 && !v && !a && !r && o3 && opc==3 && hasls64_v()) { return trans.st64bv(args); } // -> st64bv_64_memop
    if(size==3 && !v && !a && r && !o3 && (opc == 0) && trans.has_lse()) { return trans.ldadd(args); } // -> ldaddl_64_memop
    if(size==3 && !v && !a && r && !o3 && opc==1 && trans.has_lse()) { return trans.ldclr(args); } // -> ldclrl_64_memop
    if(size==3 && !v && !a && r && !o3 && opc==2 && trans.has_lse()) { return trans.ldeor(args); } // -> ldeorl_64_memop
    if(size==3 && !v && !a && r && !o3 && opc==3 && trans.has_lse()) { return trans.ldset(args); } // -> ldsetl_64_memop
    if(size==3 && !v && !a && r && !o3 && opc==4 && trans.has_lse()) { return trans.ldsmax(args); } // -> ldsmaxl_64_memop
    if(size==3 && !v && !a && r && !o3 && opc==5 && trans.has_lse()) { return trans.ldsmin(args); } // -> ldsminl_64_memop
    if(size==3 && !v && !a && r && !o3 && opc==6 && trans.has_lse()) { return trans.ldumax(args); } // -> ldumaxl_64_memop
    if(size==3 && !v && !a && r && !o3 && opc==7 && trans.has_lse()) { return trans.ldumin(args); } // -> lduminl_64_memop
    if(size==3 && !v && !a && r && o3 && (opc == 0) && trans.has_lse()) { return trans.swp(args); } // -> swpl_64_memop
    if(size==3 && !v && a && !r && !o3 && (opc == 0) && trans.has_lse()) { return trans.ldadd(args); } // -> ldadda_64_memop
    if(size==3 && !v && a && !r && !o3 && opc==1 && trans.has_lse()) { return trans.ldclr(args); } // -> ldclra_64_memop
    if(size==3 && !v && a && !r && !o3 && opc==2 && trans.has_lse()) { return trans.ldeor(args); } // -> ldeora_64_memop
    if(size==3 && !v && a && !r && !o3 && opc==3 && trans.has_lse()) { return trans.ldset(args); } // -> ldseta_64_memop
    if(size==3 && !v && a && !r && !o3 && opc==4 && trans.has_lse()) { return trans.ldsmax(args); } // -> ldsmaxa_64_memop
    if(size==3 && !v && a && !r && !o3 && opc==5 && trans.has_lse()) { return trans.ldsmin(args); } // -> ldsmina_64_memop
    if(size==3 && !v && a && !r && !o3 && opc==6 && trans.has_lse()) { return trans.ldumax(args); } // -> ldumaxa_64_memop
    if(size==3 && !v && a && !r && !o3 && opc==7 && trans.has_lse()) { return trans.ldumin(args); } // -> ldumina_64_memop
    if(size==3 && !v && a && !r && o3 && (opc == 0) && trans.has_lse()) { return trans.swp(args); } // -> swpa_64_memop
    if(size==3 && !v && a && !r && o3 && opc==4 && haslrcpc()) { return trans.ldapr(args); } // -> ldapr_64l_memop
    if(size==3 && !v && a && r && !o3 && (opc == 0) && trans.has_lse()) { return trans.ldadd(args); } // -> ldaddal_64_memop
    if(size==3 && !v && a && r && !o3 && opc==1 && trans.has_lse()) { return trans.ldclr(args); } // -> ldclral_64_memop
    if(size==3 && !v && a && r && !o3 && opc==2 && trans.has_lse()) { return trans.ldeor(args); } // -> ldeoral_64_memop
    if(size==3 && !v && a && r && !o3 && opc==3 && trans.has_lse()) { return trans.ldset(args); } // -> ldsetal_64_memop
    if(size==3 && !v && a && r && !o3 && opc==4 && trans.has_lse()) { return trans.ldsmax(args); } // -> ldsmaxal_64_memop
    if(size==3 && !v && a && r && !o3 && opc==5 && trans.has_lse()) { return trans.ldsmin(args); } // -> ldsminal_64_memop
    if(size==3 && !v && a && r && !o3 && opc==6 && trans.has_lse()) { return trans.ldumax(args); } // -> ldumaxal_64_memop
    if(size==3 && !v && a && r && !o3 && opc==7 && trans.has_lse()) { return trans.ldumin(args); } // -> lduminal_64_memop
    if(size==3 && !v && a && r && o3 && (opc == 0) && trans.has_lse()) { return trans.swp(args); } // -> swpal_64_memop

     */
    return false;
}
pub fn decode_comswap<T: Arm64DecodeTrait>(trans: &mut T, insn: u32) -> bool {
    let args: ArmInstr = ArmInstr {
        insn
    };
    let rt2 = ((insn >> 10) & 0x1f);
    let l = ((insn >> 22) & 1) != 0;
    let o0 = ((insn >> 15) & 1) != 0;
    let size = ((insn >> 30));
    if((size == 0) && !l && !o0 && rt2==0x1f && trans.has_lse_feat()) { return trans.casb(args); } // -> casb_c32_comswap
    if((size == 0) && !l && o0 && rt2==0x1f && trans.has_lse_feat()) { return trans.casb(args); } // -> caslb_c32_comswap
    if((size == 0) && l && !o0 && rt2==0x1f && trans.has_lse_feat()) { return trans.casb(args); } // -> casab_c32_comswap
    if((size == 0) && l && o0 && rt2==0x1f && trans.has_lse_feat()) { return trans.casb(args); } // -> casalb_c32_comswap
    if(size==1 && !l && !o0 && rt2==0x1f && trans.has_lse_feat()) { return trans.cash(args); } // -> cash_c32_comswap
    if(size==1 && !l && o0 && rt2==0x1f && trans.has_lse_feat()) { return trans.cash(args); } // -> caslh_c32_comswap
    if(size==1 && l && !o0 && rt2==0x1f && trans.has_lse_feat()) { return trans.cash(args); } // -> casah_c32_comswap
    if(size==1 && l && o0 && rt2==0x1f && trans.has_lse_feat()) { return trans.cash(args); } // -> casalh_c32_comswap
    if(size==2 && !l && !o0 && rt2==0x1f && trans.has_lse_feat()) { return trans.cas(args); } // -> cas_c32_comswap
    if(size==2 && !l && o0 && rt2==0x1f && trans.has_lse_feat()) { return trans.cas(args); } // -> casl_c32_comswap
    if(size==2 && l && !o0 && rt2==0x1f && trans.has_lse_feat()) { return trans.cas(args); } // -> casa_c32_comswap
    if(size==2 && l && o0 && rt2==0x1f && trans.has_lse_feat()) { return trans.cas(args); } // -> casal_c32_comswap
    if(size==3 && !l && !o0 && rt2==0x1f && trans.has_lse_feat()) { return trans.cas(args); } // -> cas_c64_comswap
    if(size==3 && !l && o0 && rt2==0x1f && trans.has_lse_feat()) { return trans.cas(args); } // -> casl_c64_comswap
    if(size==3 && l && !o0 && rt2==0x1f && trans.has_lse_feat()) { return trans.cas(args); } // -> casa_c64_comswap
    if(size==3 && l && o0 && rt2==0x1f && trans.has_lse_feat()) { return trans.cas(args); } // -> casal_c64_comswap
    return false;
}
pub fn decode_comswappr<T: Arm64DecodeTrait>(trans: &mut T, insn: u32) -> bool {
    let args: ArmInstr = ArmInstr {
        insn
    };
    let rt2 = ((insn >> 10) & 0x1f);
    let l = ((insn >> 22) & 1) != 0;
    let o0 = ((insn >> 15) & 1) != 0;
    let sz = ((insn >> 30) & 1) != 0;
    if(!sz && !l && !o0 && rt2==0x1f && trans.has_lse_feat()) { return trans.casp(args); } // -> casp_cp32_comswappr
    if(!sz && !l && o0 && rt2==0x1f && trans.has_lse_feat()) { return trans.casp(args); } // -> caspl_cp32_comswappr
    if(!sz && l && !o0 && rt2==0x1f && trans.has_lse_feat()) { return trans.casp(args); } // -> caspa_cp32_comswappr
    if(!sz && l && o0 && rt2==0x1f && trans.has_lse_feat()) { return trans.casp(args); } // -> caspal_cp32_comswappr
    if(sz && !l && !o0 && rt2==0x1f && trans.has_lse_feat()) { return trans.casp(args); } // -> casp_cp64_comswappr
    if(sz && !l && o0 && rt2==0x1f && trans.has_lse_feat()) { return trans.casp(args); } // -> caspl_cp64_comswappr
    if(sz && l && !o0 && rt2==0x1f && trans.has_lse_feat()) { return trans.casp(args); } // -> caspa_cp64_comswappr
    if(sz && l && o0 && rt2==0x1f && trans.has_lse_feat()) { return trans.casp(args); } // -> caspal_cp64_comswappr
    return false;
}
pub fn decode_ldapstl_unscaled<T: Arm64DecodeTrait>(trans: &mut T, insn: u32) -> bool {
    let args: ArmInstr = ArmInstr {
        insn
    };
    let size = (insn >> 30);
    let opc = ((insn >> 22) & 3);
    if((size == 0) && (opc == 0) && trans.has_lrcpc2()) { return trans.stlurb(args); } // -> stlurb_32_ldapstl_unscaled
    if((size == 0) && opc==1 && trans.has_lrcpc2()) { return trans.ldapurb(args); } // -> ldapurb_32_ldapstl_unscaled
    if((size == 0) && opc==2 && trans.has_lrcpc2()) { return trans.ldapursb(args); } // -> ldapursb_64_ldapstl_unscaled
    if((size == 0) && opc==3 && trans.has_lrcpc2()) { return trans.ldapursb(args); } // -> ldapursb_32_ldapstl_unscaled
    if(size==1 && (opc == 0) && trans.has_lrcpc2()) { return trans.stlurh(args); } // -> stlurh_32_ldapstl_unscaled
    if(size==1 && opc==1 && trans.has_lrcpc2()) { return trans.ldapurh(args); } // -> ldapurh_32_ldapstl_unscaled
    if(size==1 && opc==2 && trans.has_lrcpc2()) { return trans.ldapursh(args); } // -> ldapursh_64_ldapstl_unscaled
    if(size==1 && opc==3 && trans.has_lrcpc2()) { return trans.ldapursh(args); } // -> ldapursh_32_ldapstl_unscaled
    if(size==2 && (opc == 0) && trans.has_lrcpc2()) { return trans.stlur_gen(args); } // -> stlur_32_ldapstl_unscaled
    if(size==2 && opc==1 && trans.has_lrcpc2()) { return trans.ldapur_gen(args); } // -> ldapur_32_ldapstl_unscaled
    if(size==2 && opc==2 && trans.has_lrcpc2()) { return trans.ldapursw(args); } // -> ldapursw_64_ldapstl_unscaled
    if(size==3 && (opc == 0) && trans.has_lrcpc2()) { return trans.stlur_gen(args); } // -> stlur_64_ldapstl_unscaled
    if(size==3 && opc==1 && trans.has_lrcpc2()) { return trans.ldapur_gen(args); } // -> ldapur_64_ldapstl_unscaled

    return false;
}
pub fn decode_loadlit<T: Arm64DecodeTrait>(trans: &mut T, insn: u32) -> bool {
    let args: ArmInstr = ArmInstr {
        insn
    };
    let v = ((insn >> 26) & 1) != 0;
    let opc = ((insn >> 30));
    if((opc == 0) && !v) { return trans.ldr_lit_gen(args); } // -> ldr_32_loadlit
    if((opc == 0) && v) { return trans.ldr_lit_fpsimd(args); } // -> ldr_s_loadlit
    if(opc==1 && !v) { return trans.ldr_lit_gen(args); } // -> ldr_64_loadlit
    if(opc==1 && v) { return trans.ldr_lit_fpsimd(args); } // -> ldr_d_loadlit
    if(opc==2 && !v) { return trans.ldrsw_lit(args); } // -> ldrsw_64_loadlit
    if(opc==2 && v) { return trans.ldr_lit_fpsimd(args); } // -> ldr_q_loadlit
    if(opc==3 && !v) { return trans.prfm_lit(args); } // -> prfm_p_loadlit
    return false;
}
pub fn decode_ldstexclp<T: Arm64DecodeTrait>(trans: &mut T, insn: u32) -> bool {
    let args: ArmInstr = ArmInstr {
        insn
    };
    let l = ((insn >> 22) & 1) != 0;
    let o0 = ((insn >> 15) & 1) != 0;
    let sz = ((insn >> 30) & 1) != 0;
    if(!sz && !l && !o0) { return trans.stxp(args); } // -> stxp_sp32_ldstexclp
    if(!sz && !l && o0) { return trans.stlxp(args); } // -> stlxp_sp32_ldstexclp
    if(!sz && l && !o0) { return trans.ldxp(args); } // -> ldxp_lp32_ldstexclp
    if(!sz && l && o0) { return trans.ldaxp(args); } // -> ldaxp_lp32_ldstexclp
    if(sz && !l && !o0) { return trans.stxp(args); } // -> stxp_sp64_ldstexclp
    if(sz && !l && o0) { return trans.stlxp(args); } // -> stlxp_sp64_ldstexclp
    if(sz && l && !o0) { return trans.ldxp(args); } // -> ldxp_lp64_ldstexclp
    if(sz && l && o0) { return trans.ldaxp(args); } // -> ldaxp_lp64_ldstexclp
    return false;
}
pub fn decode_ldstexclr<T: Arm64DecodeTrait>(trans: &mut T, insn: u32) -> bool {
    let args: ArmInstr = ArmInstr {
        insn
    };
    let l = ((insn >> 22) & 1) != 0;
    let o0 = ((insn >> 15) & 1) != 0;
    let size = (insn >> 30);
    if((size == 0) && !l && !o0) { return trans.stxrb(args); } // -> stxrb_sr32_ldstexclr
    if((size == 0) && !l && o0) { return trans.stlxrb(args); } // -> stlxrb_sr32_ldstexclr
    if((size == 0) && l && !o0) { return trans.ldxrb(args); } // -> ldxrb_lr32_ldstexclr
    if((size == 0) && l && o0) { return trans.ldaxrb(args); } // -> ldaxrb_lr32_ldstexclr
    if(size==1 && !l && !o0) { return trans.stxrh(args); } // -> stxrh_sr32_ldstexclr
    if(size==1 && !l && o0) { return trans.stlxrh(args); } // -> stlxrh_sr32_ldstexclr
    if(size==1 && l && !o0) { return trans.ldxrh(args); } // -> ldxrh_lr32_ldstexclr
    if(size==1 && l && o0) { return trans.ldaxrh(args); } // -> ldaxrh_lr32_ldstexclr
    if(size==2 && !l && !o0) { return trans.stxr(args); } // -> stxr_sr32_ldstexclr
    if(size==2 && !l && o0) { return trans.stlxr(args); } // -> stlxr_sr32_ldstexclr
    if(size==2 && l && !o0) { return trans.ldxr(args); } // -> ldxr_lr32_ldstexclr
    if(size==2 && l && o0) { return trans.ldaxr(args); } // -> ldaxr_lr32_ldstexclr
    if(size==3 && !l && !o0) { return trans.stxr(args); } // -> stxr_sr64_ldstexclr
    if(size==3 && !l && o0) { return trans.stlxr(args); } // -> stlxr_sr64_ldstexclr
    if(size==3 && l && !o0) { return trans.ldxr(args); } // -> ldxr_lr64_ldstexclr
    if(size==3 && l && o0) { return trans.ldaxr(args); } // -> ldaxr_lr64_ldstexclr
    return false;
}
pub fn decode_ldsttags<T: Arm64DecodeTrait>(trans: &mut T, insn: u32) -> bool {
    let args: ArmInstr = ArmInstr {
        insn
    };
    let op2 = ((insn >> 10) & 3);
    let imm9 = ((insn >> 12) & 0x1ff);
    let opc = (insn >> 22) & 3;
    if((opc == 0) && (imm9 == 0) && (op2 == 0) && trans.has_mte2_feat()) { return trans.stzgm(args); } // -> stzgm_64bulk_ldsttags
    if(opc==2 && (imm9 == 0) && (op2 == 0) && trans.has_mte2_feat()) { return trans.stgm(args); } // -> stgm_64bulk_ldsttags
    if(opc==3 && (imm9 == 0) && (op2 == 0) && trans.has_mte2_feat()) { return trans.ldgm(args); } // -> ldgm_64bulk_ldsttags
    if((opc == 0) && op2==1 && trans.has_mte_feat()) { return trans.stg(args); } // -> stg_64spost_ldsttags
    if((opc == 0) && op2==2 && trans.has_mte_feat()) { return trans.stg(args); } // -> stg_64soffset_ldsttags
    if((opc == 0) && op2==3 && trans.has_mte_feat()) { return trans.stg(args); } // -> stg_64spre_ldsttags
    if(opc==1 && (op2 == 0) && trans.has_mte_feat()) { return trans.ldg(args); } // -> ldg_64loffset_ldsttags
    if(opc==1 && op2==1 && trans.has_mte_feat()) { return trans.stzg(args); } // -> stzg_64spost_ldsttags
    if(opc==1 && op2==2 && trans.has_mte_feat()) { return trans.stzg(args); } // -> stzg_64soffset_ldsttags
    if(opc==1 && op2==3 && trans.has_mte_feat()) { return trans.stzg(args); } // -> stzg_64spre_ldsttags
    if(opc==2 && op2==1 && trans.has_mte_feat()) { return trans.st2g(args); } // -> st2g_64spost_ldsttags
    if(opc==2 && op2==2 && trans.has_mte_feat()) { return trans.st2g(args); } // -> st2g_64soffset_ldsttags
    if(opc==2 && op2==3 && trans.has_mte_feat()) { return trans.st2g(args); } // -> st2g_64spre_ldsttags
    if(opc==3 && op2==1 && trans.has_mte_feat()) { return trans.stz2g(args); } // -> stz2g_64spost_ldsttags
    if(opc==3 && op2==2 && trans.has_mte_feat()) { return trans.stz2g(args); } // -> stz2g_64soffset_ldsttags
    if(opc==3 && op2==3 && trans.has_mte_feat()) { return trans.stz2g(args); } // -> stz2g_64spre_ldsttags

    return false;
}
pub fn decode_ldstnapair_offs<T: Arm64DecodeTrait>(trans: &mut T, insn: u32) -> bool {
    let args: ArmInstr = ArmInstr {
        insn
    };
    let l = ((insn >> 22) & 1) != 0;
    let v = ((insn >> 26) & 1) != 0;
    let opc = (insn >> 30);
    if((opc == 0) && !v && !l) { return trans.stnp_gen(args); } // -> stnp_32_ldstnapair_offs
    if((opc == 0) && !v && l) { return trans.ldnp_gen(args); } // -> ldnp_32_ldstnapair_offs
    if((opc == 0) && v && !l) { return trans.stnp_fpsimd(args); } // -> stnp_s_ldstnapair_offs
    if((opc == 0) && v && l) { return trans.ldnp_fpsimd(args); } // -> ldnp_s_ldstnapair_offs
    if(opc==1 && v && !l) { return trans.stnp_fpsimd(args); } // -> stnp_d_ldstnapair_offs
    if(opc==1 && v && l) { return trans.ldnp_fpsimd(args); } // -> ldnp_d_ldstnapair_offs
    if(opc==2 && !v && !l) { return trans.stnp_gen(args); } // -> stnp_64_ldstnapair_offs
    if(opc==2 && !v && l) { return trans.ldnp_gen(args); } // -> ldnp_64_ldstnapair_offs
    if(opc==2 && v && !l) { return trans.stnp_fpsimd(args); } // -> stnp_q_ldstnapair_offs
    if(opc==2 && v && l) { return trans.ldnp_fpsimd(args); } // -> ldnp_q_ldstnapair_offs
    return false;
}
pub fn decode_ldstord<T: Arm64DecodeTrait>(trans: &mut T, insn: u32) -> bool {
    let args: ArmInstr = ArmInstr {
        insn
    };
    let l = ((insn >> 22) & 1) != 0;
    let o0 = ((insn >> 15) & 1) != 0;
    let size = (insn >> 30);
    if((size == 0) && !l && !o0 && trans.has_lor_feat()) { return trans.stllrb(args); } // -> stllrb_sl32_ldstord
    if((size == 0) && !l && o0) { return trans.stlrb(args); } // -> stlrb_sl32_ldstord
    if((size == 0) && l && !o0 && trans.has_lor_feat()) { return trans.ldlarb(args); } // -> ldlarb_lr32_ldstord
    if((size == 0) && l && o0) { return trans.ldarb(args); } // -> ldarb_lr32_ldstord
    if(size==1 && !l && !o0 && trans.has_lor_feat()) { return trans.stllrh(args); } // -> stllrh_sl32_ldstord
    if(size==1 && !l && o0) { return trans.stlrh(args); } // -> stlrh_sl32_ldstord
    if(size==1 && l && !o0 && trans.has_lor_feat()) { return trans.ldlarh(args); } // -> ldlarh_lr32_ldstord
    if(size==1 && l && o0) { return trans.ldarh(args); } // -> ldarh_lr32_ldstord
    if(size==2 && !l && !o0 && trans.has_lor_feat()) { return trans.stllr(args); } // -> stllr_sl32_ldstord
    if(size==2 && !l && o0) { return trans.stlr(args); } // -> stlr_sl32_ldstord
    if(size==2 && l && !o0 && trans.has_lor_feat()) { return trans.ldlar(args); } // -> ldlar_lr32_ldstord
    if(size==2 && l && o0) { return trans.ldar(args); } // -> ldar_lr32_ldstord
    if(size==3 && !l && !o0 && trans.has_lor_feat()) { return trans.stllr(args); } // -> stllr_sl64_ldstord
    if(size==3 && !l && o0) { return trans.stlr(args); } // -> stlr_sl64_ldstord
    if(size==3 && l && !o0 && trans.has_lor_feat()) { return trans.ldlar(args); } // -> ldlar_lr64_ldstord
    if(size==3 && l && o0) { return trans.ldar(args); } // -> ldar_lr64_ldstord
    return false;
}
pub fn decode_ldst_immpost<T: Arm64DecodeTrait>(trans: &mut T, insn: u32) -> bool {
    let args: ArmInstr = ArmInstr {
        insn
    };
    let v = ((insn >> 26) & 1) != 0;
    let opc = ((insn >> 22) & 3);
    let size = (insn >> 30);
    if((size == 0) && !v && (opc == 0)) { return trans.strb_imm(args); } // -> strb_32_ldst_immpost
    if((size == 0) && !v && opc==1) { return trans.ldrb_imm(args); } // -> ldrb_32_ldst_immpost
    if((size == 0) && !v && opc==2) { return trans.ldrsb_imm(args); } // -> ldrsb_64_ldst_immpost
    if((size == 0) && !v && opc==3) { return trans.ldrsb_imm(args); } // -> ldrsb_32_ldst_immpost
    if((size == 0) && v && (opc == 0)) { return trans.str_imm_fpsimd(args); } // -> str_b_ldst_immpost
    if((size == 0) && v && opc==1) { return trans.ldr_imm_fpsimd(args); } // -> ldr_b_ldst_immpost
    if((size == 0) && v && opc==2) { return trans.str_imm_fpsimd(args); } // -> str_q_ldst_immpost
    if((size == 0) && v && opc==3) { return trans.ldr_imm_fpsimd(args); } // -> ldr_q_ldst_immpost
    if(size==1 && !v && (opc == 0)) { return trans.strh_imm(args); } // -> strh_32_ldst_immpost
    if(size==1 && !v && opc==1) { return trans.ldrh_imm(args); } // -> ldrh_32_ldst_immpost
    if(size==1 && !v && opc==2) { return trans.ldrsh_imm(args); } // -> ldrsh_64_ldst_immpost
    if(size==1 && !v && opc==3) { return trans.ldrsh_imm(args); } // -> ldrsh_32_ldst_immpost
    if(size==1 && v && (opc == 0)) { return trans.str_imm_fpsimd(args); } // -> str_h_ldst_immpost
    if(size==1 && v && opc==1) { return trans.ldr_imm_fpsimd(args); } // -> ldr_h_ldst_immpost
    if(size==2 && !v && (opc == 0)) { return trans.str_imm_gen(args); } // -> str_32_ldst_immpost
    if(size==2 && !v && opc==1) { return trans.ldr_imm_gen(args); } // -> ldr_32_ldst_immpost
    if(size==2 && !v && opc==2) { return trans.ldrsw_imm(args); } // -> ldrsw_64_ldst_immpost
    if(size==2 && v && (opc == 0)) { return trans.str_imm_fpsimd(args); } // -> str_s_ldst_immpost
    if(size==2 && v && opc==1) { return trans.ldr_imm_fpsimd(args); } // -> ldr_s_ldst_immpost
    if(size==3 && !v && (opc == 0)) { return trans.str_imm_gen(args); } // -> str_64_ldst_immpost
    if(size==3 && !v && opc==1) { return trans.ldr_imm_gen(args); } // -> ldr_64_ldst_immpost
    if(size==3 && v && (opc == 0)) { return trans.str_imm_fpsimd(args); } // -> str_d_ldst_immpost
    if(size==3 && v && opc==1) { return trans.ldr_imm_fpsimd(args); } // -> ldr_d_ldst_immpost
    return false;
}
pub fn decode_ldst_immpre<T: Arm64DecodeTrait>(trans: &mut T, insn: u32) -> bool {
    let args: ArmInstr = ArmInstr {
        insn
    };
    let v = ((insn >> 26) & 1) != 0;
    let opc = ((insn >> 22) & 3);
    let size = (insn >> 30);
    if((size == 0) && !v && (opc == 0)) { return trans.strb_imm(args); } // -> strb_32_ldst_immpre
    if((size == 0) && !v && opc==1) { return trans.ldrb_imm(args); } // -> ldrb_32_ldst_immpre
    if((size == 0) && !v && opc==2) { return trans.ldrsb_imm(args); } // -> ldrsb_64_ldst_immpre
    if((size == 0) && !v && opc==3) { return trans.ldrsb_imm(args); } // -> ldrsb_32_ldst_immpre
    if((size == 0) && v && (opc == 0)) { return trans.str_imm_fpsimd(args); } // -> str_b_ldst_immpre
    if((size == 0) && v && opc==1) { return trans.ldr_imm_fpsimd(args); } // -> ldr_b_ldst_immpre
    if((size == 0) && v && opc==2) { return trans.str_imm_fpsimd(args); } // -> str_q_ldst_immpre
    if((size == 0) && v && opc==3) { return trans.ldr_imm_fpsimd(args); } // -> ldr_q_ldst_immpre
    if(size==1 && !v && (opc == 0)) { return trans.strh_imm(args); } // -> strh_32_ldst_immpre
    if(size==1 && !v && opc==1) { return trans.ldrh_imm(args); } // -> ldrh_32_ldst_immpre
    if(size==1 && !v && opc==2) { return trans.ldrsh_imm(args); } // -> ldrsh_64_ldst_immpre
    if(size==1 && !v && opc==3) { return trans.ldrsh_imm(args); } // -> ldrsh_32_ldst_immpre
    if(size==1 && v && (opc == 0)) { return trans.str_imm_fpsimd(args); } // -> str_h_ldst_immpre
    if(size==1 && v && opc==1) { return trans.ldr_imm_fpsimd(args); } // -> ldr_h_ldst_immpre
    if(size==2 && !v && (opc == 0)) { return trans.str_imm_gen(args); } // -> str_32_ldst_immpre
    if(size==2 && !v && opc==1) { return trans.ldr_imm_gen(args); } // -> ldr_32_ldst_immpre
    if(size==2 && !v && opc==2) { return trans.ldrsw_imm(args); } // -> ldrsw_64_ldst_immpre
    if(size==2 && v && (opc == 0)) { return trans.str_imm_fpsimd(args); } // -> str_s_ldst_immpre
    if(size==2 && v && opc==1) { return trans.ldr_imm_fpsimd(args); } // -> ldr_s_ldst_immpre
    if(size==3 && !v && (opc == 0)) { return trans.str_imm_gen(args); } // -> str_64_ldst_immpre
    if(size==3 && !v && opc==1) { return trans.ldr_imm_gen(args); } // -> ldr_64_ldst_immpre
    if(size==3 && v && (opc == 0)) { return trans.str_imm_fpsimd(args); } // -> str_d_ldst_immpre
    if(size==3 && v && opc==1) { return trans.ldr_imm_fpsimd(args); } // -> ldr_d_ldst_immpre
    return false;
}
pub fn decode_ldst_pac<T: Arm64DecodeTrait>(trans: &mut T, insn: u32) -> bool {
    let args: ArmInstr = ArmInstr {
        insn
    };
    let v = ((insn >> 26) & 1) != 0;
    let m = ((insn >> 23) & 1) != 0;
    let w = ((insn >> 11) & 1) != 0;
    let size = (insn >> 30);
    if(size==3 && !v && !m && !w && trans.has_pauth_feat()) { return trans.ldra(args); } // -> ldraa_64_ldst_pac
    if(size==3 && !v && !m && w && trans.has_pauth_feat()) { return trans.ldra(args); } // -> ldraa_64w_ldst_pac
    if(size==3 && !v && m && !w && trans.has_pauth_feat()) { return trans.ldra(args); } // -> ldrab_64_ldst_pac
    if(size==3 && !v && m && w && trans.has_pauth_feat()) { return trans.ldra(args); } // -> ldrab_64w_ldst_pac
    return false;
}
pub fn decode_ldst_regoff<T: Arm64DecodeTrait>(trans: &mut T, insn: u32) -> bool {
    let args: ArmInstr = ArmInstr {
        insn
    };
    let v = ((insn >> 26) & 1) != 0;
    let size = (insn >> 30);
    let opc = ((insn >> 22) & 3);
    let option = ((insn >> 13) & 7);
    if((size == 0) && !v && (opc == 0) && option!=3) { return trans.strb_reg(args); } // -> strb_32b_ldst_regoff
    if((size == 0) && !v && (opc == 0) && option==3) { return trans.strb_reg(args); } // -> strb_32bl_ldst_regoff
    if((size == 0) && !v && opc==1 && option!=3) { return trans.ldrb_reg(args); } // -> ldrb_32b_ldst_regoff
    if((size == 0) && !v && opc==1 && option==3) { return trans.ldrb_reg(args); } // -> ldrb_32bl_ldst_regoff
    if((size == 0) && !v && opc==2 && option!=3) { return trans.ldrsb_reg(args); } // -> ldrsb_64b_ldst_regoff
    if((size == 0) && !v && opc==2 && option==3) { return trans.ldrsb_reg(args); } // -> ldrsb_64bl_ldst_regoff
    if((size == 0) && !v && opc==3 && option!=3) { return trans.ldrsb_reg(args); } // -> ldrsb_32b_ldst_regoff
    if((size == 0) && !v && opc==3 && option==3) { return trans.ldrsb_reg(args); } // -> ldrsb_32bl_ldst_regoff
    if((size == 0) && v && (opc == 0) && option!=3) { return trans.str_reg_fpsimd(args); } // -> str_b_ldst_regoff
    if((size == 0) && v && (opc == 0) && option==3) { return trans.str_reg_fpsimd(args); } // -> str_bl_ldst_regoff
    if((size == 0) && v && opc==1 && option!=3) { return trans.ldr_reg_fpsimd(args); } // -> ldr_b_ldst_regoff
    if((size == 0) && v && opc==1 && option==3) { return trans.ldr_reg_fpsimd(args); } // -> ldr_bl_ldst_regoff
    if((size == 0) && v && opc==2) { return trans.str_reg_fpsimd(args); } // -> str_q_ldst_regoff
    if((size == 0) && v && opc==3) { return trans.ldr_reg_fpsimd(args); } // -> ldr_q_ldst_regoff
    if(size==1 && !v && (opc == 0)) { return trans.strh_reg(args); } // -> strh_32_ldst_regoff
    if(size==1 && !v && opc==1) { return trans.ldrh_reg(args); } // -> ldrh_32_ldst_regoff
    if(size==1 && !v && opc==2) { return trans.ldrsh_reg(args); } // -> ldrsh_64_ldst_regoff
    if(size==1 && !v && opc==3) { return trans.ldrsh_reg(args); } // -> ldrsh_32_ldst_regoff
    if(size==1 && v && (opc == 0)) { return trans.str_reg_fpsimd(args); } // -> str_h_ldst_regoff
    if(size==1 && v && opc==1) { return trans.ldr_reg_fpsimd(args); } // -> ldr_h_ldst_regoff
    if(size==2 && !v && (opc == 0)) { return trans.str_reg_gen(args); } // -> str_32_ldst_regoff
    if(size==2 && !v && opc==1) { return trans.ldr_reg_gen(args); } // -> ldr_32_ldst_regoff
    if(size==2 && !v && opc==2) { return trans.ldrsw_reg(args); } // -> ldrsw_64_ldst_regoff
    if(size==2 && v && (opc == 0)) { return trans.str_reg_fpsimd(args); } // -> str_s_ldst_regoff
    if(size==2 && v && opc==1) { return trans.ldr_reg_fpsimd(args); } // -> ldr_s_ldst_regoff
    if(size==3 && !v && (opc == 0)) { return trans.str_reg_gen(args); } // -> str_64_ldst_regoff
    if(size==3 && !v && opc==1) { return trans.ldr_reg_gen(args); } // -> ldr_64_ldst_regoff
    if(size==3 && !v && opc==2) { return trans.prfm_reg(args); } // -> prfm_p_ldst_regoff
    if(size==3 && v && (opc == 0)) { return trans.str_reg_fpsimd(args); } // -> str_d_ldst_regoff
    if(size==3 && v && opc==1) { return trans.ldr_reg_fpsimd(args); } // -> ldr_d_ldst_regoff

    return false;
}
pub fn decode_ldst_unpriv<T: Arm64DecodeTrait>(trans: &mut T, insn: u32) -> bool {
    let args: ArmInstr = ArmInstr {
        insn
    };
    let v = ((insn >> 26) & 1) != 0;
    let size = (insn >> 30);
    let opc = ((insn >> 22) & 3);
    if((size == 0) && !v && (opc == 0)) { return trans.sttrb(args); } // -> sttrb_32_ldst_unpriv
    if((size == 0) && !v && opc==1) { return trans.ldtrb(args); } // -> ldtrb_32_ldst_unpriv
    if((size == 0) && !v && opc==2) { return trans.ldtrsb(args); } // -> ldtrsb_64_ldst_unpriv
    if((size == 0) && !v && opc==3) { return trans.ldtrsb(args); } // -> ldtrsb_32_ldst_unpriv
    if(size==1 && !v && (opc == 0)) { return trans.sttrh(args); } // -> sttrh_32_ldst_unpriv
    if(size==1 && !v && opc==1) { return trans.ldtrh(args); } // -> ldtrh_32_ldst_unpriv
    if(size==1 && !v && opc==2) { return trans.ldtrsh(args); } // -> ldtrsh_64_ldst_unpriv
    if(size==1 && !v && opc==3) { return trans.ldtrsh(args); } // -> ldtrsh_32_ldst_unpriv
    if(size==2 && !v && (opc == 0)) { return trans.sttr(args); } // -> sttr_32_ldst_unpriv
    if(size==2 && !v && opc==1) { return trans.ldtr(args); } // -> ldtr_32_ldst_unpriv
    if(size==2 && !v && opc==2) { return trans.ldtrsw(args); } // -> ldtrsw_64_ldst_unpriv
    if(size==3 && !v && (opc == 0)) { return trans.sttr(args); } // -> sttr_64_ldst_unpriv
    if(size==3 && !v && opc==1) { return trans.ldtr(args); } // -> ldtr_64_ldst_unpriv
    return false;
}
pub fn decode_ldst_unscaled<T: Arm64DecodeTrait>(trans: &mut T, insn: u32) -> bool {
    let args: ArmInstr = ArmInstr {
        insn
    };
    let v = ((insn >> 26) & 1) != 0;
    let size = (insn >> 30);
    let opc = ((insn >> 22) & 3);
    if((size == 0) && !v && (opc == 0)) { return trans.sturb(args); } // -> sturb_32_ldst_unscaled
    if((size == 0) && !v && opc==1) { return trans.ldurb(args); } // -> ldurb_32_ldst_unscaled
    if((size == 0) && !v && opc==2) { return trans.ldursb(args); } // -> ldursb_64_ldst_unscaled
    if((size == 0) && !v && opc==3) { return trans.ldursb(args); } // -> ldursb_32_ldst_unscaled
    if((size == 0) && v && (opc == 0)) { return trans.stur_fpsimd(args); } // -> stur_b_ldst_unscaled
    if((size == 0) && v && opc==1) { return trans.ldur_fpsimd(args); } // -> ldur_b_ldst_unscaled
    if((size == 0) && v && opc==2) { return trans.stur_fpsimd(args); } // -> stur_q_ldst_unscaled
    if((size == 0) && v && opc==3) { return trans.ldur_fpsimd(args); } // -> ldur_q_ldst_unscaled
    if(size==1 && !v && (opc == 0)) { return trans.sturh(args); } // -> sturh_32_ldst_unscaled
    if(size==1 && !v && opc==1) { return trans.ldurh(args); } // -> ldurh_32_ldst_unscaled
    if(size==1 && !v && opc==2) { return trans.ldursh(args); } // -> ldursh_64_ldst_unscaled
    if(size==1 && !v && opc==3) { return trans.ldursh(args); } // -> ldursh_32_ldst_unscaled
    if(size==1 && v && (opc == 0)) { return trans.stur_fpsimd(args); } // -> stur_h_ldst_unscaled
    if(size==1 && v && opc==1) { return trans.ldur_fpsimd(args); } // -> ldur_h_ldst_unscaled
    if(size==2 && !v && (opc == 0)) { return trans.stur_gen(args); } // -> stur_32_ldst_unscaled
    if(size==2 && !v && opc==1) { return trans.ldur_gen(args); } // -> ldur_32_ldst_unscaled
    if(size==2 && !v && opc==2) { return trans.ldursw(args); } // -> ldursw_64_ldst_unscaled
    if(size==2 && v && (opc == 0)) { return trans.stur_fpsimd(args); } // -> stur_s_ldst_unscaled
    if(size==2 && v && opc==1) { return trans.ldur_fpsimd(args); } // -> ldur_s_ldst_unscaled
    if(size==3 && !v && (opc == 0)) { return trans.stur_gen(args); } // -> stur_64_ldst_unscaled
    if(size==3 && !v && opc==1) { return trans.ldur_gen(args); } // -> ldur_64_ldst_unscaled
    if(size==3 && !v && opc==2) { return trans.prfum(args); } // -> prfum_p_ldst_unscaled
    if(size==3 && v && (opc == 0)) { return trans.stur_fpsimd(args); } // -> stur_d_ldst_unscaled
    if(size==3 && v && opc==1) { return trans.ldur_fpsimd(args); } // -> ldur_d_ldst_unscaled
    return false;
}
// load/store register (unsigned immediate)
pub fn decode_ldst_pos<T: Arm64DecodeTrait>(trans: &mut T, insn: u32) -> bool {
    let args: ArmInstr = ArmInstr {
        insn
    };
    let v = ((insn >> 26) & 1) != 0;
    let size = (insn >> 30);
    let opc = ((insn >> 22) & 3);
    if((size == 0) && !v && (opc == 0)) { return trans.strb_imm(args); } // -> strb_32_ldst_pos
    if((size == 0) && !v && opc==1) { return trans.ldrb_imm(args); } // -> ldrb_32_ldst_pos
    if((size == 0) && !v && opc==2) { return trans.ldrsb_imm(args); } // -> ldrsb_64_ldst_pos
    if((size == 0) && !v && opc==3) { return trans.ldrsb_imm(args); } // -> ldrsb_32_ldst_pos
    if((size == 0) && v && (opc == 0)) { return trans.str_imm_fpsimd(args); } // -> str_b_ldst_pos
    if((size == 0) && v && opc==1) { return trans.ldr_imm_fpsimd(args); } // -> ldr_b_ldst_pos
    if((size == 0) && v && opc==2) { return trans.str_imm_fpsimd(args); } // -> str_q_ldst_pos
    if((size == 0) && v && opc==3) { return trans.ldr_imm_fpsimd(args); } // -> ldr_q_ldst_pos
    if(size==1 && !v && (opc == 0)) { return trans.strh_imm(args); } // -> strh_32_ldst_pos
    if(size==1 && !v && opc==1) { return trans.ldrh_imm(args); } // -> ldrh_32_ldst_pos
    if(size==1 && !v && opc==2) { return trans.ldrsh_imm(args); } // -> ldrsh_64_ldst_pos
    if(size==1 && !v && opc==3) { return trans.ldrsh_imm(args); } // -> ldrsh_32_ldst_pos
    if(size==1 && v && (opc == 0)) { return trans.str_imm_fpsimd(args); } // -> str_h_ldst_pos
    if(size==1 && v && opc==1) { return trans.ldr_imm_fpsimd(args); } // -> ldr_h_ldst_pos
    if(size==2 && !v && (opc == 0)) { return trans.str_imm_gen(args); } // -> str_32_ldst_pos
    if(size==2 && !v && opc==1) { return trans.ldr_imm_gen(args); } // -> ldr_32_ldst_pos
    if(size==2 && !v && opc==2) { return trans.ldrsw_imm(args); } // -> ldrsw_64_ldst_pos
    if(size==2 && v && (opc == 0)) { return trans.str_imm_fpsimd(args); } // -> str_s_ldst_pos
    if(size==2 && v && opc==1) { return trans.ldr_imm_fpsimd(args); } // -> ldr_s_ldst_pos
    if(size==3 && !v && (opc == 0)) { return trans.str_imm_gen(args); } // -> str_64_ldst_pos
    if(size==3 && !v && opc==1) { return trans.ldr_imm_gen(args); } // -> ldr_64_ldst_pos
    if(size==3 && !v && opc==2) { return trans.prfm_imm(args); } // -> prfm_p_ldst_pos
    if(size==3 && v && (opc == 0)) { return trans.str_imm_fpsimd(args); } // -> str_d_ldst_pos
    if(size==3 && v && opc==1) { return trans.ldr_imm_fpsimd(args); } // -> ldr_d_ldst_pos
    return false;
}
pub fn decode_ldstpair_off<T: Arm64DecodeTrait>(trans: &mut T, insn: u32) -> bool {
    let args: ArmInstr = ArmInstr {
        insn
    };
    let v = ((insn >> 26) & 1) != 0;
    let opc = (insn >> 30);
    let l = ((insn >> 22) & 1) != 0;
    if((opc == 0) && !v && !l) { return trans.stp_gen(args); } // -> stp_32_ldstpair_off
    if((opc == 0) && !v && l) { return trans.ldp_gen(args); } // -> ldp_32_ldstpair_off
    if((opc == 0) && v && !l) { return trans.stp_fpsimd(args); } // -> stp_s_ldstpair_off
    if((opc == 0) && v && l) { return trans.ldp_fpsimd(args); } // -> ldp_s_ldstpair_off
    if(opc==1 && !v && !l && trans.has_mte_feat()) { return trans.stgp(args); } // -> stgp_64_ldstpair_off
    if(opc==1 && !v && l) { return trans.ldpsw(args); } // -> ldpsw_64_ldstpair_off
    if(opc==1 && v && !l) { return trans.stp_fpsimd(args); } // -> stp_d_ldstpair_off
    if(opc==1 && v && l) { return trans.ldp_fpsimd(args); } // -> ldp_d_ldstpair_off
    if(opc==2 && !v && !l) { return trans.stp_gen(args); } // -> stp_64_ldstpair_off
    if(opc==2 && !v && l) { return trans.ldp_gen(args); } // -> ldp_64_ldstpair_off
    if(opc==2 && v && !l) { return trans.stp_fpsimd(args); } // -> stp_q_ldstpair_off
    if(opc==2 && v && l) { return trans.ldp_fpsimd(args); } // -> ldp_q_ldstpair_off
    return false;
}
pub fn decode_ldstpair_post<T: Arm64DecodeTrait>(trans: &mut T, insn: u32) -> bool {
    let args: ArmInstr = ArmInstr {
        insn
    };
    let v = ((insn >> 26) & 1) != 0;
    let opc = (insn >> 30);
    let l = ((insn >> 22) & 1) != 0;
    if((opc == 0) && !v && !l) { return trans.stp_gen(args); } // -> stp_32_ldstpair_post
    if((opc == 0) && !v && l) { return trans.ldp_gen(args); } // -> ldp_32_ldstpair_post
    if((opc == 0) && v && !l) { return trans.stp_fpsimd(args); } // -> stp_s_ldstpair_post
    if((opc == 0) && v && l) { return trans.ldp_fpsimd(args); } // -> ldp_s_ldstpair_post
    if(opc==1 && !v && !l && trans.has_mte_feat()) { return trans.stgp(args); } // -> stgp_64_ldstpair_post
    if(opc==1 && !v && l) { return trans.ldpsw(args); } // -> ldpsw_64_ldstpair_post
    if(opc==1 && v && !l) { return trans.stp_fpsimd(args); } // -> stp_d_ldstpair_post
    if(opc==1 && v && l) { return trans.ldp_fpsimd(args); } // -> ldp_d_ldstpair_post
    if(opc==2 && !v && !l) { return trans.stp_gen(args); } // -> stp_64_ldstpair_post
    if(opc==2 && !v && l) { return trans.ldp_gen(args); } // -> ldp_64_ldstpair_post
    if(opc==2 && v && !l) { return trans.stp_fpsimd(args); } // -> stp_q_ldstpair_post
    if(opc==2 && v && l) { return trans.ldp_fpsimd(args); } // -> ldp_q_ldstpair_post
    return false;
}
pub fn decode_ldstpair_pre<T: Arm64DecodeTrait>(trans: &mut T, insn: u32) -> bool {
    let args: ArmInstr = ArmInstr {
        insn
    };
    let v = ((insn >> 26) & 1) != 0;
    let opc = (insn >> 30);
    let l = ((insn >> 22) & 1) != 0;
    if((opc == 0) && !v && !l) { return trans.stp_gen(args); } // -> stp_32_ldstpair_pre
    if((opc == 0) && !v && l) { return trans.ldp_gen(args); } // -> ldp_32_ldstpair_pre
    if((opc == 0) && v && !l) { return trans.stp_fpsimd(args); } // -> stp_s_ldstpair_pre
    if((opc == 0) && v && l) { return trans.ldp_fpsimd(args); } // -> ldp_s_ldstpair_pre
    if(opc==1 && !v && !l && trans.has_mte_feat()) { return trans.stgp(args); } // -> stgp_64_ldstpair_pre
    if(opc==1 && !v && l) { return trans.ldpsw(args); } // -> ldpsw_64_ldstpair_pre
    if(opc==1 && v && !l) { return trans.stp_fpsimd(args); } // -> stp_d_ldstpair_pre
    if(opc==1 && v && l) { return trans.ldp_fpsimd(args); } // -> ldp_d_ldstpair_pre
    if(opc==2 && !v && !l) { return trans.stp_gen(args); } // -> stp_64_ldstpair_pre
    if(opc==2 && !v && l) { return trans.ldp_gen(args); } // -> ldp_64_ldstpair_pre
    if(opc==2 && v && !l) { return trans.stp_fpsimd(args); } // -> stp_q_ldstpair_pre
    if(opc==2 && v && l) { return trans.ldp_fpsimd(args); } // -> ldp_q_ldstpair_pre
    return false;
}
pub fn decode_addsub_imm<T: Arm64DecodeTrait>(trans: &mut T, insn: u32) -> bool {
    let args: ArmInstr = ArmInstr {
        insn
    };
    let sf = (insn >> 31) != 0;
    let op = ((insn >> 30) & 1) != 0;
    let s = ((insn >> 29) & 1) != 0;
    if(!sf && !op && !s) { return trans.add_addsub_imm(args); } // -> add_32_addsub_imm
    if(!sf && !op && s) { return trans.adds_addsub_imm(args); } // -> adds_32s_addsub_imm
    if(!sf && op && !s) { return trans.sub_addsub_imm(args); } // -> sub_32_addsub_imm
    if(!sf && op && s) { return trans.subs_addsub_imm(args); } // -> subs_32s_addsub_imm
    if(sf && !op && !s) { return trans.add_addsub_imm(args); } // -> add_64_addsub_imm
    if(sf && !op && s) { return trans.adds_addsub_imm(args); } // -> adds_64s_addsub_imm
    if(sf && op && !s) { return trans.sub_addsub_imm(args); } // -> sub_64_addsub_imm
    if(sf && op && s) { return trans.subs_addsub_imm(args); } // -> subs_64s_addsub_imm
    return false;
}
pub fn decode_addsub_immtags<T: Arm64DecodeTrait>(trans: &mut T, insn: u32) -> bool {
    return false;
}
pub fn decode_bitfield<T: Arm64DecodeTrait>(trans: &mut T, insn: u32) -> bool {
    let args: ArmInstr = ArmInstr {
        insn
    };
    let sf = (insn >> 31) != 0;
    let opc = (insn >> 29) & 3;
    let n = ((insn >> 22) & 1) != 0;
    if(!sf && (opc == 0) && !n) { return trans.sbfm(args); } // -> sbfm_32m_bitfield
    if(!sf && opc==1 && !n) { return trans.bfm(args); } // -> bfm_32m_bitfield
    if(!sf && opc==2 && !n) { return trans.ubfm(args); } // -> ubfm_32m_bitfield
    if(sf && (opc == 0) && n) { return trans.sbfm(args); } // -> sbfm_64m_bitfield
    if(sf && opc==1 && n) { return trans.bfm(args); } // -> bfm_64m_bitfield
    if(sf && opc==2 && n) { return trans.ubfm(args); } // -> ubfm_64m_bitfield
    return false;
}
pub fn decode_extract<T: Arm64DecodeTrait>(trans: &mut T, insn: u32) -> bool {
    let args: ArmInstr = ArmInstr {
        insn
    };
    let sf = (insn >> 31) != 0;
    let op21 = (insn >> 29) & 3;
    let n = ((insn >> 22) & 1) != 0;
    let o0 = ((insn >> 21) & 1) != 0;
    let imms = (insn >> 10) & 0x3f;

    if(!sf && (op21 == 0) && !n && !o0 && ((imms&0x20) == 0)) { return trans.extr(args); } // -> extr_32_extract
    if(sf && (op21 == 0) && n && !o0) { return trans.extr(args); } // -> extr_64_extract
    return false;
}
pub fn decode_log_imm<T: Arm64DecodeTrait>(trans: &mut T, insn: u32) -> bool {
    let args: ArmInstr = ArmInstr {
        insn
    };
    let sf = (insn >> 31) != 0;
    let opc = (insn >> 29) & 3;
    let n = ((insn >> 21) & 1) != 0;
    if(!sf && (opc == 0) && !n) { return trans.and_log_imm(args); } // -> and_32_log_imm
    if(!sf && opc==1 && !n) { return trans.orr_log_imm(args); } // -> orr_32_log_imm
    if(!sf && opc==2 && !n) { return trans.eor_log_imm(args); } // -> eor_32_log_imm
    if(!sf && opc==3 && !n) { return trans.ands_log_imm(args); } // -> ands_32s_log_imm
    if(sf && (opc == 0)) { return trans.and_log_imm(args); } // -> and_64_log_imm
    if(sf && opc==1) { return trans.orr_log_imm(args); } // -> orr_64_log_imm
    if(sf && opc==2) { return trans.eor_log_imm(args); } // -> eor_64_log_imm
    if(sf && opc==3) { return trans.ands_log_imm(args); } // -> ands_64s_log_imm
    return false;
}
pub fn decode_movewide<T: Arm64DecodeTrait>(trans: &mut T, insn: u32) -> bool {
    let args: ArmInstr = ArmInstr {
        insn
    };
    let sf = (insn >> 31) != 0;
    let opc = (insn >> 29) & 3;
    let hw = (insn >> 21) & 3;
    if(!sf && (opc == 0) && ((hw&2) == 0)) { return trans.movn(args); } // -> movn_32_movewide
    if(!sf && opc==2 && ((hw&2) == 0)) { return trans.movz(args); } // -> movz_32_movewide
    if(!sf && opc==3 && ((hw&2) == 0)) { return trans.movk(args); } // -> movk_32_movewide
    if(sf && (opc == 0)) { return trans.movn(args); } // -> movn_64_movewide
    if(sf && opc==2) { return trans.movz(args); } // -> movz_64_movewide
    if(sf && opc==3) { return trans.movk(args); } // -> movk_64_movewide
    return false;
}
pub fn decode_addsub_ext<T: Arm64DecodeTrait>(trans: &mut T, insn: u32) -> bool {
    let args: ArmInstr = ArmInstr {
        insn
    };
    let sf = (insn >> 31) != 0;
    let op = ((insn >> 30) & 1) != 0;
    let s = ((insn >> 29) & 1) != 0;
    let opt = ((insn >> 22) & 3);
    if(!sf && !op && !s && (opt == 0)) { return trans.add_addsub_ext(args); } // -> add_32_addsub_ext
    if(!sf && !op && s && (opt == 0)) { return trans.adds_addsub_ext(args); } // -> adds_32s_addsub_ext
    if(!sf && op && !s && (opt == 0)) { return trans.sub_addsub_ext(args); } // -> sub_32_addsub_ext
    if(!sf && op && s && (opt == 0)) { return trans.subs_addsub_ext(args); } // -> subs_32s_addsub_ext
    if(sf && !op && !s && (opt == 0)) { return trans.add_addsub_ext(args); } // -> add_64_addsub_ext
    if(sf && !op && s && (opt == 0)) { return trans.adds_addsub_ext(args); } // -> adds_64s_addsub_ext
    if(sf && op && !s && (opt == 0)) { return trans.sub_addsub_ext(args); } // -> sub_64_addsub_ext
    if(sf && op && s && (opt == 0)) { return trans.subs_addsub_ext(args); } // -> subs_64s_addsub_ext
    return false;
}
pub fn decode_addsub_shift<T: Arm64DecodeTrait>(trans: &mut T, insn: u32) -> bool {
    let args: ArmInstr = ArmInstr {
        insn
    };
    let sf = (insn >> 31) != 0;
    let op = ((insn >> 30) & 1) != 0;
    let s = ((insn >> 29) & 1) != 0;
    if(!sf && !op && !s) { return trans.add_addsub_shift(args); } // -> add_32_addsub_shift
    if(!sf && !op && s) { return trans.adds_addsub_shift(args); } // -> adds_32_addsub_shift
    if(!sf && op && !s) { return trans.sub_addsub_shift(args); } // -> sub_32_addsub_shift
    if(!sf && op && s) { return trans.subs_addsub_shift(args); } // -> subs_32_addsub_shift
    if(sf && !op && !s) { return trans.add_addsub_shift(args); } // -> add_64_addsub_shift
    if(sf && !op && s) { return trans.adds_addsub_shift(args); } // -> adds_64_addsub_shift
    if(sf && op && !s) { return trans.sub_addsub_shift(args); } // -> sub_64_addsub_shift
    if(sf && op && s) { return trans.subs_addsub_shift(args); } // -> subs_64_addsub_shift
    return false;
}
pub fn decode_addsub_carry<T: Arm64DecodeTrait>(trans: &mut T, insn: u32) -> bool {
    let args: ArmInstr = ArmInstr {
        insn
    };
    let sf = (insn >> 31) != 0;
    let op = ((insn >> 30) & 1) != 0;
    let s = ((insn >> 29) & 1) != 0;
    if(!sf && !op && !s) { return trans.adc(args); } // -> adc_32_addsub_carry
    if(!sf && !op && s) { return trans.adcs(args); } // -> adcs_32_addsub_carry
    if(!sf && op && !s) { return trans.sbc(args); } // -> sbc_32_addsub_carry
    if(!sf && op && s) { return trans.sbcs(args); } // -> sbcs_32_addsub_carry
    if(sf && !op && !s) { return trans.adc(args); } // -> adc_64_addsub_carry
    if(sf && !op && s) { return trans.adcs(args); } // -> adcs_64_addsub_carry
    if(sf && op && !s) { return trans.sbc(args); } // -> sbc_64_addsub_carry
    if(sf && op && s) { return trans.sbcs(args); } // -> sbcs_64_addsub_carry
    return false;
}
pub fn decode_condcmp_imm<T: Arm64DecodeTrait>(trans: &mut T, insn: u32) -> bool {
    let args: ArmInstr = ArmInstr {
        insn
    };
    let sf = (insn >> 31) != 0;
    let op = ((insn >> 30) & 1) != 0;
    let s = ((insn >> 29) & 1) != 0;
    let o2 = ((insn>>10)&0x1) != 0;
    let o3 = ((insn>>4)&0x1) != 0;

    if(!sf && !op && s && !o2 && !o3) { return trans.ccmn_imm(args); } // -> ccmn_32_condcmp_reg
    if(!sf && op && s && !o2 && !o3) { return trans.ccmp_imm(args); } // -> ccmp_32_condcmp_reg
    if(sf && !op && s && !o2 && !o3) { return trans.ccmn_imm(args); } // -> ccmn_64_condcmp_reg
    if(sf && op && s && !o2 && !o3) { return trans.ccmp_imm(args); } // -> ccmp_64_condcmp_reg

    return false;
}
pub fn decode_condcmp_reg<T: Arm64DecodeTrait>(trans: &mut T, insn: u32) -> bool {
    let args: ArmInstr = ArmInstr {
        insn
    };
    let sf = (insn >> 31) != 0;
    let op = ((insn >> 30) & 1) != 0;
    let s = ((insn >> 29) & 1) != 0;
    let o2 = ((insn>>10)&0x1) != 0;
    let o3 = ((insn>>4)&0x1) != 0;

    if(!sf && !op && s && (o2 == false) && !o3) { return trans.ccmn_reg(args); } // -> ccmn_32_condcmp_reg
    if(!sf && op && s && (o2 == false) && !o3) { return trans.ccmp_reg(args); } // -> ccmp_32_condcmp_reg
    if(sf && !op && s && (o2 == false) && !o3) { return trans.ccmn_reg(args); } // -> ccmn_64_condcmp_reg
    if(sf && op && s && (o2 == false) && !o3) { return trans.ccmp_reg(args); } // -> ccmp_64_condcmp_reg

    return false;
}
pub fn decode_condsel<T: Arm64DecodeTrait>(trans: &mut T, insn: u32) -> bool {
    let args: ArmInstr = ArmInstr {
        insn
    };
    let sf = (insn >> 31) != 0;
    let op = ((insn >> 30) & 1) != 0;
    let s = ((insn >> 29) & 1) != 0;
    let op2 = (insn>>10)&0x3;
    if(!sf && !op && !s && (op2 == 0)) { return trans.csel(args); } // -> csel_32_condsel
    if(!sf && !op && !s && op2==1) { return trans.csinc(args); } // -> csinc_32_condsel
    if(!sf && op && !s && (op2 == 0)) { return trans.csinv(args); } // -> csinv_32_condsel
    if(!sf && op && !s && op2==1) { return trans.csneg(args); } // -> csneg_32_condsel
    if(sf && !op && !s && (op2 == 0)) { return trans.csel(args); } // -> csel_64_condsel
    if(sf && !op && !s && op2==1) { return trans.csinc(args); } // -> csinc_64_condsel
    if(sf && op && !s && (op2 == 0)) { return trans.csinv(args); } // -> csinv_64_condsel
    if(sf && op && !s && op2==1) { return trans.csneg(args); } // -> csneg_64_condsel
    return false;
}
pub fn decode_dp_1src<T: Arm64DecodeTrait>(trans: &mut T, insn: u32) -> bool {
    let args: ArmInstr = ArmInstr {
        insn
    };
    let sf = (insn >> 31) != 0;
    let s = ((insn >> 29) & 1) != 0;
    let opcode2 = (insn>>16)&0x1f;
    let opcode = (insn>>10)&0x3f;
    let rn = (insn>>5)&0x1f;
    if sf && !s && opcode2==1 && opcode==8 && rn==0x1f && trans.has_pauth_feat() {
        return trans.pacia(args);
    }
    if sf && !s && opcode2==1 && opcode==9 && rn==0x1f && trans.has_pauth_feat() {
        return trans.pacib(args);
    }
    if sf && !s && opcode2==1 && opcode==10 && rn==0x1f && trans.has_pauth_feat() {
        return trans.pacda(args);
    }
    if sf && !s && opcode2==1 && opcode==11 && rn==0x1f && trans.has_pauth_feat() {
        return trans.pacdb(args);
    }
    if sf && !s && opcode2==1 && opcode==12 && rn==0x1f && trans.has_pauth_feat() {
        return trans.autia(args);
    }
    if sf && !s && opcode2==1 && opcode==13 && rn==0x1f && trans.has_pauth_feat() {
        return trans.autib(args);
    }
    if sf && !s && opcode2==1 && opcode==14 && rn==0x1f && trans.has_pauth_feat() {
        return trans.autda(args);
    }
    if sf && !s && opcode2==1 && opcode==15 && rn==0x1f && trans.has_pauth_feat() {
        return trans.autdb(args);
    }
    if(!sf && !s && (opcode2 == 0) && (opcode == 0)) {
        return trans.rbit_int(args);
    }
    if(!sf && !s && (opcode2 == 0) && opcode==1) {
        return trans.rev16_int(args);
    }
    if(!sf && !s && (opcode2 == 0) && opcode==2) { return trans.rev(args); } // -> rev_32_dp_1src
    if(!sf && !s && (opcode2 == 0) && opcode==4) { return trans.clz_int(args); } // -> clz_32_dp_1src
    if(!sf && !s && (opcode2 == 0) && opcode==5) { return trans.cls_int(args); } // -> cls_32_dp_1src
    if(sf && !s && (opcode2 == 0) && (opcode == 0)) { return trans.rbit_int(args); } // -> rbit_64_dp_1src
    if(sf && !s && (opcode2 == 0) && opcode==1) { return trans.rev16_int(args); } // -> rev16_64_dp_1src
    if(sf && !s && (opcode2 == 0) && opcode==2) { return trans.rev32_int(args); } // -> rev32_64_dp_1src
    if(sf && !s && (opcode2 == 0) && opcode==3) { return trans.rev(args); } // -> rev_64_dp_1src
    if(sf && !s && (opcode2 == 0) && opcode==4) { return trans.clz_int(args); } // -> clz_64_dp_1src
    if(sf && !s && (opcode2 == 0) && opcode==5) { return trans.cls_int(args); } // -> cls_64_dp_1src
    if(sf && !s && opcode2==1 && (opcode == 0) && trans.has_pauth_feat()) { return trans.pacia(args); } // -> pacia_64p_dp_1src
    if(sf && !s && opcode2==1 && opcode==1 && trans.has_pauth_feat()) { return trans.pacib(args); } // -> pacib_64p_dp_1src
    if(sf && !s && opcode2==1 && opcode==2 && trans.has_pauth_feat()) { return trans.pacda(args); } // -> pacda_64p_dp_1src
    if(sf && !s && opcode2==1 && opcode==3 && trans.has_pauth_feat()) { return trans.pacdb(args); } // -> pacdb_64p_dp_1src
    if(sf && !s && opcode2==1 && opcode==4 && trans.has_pauth_feat()) { return trans.autia(args); } // -> autia_64p_dp_1src
    if(sf && !s && opcode2==1 && opcode==5 && trans.has_pauth_feat()) { return trans.autib(args); } // -> autib_64p_dp_1src
    if(sf && !s && opcode2==1 && opcode==6 && trans.has_pauth_feat()) { return trans.autda(args); } // -> autda_64p_dp_1src
    if(sf && !s && opcode2==1 && opcode==7 && trans.has_pauth_feat()) { return trans.autdb(args); } // -> autdb_64p_dp_1src
    return false;
}
pub fn decode_dp_2src<T: Arm64DecodeTrait>(trans: &mut T, insn: u32) -> bool {
    let args: ArmInstr = ArmInstr {
        insn
    };
    let sf = (insn >> 31) != 0;
    let s = ((insn >> 29) & 1) != 0;
    let opcode = (insn>>10)&0x3f;
    if(!sf && !s && opcode==2) { return trans.udiv(args); } // -> udiv_32_dp_2src
    if(!sf && !s && opcode==3) { return trans.sdiv(args); } // -> sdiv_32_dp_2src
    if(!sf && !s && opcode==8) { return trans.lslv(args); } // -> lslv_32_dp_2src
    if(!sf && !s && opcode==9) { return trans.lsrv(args); } // -> lsrv_32_dp_2src
    if(!sf && !s && opcode==10) { return trans.asrv(args); } // -> asrv_32_dp_2src
    if(!sf && !s && opcode==11) { return trans.rorv(args); } // -> rorv_32_dp_2src

    if(!sf && !s && opcode==0x11) { return trans.crc32(args); } // -> crc32h_32c_dp_2src
    if(!sf && !s && opcode==0x12) { return trans.crc32(args); } // -> crc32w_32c_dp_2src
    if(!sf && !s && opcode==0x14) { return trans.crc32c(args); } // -> crc32cb_32c_dp_2src
    if(!sf && !s && opcode==0x15) { return trans.crc32c(args); } // -> crc32ch_32c_dp_2src
    if(!sf && !s && opcode==0x16) { return trans.crc32c(args); } // -> crc32cw_32c_dp_2src

    if sf && !s && (opcode == 0) && trans.has_mte_feat() { return trans.subp(args); } // -> subp_64s_dp_2src
    if(sf && !s && opcode==2) { return trans.udiv(args); } // -> udiv_64_dp_2src
    if(sf && !s && opcode==3) { return trans.sdiv(args); } // -> sdiv_64_dp_2src
    if(sf && !s && opcode==4 && trans.has_mte_feat()) { return trans.irg(args); } // -> irg_64i_dp_2src
    if(sf && !s && opcode==5 && trans.has_mte_feat()) { return trans.gmi(args); } // -> gmi_64g_dp_2src
    if(sf && !s && opcode==8) { return trans.lslv(args); } // -> lslv_64_dp_2src
    if(sf && !s && opcode==9) { return trans.lsrv(args); } // -> lsrv_64_dp_2src
    if(sf && !s && opcode==10) { return trans.asrv(args); } // -> asrv_64_dp_2src
    if(sf && !s && opcode==11) { return trans.rorv(args); } // -> rorv_64_dp_2src
    if(sf && !s && opcode==12 && trans.has_pauth_feat()) { return trans.pacga(args); } // -> pacga_64p_dp_2src
    if(sf && !s && opcode==0x13) { return trans.crc32(args); } // -> crc32x_64c_dp_2src
    if(sf && !s && opcode==0x17) { return trans.crc32c(args); } // -> crc32cx_64c_dp_2src
    if(sf && s && (opcode == 0) && trans.has_mte_feat()) { return trans.subps(args); } // -> subps_64s_dp_2src
    return false;
}
pub fn decode_dp_3src<T: Arm64DecodeTrait>(trans: &mut T, insn: u32) -> bool {
    let sf = (insn >> 31) != 0;
    let op54 = ((insn >> 29) & 3);
    let op31 = ((insn >> 21) & 7);
    let o0 = ((insn >> 15) & 1) != 0;

    let args: ArmInstr = ArmInstr {
        insn
    };
    if(!sf && (op54 == 0) && (op31 == 0) && !o0) { return trans.madd(args); } // -> madd_32a_dp_3src
    if(!sf && (op54 == 0) && (op31 == 0) && o0) { return trans.msub(args); } // -> msub_32a_dp_3src
    if(sf && (op54 == 0) && (op31 == 0) && !o0) { return trans.madd(args); } // -> madd_64a_dp_3src
    if(sf && (op54 == 0) && (op31 == 0) && o0) { return trans.msub(args); } // -> msub_64a_dp_3src
    if(sf && (op54 == 0) && op31==1 && !o0) { return trans.smaddl(args); } // -> smaddl_64wa_dp_3src
    if(sf && (op54 == 0) && op31==1 && o0) { return trans.smsubl(args); } // -> smsubl_64wa_dp_3src
    if(sf && (op54 == 0) && op31==2 && !o0) { return trans.smulh(args); } // -> smulh_64_dp_3src
    if(sf && (op54 == 0) && op31==5 && !o0) { return trans.umaddl(args); } // -> umaddl_64wa_dp_3src
    if(sf && (op54 == 0) && op31==5 && o0) { return trans.umsubl(args); } // -> umsubl_64wa_dp_3src
    if(sf && (op54 == 0) && op31==6 && !o0) { return trans.umulh(args); } // -> umulh_64_dp_3src
    return false;
}
pub fn decode_setf<T: Arm64DecodeTrait>(trans: &mut T, insn: u32) -> bool {
    return false;
}
pub fn decode_log_shift<T: Arm64DecodeTrait>(trans: &mut T, insn: u32) -> bool {
    let sf = (insn >> 31) != 0;
    let opc = ((insn >> 29) & 3);
    // let imm6 = ((insn >> 10) & 0x3f);
    let n = ((insn >> 21) & 1) != 0;
    let args: ArmInstr = ArmInstr {
        insn
    };
    if(!sf && (opc == 0) && !n) { return trans.and_log_shift(args); } // -> and_32_log_shift
    if(!sf && (opc == 0) && n) { return trans.bic_log_shift(args); } // -> bic_32_log_shift
    if(!sf && opc==1 && !n) { return trans.orr_log_shift(args); } // -> orr_32_log_shift
    if(!sf && opc==1 && n) { return trans.orn_log_shift(args); } // -> orn_32_log_shift
    if(!sf && opc==2 && !n) { return trans.eor_log_shift(args); } // -> eor_32_log_shift
    if(!sf && opc==2 && n) { return trans.eon(args); } // -> eon_32_log_shift
    if(!sf && opc==3 && !n) { return trans.ands_log_shift(args); } // -> ands_32_log_shift
    if(!sf && opc==3 && n) { return trans.bics(args); } // -> bics_32_log_shift
    if(sf && (opc == 0) && !n) { return trans.and_log_shift(args); } // -> and_64_log_shift
    if(sf && (opc == 0) && n) { return trans.bic_log_shift(args); } // -> bic_64_log_shift
    if(sf && opc==1 && !n) { return trans.orr_log_shift(args); } // -> orr_64_log_shift
    if(sf && opc==1 && n) { return trans.orn_log_shift(args); } // -> orn_64_log_shift
    if(sf && opc==2 && !n) { return trans.eor_log_shift(args); } // -> eor_64_log_shift
    if(sf && opc==2 && n) { return trans.eon(args); } // -> eon_64_log_shift
    if(sf && opc==3 && !n) { return trans.ands_log_shift(args); } // -> ands_64_log_shift
    if(sf && opc==3 && n) { return trans.bics(args); } // -> bics_64_log_shift
    return false;
}
pub fn decode_rmif<T: Arm64DecodeTrait>(trans: &mut T, insn: u32) -> bool {
    let args: ArmInstr = ArmInstr {
        insn
    };
    let sf = ((insn >> 31) & 1) != 0;
    let op = ((insn >> 30) & 1) != 0;
    let s = ((insn >> 29) & 1) != 0;
    let o2 = ((insn >> 4) & 1) != 0;

    if(sf && !op && s && !o2 && trans.has_flagm()) { return trans.rmif(args); } // -> rmif_only_rmif
    return false;
}
pub fn decode_asimdall<T: Arm64DecodeTrait>(trans: &mut T, insn: u32) -> bool {
    let args: ArmInstr = ArmInstr {
        insn
    };
    let u = ((insn >> 29) & 1) != 0;
    let size = ((insn >> 22) & 3);
    let opcode = ((insn >> 12) & 0x1f);
    
    if(!u && (size == 0) && opcode==12 && trans.has_fp16()) { return trans.fmaxnmv_advsimd(args); } // -> fmaxnmv_asimdall_only_h
    if(!u && (size == 0) && opcode==15 && trans.has_fp16()) { return trans.fmaxv_advsimd(args); } // -> fmaxv_asimdall_only_h
    if(!u && size==2 && opcode==12 && trans.has_fp16()) { return trans.fminnmv_advsimd(args); } // -> fminnmv_asimdall_only_h
    if(!u && size==2 && opcode==15 && trans.has_fp16()) { return trans.fminv_advsimd(args); } // -> fminv_asimdall_only_h
    if(u && ((size&2) == 0) && opcode==12) { return trans.fmaxnmv_advsimd(args); } // -> fmaxnmv_asimdall_only_sd
    if(u && ((size&2) == 0) && opcode==15) { return trans.fmaxv_advsimd(args); } // -> fmaxv_asimdall_only_sd
    if(u && (size&2)==2 && opcode==12) { return trans.fminnmv_advsimd(args); } // -> fminnmv_asimdall_only_sd
    if(u && (size&2)==2 && opcode==15) { return trans.fminv_advsimd(args); } // -> fminv_asimdall_only_sd
    if(!u && opcode==3) { return trans.saddlv_advsimd(args); } // -> saddlv_asimdall_only
    if(!u && opcode==10) { return trans.smaxv_advsimd(args); } // -> smaxv_asimdall_only
    if(!u && opcode==0x1a) { return trans.sminv_advsimd(args); } // -> sminv_asimdall_only
    if(!u && opcode==0x1b) { return trans.addv_advsimd(args); } // -> addv_asimdall_only
    if(u && opcode==3) { return trans.uaddlv_advsimd(args); } // -> uaddlv_asimdall_only
    if(u && opcode==10) { return trans.umaxv_advsimd(args); } // -> umaxv_asimdall_only
    if(u && opcode==0x1a) { return trans.uminv_advsimd(args); } // -> uminv_asimdall_only
    return false;
}
pub fn decode_asimdins<T: Arm64DecodeTrait>(trans: &mut T, insn: u32) -> bool {
    let args: ArmInstr = ArmInstr {
        insn
    };
    let q = ((insn >> 30) & 1) != 0;
    let op = ((insn >> 29) & 1) != 0;
    let imm4 = ((insn >> 11) & 15);
    let imm5 = ((insn >> 16) & 0x1f);

    if(q && !op && (imm5&15)==8 && imm4==7) { return trans.umov_advsimd(args); } // -> umov_asimdins_x_x
    if(!q && !op && imm4==5) { return trans.smov_advsimd(args); } // -> smov_asimdins_w_w
    if(!q && !op && imm4==7) { return trans.umov_advsimd(args); } // -> umov_asimdins_w_w
    if(q && !op && imm4==3) { return trans.ins_advsimd_gen(args); } // -> ins_asimdins_ir_r
    if(q && !op && imm4==5) { return trans.smov_advsimd(args); } // -> smov_asimdins_x_x
    if(!op && imm4==0) { return trans.dup_advsimd_elt(args); } // -> dup_asimdins_dv_v
    if(!op && imm4==1) { return trans.dup_advsimd_gen(args); } // -> dup_asimdins_dr_r
    if(q && op) { return trans.ins_advsimd_elt(args); } // -> ins_asimdins_iv_v

    return false;
}
pub fn decode_asimdext<T: Arm64DecodeTrait>(trans: &mut T, insn: u32) -> bool {
    let args: ArmInstr = ArmInstr {
        insn
    };
    let op2 = ((insn >> 22) & 3);;
    if(op2 == 0) { return trans.ext_advsimd(args); } // -> ext_asimdext_only
    return false;
}
pub fn decode_asimdimm<T: Arm64DecodeTrait>(trans: &mut T, insn: u32) -> bool {
    let args: ArmInstr = ArmInstr {
        insn
    };
    let q = ((insn >> 30) & 1) != 0;
    let op = ((insn >> 29) & 1) != 0;
    let cmode = ((insn >> 12) & 15);
    let o2 = ((insn >> 11) & 1);

    if(!q && op && cmode==14 && (o2 == 0)) { return trans.movi_advsimd(args); } // -> movi_asimdimm_d_ds
    if(q && op && cmode==14 && (o2 == 0)) { return trans.movi_advsimd(args); } // -> movi_asimdimm_d2_d
    if(q && op && cmode==15 && (o2 == 0)) { return trans.fmov_advsimd(args); } // -> fmov_asimdimm_d2_d
    if(!op && cmode==14 && (o2 == 0)) { return trans.movi_advsimd(args); } // -> movi_asimdimm_n_b
    if(!op && cmode==15 && (o2 == 0)) { return trans.fmov_advsimd(args); } // -> fmov_asimdimm_s_s
    if(!op && cmode==15 && (o2 != 0) && trans.has_fp16()) { return trans.fmov_advsimd(args); } // -> fmov_asimdimm_h_h
    if(!op && (cmode&13)==8 && (o2 == 0)) { return trans.movi_advsimd(args); } // -> movi_asimdimm_l_hl
    if(!op && (cmode&13)==9 && (o2 == 0)) { return trans.orr_advsimd_imm(args); } // -> orr_asimdimm_l_hl
    if(!op && (cmode&14)==12 && (o2 == 0)) { return trans.movi_advsimd(args); } // -> movi_asimdimm_m_sm
    if(op && (cmode&13)==8 && (o2 == 0)) { return trans.mvni_advsimd(args); } // -> mvni_asimdimm_l_hl
    if(op && (cmode&13)==9 && (o2 == 0)) { return trans.bic_advsimd_imm(args); } // -> bic_asimdimm_l_hl
    if(op && (cmode&14)==12 && (o2 == 0)) { return trans.mvni_advsimd(args); } // -> mvni_asimdimm_m_sm
    if(!op && ((cmode&9) == 0) && (o2 == 0)) { return trans.movi_advsimd(args); } // -> movi_asimdimm_l_sl
    if(!op && (cmode&9)==1 && (o2 == 0)) { return trans.orr_advsimd_imm(args); } // -> orr_asimdimm_l_sl
    if(op && ((cmode&9) == 0) && (o2 == 0)) { return trans.mvni_advsimd(args); } // -> mvni_asimdimm_l_sl
    if(op && (cmode&9)==1 && (o2 == 0)) { return trans.bic_advsimd_imm(args); } // -> bic_asimdimm_l_sl
    return false;
}
pub fn decode_asimdperm<T: Arm64DecodeTrait>(trans: &mut T, insn: u32) -> bool {
    let args: ArmInstr = ArmInstr {
        insn
    };
    let opcode = ((insn >> 12) & 7);
    if(opcode==1) { return trans.uzp1_advsimd(args); } // -> uzp1_asimdperm_only
    if(opcode==2) { return trans.trn1_advsimd(args); } // -> trn1_asimdperm_only
    if(opcode==3) { return trans.zip1_advsimd(args); } // -> zip1_asimdperm_only
    if(opcode==5) { return trans.uzp2_advsimd(args); } // -> uzp2_asimdperm_only
    if(opcode==6) { return trans.trn2_advsimd(args); } // -> trn2_asimdperm_only
    if(opcode==7) { return trans.zip2_advsimd(args); } // -> zip2_asimdperm_only

    return false;
}
pub fn decode_asisdone<T: Arm64DecodeTrait>(trans: &mut T, insn: u32) -> bool {
    let args: ArmInstr = ArmInstr {
        insn
    };
    let imm4 = ((insn >> 11) & 15);
    let op = ((insn >> 29) & 1) != 0;

    if(!op && (imm4 == 0)) { return trans.dup_advsimd_elt(args); } // -> dup_asisdone_only
    return false;
}
pub fn decode_asisdpair<T: Arm64DecodeTrait>(trans: &mut T, insn: u32) -> bool {
    let args: ArmInstr = ArmInstr {
        insn
    };
    let opcode = ((insn >> 12) & 0x1f);
    let size = ((insn >> 22) & 3);

    let u = ((insn >> 29) & 1) != 0;
    
    if(!u && ((size&2) == 0) && opcode==12 && trans.has_fp16()) { return trans.fmaxnmp_advsimd_pair(args); } // -> fmaxnmp_asisdpair_only_h
    if(!u && ((size&2) == 0) && opcode==13 && trans.has_fp16()) { return trans.faddp_advsimd_pair(args); } // -> faddp_asisdpair_only_h
    if(!u && ((size&2) == 0) && opcode==15 && trans.has_fp16()) { return trans.fmaxp_advsimd_pair(args); } // -> fmaxp_asisdpair_only_h
    if(!u && (size&2)==2 && opcode==12 && trans.has_fp16()) { return trans.fminnmp_advsimd_pair(args); } // -> fminnmp_asisdpair_only_h
    if(!u && (size&2)==2 && opcode==15 && trans.has_fp16()) { return trans.fminp_advsimd_pair(args); } // -> fminp_asisdpair_only_h
    if(u && ((size&2) == 0) && opcode==12) { return trans.fmaxnmp_advsimd_pair(args); } // -> fmaxnmp_asisdpair_only_sd
    if(u && ((size&2) == 0) && opcode==13) { return trans.faddp_advsimd_pair(args); } // -> faddp_asisdpair_only_sd
    if(u && ((size&2) == 0) && opcode==15) { return trans.fmaxp_advsimd_pair(args); } // -> fmaxp_asisdpair_only_sd
    if(u && (size&2)==2 && opcode==12) { return trans.fminnmp_advsimd_pair(args); } // -> fminnmp_asisdpair_only_sd
    if(u && (size&2)==2 && opcode==15) { return trans.fminp_advsimd_pair(args); } // -> fminp_asisdpair_only_sd
    if(!u && opcode==0x1b) { return trans.addp_advsimd_pair(args); } // -> addp_asisdpair_only
    return false;
}
pub fn decode_asisdshf<T: Arm64DecodeTrait>(trans: &mut T, insn: u32) -> bool {
    let args: ArmInstr = ArmInstr {
        insn
    };
    let opcode = ((insn >> 11) & 0x1f);
    let immh = ((insn >> 19) & 15);

    let u = ((insn >> 29) & 1) != 0;

    if(!u && (immh != 0) && (opcode == 0)) { return trans.sshr_advsimd(args); } // -> sshr_asisdshf_r
    if(!u && (immh != 0) && opcode==2) { return trans.ssra_advsimd(args); } // -> ssra_asisdshf_r
    if(!u && (immh != 0) && opcode==4) { return trans.srshr_advsimd(args); } // -> srshr_asisdshf_r
    if(!u && (immh != 0) && opcode==6) { return trans.srsra_advsimd(args); } // -> srsra_asisdshf_r
    if(!u && (immh != 0) && opcode==10) { return trans.shl_advsimd(args); } // -> shl_asisdshf_r
    if(!u && (immh != 0) && opcode==14) { return trans.sqshl_advsimd_imm(args); } // -> sqshl_asisdshf_r
    if(!u && (immh != 0) && opcode==0x12) { return trans.sqshrn_advsimd(args); } // -> sqshrn_asisdshf_n
    if(!u && (immh != 0) && opcode==0x13) { return trans.sqrshrn_advsimd(args); } // -> sqrshrn_asisdshf_n
    if(!u && (immh != 0) && opcode==0x1c) { return trans.scvtf_advsimd_fix(args); } // -> scvtf_asisdshf_c
    if(!u && (immh != 0) && opcode==0x1f) { return trans.fcvtzs_advsimd_fix(args); } // -> fcvtzs_asisdshf_c
    if(u && (immh != 0) && (opcode == 0)) { return trans.ushr_advsimd(args); } // -> ushr_asisdshf_r
    if(u && (immh != 0) && opcode==2) { return trans.usra_advsimd(args); } // -> usra_asisdshf_r
    if(u && (immh != 0) && opcode==4) { return trans.urshr_advsimd(args); } // -> urshr_asisdshf_r
    if(u && (immh != 0) && opcode==6) { return trans.ursra_advsimd(args); } // -> ursra_asisdshf_r
    if(u && (immh != 0) && opcode==8) { return trans.sri_advsimd(args); } // -> sri_asisdshf_r
    if(u && (immh != 0) && opcode==10) { return trans.sli_advsimd(args); } // -> sli_asisdshf_r
    if(u && (immh != 0) && opcode==12) { return trans.sqshlu_advsimd(args); } // -> sqshlu_asisdshf_r
    if(u && (immh != 0) && opcode==14) { return trans.uqshl_advsimd_imm(args); } // -> uqshl_asisdshf_r
    if(u && (immh != 0) && opcode==0x10) { return trans.sqshrun_advsimd(args); } // -> sqshrun_asisdshf_n
    if(u && (immh != 0) && opcode==0x11) { return trans.sqrshrun_advsimd(args); } // -> sqrshrun_asisdshf_n
    if(u && (immh != 0) && opcode==0x12) { return trans.uqshrn_advsimd(args); } // -> uqshrn_asisdshf_n
    if(u && (immh != 0) && opcode==0x13) { return trans.uqrshrn_advsimd(args); } // -> uqrshrn_asisdshf_n
    if(u && (immh != 0) && opcode==0x1c) { return trans.ucvtf_advsimd_fix(args); } // -> ucvtf_asisdshf_c
    if(u && (immh != 0) && opcode==0x1f) { return trans.fcvtzu_advsimd_fix(args); } // -> fcvtzu_asisdshf_c
    return false;
}
pub fn decode_asisddiff<T: Arm64DecodeTrait>(trans: &mut T, insn: u32) -> bool {
    let args: ArmInstr = ArmInstr {
        insn
    };
    let opcode = ((insn >> 12) & 0x1f);
    let u = ((insn >> 29) & 1) != 0;
    
    if(!u && opcode==9) { return trans.sqdmlal_advsimd_vec(args); } // -> sqdmlal_asisddiff_only
    if(!u && opcode==11) { return trans.sqdmlsl_advsimd_vec(args); } // -> sqdmlsl_asisddiff_only
    if(!u && opcode==13) { return trans.sqdmull_advsimd_vec(args); } // -> sqdmull_asisddiff_only

    return false;
}
pub fn decode_asisdsame<T: Arm64DecodeTrait>(trans: &mut T, insn: u32) -> bool {
    let args: ArmInstr = ArmInstr {
        insn
    };
    let opcode = ((insn >> 11) & 0x1f);
    let u = ((insn >> 29) & 1) != 0;
    let size = ((insn >> 22) & 3);

    if(!u && ((size&2) == 0) && opcode==0x1b) { return trans.fmulx_advsimd_vec(args); } // -> fmulx_asisdsame_only
    if(!u && ((size&2) == 0) && opcode==0x1c) { return trans.fcmeq_advsimd_reg(args); } // -> fcmeq_asisdsame_only
    if(!u && ((size&2) == 0) && opcode==0x1f) { return trans.frecps_advsimd(args); } // -> frecps_asisdsame_only
    if(!u && (size&2)==2 && opcode==0x1f) { return trans.frsqrts_advsimd(args); } // -> frsqrts_asisdsame_only
    if(u && ((size&2) == 0) && opcode==0x1c) { return trans.fcmge_advsimd_reg(args); } // -> fcmge_asisdsame_only
    if(u && ((size&2) == 0) && opcode==0x1d) { return trans.facge_advsimd(args); } // -> facge_asisdsame_only
    if(u && (size&2)==2 && opcode==0x1a) { return trans.fabd_advsimd(args); } // -> fabd_asisdsame_only
    if(u && (size&2)==2 && opcode==0x1c) { return trans.fcmgt_advsimd_reg(args); } // -> fcmgt_asisdsame_only
    if(u && (size&2)==2 && opcode==0x1d) { return trans.facgt_advsimd(args); } // -> facgt_asisdsame_only
    if(!u && opcode==1) { return trans.sqadd_advsimd(args); } // -> sqadd_asisdsame_only
    if(!u && opcode==5) { return trans.sqsub_advsimd(args); } // -> sqsub_asisdsame_only
    if(!u && opcode==6) { return trans.cmgt_advsimd_reg(args); } // -> cmgt_asisdsame_only
    if(!u && opcode==7) { return trans.cmge_advsimd_reg(args); } // -> cmge_asisdsame_only
    if(!u && opcode==8) { return trans.sshl_advsimd(args); } // -> sshl_asisdsame_only
    if(!u && opcode==9) { return trans.sqshl_advsimd_reg(args); } // -> sqshl_asisdsame_only
    if(!u && opcode==10) { return trans.srshl_advsimd(args); } // -> srshl_asisdsame_only
    if(!u && opcode==11) { return trans.sqrshl_advsimd(args); } // -> sqrshl_asisdsame_only
    if(!u && opcode==0x10) { return trans.add_advsimd(args); } // -> add_asisdsame_only
    if(!u && opcode==0x11) { return trans.cmtst_advsimd(args); } // -> cmtst_asisdsame_only
    if(!u && opcode==0x16) { return trans.sqdmulh_advsimd_vec(args); } // -> sqdmulh_asisdsame_only
    if(u && opcode==1) { return trans.uqadd_advsimd(args); } // -> uqadd_asisdsame_only
    if(u && opcode==5) { return trans.uqsub_advsimd(args); } // -> uqsub_asisdsame_only
    if(u && opcode==6) { return trans.cmhi_advsimd(args); } // -> cmhi_asisdsame_only
    if(u && opcode==7) { return trans.cmhs_advsimd(args); } // -> cmhs_asisdsame_only
    if(u && opcode==8) { return trans.ushl_advsimd(args); } // -> ushl_asisdsame_only
    if(u && opcode==9) { return trans.uqshl_advsimd_reg(args); } // -> uqshl_asisdsame_only
    if(u && opcode==10) { return trans.urshl_advsimd(args); } // -> urshl_asisdsame_only
    if(u && opcode==11) { return trans.uqrshl_advsimd(args); } // -> uqrshl_asisdsame_only
    if(u && opcode==0x10) { return trans.sub_advsimd(args); } // -> sub_asisdsame_only
    if(u && opcode==0x11) { return trans.cmeq_advsimd_reg(args); } // -> cmeq_asisdsame_only
    if(u && opcode==0x16) { return trans.sqrdmulh_advsimd_vec(args); } // -> sqrdmulh_asisdsame_only
    return false;
}
pub fn decode_asisdsamefp16<T: Arm64DecodeTrait>(trans: &mut T, insn: u32) -> bool {
    let args: ArmInstr = ArmInstr {
        insn
    };
    let opcode = ((insn >> 11) & 7);
    let u = ((insn >> 29) & 1) != 0;
    let a = ((insn >> 23) & 1) != 0;

    if(!u && !a && opcode==3 && trans.has_fp16()) { return trans.fmulx_advsimd_vec(args); } // -> fmulx_asisdsamefp16_only
    if(!u && !a && opcode==4 && trans.has_fp16()) { return trans.fcmeq_advsimd_reg(args); } // -> fcmeq_asisdsamefp16_only
    if(!u && !a && opcode==7 && trans.has_fp16()) { return trans.frecps_advsimd(args); } // -> frecps_asisdsamefp16_only
    if(!u && a && opcode==7 && trans.has_fp16()) { return trans.frsqrts_advsimd(args); } // -> frsqrts_asisdsamefp16_only
    if(u && !a && opcode==4 && trans.has_fp16()) { return trans.fcmge_advsimd_reg(args); } // -> fcmge_asisdsamefp16_only
    if(u && !a && opcode==5 && trans.has_fp16()) { return trans.facge_advsimd(args); } // -> facge_asisdsamefp16_only
    if(u && a && opcode==2 && trans.has_fp16()) { return trans.fabd_advsimd(args); } // -> fabd_asisdsamefp16_only
    if(u && a && opcode==4 && trans.has_fp16()) { return trans.fcmgt_advsimd_reg(args); } // -> fcmgt_asisdsamefp16_only
    if(u && a && opcode==5 && trans.has_fp16()) { return trans.facgt_advsimd(args); } // -> facgt_asisdsamefp16_only

    return false;
}
pub fn decode_asisdsame2<T: Arm64DecodeTrait>(trans: &mut T, insn: u32) -> bool {
    let args: ArmInstr = ArmInstr {
        insn
    };
    let opcode = ((insn >> 11) & 15);
    let u = ((insn >> 29) & 1) != 0;

    if(u && (opcode == 0) && trans.has_rdm()) { return trans.sqrdmlah_advsimd_vec(args); } // -> sqrdmlah_asisdsame2_only
    if(u && opcode==1 && trans.has_rdm()) { return trans.sqrdmlsh_advsimd_vec(args); } // -> sqrdmlsh_asisdsame2_only

    return false;
}
pub fn decode_asisdmisc<T: Arm64DecodeTrait>(trans: &mut T, insn: u32) -> bool {
    let args: ArmInstr = ArmInstr {
        insn
    };
    let opcode = ((insn >> 12) & 0x1f);
    let size = ((insn >> 22) & 3);
    let u = ((insn >> 29) & 1) != 0;

    if(!u && ((size&2) == 0) && opcode==0x1a) { return trans.fcvtns_advsimd(args); } // -> fcvtns_asisdmisc_r
    if(!u && ((size&2) == 0) && opcode==0x1b) { return trans.fcvtms_advsimd(args); } // -> fcvtms_asisdmisc_r
    if(!u && ((size&2) == 0) && opcode==0x1c) { return trans.fcvtas_advsimd(args); } // -> fcvtas_asisdmisc_r
    if(!u && ((size&2) == 0) && opcode==0x1d) { return trans.scvtf_advsimd_int(args); } // -> scvtf_asisdmisc_r
    if(!u && (size&2)==2 && opcode==12) { return trans.fcmgt_advsimd_zero(args); } // -> fcmgt_asisdmisc_fz
    if(!u && (size&2)==2 && opcode==13) { return trans.fcmeq_advsimd_zero(args); } // -> fcmeq_asisdmisc_fz
    if(!u && (size&2)==2 && opcode==14) { return trans.fcmlt_advsimd(args); } // -> fcmlt_asisdmisc_fz
    if(!u && (size&2)==2 && opcode==0x1a) { return trans.fcvtps_advsimd(args); } // -> fcvtps_asisdmisc_r
    if(!u && (size&2)==2 && opcode==0x1b) { return trans.fcvtzs_advsimd_int(args); } // -> fcvtzs_asisdmisc_r
    if(!u && (size&2)==2 && opcode==0x1d) { return trans.frecpe_advsimd(args); } // -> frecpe_asisdmisc_r
    if(!u && (size&2)==2 && opcode==0x1f) { return trans.frecpx_advsimd(args); } // -> frecpx_asisdmisc_r
    if(u && ((size&2) == 0) && opcode==0x16) { return trans.fcvtxn_advsimd(args); } // -> fcvtxn_asisdmisc_n
    if(u && ((size&2) == 0) && opcode==0x1a) { return trans.fcvtnu_advsimd(args); } // -> fcvtnu_asisdmisc_r
    if(u && ((size&2) == 0) && opcode==0x1b) { return trans.fcvtmu_advsimd(args); } // -> fcvtmu_asisdmisc_r
    if(u && ((size&2) == 0) && opcode==0x1c) { return trans.fcvtau_advsimd(args); } // -> fcvtau_asisdmisc_r
    if(u && ((size&2) == 0) && opcode==0x1d) { return trans.ucvtf_advsimd_int(args); } // -> ucvtf_asisdmisc_r
    if(u && (size&2)==2 && opcode==12) { return trans.fcmge_advsimd_zero(args); } // -> fcmge_asisdmisc_fz
    if(u && (size&2)==2 && opcode==13) { return trans.fcmle_advsimd(args); } // -> fcmle_asisdmisc_fz
    if(u && (size&2)==2 && opcode==0x1a) { return trans.fcvtpu_advsimd(args); } // -> fcvtpu_asisdmisc_r
    if(u && (size&2)==2 && opcode==0x1b) { return trans.fcvtzu_advsimd_int(args); } // -> fcvtzu_asisdmisc_r
    if(u && (size&2)==2 && opcode==0x1d) { return trans.frsqrte_advsimd(args); } // -> frsqrte_asisdmisc_r
    if(!u && opcode==3) { return trans.suqadd_advsimd(args); } // -> suqadd_asisdmisc_r
    if(!u && opcode==7) { return trans.sqabs_advsimd(args); } // -> sqabs_asisdmisc_r
    if(!u && opcode==8) { return trans.cmgt_advsimd_zero(args); } // -> cmgt_asisdmisc_z
    if(!u && opcode==9) { return trans.cmeq_advsimd_zero(args); } // -> cmeq_asisdmisc_z
    if(!u && opcode==10) { return trans.cmlt_advsimd(args); } // -> cmlt_asisdmisc_z
    if(!u && opcode==11) { return trans.abs_advsimd(args); } // -> abs_asisdmisc_r
    if(!u && opcode==0x14) { return trans.sqxtn_advsimd(args); } // -> sqxtn_asisdmisc_n
    if(u && opcode==3) { return trans.usqadd_advsimd(args); } // -> usqadd_asisdmisc_r
    if(u && opcode==7) { return trans.sqneg_advsimd(args); } // -> sqneg_asisdmisc_r
    if(u && opcode==8) { return trans.cmge_advsimd_zero(args); } // -> cmge_asisdmisc_z
    if(u && opcode==9) { return trans.cmle_advsimd(args); } // -> cmle_asisdmisc_z
    if(u && opcode==11) { return trans.neg_advsimd(args); } // -> neg_asisdmisc_r
    if(u && opcode==0x12) { return trans.sqxtun_advsimd(args); } // -> sqxtun_asisdmisc_n
    if(u && opcode==0x14) { return trans.uqxtn_advsimd(args); } // -> uqxtn_asisdmisc_n
    return false;
}
pub fn decode_asisdmiscfp16<T: Arm64DecodeTrait>(trans: &mut T, insn: u32) -> bool {
    let args: ArmInstr = ArmInstr {
        insn
    };
    let opcode = ((insn >> 12) & 0x1f);
    let a = ((insn >> 23) & 1) != 0;
    let u = ((insn >> 29) & 1) != 0;

    if(!u && !a && opcode==0x1a && trans.has_fp16()) { return trans.fcvtns_advsimd(args); } // -> fcvtns_asisdmiscfp16_r
    if(!u && !a && opcode==0x1b && trans.has_fp16()) { return trans.fcvtms_advsimd(args); } // -> fcvtms_asisdmiscfp16_r
    if(!u && !a && opcode==0x1c && trans.has_fp16()) { return trans.fcvtas_advsimd(args); } // -> fcvtas_asisdmiscfp16_r
    if(!u && !a && opcode==0x1d && trans.has_fp16()) { return trans.scvtf_advsimd_int(args); } // -> scvtf_asisdmiscfp16_r
    if(!u && a && opcode==12 && trans.has_fp16()) { return trans.fcmgt_advsimd_zero(args); } // -> fcmgt_asisdmiscfp16_fz
    if(!u && a && opcode==13 && trans.has_fp16()) { return trans.fcmeq_advsimd_zero(args); } // -> fcmeq_asisdmiscfp16_fz
    if(!u && a && opcode==14 && trans.has_fp16()) { return trans.fcmlt_advsimd(args); } // -> fcmlt_asisdmiscfp16_fz
    if(!u && a && opcode==0x1a && trans.has_fp16()) { return trans.fcvtps_advsimd(args); } // -> fcvtps_asisdmiscfp16_r
    if(!u && a && opcode==0x1b && trans.has_fp16()) { return trans.fcvtzs_advsimd_int(args); } // -> fcvtzs_asisdmiscfp16_r
    if(!u && a && opcode==0x1d && trans.has_fp16()) { return trans.frecpe_advsimd(args); } // -> frecpe_asisdmiscfp16_r
    if(!u && a && opcode==0x1f && trans.has_fp16()) { return trans.frecpx_advsimd(args); } // -> frecpx_asisdmiscfp16_r
    if(u && !a && opcode==0x1a && trans.has_fp16()) { return trans.fcvtnu_advsimd(args); } // -> fcvtnu_asisdmiscfp16_r
    if(u && !a && opcode==0x1b && trans.has_fp16()) { return trans.fcvtmu_advsimd(args); } // -> fcvtmu_asisdmiscfp16_r
    if(u && !a && opcode==0x1c && trans.has_fp16()) { return trans.fcvtau_advsimd(args); } // -> fcvtau_asisdmiscfp16_r
    if(u && !a && opcode==0x1d && trans.has_fp16()) { return trans.ucvtf_advsimd_int(args); } // -> ucvtf_asisdmiscfp16_r
    if(u && a && opcode==12 && trans.has_fp16()) { return trans.fcmge_advsimd_zero(args); } // -> fcmge_asisdmiscfp16_fz
    if(u && a && opcode==13 && trans.has_fp16()) { return trans.fcmle_advsimd(args); } // -> fcmle_asisdmiscfp16_fz
    if(u && a && opcode==0x1a && trans.has_fp16()) { return trans.fcvtpu_advsimd(args); } // -> fcvtpu_asisdmiscfp16_r
    if(u && a && opcode==0x1b && trans.has_fp16()) { return trans.fcvtzu_advsimd_int(args); } // -> fcvtzu_asisdmiscfp16_r
    if(u && a && opcode==0x1d && trans.has_fp16()) { return trans.frsqrte_advsimd(args); } // -> frsqrte_asisdmiscfp16_r

    return false;
}
pub fn decode_asisdelem<T: Arm64DecodeTrait>(trans: &mut T, insn: u32) -> bool {
    let args: ArmInstr = ArmInstr {
        insn
    };
    let opcode = ((insn >> 12) & 15);
    let size = ((insn >> 22) & 3);
    let u = ((insn >> 29) & 1) != 0;

    if(!u && (size == 0) && opcode==1 && trans.has_fp16()) { return trans.fmla_advsimd_elt(args); } // -> fmla_asisdelem_rh_h
    if(!u && (size == 0) && opcode==5 && trans.has_fp16()) { return trans.fmls_advsimd_elt(args); } // -> fmls_asisdelem_rh_h
    if(!u && (size == 0) && opcode==9 && trans.has_fp16()) { return trans.fmul_advsimd_elt(args); } // -> fmul_asisdelem_rh_h
    if(u && (size == 0) && opcode==9 && trans.has_fp16()) { return trans.fmulx_advsimd_elt(args); } // -> fmulx_asisdelem_rh_h
    if(!u && (size&2)==2 && opcode==1) { return trans.fmla_advsimd_elt(args); } // -> fmla_asisdelem_r_sd
    if(!u && (size&2)==2 && opcode==5) { return trans.fmls_advsimd_elt(args); } // -> fmls_asisdelem_r_sd
    if(!u && (size&2)==2 && opcode==9) { return trans.fmul_advsimd_elt(args); } // -> fmul_asisdelem_r_sd
    if(u && (size&2)==2 && opcode==9) { return trans.fmulx_advsimd_elt(args); } // -> fmulx_asisdelem_r_sd
    if(!u && opcode==3) { return trans.sqdmlal_advsimd_elt(args); } // -> sqdmlal_asisdelem_l
    if(!u && opcode==7) { return trans.sqdmlsl_advsimd_elt(args); } // -> sqdmlsl_asisdelem_l
    if(!u && opcode==11) { return trans.sqdmull_advsimd_elt(args); } // -> sqdmull_asisdelem_l
    if(!u && opcode==12) { return trans.sqdmulh_advsimd_elt(args); } // -> sqdmulh_asisdelem_r
    if(!u && opcode==13) { return trans.sqrdmulh_advsimd_elt(args); } // -> sqrdmulh_asisdelem_r
    if(u && opcode==13 && trans.has_rdm()) { return trans.sqrdmlah_advsimd_elt(args); } // -> sqrdmlah_asisdelem_r
    if(u && opcode==15 && trans.has_rdm()) { return trans.sqrdmlsh_advsimd_elt(args); } // -> sqrdmlsh_asisdelem_r
    return false;
}
pub fn decode_asimdshf<T: Arm64DecodeTrait>(trans: &mut T, insn: u32) -> bool {
    let args: ArmInstr = ArmInstr {
        insn
    };
    let opcode = ((insn >> 11) & 0x1f);
    let u = ((insn >> 29) & 1) != 0;

    if(!u && (opcode == 0)) { return trans.sshr_advsimd(args); } // -> sshr_asimdshf_r
    if(!u && opcode==2) { return trans.ssra_advsimd(args); } // -> ssra_asimdshf_r
    if(!u && opcode==4) { return trans.srshr_advsimd(args); } // -> srshr_asimdshf_r
    if(!u && opcode==6) { return trans.srsra_advsimd(args); } // -> srsra_asimdshf_r
    if(!u && opcode==10) { return trans.shl_advsimd(args); } // -> shl_asimdshf_r
    if(!u && opcode==14) { return trans.sqshl_advsimd_imm(args); } // -> sqshl_asimdshf_r
    if(!u && opcode==0x10) { return trans.shrn_advsimd(args); } // -> shrn_asimdshf_n
    if(!u && opcode==0x11) { return trans.rshrn_advsimd(args); } // -> rshrn_asimdshf_n
    if(!u && opcode==0x12) { return trans.sqshrn_advsimd(args); } // -> sqshrn_asimdshf_n
    if(!u && opcode==0x13) { return trans.sqrshrn_advsimd(args); } // -> sqrshrn_asimdshf_n
    if(!u && opcode==0x14) { return trans.sshll_advsimd(args); } // -> sshll_asimdshf_l
    if(!u && opcode==0x1c) { return trans.scvtf_advsimd_fix(args); } // -> scvtf_asimdshf_c
    if(!u && opcode==0x1f) { return trans.fcvtzs_advsimd_fix(args); } // -> fcvtzs_asimdshf_c
    if(u && (opcode == 0)) { return trans.ushr_advsimd(args); } // -> ushr_asimdshf_r
    if(u && opcode==2) { return trans.usra_advsimd(args); } // -> usra_asimdshf_r
    if(u && opcode==4) { return trans.urshr_advsimd(args); } // -> urshr_asimdshf_r
    if(u && opcode==6) { return trans.ursra_advsimd(args); } // -> ursra_asimdshf_r
    if(u && opcode==8) { return trans.sri_advsimd(args); } // -> sri_asimdshf_r
    if(u && opcode==10) { return trans.sli_advsimd(args); } // -> sli_asimdshf_r
    if(u && opcode==12) { return trans.sqshlu_advsimd(args); } // -> sqshlu_asimdshf_r
    if(u && opcode==14) { return trans.uqshl_advsimd_imm(args); } // -> uqshl_asimdshf_r
    if(u && opcode==0x10) { return trans.sqshrun_advsimd(args); } // -> sqshrun_asimdshf_n
    if(u && opcode==0x11) { return trans.sqrshrun_advsimd(args); } // -> sqrshrun_asimdshf_n
    if(u && opcode==0x12) { return trans.uqshrn_advsimd(args); } // -> uqshrn_asimdshf_n
    if(u && opcode==0x13) { return trans.uqrshrn_advsimd(args); } // -> uqrshrn_asimdshf_n
    if(u && opcode==0x14) { return trans.ushll_advsimd(args); } // -> ushll_asimdshf_l
    if(u && opcode==0x1c) { return trans.ucvtf_advsimd_fix(args); } // -> ucvtf_asimdshf_c
    if(u && opcode==0x1f) { return trans.fcvtzu_advsimd_fix(args); } // -> fcvtzu_asimdshf_c
    return false;
}
pub fn decode_asimdtbl<T: Arm64DecodeTrait>(trans: &mut T, insn: u32) -> bool {
    let args: ArmInstr = ArmInstr {
        insn
    };
    let op2 = ((insn >> 22) & 3);
    let op = ((insn >> 12) & 1) != 0;
    let len = ((insn >> 13) & 3);

    if((op2 == 0) && (len==0) && !op) { return trans.tbl_advsimd(args); } // -> tbl_asimdtbl_l1_1
    if((op2 == 0) && (len==0) && op) { return trans.tbx_advsimd(args); } // -> tbx_asimdtbl_l1_1
    if((op2 == 0) && len==1 && !op) { return trans.tbl_advsimd(args); } // -> tbl_asimdtbl_l2_2
    if((op2 == 0) && len==1 && op) { return trans.tbx_advsimd(args); } // -> tbx_asimdtbl_l2_2
    if((op2 == 0) && len==2 && !op) { return trans.tbl_advsimd(args); } // -> tbl_asimdtbl_l3_3
    if((op2 == 0) && len==2 && op) { return trans.tbx_advsimd(args); } // -> tbx_asimdtbl_l3_3
    if((op2 == 0) && len==3 && !op) { return trans.tbl_advsimd(args); } // -> tbl_asimdtbl_l4_4
    if((op2 == 0) && len==3 && op) { return trans.tbx_advsimd(args); } // -> tbx_asimdtbl_l4_4
    return false;
}
pub fn decode_asimddiff<T: Arm64DecodeTrait>(trans: &mut T, insn: u32) -> bool {
    let args: ArmInstr = ArmInstr {
        insn
    };
    let u = ((insn >> 29) & 1) != 0;
    let opcode = ((insn >> 12) & 15);

    if(!u && (opcode == 0)) { return trans.saddl_advsimd(args); } // -> saddl_asimddiff_l
    if(!u && opcode==1) { return trans.saddw_advsimd(args); } // -> saddw_asimddiff_w
    if(!u && opcode==2) { return trans.ssubl_advsimd(args); } // -> ssubl_asimddiff_l
    if(!u && opcode==3) { return trans.ssubw_advsimd(args); } // -> ssubw_asimddiff_w
    if(!u && opcode==4) { return trans.addhn_advsimd(args); } // -> addhn_asimddiff_n
    if(!u && opcode==5) { return trans.sabal_advsimd(args); } // -> sabal_asimddiff_l
    if(!u && opcode==6) { return trans.subhn_advsimd(args); } // -> subhn_asimddiff_n
    if(!u && opcode==7) { return trans.sabdl_advsimd(args); } // -> sabdl_asimddiff_l
    if(!u && opcode==8) { return trans.smlal_advsimd_vec(args); } // -> smlal_asimddiff_l
    if(!u && opcode==9) { return trans.sqdmlal_advsimd_vec(args); } // -> sqdmlal_asimddiff_l
    if(!u && opcode==10) { return trans.smlsl_advsimd_vec(args); } // -> smlsl_asimddiff_l
    if(!u && opcode==11) { return trans.sqdmlsl_advsimd_vec(args); } // -> sqdmlsl_asimddiff_l
    if(!u && opcode==12) { return trans.smull_advsimd_vec(args); } // -> smull_asimddiff_l
    if(!u && opcode==13) { return trans.sqdmull_advsimd_vec(args); } // -> sqdmull_asimddiff_l
    if(!u && opcode==14) { return trans.pmull_advsimd(args); } // -> pmull_asimddiff_l
    if(u && (opcode == 0)) { return trans.uaddl_advsimd(args); } // -> uaddl_asimddiff_l
    if(u && opcode==1) { return trans.uaddw_advsimd(args); } // -> uaddw_asimddiff_w
    if(u && opcode==2) { return trans.usubl_advsimd(args); } // -> usubl_asimddiff_l
    if(u && opcode==3) { return trans.usubw_advsimd(args); } // -> usubw_asimddiff_w
    if(u && opcode==4) { return trans.raddhn_advsimd(args); } // -> raddhn_asimddiff_n
    if(u && opcode==5) { return trans.uabal_advsimd(args); } // -> uabal_asimddiff_l
    if(u && opcode==6) { return trans.rsubhn_advsimd(args); } // -> rsubhn_asimddiff_n
    if(u && opcode==7) { return trans.uabdl_advsimd(args); } // -> uabdl_asimddiff_l
    if(u && opcode==8) { return trans.umlal_advsimd_vec(args); } // -> umlal_asimddiff_l
    if(u && opcode==10) { return trans.umlsl_advsimd_vec(args); } // -> umlsl_asimddiff_l
    if(u && opcode==12) { return trans.umull_advsimd_vec(args); } // -> umull_asimddiff_l
    return false;
}
pub fn decode_asimdsame<T: Arm64DecodeTrait>(trans: &mut T, insn: u32) -> bool {
    let args: ArmInstr = ArmInstr {
        insn
    };
    let u = ((insn >> 29) & 1) != 0;
    let size = ((insn >> 22) & 3);
    let opcode = ((insn >> 11) & 0x1f);
    
    if(!u && (size == 0) && opcode==3) { return trans.and_advsimd(args); } // -> and_asimdsame_only
    if(!u && (size == 0) && opcode==0x1d && trans.has_fhm()) { return trans.fmlal_advsimd_vec(args); } // -> fmlal_asimdsame_f
    if(!u && size==1 && opcode==3) { return trans.bic_advsimd_reg(args); } // -> bic_asimdsame_only
    if(!u && size==2 && opcode==3) { return trans.orr_advsimd_reg(args); } // -> orr_asimdsame_only
    if(!u && size==2 && opcode==0x1d && trans.has_fhm()) { return trans.fmlsl_advsimd_vec(args); } // -> fmlsl_asimdsame_f
    if(!u && size==3 && opcode==3) { return trans.orn_advsimd(args); } // -> orn_asimdsame_only
    if(u && (size == 0) && opcode==3) { return trans.eor_advsimd(args); } // -> eor_asimdsame_only
    if(u && (size == 0) && opcode==0x19 && trans.has_fhm()) { return trans.fmlal_advsimd_vec(args); } // -> fmlal2_asimdsame_f
    if(u && size==1 && opcode==3) { return trans.bsl_advsimd(args); } // -> bsl_asimdsame_only
    if(u && size==2 && opcode==3) { return trans.bit_advsimd(args); } // -> bit_asimdsame_only
    if(u && size==2 && opcode==0x19 && trans.has_fhm()) { return trans.fmlsl_advsimd_vec(args); } // -> fmlsl2_asimdsame_f
    if(u && size==3 && opcode==3) { return trans.bif_advsimd(args); } // -> bif_asimdsame_only
    if(!u && ((size&2) == 0) && opcode==0x18) { return trans.fmaxnm_advsimd(args); } // -> fmaxnm_asimdsame_only
    if(!u && ((size&2) == 0) && opcode==0x19) { return trans.fmla_advsimd_vec(args); } // -> fmla_asimdsame_only
    if(!u && ((size&2) == 0) && opcode==0x1a) { return trans.fadd_advsimd(args); } // -> fadd_asimdsame_only
    if(!u && ((size&2) == 0) && opcode==0x1b) { return trans.fmulx_advsimd_vec(args); } // -> fmulx_asimdsame_only
    if(!u && ((size&2) == 0) && opcode==0x1c) { return trans.fcmeq_advsimd_reg(args); } // -> fcmeq_asimdsame_only
    if(!u && ((size&2) == 0) && opcode==0x1e) { return trans.fmax_advsimd(args); } // -> fmax_asimdsame_only
    if(!u && ((size&2) == 0) && opcode==0x1f) { return trans.frecps_advsimd(args); } // -> frecps_asimdsame_only
    if(!u && (size&2)==2 && opcode==0x18) { return trans.fminnm_advsimd(args); } // -> fminnm_asimdsame_only
    if(!u && (size&2)==2 && opcode==0x19) { return trans.fmls_advsimd_vec(args); } // -> fmls_asimdsame_only
    if(!u && (size&2)==2 && opcode==0x1a) { return trans.fsub_advsimd(args); } // -> fsub_asimdsame_only
    if(!u && (size&2)==2 && opcode==0x1e) { return trans.fmin_advsimd(args); } // -> fmin_asimdsame_only
    if(!u && (size&2)==2 && opcode==0x1f) { return trans.frsqrts_advsimd(args); } // -> frsqrts_asimdsame_only
    if(u && ((size&2) == 0) && opcode==0x18) { return trans.fmaxnmp_advsimd_vec(args); } // -> fmaxnmp_asimdsame_only
    if(u && ((size&2) == 0) && opcode==0x1a) { return trans.faddp_advsimd_vec(args); } // -> faddp_asimdsame_only
    if(u && ((size&2) == 0) && opcode==0x1b) { return trans.fmul_advsimd_vec(args); } // -> fmul_asimdsame_only
    if(u && ((size&2) == 0) && opcode==0x1c) { return trans.fcmge_advsimd_reg(args); } // -> fcmge_asimdsame_only
    if(u && ((size&2) == 0) && opcode==0x1d) { return trans.facge_advsimd(args); } // -> facge_asimdsame_only
    if(u && ((size&2) == 0) && opcode==0x1e) { return trans.fmaxp_advsimd_vec(args); } // -> fmaxp_asimdsame_only
    if(u && ((size&2) == 0) && opcode==0x1f) { return trans.fdiv_advsimd(args); } // -> fdiv_asimdsame_only
    if(u && (size&2)==2 && opcode==0x18) { return trans.fminnmp_advsimd_vec(args); } // -> fminnmp_asimdsame_only
    if(u && (size&2)==2 && opcode==0x1a) { return trans.fabd_advsimd(args); } // -> fabd_asimdsame_only
    if(u && (size&2)==2 && opcode==0x1c) { return trans.fcmgt_advsimd_reg(args); } // -> fcmgt_asimdsame_only
    if(u && (size&2)==2 && opcode==0x1d) { return trans.facgt_advsimd(args); } // -> facgt_asimdsame_only
    if(u && (size&2)==2 && opcode==0x1e) { return trans.fminp_advsimd_vec(args); } // -> fminp_asimdsame_only
    if(!u && (opcode == 0)) { return trans.shadd_advsimd(args); } // -> shadd_asimdsame_only
    if(!u && opcode==1) { return trans.sqadd_advsimd(args); } // -> sqadd_asimdsame_only
    if(!u && opcode==2) { return trans.srhadd_advsimd(args); } // -> srhadd_asimdsame_only
    if(!u && opcode==4) { return trans.shsub_advsimd(args); } // -> shsub_asimdsame_only
    if(!u && opcode==5) { return trans.sqsub_advsimd(args); } // -> sqsub_asimdsame_only
    if(!u && opcode==6) { return trans.cmgt_advsimd_reg(args); } // -> cmgt_asimdsame_only
    if(!u && opcode==7) { return trans.cmge_advsimd_reg(args); } // -> cmge_asimdsame_only
    if(!u && opcode==8) { return trans.sshl_advsimd(args); } // -> sshl_asimdsame_only
    if(!u && opcode==9) { return trans.sqshl_advsimd_reg(args); } // -> sqshl_asimdsame_only
    if(!u && opcode==10) { return trans.srshl_advsimd(args); } // -> srshl_asimdsame_only
    if(!u && opcode==11) { return trans.sqrshl_advsimd(args); } // -> sqrshl_asimdsame_only
    if(!u && opcode==12) { return trans.smax_advsimd(args); } // -> smax_asimdsame_only
    if(!u && opcode==13) { return trans.smin_advsimd(args); } // -> smin_asimdsame_only
    if(!u && opcode==14) { return trans.sabd_advsimd(args); } // -> sabd_asimdsame_only
    if(!u && opcode==15) { return trans.saba_advsimd(args); } // -> saba_asimdsame_only
    if(!u && opcode==0x10) { return trans.add_advsimd(args); } // -> add_asimdsame_only
    if(!u && opcode==0x11) { return trans.cmtst_advsimd(args); } // -> cmtst_asimdsame_only
    if(!u && opcode==0x12) { return trans.mla_advsimd_vec(args); } // -> mla_asimdsame_only
    if(!u && opcode==0x13) { return trans.mul_advsimd_vec(args); } // -> mul_asimdsame_only
    if(!u && opcode==0x14) { return trans.smaxp_advsimd(args); } // -> smaxp_asimdsame_only
    if(!u && opcode==0x15) { return trans.sminp_advsimd(args); } // -> sminp_asimdsame_only
    if(!u && opcode==0x16) { return trans.sqdmulh_advsimd_vec(args); } // -> sqdmulh_asimdsame_only
    if(!u && opcode==0x17) { return trans.addp_advsimd_vec(args); } // -> addp_asimdsame_only
    if(u && (opcode == 0)) { return trans.uhadd_advsimd(args); } // -> uhadd_asimdsame_only
    if(u && opcode==1) { return trans.uqadd_advsimd(args); } // -> uqadd_asimdsame_only
    if(u && opcode==2) { return trans.urhadd_advsimd(args); } // -> urhadd_asimdsame_only
    if(u && opcode==4) { return trans.uhsub_advsimd(args); } // -> uhsub_asimdsame_only
    if(u && opcode==5) { return trans.uqsub_advsimd(args); } // -> uqsub_asimdsame_only
    if(u && opcode==6) { return trans.cmhi_advsimd(args); } // -> cmhi_asimdsame_only
    if(u && opcode==7) { return trans.cmhs_advsimd(args); } // -> cmhs_asimdsame_only
    if(u && opcode==8) { return trans.ushl_advsimd(args); } // -> ushl_asimdsame_only
    if(u && opcode==9) { return trans.uqshl_advsimd_reg(args); } // -> uqshl_asimdsame_only
    if(u && opcode==10) { return trans.urshl_advsimd(args); } // -> urshl_asimdsame_only
    if(u && opcode==11) { return trans.uqrshl_advsimd(args); } // -> uqrshl_asimdsame_only
    if(u && opcode==12) { return trans.umax_advsimd(args); } // -> umax_asimdsame_only
    if(u && opcode==13) { return trans.umin_advsimd(args); } // -> umin_asimdsame_only
    if(u && opcode==14) { return trans.uabd_advsimd(args); } // -> uabd_asimdsame_only
    if(u && opcode==15) { return trans.uaba_advsimd(args); } // -> uaba_asimdsame_only
    if(u && opcode==0x10) { return trans.sub_advsimd(args); } // -> sub_asimdsame_only
    if(u && opcode==0x11) { return trans.cmeq_advsimd_reg(args); } // -> cmeq_asimdsame_only
    if(u && opcode==0x12) { return trans.mls_advsimd_vec(args); } // -> mls_asimdsame_only
    if(u && opcode==0x13) { return trans.pmul_advsimd(args); } // -> pmul_asimdsame_only
    if(u && opcode==0x14) { return trans.umaxp_advsimd(args); } // -> umaxp_asimdsame_only
    if(u && opcode==0x15) { return trans.uminp_advsimd(args); } // -> uminp_asimdsame_only
    if(u && opcode==0x16) { return trans.sqrdmulh_advsimd_vec(args); } // -> sqrdmulh_asimdsame_only
    return false;
}
pub fn decode_asimdsamefp16<T: Arm64DecodeTrait>(trans: &mut T, insn: u32) -> bool {
    let args: ArmInstr = ArmInstr {
        insn
    };
    let a = ((insn >> 23) & 1) != 0;
    let u = ((insn >> 29) & 1) != 0;
    let opcode = ((insn >> 11) & 7);

    if(!u && !a && (opcode == 0) && trans.has_fp16()) { return trans.fmaxnm_advsimd(args); } // -> fmaxnm_asimdsamefp16_only
    if(!u && !a && opcode==1 && trans.has_fp16()) { return trans.fmla_advsimd_vec(args); } // -> fmla_asimdsamefp16_only
    if(!u && !a && opcode==2 && trans.has_fp16()) { return trans.fadd_advsimd(args); } // -> fadd_asimdsamefp16_only
    if(!u && !a && opcode==3 && trans.has_fp16()) { return trans.fmulx_advsimd_vec(args); } // -> fmulx_asimdsamefp16_only
    if(!u && !a && opcode==4 && trans.has_fp16()) { return trans.fcmeq_advsimd_reg(args); } // -> fcmeq_asimdsamefp16_only
    if(!u && !a && opcode==6 && trans.has_fp16()) { return trans.fmax_advsimd(args); } // -> fmax_asimdsamefp16_only
    if(!u && !a && opcode==7 && trans.has_fp16()) { return trans.frecps_advsimd(args); } // -> frecps_asimdsamefp16_only
    if(!u && a && (opcode == 0) && trans.has_fp16()) { return trans.fminnm_advsimd(args); } // -> fminnm_asimdsamefp16_only
    if(!u && a && opcode==1 && trans.has_fp16()) { return trans.fmls_advsimd_vec(args); } // -> fmls_asimdsamefp16_only
    if(!u && a && opcode==2 && trans.has_fp16()) { return trans.fsub_advsimd(args); } // -> fsub_asimdsamefp16_only
    if(!u && a && opcode==6 && trans.has_fp16()) { return trans.fmin_advsimd(args); } // -> fmin_asimdsamefp16_only
    if(!u && a && opcode==7 && trans.has_fp16()) { return trans.frsqrts_advsimd(args); } // -> frsqrts_asimdsamefp16_only
    if(u && !a && (opcode == 0) && trans.has_fp16()) { return trans.fmaxnmp_advsimd_vec(args); } // -> fmaxnmp_asimdsamefp16_only
    if(u && !a && opcode==2 && trans.has_fp16()) { return trans.faddp_advsimd_vec(args); } // -> faddp_asimdsamefp16_only
    if(u && !a && opcode==3 && trans.has_fp16()) { return trans.fmul_advsimd_vec(args); } // -> fmul_asimdsamefp16_only
    if(u && !a && opcode==4 && trans.has_fp16()) { return trans.fcmge_advsimd_reg(args); } // -> fcmge_asimdsamefp16_only
    if(u && !a && opcode==5 && trans.has_fp16()) { return trans.facge_advsimd(args); } // -> facge_asimdsamefp16_only
    if(u && !a && opcode==6 && trans.has_fp16()) { return trans.fmaxp_advsimd_vec(args); } // -> fmaxp_asimdsamefp16_only
    if(u && !a && opcode==7 && trans.has_fp16()) { return trans.fdiv_advsimd(args); } // -> fdiv_asimdsamefp16_only
    if(u && a && (opcode == 0) && trans.has_fp16()) { return trans.fminnmp_advsimd_vec(args); } // -> fminnmp_asimdsamefp16_only
    if(u && a && opcode==2 && trans.has_fp16()) { return trans.fabd_advsimd(args); } // -> fabd_asimdsamefp16_only
    if(u && a && opcode==4 && trans.has_fp16()) { return trans.fcmgt_advsimd_reg(args); } // -> fcmgt_asimdsamefp16_only
    if(u && a && opcode==5 && trans.has_fp16()) { return trans.facgt_advsimd(args); } // -> facgt_asimdsamefp16_only
    if(u && a && opcode==6 && trans.has_fp16()) { return trans.fminp_advsimd_vec(args); } // -> fminp_asimdsamefp16_only
    return false;
}
pub fn decode_asimdsame2<T: Arm64DecodeTrait>(trans: &mut T, insn: u32) -> bool {
    let args: ArmInstr = ArmInstr {
        insn
    };
    let q = ((insn >> 30) & 1) != 0;
    let u = ((insn >> 29) & 1) != 0;
    let size = ((insn >> 22) & 3);
    let opcode = ((insn >> 11) & 15);

    if(q && !u && size==2 && opcode==4 && trans.has_i8mm()) { return trans.smmla_advsimd_vec(args); } // -> smmla_asimdsame2_g
    if(q && !u && size==2 && opcode==5 && trans.has_i8mm()) { return trans.usmmla_advsimd_vec(args); } // -> usmmla_asimdsame2_g
    if(q && u && size==1 && opcode==13 && trans.has_bf16()) { return trans.bfmmla_advsimd(args); } // -> bfmmla_asimdsame2_e
    if(q && u && size==2 && opcode==4 && trans.has_i8mm()) { return trans.ummla_advsimd_vec(args); } // -> ummla_asimdsame2_g
    if(!u && size==2 && opcode==3 && trans.has_i8mm()) { return trans.usdot_advsimd_vec(args); } // -> usdot_asimdsame2_d
    if(u && size==1 && opcode==15 && trans.has_bf16()) { return trans.bfdot_advsimd_vec(args); } // -> bfdot_asimdsame2_d
    if(u && size==3 && opcode==15 && trans.has_bf16()) { return trans.bfmlal_advsimd_vec(args); } // -> bfmlal_asimdsame2_f_
    if(!u && opcode==2 && trans.has_dotprod()) { return trans.sdot_advsimd_vec(args); } // -> sdot_asimdsame2_d
    if(u && (opcode == 0) && trans.has_rdm()) { return trans.sqrdmlah_advsimd_vec(args); } // -> sqrdmlah_asimdsame2_only
    if(u && opcode==1 && trans.has_rdm()) { return trans.sqrdmlsh_advsimd_vec(args); } // -> sqrdmlsh_asimdsame2_only
    if(u && opcode==2 && trans.has_dotprod()) { return trans.udot_advsimd_vec(args); } // -> udot_asimdsame2_d
    if(u && (opcode&13)==12 && trans.has_fcma()) { return trans.fcadd_advsimd_vec(args); } // -> fcadd_asimdsame2_c
    if(u && (opcode&12)==8 && trans.has_fcma()) { return trans.fcmla_advsimd_vec(args); } // -> fcmla_asimdsame2_c
    return false;
}
pub fn decode_asimdmisc<T: Arm64DecodeTrait>(trans: &mut T, insn: u32) -> bool {
    let args: ArmInstr = ArmInstr {
        insn
    };
    let u = ((insn >> 29) & 1) != 0;
    let size = ((insn >> 22) & 3);
    let opcode = ((insn >> 12) & 0x1f);

    if(!u && size==2 && opcode==0x16 && trans.has_bf16()) { return trans.bfcvtn_advsimd(args); } // -> bfcvtn_asimdmisc_4s
    if(u && (size == 0) && opcode==5) { return trans.not_advsimd(args); } // -> not_asimdmisc_r
    if(u && size==1 && opcode==5) { return trans.rbit_advsimd(args); } // -> rbit_asimdmisc_r
    if(!u && ((size&2) == 0) && opcode==0x16) { return trans.fcvtn_advsimd(args); } // -> fcvtn_asimdmisc_n
    if(!u && ((size&2) == 0) && opcode==0x17) { return trans.fcvtl_advsimd(args); } // -> fcvtl_asimdmisc_l
    if(!u && ((size&2) == 0) && opcode==0x18) { return trans.frintn_advsimd(args); } // -> frintn_asimdmisc_r
    if(!u && ((size&2) == 0) && opcode==0x19) { return trans.frintm_advsimd(args); } // -> frintm_asimdmisc_r
    if(!u && ((size&2) == 0) && opcode==0x1a) { return trans.fcvtns_advsimd(args); } // -> fcvtns_asimdmisc_r
    if(!u && ((size&2) == 0) && opcode==0x1b) { return trans.fcvtms_advsimd(args); } // -> fcvtms_asimdmisc_r
    if(!u && ((size&2) == 0) && opcode==0x1c) { return trans.fcvtas_advsimd(args); } // -> fcvtas_asimdmisc_r
    if(!u && ((size&2) == 0) && opcode==0x1d) { return trans.scvtf_advsimd_int(args); } // -> scvtf_asimdmisc_r
    if(!u && ((size&2) == 0) && opcode==0x1e && trans.has_frintts()) { return trans.frint32z_advsimd(args); } // -> frint32z_asimdmisc_r
    if(!u && ((size&2) == 0) && opcode==0x1f && trans.has_frintts()) { return trans.frint64z_advsimd(args); } // -> frint64z_asimdmisc_r
    if(!u && (size&2)==2 && opcode==12) { return trans.fcmgt_advsimd_zero(args); } // -> fcmgt_asimdmisc_fz
    if(!u && (size&2)==2 && opcode==13) { return trans.fcmeq_advsimd_zero(args); } // -> fcmeq_asimdmisc_fz
    if(!u && (size&2)==2 && opcode==14) { return trans.fcmlt_advsimd(args); } // -> fcmlt_asimdmisc_fz
    if(!u && (size&2)==2 && opcode==15) { return trans.fabs_advsimd(args); } // -> fabs_asimdmisc_r
    if(!u && (size&2)==2 && opcode==0x18) { return trans.frintp_advsimd(args); } // -> frintp_asimdmisc_r
    if(!u && (size&2)==2 && opcode==0x19) { return trans.frintz_advsimd(args); } // -> frintz_asimdmisc_r
    if(!u && (size&2)==2 && opcode==0x1a) { return trans.fcvtps_advsimd(args); } // -> fcvtps_asimdmisc_r
    if(!u && (size&2)==2 && opcode==0x1b) { return trans.fcvtzs_advsimd_int(args); } // -> fcvtzs_asimdmisc_r
    if(!u && (size&2)==2 && opcode==0x1c) { return trans.urecpe_advsimd(args); } // -> urecpe_asimdmisc_r
    if(!u && (size&2)==2 && opcode==0x1d) { return trans.frecpe_advsimd(args); } // -> frecpe_asimdmisc_r
    if(u && ((size&2) == 0) && opcode==0x16) { return trans.fcvtxn_advsimd(args); } // -> fcvtxn_asimdmisc_n
    if(u && ((size&2) == 0) && opcode==0x18) { return trans.frinta_advsimd(args); } // -> frinta_asimdmisc_r
    if(u && ((size&2) == 0) && opcode==0x19) { return trans.frintx_advsimd(args); } // -> frintx_asimdmisc_r
    if(u && ((size&2) == 0) && opcode==0x1a) { return trans.fcvtnu_advsimd(args); } // -> fcvtnu_asimdmisc_r
    if(u && ((size&2) == 0) && opcode==0x1b) { return trans.fcvtmu_advsimd(args); } // -> fcvtmu_asimdmisc_r
    if(u && ((size&2) == 0) && opcode==0x1c) { return trans.fcvtau_advsimd(args); } // -> fcvtau_asimdmisc_r
    if(u && ((size&2) == 0) && opcode==0x1d) { return trans.ucvtf_advsimd_int(args); } // -> ucvtf_asimdmisc_r
    if(u && ((size&2) == 0) && opcode==0x1e && trans.has_frintts()) { return trans.frint32x_advsimd(args); } // -> frint32x_asimdmisc_r
    if(u && ((size&2) == 0) && opcode==0x1f && trans.has_frintts()) { return trans.frint64x_advsimd(args); } // -> frint64x_asimdmisc_r
    if(u && (size&2)==2 && opcode==12) { return trans.fcmge_advsimd_zero(args); } // -> fcmge_asimdmisc_fz
    if(u && (size&2)==2 && opcode==13) { return trans.fcmle_advsimd(args); } // -> fcmle_asimdmisc_fz
    if(u && (size&2)==2 && opcode==15) { return trans.fneg_advsimd(args); } // -> fneg_asimdmisc_r
    if(u && (size&2)==2 && opcode==0x19) { return trans.frinti_advsimd(args); } // -> frinti_asimdmisc_r
    if(u && (size&2)==2 && opcode==0x1a) { return trans.fcvtpu_advsimd(args); } // -> fcvtpu_asimdmisc_r
    if(u && (size&2)==2 && opcode==0x1b) { return trans.fcvtzu_advsimd_int(args); } // -> fcvtzu_asimdmisc_r
    if(u && (size&2)==2 && opcode==0x1c) { return trans.ursqrte_advsimd(args); } // -> ursqrte_asimdmisc_r
    if(u && (size&2)==2 && opcode==0x1d) { return trans.frsqrte_advsimd(args); } // -> frsqrte_asimdmisc_r
    if(u && (size&2)==2 && opcode==0x1f) { return trans.fsqrt_advsimd(args); } // -> fsqrt_asimdmisc_r
    if(!u && (opcode == 0)) { return trans.rev64_advsimd(args); } // -> rev64_asimdmisc_r
    if(!u && opcode==1) { return trans.rev16_advsimd(args); } // -> rev16_asimdmisc_r
    if(!u && opcode==2) { return trans.saddlp_advsimd(args); } // -> saddlp_asimdmisc_p
    if(!u && opcode==3) { return trans.suqadd_advsimd(args); } // -> suqadd_asimdmisc_r
    if(!u && opcode==4) { return trans.cls_advsimd(args); } // -> cls_asimdmisc_r
    if(!u && opcode==5) { return trans.cnt_advsimd(args); } // -> cnt_asimdmisc_r
    if(!u && opcode==6) { return trans.sadalp_advsimd(args); } // -> sadalp_asimdmisc_p
    if(!u && opcode==7) { return trans.sqabs_advsimd(args); } // -> sqabs_asimdmisc_r
    if(!u && opcode==8) { return trans.cmgt_advsimd_zero(args); } // -> cmgt_asimdmisc_z
    if(!u && opcode==9) { return trans.cmeq_advsimd_zero(args); } // -> cmeq_asimdmisc_z
    if(!u && opcode==10) { return trans.cmlt_advsimd(args); } // -> cmlt_asimdmisc_z
    if(!u && opcode==11) { return trans.abs_advsimd(args); } // -> abs_asimdmisc_r
    if(!u && opcode==0x12) { return trans.xtn_advsimd(args); } // -> xtn_asimdmisc_n
    if(!u && opcode==0x14) { return trans.sqxtn_advsimd(args); } // -> sqxtn_asimdmisc_n
    if(u && (opcode == 0)) { return trans.rev32_advsimd(args); } // -> rev32_asimdmisc_r
    if(u && opcode==2) { return trans.uaddlp_advsimd(args); } // -> uaddlp_asimdmisc_p
    if(u && opcode==3) { return trans.usqadd_advsimd(args); } // -> usqadd_asimdmisc_r
    if(u && opcode==4) { return trans.clz_advsimd(args); } // -> clz_asimdmisc_r
    if(u && opcode==6) { return trans.uadalp_advsimd(args); } // -> uadalp_asimdmisc_p
    if(u && opcode==7) { return trans.sqneg_advsimd(args); } // -> sqneg_asimdmisc_r
    if(u && opcode==8) { return trans.cmge_advsimd_zero(args); } // -> cmge_asimdmisc_z
    if(u && opcode==9) { return trans.cmle_advsimd(args); } // -> cmle_asimdmisc_z
    if(u && opcode==11) { return trans.neg_advsimd(args); } // -> neg_asimdmisc_r
    if(u && opcode==0x12) { return trans.sqxtun_advsimd(args); } // -> sqxtun_asimdmisc_n
    if(u && opcode==0x13) { return trans.shll_advsimd(args); } // -> shll_asimdmisc_s
    if(u && opcode==0x14) { return trans.uqxtn_advsimd(args); } // -> uqxtn_asimdmisc_n
    return false;
}
pub fn decode_asimdmiscfp16<T: Arm64DecodeTrait>(trans: &mut T, insn: u32) -> bool {
    let args: ArmInstr = ArmInstr {
        insn
    };
    let u = ((insn >> 29) & 1) != 0;
    let a = ((insn >> 23) & 1) != 0;
    let opcode = ((insn >> 12) & 0x1f);

    if(!u && !a && opcode==0x18 && trans.has_fp16()) { return trans.frintn_advsimd(args); } // -> frintn_asimdmiscfp16_r
    if(!u && !a && opcode==0x19 && trans.has_fp16()) { return trans.frintm_advsimd(args); } // -> frintm_asimdmiscfp16_r
    if(!u && !a && opcode==0x1a && trans.has_fp16()) { return trans.fcvtns_advsimd(args); } // -> fcvtns_asimdmiscfp16_r
    if(!u && !a && opcode==0x1b && trans.has_fp16()) { return trans.fcvtms_advsimd(args); } // -> fcvtms_asimdmiscfp16_r
    if(!u && !a && opcode==0x1c && trans.has_fp16()) { return trans.fcvtas_advsimd(args); } // -> fcvtas_asimdmiscfp16_r
    if(!u && !a && opcode==0x1d && trans.has_fp16()) { return trans.scvtf_advsimd_int(args); } // -> scvtf_asimdmiscfp16_r
    if(!u && a && opcode==12 && trans.has_fp16()) { return trans.fcmgt_advsimd_zero(args); } // -> fcmgt_asimdmiscfp16_fz
    if(!u && a && opcode==13 && trans.has_fp16()) { return trans.fcmeq_advsimd_zero(args); } // -> fcmeq_asimdmiscfp16_fz
    if(!u && a && opcode==14 && trans.has_fp16()) { return trans.fcmlt_advsimd(args); } // -> fcmlt_asimdmiscfp16_fz
    if(!u && a && opcode==15 && trans.has_fp16()) { return trans.fabs_advsimd(args); } // -> fabs_asimdmiscfp16_r
    if(!u && a && opcode==0x18 && trans.has_fp16()) { return trans.frintp_advsimd(args); } // -> frintp_asimdmiscfp16_r
    if(!u && a && opcode==0x19 && trans.has_fp16()) { return trans.frintz_advsimd(args); } // -> frintz_asimdmiscfp16_r
    if(!u && a && opcode==0x1a && trans.has_fp16()) { return trans.fcvtps_advsimd(args); } // -> fcvtps_asimdmiscfp16_r
    if(!u && a && opcode==0x1b && trans.has_fp16()) { return trans.fcvtzs_advsimd_int(args); } // -> fcvtzs_asimdmiscfp16_r
    if(!u && a && opcode==0x1d && trans.has_fp16()) { return trans.frecpe_advsimd(args); } // -> frecpe_asimdmiscfp16_r
    if(u && !a && opcode==0x18 && trans.has_fp16()) { return trans.frinta_advsimd(args); } // -> frinta_asimdmiscfp16_r
    if(u && !a && opcode==0x19 && trans.has_fp16()) { return trans.frintx_advsimd(args); } // -> frintx_asimdmiscfp16_r
    if(u && !a && opcode==0x1a && trans.has_fp16()) { return trans.fcvtnu_advsimd(args); } // -> fcvtnu_asimdmiscfp16_r
    if(u && !a && opcode==0x1b && trans.has_fp16()) { return trans.fcvtmu_advsimd(args); } // -> fcvtmu_asimdmiscfp16_r
    if(u && !a && opcode==0x1c && trans.has_fp16()) { return trans.fcvtau_advsimd(args); } // -> fcvtau_asimdmiscfp16_r
    if(u && !a && opcode==0x1d && trans.has_fp16()) { return trans.ucvtf_advsimd_int(args); } // -> ucvtf_asimdmiscfp16_r
    if(u && a && opcode==12 && trans.has_fp16()) { return trans.fcmge_advsimd_zero(args); } // -> fcmge_asimdmiscfp16_fz
    if(u && a && opcode==13 && trans.has_fp16()) { return trans.fcmle_advsimd(args); } // -> fcmle_asimdmiscfp16_fz
    if(u && a && opcode==15 && trans.has_fp16()) { return trans.fneg_advsimd(args); } // -> fneg_asimdmiscfp16_r
    if(u && a && opcode==0x19 && trans.has_fp16()) { return trans.frinti_advsimd(args); } // -> frinti_asimdmiscfp16_r
    if(u && a && opcode==0x1a && trans.has_fp16()) { return trans.fcvtpu_advsimd(args); } // -> fcvtpu_asimdmiscfp16_r
    if(u && a && opcode==0x1b && trans.has_fp16()) { return trans.fcvtzu_advsimd_int(args); } // -> fcvtzu_asimdmiscfp16_r
    if(u && a && opcode==0x1d && trans.has_fp16()) { return trans.frsqrte_advsimd(args); } // -> frsqrte_asimdmiscfp16_r
    if(u && a && opcode==0x1f && trans.has_fp16()) { return trans.fsqrt_advsimd(args); } // -> fsqrt_asimdmiscfp16_r
    return false;
}
pub fn decode_asimdelem<T: Arm64DecodeTrait>(trans: &mut T, insn: u32) -> bool {
    let args: ArmInstr = ArmInstr {
        insn
    };
    let size = (insn >> 22) & 3;
    let u = ((insn >> 29) & 1) != 0;
    let opcode = ((insn >> 12) & 15);

    if(!u && (size == 0) && opcode==1 && trans.has_fp16()) { return trans.fmla_advsimd_elt(args); } // -> fmla_asimdelem_rh_h
    if(!u && (size == 0) && opcode==5 && trans.has_fp16()) { return trans.fmls_advsimd_elt(args); } // -> fmls_asimdelem_rh_h
    if(!u && (size == 0) && opcode==9 && trans.has_fp16()) { return trans.fmul_advsimd_elt(args); } // -> fmul_asimdelem_rh_h
    if(!u && (size == 0) && opcode==15 && trans.has_i8mm()) { return trans.sudot_advsimd_elt(args); } // -> sudot_asimdelem_d
    if(!u && size==1 && opcode==15 && trans.has_bf16()) { return trans.bfdot_advsimd_elt(args); } // -> bfdot_asimdelem_e
    if(!u && size==2 && (opcode == 0) && trans.has_fhm()) { return trans.fmlal_advsimd_elt(args); } // -> fmlal_asimdelem_lh
    if(!u && size==2 && opcode==4 && trans.has_fhm()) { return trans.fmlsl_advsimd_elt(args); } // -> fmlsl_asimdelem_lh
    if(!u && size==2 && opcode==15 && trans.has_i8mm()) { return trans.usdot_advsimd_elt(args); } // -> usdot_asimdelem_d
    if(!u && size==3 && opcode==15 && trans.has_bf16()) { return trans.bfmlal_advsimd_elt(args); } // -> bfmlal_asimdelem_f
    if(u && (size == 0) && opcode==9 && trans.has_fp16()) { return trans.fmulx_advsimd_elt(args); } // -> fmulx_asimdelem_rh_h
    if(u && size==2 && opcode==8 && trans.has_fhm()) { return trans.fmlal_advsimd_elt(args); } // -> fmlal2_asimdelem_lh
    if(u && size==2 && opcode==12 && trans.has_fhm()) { return trans.fmlsl_advsimd_elt(args); } // -> fmlsl2_asimdelem_lh
    if(!u && (size&2)==2 && opcode==1) { return trans.fmla_advsimd_elt(args); } // -> fmla_asimdelem_r_sd
    if(!u && (size&2)==2 && opcode==5) { return trans.fmls_advsimd_elt(args); } // -> fmls_asimdelem_r_sd
    if(!u && (size&2)==2 && opcode==9) { return trans.fmul_advsimd_elt(args); } // -> fmul_asimdelem_r_sd
    if(u && (size&2)==2 && opcode==9) { return trans.fmulx_advsimd_elt(args); } // -> fmulx_asimdelem_r_sd
    if(!u && opcode==2) { return trans.smlal_advsimd_elt(args); } // -> smlal_asimdelem_l
    if(!u && opcode==3) { return trans.sqdmlal_advsimd_elt(args); } // -> sqdmlal_asimdelem_l
    if(!u && opcode==6) { return trans.smlsl_advsimd_elt(args); } // -> smlsl_asimdelem_l
    if(!u && opcode==7) { return trans.sqdmlsl_advsimd_elt(args); } // -> sqdmlsl_asimdelem_l
    if(!u && opcode==8) { return trans.mul_advsimd_elt(args); } // -> mul_asimdelem_r
    if(!u && opcode==10) { return trans.smull_advsimd_elt(args); } // -> smull_asimdelem_l
    if(!u && opcode==11) { return trans.sqdmull_advsimd_elt(args); } // -> sqdmull_asimdelem_l
    if(!u && opcode==12) { return trans.sqdmulh_advsimd_elt(args); } // -> sqdmulh_asimdelem_r
    if(!u && opcode==13) { return trans.sqrdmulh_advsimd_elt(args); } // -> sqrdmulh_asimdelem_r
    if(!u && opcode==14 && trans.has_dotprod()) { return trans.sdot_advsimd_elt(args); } // -> sdot_asimdelem_d
    if(u && (opcode == 0)) { return trans.mla_advsimd_elt(args); } // -> mla_asimdelem_r
    if(u && opcode==2) { return trans.umlal_advsimd_elt(args); } // -> umlal_asimdelem_l
    if(u && opcode==4) { return trans.mls_advsimd_elt(args); } // -> mls_asimdelem_r
    if(u && opcode==6) { return trans.umlsl_advsimd_elt(args); } // -> umlsl_asimdelem_l
    if(u && opcode==10) { return trans.umull_advsimd_elt(args); } // -> umull_asimdelem_l
    if(u && opcode==13 && trans.has_rdm()) { return trans.sqrdmlah_advsimd_elt(args); } // -> sqrdmlah_asimdelem_r
    if(u && opcode==14 && trans.has_dotprod()) { return trans.udot_advsimd_elt(args); } // -> udot_asimdelem_d
    if(u && opcode==15 && trans.has_rdm()) { return trans.sqrdmlsh_advsimd_elt(args); } // -> sqrdmlsh_asimdelem_r
    if(u && size==1 && (opcode&9)==1 && trans.has_fcma()) { return trans.fcmla_advsimd_elt(args); } // -> fcmla_asimdelem_c_h
    if(u && size==2 && (opcode&9)==1 && trans.has_fcma()) { return trans.fcmla_advsimd_elt(args); } // -> fcmla_asimdelem_c_s
    return false;
}
pub fn decode_float2fix<T: Arm64DecodeTrait>(trans: &mut T, insn: u32) -> bool {
    let args: ArmInstr = ArmInstr {
        insn
    };
    let ptype = (insn >> 22) & 3;
    let sf = ((insn >> 31) & 1) != 0;
    let s = ((insn >> 29) & 1) != 0;
    let rmode = ((insn >> 19) & 3);
    let opcode = ((insn >> 16) & 7);
    let scale = ((insn >> 10) & 0x3f);

    if(!sf && !s && (ptype == 0) && (rmode == 0) && opcode==2) { return trans.scvtf_float_fix(args); } // -> scvtf_s32_float2fix
    if(!sf && !s && (ptype == 0) && (rmode == 0) && opcode==3) { return trans.ucvtf_float_fix(args); } // -> ucvtf_s32_float2fix
    if(!sf && !s && (ptype == 0) && rmode==3 && (opcode == 0)) { return trans.fcvtzs_float_fix(args); } // -> fcvtzs_32s_float2fix
    if(!sf && !s && (ptype == 0) && rmode==3 && opcode==1) { return trans.fcvtzu_float_fix(args); } // -> fcvtzu_32s_float2fix
    if(!sf && !s && ptype==1 && (rmode == 0) && opcode==2) { return trans.scvtf_float_fix(args); } // -> scvtf_d32_float2fix
    if(!sf && !s && ptype==1 && (rmode == 0) && opcode==3) { return trans.ucvtf_float_fix(args); } // -> ucvtf_d32_float2fix
    if(!sf && !s && ptype==1 && rmode==3 && (opcode == 0)) { return trans.fcvtzs_float_fix(args); } // -> fcvtzs_32d_float2fix
    if(!sf && !s && ptype==1 && rmode==3 && opcode==1) { return trans.fcvtzu_float_fix(args); } // -> fcvtzu_32d_float2fix
    if(!sf && !s && ptype==3 && (rmode == 0) && opcode==2 && trans.has_fp16()) { return trans.scvtf_float_fix(args); } // -> scvtf_h32_float2fix
    if(!sf && !s && ptype==3 && (rmode == 0) && opcode==3 && trans.has_fp16()) { return trans.ucvtf_float_fix(args); } // -> ucvtf_h32_float2fix
    if(!sf && !s && ptype==3 && rmode==3 && (opcode == 0) && trans.has_fp16()) { return trans.fcvtzs_float_fix(args); } // -> fcvtzs_32h_float2fix
    if(!sf && !s && ptype==3 && rmode==3 && opcode==1 && trans.has_fp16()) { return trans.fcvtzu_float_fix(args); } // -> fcvtzu_32h_float2fix
    if(sf && !s && (ptype == 0) && (rmode == 0) && opcode==2) { return trans.scvtf_float_fix(args); } // -> scvtf_s64_float2fix
    if(sf && !s && (ptype == 0) && (rmode == 0) && opcode==3) { return trans.ucvtf_float_fix(args); } // -> ucvtf_s64_float2fix
    if(sf && !s && (ptype == 0) && rmode==3 && (opcode == 0)) { return trans.fcvtzs_float_fix(args); } // -> fcvtzs_64s_float2fix
    if(sf && !s && (ptype == 0) && rmode==3 && opcode==1) { return trans.fcvtzu_float_fix(args); } // -> fcvtzu_64s_float2fix
    if(sf && !s && ptype==1 && (rmode == 0) && opcode==2) { return trans.scvtf_float_fix(args); } // -> scvtf_d64_float2fix
    if(sf && !s && ptype==1 && (rmode == 0) && opcode==3) { return trans.ucvtf_float_fix(args); } // -> ucvtf_d64_float2fix
    if(sf && !s && ptype==1 && rmode==3 && (opcode == 0)) { return trans.fcvtzs_float_fix(args); } // -> fcvtzs_64d_float2fix
    if(sf && !s && ptype==1 && rmode==3 && opcode==1) { return trans.fcvtzu_float_fix(args); } // -> fcvtzu_64d_float2fix
    if(sf && !s && ptype==3 && (rmode == 0) && opcode==2 && trans.has_fp16()) { return trans.scvtf_float_fix(args); } // -> scvtf_h64_float2fix
    if(sf && !s && ptype==3 && (rmode == 0) && opcode==3 && trans.has_fp16()) { return trans.ucvtf_float_fix(args); } // -> ucvtf_h64_float2fix
    if(sf && !s && ptype==3 && rmode==3 && (opcode == 0) && trans.has_fp16()) { return trans.fcvtzs_float_fix(args); } // -> fcvtzs_64h_float2fix
    if(sf && !s && ptype==3 && rmode==3 && opcode==1 && trans.has_fp16()) { return trans.fcvtzu_float_fix(args); } // -> fcvtzu_64h_float2fix
    return false;
}
pub fn decode_float2int<T: Arm64DecodeTrait>(trans: &mut T, insn: u32) -> bool {
    let args: ArmInstr = ArmInstr {
        insn
    };
    let ptype = (insn >> 22) & 3;
    let sf = ((insn >> 31) & 1) != 0;
    let s = ((insn >> 29) & 1) != 0;
    let rmode = ((insn >> 19) & 3);
    let opcode = ((insn >> 16) & 7);
    if(!sf && !s && (ptype == 0) && (rmode == 0) && (opcode == 0)) { return trans.fcvtns_float(args); } // -> fcvtns_32s_float2int
    if(!sf && !s && (ptype == 0) && (rmode == 0) && opcode==1) { return trans.fcvtnu_float(args); } // -> fcvtnu_32s_float2int
    if(!sf && !s && (ptype == 0) && (rmode == 0) && opcode==2) { return trans.scvtf_float_int(args); } // -> scvtf_s32_float2int
    if(!sf && !s && (ptype == 0) && (rmode == 0) && opcode==3) { return trans.ucvtf_float_int(args); } // -> ucvtf_s32_float2int
    if(!sf && !s && (ptype == 0) && (rmode == 0) && opcode==4) { return trans.fcvtas_float(args); } // -> fcvtas_32s_float2int
    if(!sf && !s && (ptype == 0) && (rmode == 0) && opcode==5) { return trans.fcvtau_float(args); } // -> fcvtau_32s_float2int
    if(!sf && !s && (ptype == 0) && (rmode == 0) && opcode==6) { return trans.fmov_float_gen(args); } // -> fmov_32s_float2int
    if(!sf && !s && (ptype == 0) && (rmode == 0) && opcode==7) { return trans.fmov_float_gen(args); } // -> fmov_s32_float2int
    if(!sf && !s && (ptype == 0) && rmode==1 && (opcode == 0)) { return trans.fcvtps_float(args); } // -> fcvtps_32s_float2int
    if(!sf && !s && (ptype == 0) && rmode==1 && opcode==1) { return trans.fcvtpu_float(args); } // -> fcvtpu_32s_float2int
    if(!sf && !s && (ptype == 0) && rmode==2 && (opcode == 0)) { return trans.fcvtms_float(args); } // -> fcvtms_32s_float2int
    if(!sf && !s && (ptype == 0) && rmode==2 && opcode==1) { return trans.fcvtmu_float(args); } // -> fcvtmu_32s_float2int
    if(!sf && !s && (ptype == 0) && rmode==3 && (opcode == 0)) { return trans.fcvtzs_float_int(args); } // -> fcvtzs_32s_float2int
    if(!sf && !s && (ptype == 0) && rmode==3 && opcode==1) { return trans.fcvtzu_float_int(args); } // -> fcvtzu_32s_float2int
    if(!sf && !s && ptype==1 && (rmode == 0) && (opcode == 0)) { return trans.fcvtns_float(args); } // -> fcvtns_32d_float2int
    if(!sf && !s && ptype==1 && (rmode == 0) && opcode==1) { return trans.fcvtnu_float(args); } // -> fcvtnu_32d_float2int
    if(!sf && !s && ptype==1 && (rmode == 0) && opcode==2) { return trans.scvtf_float_int(args); } // -> scvtf_d32_float2int
    if(!sf && !s && ptype==1 && (rmode == 0) && opcode==3) { return trans.ucvtf_float_int(args); } // -> ucvtf_d32_float2int
    if(!sf && !s && ptype==1 && (rmode == 0) && opcode==4) { return trans.fcvtas_float(args); } // -> fcvtas_32d_float2int
    if(!sf && !s && ptype==1 && (rmode == 0) && opcode==5) { return trans.fcvtau_float(args); } // -> fcvtau_32d_float2int
    if(!sf && !s && ptype==1 && rmode==1 && (opcode == 0)) { return trans.fcvtps_float(args); } // -> fcvtps_32d_float2int
    if(!sf && !s && ptype==1 && rmode==1 && opcode==1) { return trans.fcvtpu_float(args); } // -> fcvtpu_32d_float2int
    if(!sf && !s && ptype==1 && rmode==2 && (opcode == 0)) { return trans.fcvtms_float(args); } // -> fcvtms_32d_float2int
    if(!sf && !s && ptype==1 && rmode==2 && opcode==1) { return trans.fcvtmu_float(args); } // -> fcvtmu_32d_float2int
    if(!sf && !s && ptype==1 && rmode==3 && (opcode == 0)) { return trans.fcvtzs_float_int(args); } // -> fcvtzs_32d_float2int
    if(!sf && !s && ptype==1 && rmode==3 && opcode==1) { return trans.fcvtzu_float_int(args); } // -> fcvtzu_32d_float2int
    if(!sf && !s && ptype==1 && rmode==3 && opcode==6 && trans.has_jscvt()) { return trans.fjcvtzs(args); } // -> fjcvtzs_32d_float2int
    if(!sf && !s && ptype==3 && (rmode == 0) && (opcode == 0) && trans.has_fp16()) { return trans.fcvtns_float(args); } // -> fcvtns_32h_float2int
    if(!sf && !s && ptype==3 && (rmode == 0) && opcode==1 && trans.has_fp16()) { return trans.fcvtnu_float(args); } // -> fcvtnu_32h_float2int
    if(!sf && !s && ptype==3 && (rmode == 0) && opcode==2 && trans.has_fp16()) { return trans.scvtf_float_int(args); } // -> scvtf_h32_float2int
    if(!sf && !s && ptype==3 && (rmode == 0) && opcode==3 && trans.has_fp16()) { return trans.ucvtf_float_int(args); } // -> ucvtf_h32_float2int
    if(!sf && !s && ptype==3 && (rmode == 0) && opcode==4 && trans.has_fp16()) { return trans.fcvtas_float(args); } // -> fcvtas_32h_float2int
    if(!sf && !s && ptype==3 && (rmode == 0) && opcode==5 && trans.has_fp16()) { return trans.fcvtau_float(args); } // -> fcvtau_32h_float2int
    if(!sf && !s && ptype==3 && (rmode == 0) && opcode==6 && trans.has_fp16()) { return trans.fmov_float_gen(args); } // -> fmov_32h_float2int
    if(!sf && !s && ptype==3 && (rmode == 0) && opcode==7 && trans.has_fp16()) { return trans.fmov_float_gen(args); } // -> fmov_h32_float2int
    if(!sf && !s && ptype==3 && rmode==1 && (opcode == 0) && trans.has_fp16()) { return trans.fcvtps_float(args); } // -> fcvtps_32h_float2int
    if(!sf && !s && ptype==3 && rmode==1 && opcode==1 && trans.has_fp16()) { return trans.fcvtpu_float(args); } // -> fcvtpu_32h_float2int
    if(!sf && !s && ptype==3 && rmode==2 && (opcode == 0) && trans.has_fp16()) { return trans.fcvtms_float(args); } // -> fcvtms_32h_float2int
    if(!sf && !s && ptype==3 && rmode==2 && opcode==1 && trans.has_fp16()) { return trans.fcvtmu_float(args); } // -> fcvtmu_32h_float2int
    if(!sf && !s && ptype==3 && rmode==3 && (opcode == 0) && trans.has_fp16()) { return trans.fcvtzs_float_int(args); } // -> fcvtzs_32h_float2int
    if(!sf && !s && ptype==3 && rmode==3 && opcode==1 && trans.has_fp16()) { return trans.fcvtzu_float_int(args); } // -> fcvtzu_32h_float2int
    if(sf && !s && (ptype == 0) && (rmode == 0) && (opcode == 0)) { return trans.fcvtns_float(args); } // -> fcvtns_64s_float2int
    if(sf && !s && (ptype == 0) && (rmode == 0) && opcode==1) { return trans.fcvtnu_float(args); } // -> fcvtnu_64s_float2int
    if(sf && !s && (ptype == 0) && (rmode == 0) && opcode==2) { return trans.scvtf_float_int(args); } // -> scvtf_s64_float2int
    if(sf && !s && (ptype == 0) && (rmode == 0) && opcode==3) { return trans.ucvtf_float_int(args); } // -> ucvtf_s64_float2int
    if(sf && !s && (ptype == 0) && (rmode == 0) && opcode==4) { return trans.fcvtas_float(args); } // -> fcvtas_64s_float2int
    if(sf && !s && (ptype == 0) && (rmode == 0) && opcode==5) { return trans.fcvtau_float(args); } // -> fcvtau_64s_float2int
    if(sf && !s && (ptype == 0) && rmode==1 && (opcode == 0)) { return trans.fcvtps_float(args); } // -> fcvtps_64s_float2int
    if(sf && !s && (ptype == 0) && rmode==1 && opcode==1) { return trans.fcvtpu_float(args); } // -> fcvtpu_64s_float2int
    if(sf && !s && (ptype == 0) && rmode==2 && (opcode == 0)) { return trans.fcvtms_float(args); } // -> fcvtms_64s_float2int
    if(sf && !s && (ptype == 0) && rmode==2 && opcode==1) { return trans.fcvtmu_float(args); } // -> fcvtmu_64s_float2int
    if(sf && !s && (ptype == 0) && rmode==3 && (opcode == 0)) { return trans.fcvtzs_float_int(args); } // -> fcvtzs_64s_float2int
    if(sf && !s && (ptype == 0) && rmode==3 && opcode==1) { return trans.fcvtzu_float_int(args); } // -> fcvtzu_64s_float2int
    if(sf && !s && ptype==1 && (rmode == 0) && (opcode == 0)) { return trans.fcvtns_float(args); } // -> fcvtns_64d_float2int
    if(sf && !s && ptype==1 && (rmode == 0) && opcode==1) { return trans.fcvtnu_float(args); } // -> fcvtnu_64d_float2int
    if(sf && !s && ptype==1 && (rmode == 0) && opcode==2) { return trans.scvtf_float_int(args); } // -> scvtf_d64_float2int
    if(sf && !s && ptype==1 && (rmode == 0) && opcode==3) { return trans.ucvtf_float_int(args); } // -> ucvtf_d64_float2int
    if(sf && !s && ptype==1 && (rmode == 0) && opcode==4) { return trans.fcvtas_float(args); } // -> fcvtas_64d_float2int
    if(sf && !s && ptype==1 && (rmode == 0) && opcode==5) { return trans.fcvtau_float(args); } // -> fcvtau_64d_float2int
    if(sf && !s && ptype==1 && (rmode == 0) && opcode==6) { return trans.fmov_float_gen(args); } // -> fmov_64d_float2int
    if(sf && !s && ptype==1 && (rmode == 0) && opcode==7) { return trans.fmov_float_gen(args); } // -> fmov_d64_float2int
    if(sf && !s && ptype==1 && rmode==1 && (opcode == 0)) { return trans.fcvtps_float(args); } // -> fcvtps_64d_float2int
    if(sf && !s && ptype==1 && rmode==1 && opcode==1) { return trans.fcvtpu_float(args); } // -> fcvtpu_64d_float2int
    if(sf && !s && ptype==1 && rmode==2 && (opcode == 0)) { return trans.fcvtms_float(args); } // -> fcvtms_64d_float2int
    if(sf && !s && ptype==1 && rmode==2 && opcode==1) { return trans.fcvtmu_float(args); } // -> fcvtmu_64d_float2int
    if(sf && !s && ptype==1 && rmode==3 && (opcode == 0)) { return trans.fcvtzs_float_int(args); } // -> fcvtzs_64d_float2int
    if(sf && !s && ptype==1 && rmode==3 && opcode==1) { return trans.fcvtzu_float_int(args); } // -> fcvtzu_64d_float2int
    if(sf && !s && ptype==2 && rmode==1 && opcode==6) { return trans.fmov_float_gen(args); } // -> fmov_64vx_float2int
    if(sf && !s && ptype==2 && rmode==1 && opcode==7) { return trans.fmov_float_gen(args); } // -> fmov_v64i_float2int
    return false;
}
pub fn decode_cryptoaes<T: Arm64DecodeTrait>(trans: &mut T, insn: u32) -> bool {
    return false;
}
pub fn decode_crypto4<T: Arm64DecodeTrait>(trans: &mut T, insn: u32) -> bool {
    return false;
}
pub fn decode_cryptosha3<T: Arm64DecodeTrait>(trans: &mut T, insn: u32) -> bool {
    return false;
}
pub fn decode_cryptosha512_3<T: Arm64DecodeTrait>(trans: &mut T, insn: u32) -> bool {
    return false;
}
pub fn decode_crypto3_imm2<T: Arm64DecodeTrait>(trans: &mut T, insn: u32) -> bool {
    let args: ArmInstr = ArmInstr {
        insn
    };
    let opcode = (insn >> 10) & 3;
    if !trans.has_sm3() {
        return false;
    }
    if(opcode == 0) { return trans.sm3tt1a_advsimd(args); } // -> sm3tt1a_vvv4_crypto3_imm2
    if(opcode==1) { return trans.sm3tt1b_advsimd(args); } // -> sm3tt1b_vvv4_crypto3_imm2
    if(opcode==2) { return trans.sm3tt2a_advsimd(args); } // -> sm3tt2a_vvv4_crypto3_imm2
    if(opcode==3) { return trans.sm3tt2b_advsimd(args); } // -> sm3tt2b_vvv_crypto3_imm2
    return false;
}
pub fn decode_crypto3_imm6<T: Arm64DecodeTrait>(trans: &mut T, insn: u32) -> bool {
    let args: ArmInstr = ArmInstr {
        insn
    };
    return trans.xar_advsimd(args);
}
pub fn decode_cryptosha2<T: Arm64DecodeTrait>(trans: &mut T, insn: u32) -> bool {
    let args: ArmInstr = ArmInstr {
        insn
    };
    let opcode = (insn >> 12) & 0x1f;
    let size = (insn >> 22) & 3;
    if((size ==0) && (opcode == 0)) { return trans.sha1h_advsimd(args); } // -> sha1h_ss_cryptosha2
    if((size ==0) && opcode==1) { return trans.sha1su1_advsimd(args); } // -> sha1su1_vv_cryptosha2
    if((size ==0) && opcode==2) { return trans.sha256su0_advsimd(args); } // -> sha256su0_vv_cryptosha2

    return false;
}
pub fn decode_cryptosha512_2<T: Arm64DecodeTrait>(trans: &mut T, insn: u32) -> bool {
    let args: ArmInstr = ArmInstr {
        insn
    };
    let opcode = (insn >> 10) & 3;
    if((opcode == 0) && trans.has_sha512()) { return trans.sha512su0_advsimd(args); } // -> sha512su0_vv2_cryptosha512_2
    if(opcode==1 && trans.has_shm4()) { return trans.sm4e_advsimd(args); } // -> sm4e_vv4_cryptosha512_2
    return false;
}
pub fn decode_floatcmp<T: Arm64DecodeTrait>(trans: &mut T, insn: u32) -> bool {
    let args: ArmInstr = ArmInstr {
        insn
    };
    let ptype = (insn >> 22) & 3;
    let m = ((insn >> 31) & 1) != 0;
    let s = ((insn >> 29) & 1) != 0;
    let op = ((insn >> 14) & 3);
    let opcode2 = args.insn & 0x1f;
    if(!m && !s && (ptype == 0) && (op == 0) && (opcode2 == 0)) { return trans.fcmp_float(args); } // -> fcmp_s_floatcmp
    if(!m && !s && (ptype == 0) && (op == 0) && opcode2==8) { return trans.fcmp_float(args); } // -> fcmp_sz_floatcmp
    if(!m && !s && (ptype == 0) && (op == 0) && opcode2==0x10) { return trans.fcmpe_float(args); } // -> fcmpe_s_floatcmp
    if(!m && !s && (ptype == 0) && (op == 0) && opcode2==0x18) { return trans.fcmpe_float(args); } // -> fcmpe_sz_floatcmp
    if(!m && !s && ptype==1 && (op == 0) && (opcode2 == 0)) { return trans.fcmp_float(args); } // -> fcmp_d_floatcmp
    if(!m && !s && ptype==1 && (op == 0) && opcode2==8) { return trans.fcmp_float(args); } // -> fcmp_dz_floatcmp
    if(!m && !s && ptype==1 && (op == 0) && opcode2==0x10) { return trans.fcmpe_float(args); } // -> fcmpe_d_floatcmp
    if(!m && !s && ptype==1 && (op == 0) && opcode2==0x18) { return trans.fcmpe_float(args); } // -> fcmpe_dz_floatcmp
    if(!m && !s && ptype==3 && (op == 0) && (opcode2 == 0) && trans.has_fp16()) { return trans.fcmp_float(args); } // -> fcmp_h_floatcmp
    if(!m && !s && ptype==3 && (op == 0) && opcode2==8 && trans.has_fp16()) { return trans.fcmp_float(args); } // -> fcmp_hz_floatcmp
    if(!m && !s && ptype==3 && (op == 0) && opcode2==0x10 && trans.has_fp16()) { return trans.fcmpe_float(args); } // -> fcmpe_h_floatcmp
    if(!m && !s && ptype==3 && (op == 0) && opcode2==0x18 && trans.has_fp16()) { return trans.fcmpe_float(args); } // -> fcmpe_hz_floatcmp
    return false;
}
pub fn decode_floatccmp<T: Arm64DecodeTrait>(trans: &mut T, insn: u32) -> bool {
    let args: ArmInstr = ArmInstr {
        insn
    };
    let ptype = (insn >> 22) & 3;
    let m = ((insn >> 31) & 1) != 0;
    let s = ((insn >> 29) & 1) != 0;
    let op = ((insn >> 4) & 1) != 0;

    if(!m && !s && (ptype == 0) && !op) { return trans.fccmp_float(args); } // -> fccmp_s_floatccmp
    if(!m && !s && (ptype == 0) && op) { return trans.fccmpe_float(args); } // -> fccmpe_s_floatccmp
    if(!m && !s && ptype==1 && !op) { return trans.fccmp_float(args); } // -> fccmp_d_floatccmp
    if(!m && !s && ptype==1 && op) { return trans.fccmpe_float(args); } // -> fccmpe_d_floatccmp
    if(!m && !s && ptype==3 && !op && trans.has_fp16()) { return trans.fccmp_float(args); } // -> fccmp_h_floatccmp
    if(!m && !s && ptype==3 && op && trans.has_fp16()) { return trans.fccmpe_float(args); } // -> fccmpe_h_floatccmp
    return false;
}
pub fn decode_floatsel<T: Arm64DecodeTrait>(trans: &mut T, insn: u32) -> bool {
    let args: ArmInstr = ArmInstr {
        insn
    };
    let ptype = (insn >> 22) & 3;
    let m = ((insn >> 31) & 1) != 0;
    let s = ((insn >> 29) & 1) != 0;
    if(!m && !s && (ptype == 0)) { return trans.fcsel_float(args); } // -> fcsel_s_floatsel
    if(!m && !s && ptype==1) { return trans.fcsel_float(args); } // -> fcsel_d_floatsel
    if(!m && !s && ptype==3 && trans.has_fp16()) { return trans.fcsel_float(args); } // -> fcsel_h_floatsel

    return false;
}
pub fn decode_floatdp1<T: Arm64DecodeTrait>(trans: &mut T, insn: u32) -> bool {
    let args: ArmInstr = ArmInstr {
        insn
    };
    let opcode = (insn >> 15) & 0x3f;
    let ptype = (insn >> 22) & 3;
    let m = ((insn >> 31) & 1) != 0;
    let s = ((insn >> 29) & 1) != 0;
    if(!m && !s && (ptype == 0) && (opcode == 0)) { return trans.fmov_float(args); } // -> fmov_s_floatdp1
    if(!m && !s && (ptype == 0) && opcode==1) { return trans.fabs_float(args); } // -> fabs_s_floatdp1
    if(!m && !s && (ptype == 0) && opcode==2) { return trans.fneg_float(args); } // -> fneg_s_floatdp1
    if(!m && !s && (ptype == 0) && opcode==3) { return trans.fsqrt_float(args); } // -> fsqrt_s_floatdp1
    if(!m && !s && (ptype == 0) && opcode==5) { return trans.fcvt_float(args); } // -> fcvt_ds_floatdp1
    if(!m && !s && (ptype == 0) && opcode==7) { return trans.fcvt_float(args); } // -> fcvt_hs_floatdp1
    if(!m && !s && (ptype == 0) && opcode==8) { return trans.frintn_float(args); } // -> frintn_s_floatdp1
    if(!m && !s && (ptype == 0) && opcode==9) { return trans.frintp_float(args); } // -> frintp_s_floatdp1
    if(!m && !s && (ptype == 0) && opcode==10) { return trans.frintm_float(args); } // -> frintm_s_floatdp1
    if(!m && !s && (ptype == 0) && opcode==11) { return trans.frintz_float(args); } // -> frintz_s_floatdp1
    if(!m && !s && (ptype == 0) && opcode==12) { return trans.frinta_float(args); } // -> frinta_s_floatdp1
    if(!m && !s && (ptype == 0) && opcode==14) { return trans.frintx_float(args); } // -> frintx_s_floatdp1
    if(!m && !s && (ptype == 0) && opcode==15) { return trans.frinti_float(args); } // -> frinti_s_floatdp1
    if(!m && !s && (ptype == 0) && opcode==0x10 && trans.has_frints_feat()) { return trans.frint32z_float(args); } // -> frint32z_s_floatdp1
    if(!m && !s && (ptype == 0) && opcode==0x11 && trans.has_frints_feat()) { return trans.frint32x_float(args); } // -> frint32x_s_floatdp1
    if(!m && !s && (ptype == 0) && opcode==0x12 && trans.has_frints_feat()) { return trans.frint64z_float(args); } // -> frint64z_s_floatdp1
    if(!m && !s && (ptype == 0) && opcode==0x13 && trans.has_frints_feat()) { return trans.frint64x_float(args); } // -> frint64x_s_floatdp1
    if(!m && !s && ptype==1 && (opcode == 0)) { return trans.fmov_float(args); } // -> fmov_d_floatdp1
    if(!m && !s && ptype==1 && opcode==1) { return trans.fabs_float(args); } // -> fabs_d_floatdp1
    if(!m && !s && ptype==1 && opcode==2) { return trans.fneg_float(args); } // -> fneg_d_floatdp1
    if(!m && !s && ptype==1 && opcode==3) { return trans.fsqrt_float(args); } // -> fsqrt_d_floatdp1
    if(!m && !s && ptype==1 && opcode==4) { return trans.fcvt_float(args); } // -> fcvt_sd_floatdp1
    if(!m && !s && ptype==1 && opcode==6 && trans.has_bf16()) { return trans.bfcvt_float(args); } // -> bfcvt_bs_floatdp1
    if(!m && !s && ptype==1 && opcode==7) { return trans.fcvt_float(args); } // -> fcvt_hd_floatdp1
    if(!m && !s && ptype==1 && opcode==8) { return trans.frintn_float(args); } // -> frintn_d_floatdp1
    if(!m && !s && ptype==1 && opcode==9) { return trans.frintp_float(args); } // -> frintp_d_floatdp1
    if(!m && !s && ptype==1 && opcode==10) { return trans.frintm_float(args); } // -> frintm_d_floatdp1
    if(!m && !s && ptype==1 && opcode==11) { return trans.frintz_float(args); } // -> frintz_d_floatdp1
    if(!m && !s && ptype==1 && opcode==12) { return trans.frinta_float(args); } // -> frinta_d_floatdp1
    if(!m && !s && ptype==1 && opcode==14) { return trans.frintx_float(args); } // -> frintx_d_floatdp1
    if(!m && !s && ptype==1 && opcode==15) { return trans.frinti_float(args); } // -> frinti_d_floatdp1
    if(!m && !s && ptype==1 && opcode==0x10 && trans.has_frints_feat()) { return trans.frint32z_float(args); } // -> frint32z_d_floatdp1
    if(!m && !s && ptype==1 && opcode==0x11 && trans.has_frints_feat()) { return trans.frint32x_float(args); } // -> frint32x_d_floatdp1
    if(!m && !s && ptype==1 && opcode==0x12 && trans.has_frints_feat()) { return trans.frint64z_float(args); } // -> frint64z_d_floatdp1
    if(!m && !s && ptype==1 && opcode==0x13 && trans.has_frints_feat()) { return trans.frint64x_float(args); } // -> frint64x_d_floatdp1
    if(!m && !s && ptype==3 && (opcode == 0) && trans.has_fp16()) { return trans.fmov_float(args); } // -> fmov_h_floatdp1
    if(!m && !s && ptype==3 && opcode==1 && trans.has_fp16()) { return trans.fabs_float(args); } // -> fabs_h_floatdp1
    if(!m && !s && ptype==3 && opcode==2 && trans.has_fp16()) { return trans.fneg_float(args); } // -> fneg_h_floatdp1
    if(!m && !s && ptype==3 && opcode==3 && trans.has_fp16()) { return trans.fsqrt_float(args); } // -> fsqrt_h_floatdp1
    if(!m && !s && ptype==3 && opcode==4) { return trans.fcvt_float(args); } // -> fcvt_sh_floatdp1
    if(!m && !s && ptype==3 && opcode==5) { return trans.fcvt_float(args); } // -> fcvt_dh_floatdp1
    if(!m && !s && ptype==3 && opcode==8 && trans.has_fp16()) { return trans.frintn_float(args); } // -> frintn_h_floatdp1
    if(!m && !s && ptype==3 && opcode==9 && trans.has_fp16()) { return trans.frintp_float(args); } // -> frintp_h_floatdp1
    if(!m && !s && ptype==3 && opcode==10 && trans.has_fp16()) { return trans.frintm_float(args); } // -> frintm_h_floatdp1
    if(!m && !s && ptype==3 && opcode==11 && trans.has_fp16()) { return trans.frintz_float(args); } // -> frintz_h_floatdp1
    if(!m && !s && ptype==3 && opcode==12 && trans.has_fp16()) { return trans.frinta_float(args); } // -> frinta_h_floatdp1
    if(!m && !s && ptype==3 && opcode==14 && trans.has_fp16()) { return trans.frintx_float(args); } // -> frintx_h_floatdp1
    if(!m && !s && ptype==3 && opcode==15 && trans.has_fp16()) { return trans.frinti_float(args); } // -> frinti_h_floatdp1
    return false;
}
pub fn decode_floatdp2<T: Arm64DecodeTrait>(trans: &mut T, insn: u32) -> bool {
    let args: ArmInstr = ArmInstr {
        insn
    };
    let opcode = (insn >> 12) & 15;
    let ptype = (insn >> 22) & 3;
    let m = ((insn >> 31) & 1) != 0;
    let s = ((insn >> 29) & 1) != 0;
    if(!m && !s && (ptype == 0) && (opcode == 0)) { return trans.fmul_float(args); } // -> fmul_s_floatdp2
    if(!m && !s && (ptype == 0) && opcode==1) { return trans.fdiv_float(args); } // -> fdiv_s_floatdp2
    if(!m && !s && (ptype == 0) && opcode==2) { return trans.fadd_float(args); } // -> fadd_s_floatdp2
    if(!m && !s && (ptype == 0) && opcode==3) { return trans.fsub_float(args); } // -> fsub_s_floatdp2
    if(!m && !s && (ptype == 0) && opcode==4) { return trans.fmax_float(args); } // -> fmax_s_floatdp2
    if(!m && !s && (ptype == 0) && opcode==5) { return trans.fmin_float(args); } // -> fmin_s_floatdp2
    if(!m && !s && (ptype == 0) && opcode==6) { return trans.fmaxnm_float(args); } // -> fmaxnm_s_floatdp2
    if(!m && !s && (ptype == 0) && opcode==7) { return trans.fminnm_float(args); } // -> fminnm_s_floatdp2
    if(!m && !s && (ptype == 0) && opcode==8) { return trans.fnmul_float(args); } // -> fnmul_s_floatdp2
    if(!m && !s && ptype==1 && (opcode == 0)) { return trans.fmul_float(args); } // -> fmul_d_floatdp2
    if(!m && !s && ptype==1 && opcode==1) { return trans.fdiv_float(args); } // -> fdiv_d_floatdp2
    if(!m && !s && ptype==1 && opcode==2) { return trans.fadd_float(args); } // -> fadd_d_floatdp2
    if(!m && !s && ptype==1 && opcode==3) { return trans.fsub_float(args); } // -> fsub_d_floatdp2
    if(!m && !s && ptype==1 && opcode==4) { return trans.fmax_float(args); } // -> fmax_d_floatdp2
    if(!m && !s && ptype==1 && opcode==5) { return trans.fmin_float(args); } // -> fmin_d_floatdp2
    if(!m && !s && ptype==1 && opcode==6) { return trans.fmaxnm_float(args); } // -> fmaxnm_d_floatdp2
    if(!m && !s && ptype==1 && opcode==7) { return trans.fminnm_float(args); } // -> fminnm_d_floatdp2
    if(!m && !s && ptype==1 && opcode==8) { return trans.fnmul_float(args); } // -> fnmul_d_floatdp2
    if(!m && !s && ptype==3 && (opcode == 0) && trans.has_fp16()) { return trans.fmul_float(args); } // -> fmul_h_floatdp2
    if(!m && !s && ptype==3 && opcode==1 && trans.has_fp16()) { return trans.fdiv_float(args); } // -> fdiv_h_floatdp2
    if(!m && !s && ptype==3 && opcode==2 && trans.has_fp16()) { return trans.fadd_float(args); } // -> fadd_h_floatdp2
    if(!m && !s && ptype==3 && opcode==3 && trans.has_fp16()) { return trans.fsub_float(args); } // -> fsub_h_floatdp2
    if(!m && !s && ptype==3 && opcode==4 && trans.has_fp16()) { return trans.fmax_float(args); } // -> fmax_h_floatdp2
    if(!m && !s && ptype==3 && opcode==5 && trans.has_fp16()) { return trans.fmin_float(args); } // -> fmin_h_floatdp2
    if(!m && !s && ptype==3 && opcode==6 && trans.has_fp16()) { return trans.fmaxnm_float(args); } // -> fmaxnm_h_floatdp2
    if(!m && !s && ptype==3 && opcode==7 && trans.has_fp16()) { return trans.fminnm_float(args); } // -> fminnm_h_floatdp2
    if(!m && !s && ptype==3 && opcode==8 && trans.has_fp16()) { return trans.fnmul_float(args); } // -> fnmul_h_floatdp2
    return false;
}
pub fn decode_floatdp3<T: Arm64DecodeTrait>(trans: &mut T, insn: u32) -> bool {
    let args: ArmInstr = ArmInstr {
        insn
    };
    let m = ((insn >> 31) & 1) != 0;
    let s = ((insn >> 29) & 1) != 0;
    let ptype = (insn >> 22) & 3;
    let o1 = ((insn >> 21) & 1) != 0;
    let o0 = ((insn >> 15) & 1) != 0;
    if(!m && !s && (ptype == 0) && !o1 && !o0) { return trans.fmadd_float(args); } // -> fmadd_s_floatdp3
    if(!m && !s && (ptype == 0) && !o1 && o0) { return trans.fmsub_float(args); } // -> fmsub_s_floatdp3
    if(!m && !s && (ptype == 0) && o1 && !o0) { return trans.fnmadd_float(args); } // -> fnmadd_s_floatdp3
    if(!m && !s && (ptype == 0) && o1 && o0) { return trans.fnmsub_float(args); } // -> fnmsub_s_floatdp3
    if(!m && !s && ptype==1 && !o1 && !o0) { return trans.fmadd_float(args); } // -> fmadd_d_floatdp3
    if(!m && !s && ptype==1 && !o1 && o0) { return trans.fmsub_float(args); } // -> fmsub_d_floatdp3
    if(!m && !s && ptype==1 && o1 && !o0) { return trans.fnmadd_float(args); } // -> fnmadd_d_floatdp3
    if(!m && !s && ptype==1 && o1 && o0) { return trans.fnmsub_float(args); } // -> fnmsub_d_floatdp3
    if(!m && !s && ptype==3 && !o1 && !o0 && trans.has_fp16()) { return trans.fmadd_float(args); } // -> fmadd_h_floatdp3
    if(!m && !s && ptype==3 && !o1 && o0 && trans.has_fp16()) { return trans.fmsub_float(args); } // -> fmsub_h_floatdp3
    if(!m && !s && ptype==3 && o1 && !o0 && trans.has_fp16()) { return trans.fnmadd_float(args); } // -> fnmadd_h_floatdp3
    if(!m && !s && ptype==3 && o1 && o0 && trans.has_fp16()) { return trans.fnmsub_float(args); } // -> fnmsub_h_floatdp3
    return false;
}
pub fn decode_floatimm<T: Arm64DecodeTrait>(trans: &mut T, insn: u32) -> bool {
    let args: ArmInstr = ArmInstr {
        insn
    };
    let m = ((insn >> 31) & 1) != 0;
    let s = ((insn >> 29) & 1) != 0;
    let imm5 = (insn>>5)&0x1f;
    let ptype = (insn>>22)&3;

    if (!m && !s && (ptype == 0) && (imm5 == 0)) { return trans.fmov_float_imm(args); }
    if (!m && !s && (ptype == 1) && (imm5 == 0)) { return trans.fmov_float_imm(args); }
    if (!m && !s && (ptype == 3) && (imm5 == 0) && trans.has_fp16()) { return trans.fmov_float_imm(args); }
    return false;
}
pub fn decode_perm_undef<T: Arm64DecodeTrait>(trans: &mut T, insn: u32) -> bool {
    return false;
}