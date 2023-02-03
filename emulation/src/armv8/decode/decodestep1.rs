use crate::armv8::decodedefs::*;
use crate::armv8::decoder64::Arm64DecodeTrait;
use crate::armv8::decode::decodestep2::*;
pub fn root_decode<T: Arm64DecodeTrait>(trans: &mut T, insn: u32) -> bool {
    let op0 = insn >> 31;
    let op1 = (insn >> 25) & 15;
    if (op0 == 0) && (op1 == 0) {
        let op00 = (insn >> 29) & 3;
        let op11 = (insn >> 16) & 0x1ff;
        return if (op00 == 0) && (op11 == 0) {
            decode_undef(trans, insn)
        } else {
            false
        }
    }
    if (op0 != 0) && (op1 == 0) {
        return false;
        // sme
    }
    if op1 == 1 || op1 == 3 {
        return false;
    }
    if op1 == 2 {
        // sve
        return false;
    }

    if (op1 & 14) == 8 {
        // data-processsing immediate
        let op00 = (insn>>23) & 7;
        if (op00 & 6) == 0 {
            return decode_pcreladdr(trans, insn);
        }
        return match op00 {
            2 => decode_addsub_imm(trans, insn),
            3 => decode_addsub_immtags(trans, insn),
            4 => decode_log_imm(trans, insn),
            5 => decode_movewide(trans, insn),
            6 => decode_bitfield(trans, insn),
            7 => decode_extract(trans, insn),
            _ => false
        }
    }
    if (op1&14) == 10 {
        // control
        let op00 = insn>>29;
        let op11 = (insn>>12)&0x3fff;
        let op22 = insn&0x1f;
        if op00==2 && ((op11&0x2000) == 0) {
            return decode_condbranch(trans, insn);
        }
        if op00==6 && ((op11&0x3000) == 0) {
            return decode_exception(trans, insn);
        }
        if op00==6 && op11==0x1031 {
            return decode_systeminstrswithreg(trans, insn);
        }
        if op00==6 && op11==0x1032 && op22==0x1f {
            return decode_hints(trans, insn);
        }
        if op00==6 && op11==0x1033 {
            return decode_barriers(trans, insn);
        }
        if op00==6 && (op11&0x3f8f)==0x1004 {
            return decode_pstate(trans, insn);
        }
        if op00==6 && (op11&0x3f80)==0x1200 {
            return decode_systemresult(trans, insn);
        }
        if op00==6 && (op11&0x3d80)==0x1080 {
            return decode_systeminstrs(trans, insn);
        }
        if op00==6 && (op11&0x3d00)==0x1100 {
            return decode_systemmove(trans, insn);
        }
        if op00==6 && (op11&0x2000)==0x2000 {
            return decode_branch_reg(trans, insn);
        }
        if (op00 & 3) == 0 {
            return decode_branch_imm(trans, insn);
        }
        if ((op00 & 3) == 1) && ((op11 & 0x2000) == 0) {
            return decode_compbranch(trans, insn);
        }
        if ((op00 & 3) == 1) && ((op11 & 0x2000) == 0x2000) {
            return decode_testbranch(trans, insn);
        }
        return false;
    }
    if (op1&5)==4 {
        // ldst
        let op00 = insn>>28;
        let op11 = (insn>>26)&1;
        let op22 = (insn>>23)&3;
        let op33 = (insn>>16)&0x3f;
        let op44 = (insn>>10)&3;
        if ((op00&11) == 0) && (op11 == 0) && (op22 == 0) && (op33&0x20)==0x20 {
            return decode_comswappr(trans, insn);
        }
        if ((op00&11) == 0) && (op11 != 0) && (op22 == 0) && (op33 == 0) {
            return decode_asisdlse(trans, insn);
        }
        if ((op00&11) == 0) && (op11 != 0) && op22==1 && ((op33&0x20) == 0) {
            return decode_asisdlsep(trans, insn);
        }
        if ((op00&11) == 0) && (op11 != 0) && op22==2 && ((op33&0x1f) == 0) {
            return decode_asisdlso(trans, insn);
        }
        if ((op00&11) == 0) && (op11 != 0) && op22==3 {
            return decode_asisdlsop(trans, insn);
        }
        if op00==13 && (op11 == 0) && (op22&2)==2 && (op33&0x20)==0x20 {
            return decode_ldsttags(trans, insn);
        }
        if (op00&11)==8 && (op11 == 0) && (op22 == 0) && (op33&0x20)==0x20 {
            return decode_ldstexclp(trans, insn);
        }
        if ((op00&3) == 0) && (op11 == 0) && (op22 == 0) && ((op33&0x20) == 0) {
            return decode_ldstexclr(trans, insn);
        }
        if ((op00&3) == 0) && (op11 == 0) && op22==1 && ((op33&0x20) == 0) {
            return decode_ldstord(trans, insn);
        }
        if ((op00&3) == 0) && (op11 == 0) && op22==1 && (op33&0x20)==0x20 {
            return decode_comswap(trans, insn);
        }
        if (op00&3)==1 && (op11 == 0) && (op22&2)==2
            && ((op33&0x20) == 0) && (op44 == 0)
        {
            return decode_ldapstl_unscaled(trans, insn);
        }
        if (op00&3)==1 && ((op22&2) == 0)
        {
            return decode_loadlit(trans, insn);
        }
        if (op00&3)==2 && (op22 == 0)
        {
            return decode_ldstnapair_offs(trans, insn);
        }
        if (op00&3)==2 && op22==1
        {
            return decode_ldstpair_post(trans, insn);
        }
        if (op00&3)==2 && op22==2
        {
            return decode_ldstpair_off(trans, insn);
        }
        if (op00&3)==2 && op22==3
        {
            return decode_ldstpair_pre(trans, insn);
        }
        if (op00&3)==3 && ((op22&2) == 0) && ((op33&0x20) == 0) && (op44 == 0)
        {
            return decode_ldst_unscaled(trans, insn);
        }
        if (op00&3)==3 && ((op22&2) == 0) && ((op33&0x20) == 0) && op44==1
        {
            return decode_ldst_immpost(trans, insn);
        }
        if (op00&3)==3 && ((op22&2) == 0) && ((op33&0x20) == 0) && op44==2
        {
            return decode_ldst_unpriv(trans, insn);
        }
        if (op00&3)==3 && ((op22&2) == 0) && ((op33&0x20) == 0) && op44==3
        {
            return decode_ldst_immpre(trans, insn);
        }
        if (op00&3)==3 && ((op22&2) == 0) && (op33&0x20)==0x20 && (op44 == 0)
        {
            return decode_memop(trans, insn);
        }
        if (op00&3)==3 && ((op22&2) == 0) && (op33&0x20)==0x20 && op44==2
        {
            return decode_ldst_regoff(trans, insn);
        }
        if (op00&3)==3 && ((op22&2) == 0) && (op33&0x20)==0x20 && ((op44&1) != 0)
        {
            return decode_ldst_pac(trans, insn);
        }
        if (op00&3)==3 && (op22&2)==2
        {
            return decode_ldst_pos(trans, insn);
        }
        return false;
    }
    if (op1&7)==5 {
        // dpreg
        let op0 = (insn>>30)&1;
        let op1 = (insn>>28)&1;
        let op2 = (insn>>21)&15;
        let op3 = (insn>>10)&0x3f;
        if (op0 == 0) && (op1 != 0) && op2==6
        {
            return decode_dp_2src(trans, insn);
        }
        if (op0 != 0) && (op1 != 0) && op2==6
        {
            return decode_dp_1src(trans, insn);
        }
        if (op1 == 0) && ((op2&8) == 0)
        {
            return decode_log_shift(trans, insn);
        }
        if (op1 == 0) && (op2&9)==8
        {
            return decode_addsub_shift(trans, insn);
        }
        if (op1 == 0) && (op2&9)==9
        {
            return decode_addsub_ext(trans, insn);
        }
        if (op1 != 0) && (op2 == 0) && (op3 == 0)
        {
            return decode_addsub_carry(trans, insn);
        }
        if (op1 != 0) && (op2 == 0) && (op3&0x1f)==1
        {
            return decode_rmif(trans, insn);
        }
        if (op1 != 0) && (op2 == 0) && (op3&15)==2
        {
            return decode_setf(trans, insn);
        }
        if (op1 != 0) && op2==2 && ((op3&2) == 0)
        {
            return decode_condcmp_reg(trans, insn);
        }
        if (op1 != 0) && op2==2 && (op3&2)==2
        {
            return decode_condcmp_imm(trans, insn);
        }
        if (op1 != 0) && op2==4
        {
            return decode_condsel(trans, insn);
        }
        if (op1 != 0) && (op2&8)==8
        {
            return decode_dp_3src(trans, insn);
        }
    }
    if (op1&7)==7 {
        // simd_dp
        let op0 = insn>>28;
        let op1 = (insn>>23)&3;
        let op2 = (insn>>19)&15;
        let op3 = (insn>>10)&0x1ff;
        if op0==4 && ((op1&2) == 0) && (op2&7)==5 && (op3&0x183)==2
        {
            return decode_cryptoaes(trans, insn);
        }
        if(op0==5 && ((op1&2) == 0) && ((op2&4) == 0) && ((op3&0x23) == 0))
        {
            return decode_cryptosha3(trans, insn);
        }
        if(op0==5 && ((op1&2) == 0) && (op2&7)==5 && (op3&0x183)==2)
        {
            return decode_cryptosha2(trans, insn);
        }
        if((op0&13)==5 && (op1 == 0) && ((op2&12) == 0) && (op3&0x21)==1)
        {
            return decode_asisdone(trans, insn);
        }
        if((op0&13)==5 && ((op1&2) == 0) && (op2&12)==8 && (op3&0x31)==1)
        {
            return decode_asisdsamefp16(trans, insn);
        }
        if((op0&13)==5 && ((op1&2) == 0) && op2==15 && (op3&0x183)==2)
        {
            return decode_asisdmiscfp16(trans, insn);
        }
        if((op0&13)==5 && ((op1&2) == 0) && ((op2&4) == 0) && (op3&0x21)==0x21)
        {
            return decode_asisdsame2(trans, insn);
        }
        if((op0&13)==5 && ((op1&2) == 0) && (op2&7)==4 && (op3&0x183)==2)
        {
            return decode_asisdmisc(trans, insn);
        }
        if((op0&13)==5 && ((op1&2) == 0) && (op2&7)==6 && (op3&0x183)==2)
        {
            return decode_asisdpair(trans, insn);
        }
        if((op0&13)==5 && ((op1&2) == 0) && (op2&4)==4 && ((op3&3) == 0))
        {
            return decode_asisddiff(trans, insn);
        }
        if((op0&13)==5 && ((op1&2) == 0) && (op2&4)==4 && ((op3&1) != 0))
        {
            return decode_asisdsame(trans, insn);
        }
        if((op0&13)==5 && op1==2 && ((op3&1) != 0))
        {
            return decode_asisdshf(trans, insn);
        }
        if((op0&13)==5 && (op1&2)==2 && ((op3&1) == 0))
        {
            return decode_asisdelem(trans, insn);
        }
        if(((op0&11) == 0) && ((op1&2) == 0) && ((op2&4) == 0) && ((op3&0x23) == 0))
        {
            return decode_asimdtbl(trans, insn);
        }
        if(((op0&11) == 0) && ((op1&2) == 0) && ((op2&4) == 0) && (op3&0x23)==2)
        {
            return decode_asimdperm(trans, insn);
        }
        if((op0&11)==2 && ((op1&2) == 0) && ((op2&4) == 0) && ((op3&0x21) == 0))
        {
            return decode_asimdext(trans, insn);
        }
        if(((op0&9) == 0) && (op1 == 0) && ((op2&12) == 0) && (op3&0x21)==1)
        {
            return decode_asimdins(trans, insn);
        }
        if(((op0&9) == 0) && ((op1&2) == 0) && (op2&12)==8 && (op3&0x31)==1)
        {
            return decode_asimdsamefp16(trans, insn);
        }
        if(((op0&9) == 0) && ((op1&2) == 0) && op2==15 && (op3&0x183)==2)
        {
            return decode_asimdmiscfp16(trans, insn);
        }
        if(((op0&9) == 0) && ((op1&2) == 0) && ((op2&4) == 0) && (op3&0x21)==0x21)
        {
            return decode_asimdsame2(trans, insn);
        }
        if(((op0&9) == 0) && ((op1&2) == 0) && (op2&7)==4 && (op3&0x183)==2)
        {
            return decode_asimdmisc(trans, insn);
        }
        if(((op0&9) == 0) && ((op1&2) == 0) && (op2&7)==6 && (op3&0x183)==2)
        {
            return decode_asimdall(trans, insn);
        }
        if(((op0&9) == 0) && ((op1&2) == 0) && (op2&4)==4 && ((op3&3) == 0))
        {
            return decode_asimddiff(trans, insn);
        }
        if ((op0&9) == 0) && ((op1&2) == 0) && (op2&4)==4 && ((op3&1) != 0)
        {
            return decode_asimdsame(trans, insn);
        }
        if(((op0&9) == 0) && op1==2 && (op2 == 0) && (op3&1 != 0))
        {
            return decode_asimdimm(trans, insn);
        }
        if(((op0&9) == 0) && op1==2 && (op2 != 0) && (op3&1 != 0))
        {
            return decode_asimdshf(trans, insn);
        }
        if(((op0&9) == 0) && (op1&2)==2 && ((op3&1) == 0))
        {
            return decode_asimdelem(trans, insn);
        }
        if(op0==12 && (op1 == 0) && (op2&12)==8 && (op3&0x30)==0x20)
        {
            return decode_crypto3_imm2(trans, insn);
        }
        if(op0==12 && (op1 == 0) && (op2&12)==12 && (op3&0x2c)==0x20)
        {
            return decode_cryptosha512_3(trans, insn);
        }
        if(op0==12 && (op1 == 0) && ((op3&0x20) == 0))
        {
            return decode_crypto4(trans, insn);
        }
        if(op0==12 && op1==1 && ((op2&12) == 0))
        {
            return decode_crypto3_imm6(trans, insn);
        }
        if(op0==12 && op1==1 && op2==8 && (op3&0x1fc)==0x20)
        {
            return decode_cryptosha512_2(trans, insn);
        }
        if((op0&5)==1 && ((op1&2) == 0) && ((op2&4) == 0))
        {
            return decode_float2fix(trans, insn);
        }
        if((op0&5)==1 && ((op1&2) == 0) && (op2&4)==4 && ((op3&0x3f) == 0))
        {
            return decode_float2int(trans, insn);
        }
        if((op0&5)==1 && ((op1&2) == 0) && (op2&4)==4 && (op3&0x1f)==0x10)
        {
            return decode_floatdp1(trans, insn);
        }
        if((op0&5)==1 && ((op1&2) == 0) && (op2&4)==4 && (op3&15)==8)
        {
            return decode_floatcmp(trans, insn);
        }
        if((op0&5)==1 && ((op1&2) == 0) && (op2&4)==4 && (op3&7)==4)
        {
            return decode_floatimm(trans, insn);
        }
        if((op0&5)==1 && ((op1&2) == 0) && (op2&4)==4 && (op3&3)==1)
        {
            return decode_floatccmp(trans, insn);
        }
        if((op0&5)==1 && ((op1&2) == 0) && (op2&4)==4 && (op3&3)==2)
        {
            return decode_floatdp2(trans, insn);
        }
        if((op0&5)==1 && ((op1&2) == 0) && (op2&4)==4 && (op3&3)==3)
        {
            return decode_floatsel(trans, insn);
        }
        if((op0&5)==1 && (op1&2)==2)
        {
            return decode_floatdp3(trans, insn);
        }
    }
    false
}