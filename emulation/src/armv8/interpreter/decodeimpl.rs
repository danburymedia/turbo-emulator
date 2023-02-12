use crate::armv8::decodedefs::ArmInstr;
use crate::armv8::decoder64::Arm64DecodeTrait;
pub use crate::armv8::interpreter::defs;
use crate::armv8::interpreter::main::Arm64Cpu;

impl Arm64DecodeTrait for Arm64Cpu {
    fn udiv(&mut self, args: ArmInstr) -> bool {
        defs::udiv(self, &args);
        return true;
    }
    fn sdiv(&mut self, args: ArmInstr) -> bool {
        defs::sdiv(self, &args);
        return true;
    }
    fn lslv(&mut self, args: ArmInstr) -> bool {
        defs::lslv(self, &args);
        return true;
    }
    fn lsrv(&mut self, args: ArmInstr) -> bool {
        defs::lsrv(self, &args);
        return true;
    }
    fn asrv(&mut self, args: ArmInstr) -> bool {
        defs::asrv(self, &args);
        return true;
    }
    fn rorv(&mut self, args: ArmInstr) -> bool {
        defs::rorv(self, &args);
        return true;
    }
    fn rbit_int(&mut self, args: ArmInstr) -> bool {
        defs::rbit_int(self, &args);
        return true;
    }
    fn blr(&mut self, args: ArmInstr) -> bool {
        defs::blr(self, &args);
        return true;
    }
    fn ands_log_imm(&mut self, args: ArmInstr) -> bool {
        defs::ands_log_imm(self, &args);
        return true;
    }
    fn rev16_int(&mut self, args: ArmInstr) -> bool {
        defs::rev16_int(self, &args);
        return true;
    }
    fn rev32_int(&mut self, args: ArmInstr) -> bool {
        defs::rev32_int(self, &args);
        return true;
    }
    fn clz_int(&mut self, args: ArmInstr) -> bool {
        defs::clz_int(self, &args);
        return true;
    }
    fn and_log_shift(&mut self, args: ArmInstr) -> bool {
        defs::and_log_shift(self, &args);
        return true;
    }
    fn bic_log_shift(&mut self, args: ArmInstr) -> bool {
        defs::bic_log_shift(self, &args);
        return true;
    }
    fn orr_log_shift(&mut self, args: ArmInstr) -> bool {
        defs::orr_log_shift(self, &args);
        return true;
    }
    fn dup_advsimd_gen(&mut self, args: ArmInstr) -> bool {
        defs::dup_advsimd_gen(self, &args);
        return true;
    }
    fn str_reg_fpsimd(&mut self, args: ArmInstr) -> bool {
        defs::str_reg_fpsimd(self, &args);
        return true;
    }
    fn ldr_reg_fpsimd(&mut self, args: ArmInstr) -> bool {
        defs::ldr_reg_fpsimd(self, &args);
        return true;
    }
    fn orn_log_shift(&mut self, args: ArmInstr) -> bool {
        defs::orn_log_shift(self, &args);
        return true;
    }
    fn eor_log_shift(&mut self, args: ArmInstr) -> bool {
        defs::eor_log_shift(self, &args);
        return true;
    }
    fn eon(&mut self, args: ArmInstr) -> bool {
        defs::eon(self, &args);
        return true;
    }
    fn ands_log_shift(&mut self, args: ArmInstr) -> bool {
        defs::ands_log_shift(self, &args);
        return true;
    }
    fn bics(&mut self, args: ArmInstr) -> bool {
        defs::bics(self, &args);
        return true;
    }
    fn add_addsub_shift(&mut self, args: ArmInstr) -> bool {
        defs::add_addsub_shift(self, &args);
        return true;
    }
    fn str_imm_fpsimd(&mut self, args: ArmInstr) -> bool {
        defs::str_imm_fpsimd(self, &args);
        return true;
    }
    fn ldr_imm_fpsimd(&mut self, args: ArmInstr) -> bool {
        defs::ldr_imm_fpsimd(self, &args);
        return true;
    }
    fn ldp_fpsimd(&mut self, args: ArmInstr) -> bool {
        defs::ldp_fpsimd(self, &args);
        return true;
    }
    fn stp_fpsimd(&mut self, args: ArmInstr) -> bool {
        defs::stp_fpsimd(self, &args);
        return true;
    }
    fn ret(&mut self, args: ArmInstr) -> bool {
        defs::ret(self, &args);
        return true;
    }
    fn ccmp_imm(&mut self, args: ArmInstr) -> bool {
        defs::ccmp_imm(self, &args);
        return true;
    }
    fn mrs(&mut self, args: ArmInstr) -> bool {
        defs::mrs(self, &args);
        return true;
    }
    fn br(&mut self, args: ArmInstr) -> bool {
        defs::br(self, &args);
        return true;
    }
    fn adds_addsub_shift(&mut self, args: ArmInstr) -> bool {
        defs::adds_addsub_shift(self, &args);
        return true;
    }
    fn sub_addsub_shift(&mut self, args: ArmInstr) -> bool {
        defs::sub_addsub_shift(self, &args);
        return true;
    }
    fn subs_addsub_shift(&mut self, args: ArmInstr) -> bool {
        defs::subs_addsub_shift(self, &args);
        return true;
    }
    fn add_addsub_ext(&mut self, args: ArmInstr) -> bool {
        defs::add_addsub_ext(self, &args);
        return true;
    }
    fn adds_addsub_ext(&mut self, args: ArmInstr) -> bool {
        defs::adds_addsub_ext(self, &args);
        return true;
    }
    fn sub_addsub_ext(&mut self, args: ArmInstr) -> bool {
        defs::sub_addsub_ext(self, &args);
        return true;
    }
    fn subs_addsub_ext(&mut self, args: ArmInstr) -> bool {
        defs::subs_addsub_ext(self, &args);
        return true;
    }
    fn adc(&mut self, args: ArmInstr) -> bool {
        defs::adc(self, &args);
        return true;
    }
    fn adcs(&mut self, args: ArmInstr) -> bool {
        defs::adcs(self, &args);
        return true;
    }
    fn sbc(&mut self, args: ArmInstr) -> bool {
        defs::sbc(self, &args);
        return true;
    }
    fn sbcs(&mut self, args: ArmInstr) -> bool {
        defs::sbcs(self, &args);
        return true;
    }
    fn ccmn_reg(&mut self, args: ArmInstr) -> bool {
        defs::ccmn_reg(self, &args);
        return true;
    }
    fn ccmp_reg(&mut self, args: ArmInstr) -> bool {
        defs::ccmp_reg(self, &args);
        return true;
    }
    fn ccmn_imm(&mut self, args: ArmInstr) -> bool {
        defs::ccmn_imm(self, &args);
        return true;
    }
    fn csel(&mut self, args: ArmInstr) -> bool {
        defs::csel(self, &args);
        return true;
    }
    fn csinc(&mut self, args: ArmInstr) -> bool {
        defs::csinc(self, &args);
        return true;
    }
    fn csinv(&mut self, args: ArmInstr) -> bool {
        defs::csinv(self, &args);
        return true;
    }
    fn csneg(&mut self, args: ArmInstr) -> bool {
        defs::csneg(self, &args);
        return true;
    }
    fn madd(&mut self, args: ArmInstr) -> bool {
        defs::madd(self, &args);
        return true;
    }
    fn msub(&mut self, args: ArmInstr) -> bool {
        defs::msub(self, &args);
        return true;
    }
    fn smaddl(&mut self, args: ArmInstr) -> bool {
        defs::smaddl(self, &args);
        return true;
    }
    fn smsubl(&mut self, args: ArmInstr) -> bool {
        defs::smsubl(self, &args);
        return true;
    }
    fn smulh(&mut self, args: ArmInstr) -> bool {
        defs::smulh(self, &args);
        return true;
    }
    fn umaddl(&mut self, args: ArmInstr) -> bool {
        defs::umaddl(self, &args);
        return true;
    }
    fn umsubl(&mut self, args: ArmInstr) -> bool {
        defs::umsubl(self, &args);
        return true;
    }
    fn umulh(&mut self, args: ArmInstr) -> bool {
        defs::umulh(self, &args);
        return true;
    }
    fn adr(&mut self, args: ArmInstr) -> bool {
        defs::adr(self, &args);
        return true;
    }
    fn adrp(&mut self, args: ArmInstr) -> bool {
        defs::adrp(self, &args);
        return true;
    }
    fn add_addsub_imm(&mut self, args: ArmInstr) -> bool {
        defs::add_addsub_imm(self, &args);
        return true;
    }
    fn adds_addsub_imm(&mut self, args: ArmInstr) -> bool {
        defs::adds_addsub_imm(self, &args);
        return true;
    }
    fn sub_addsub_imm(&mut self, args: ArmInstr) -> bool {
        defs::sub_addsub_imm(self, &args);
        return true;
    }
    fn subs_addsub_imm(&mut self, args: ArmInstr) -> bool {
        defs::subs_addsub_imm(self, &args);
        return true;
    }
    fn stur_fpsimd(&mut self, args: ArmInstr) -> bool {
        defs::stur_fpsimd(self, &args);
        return true;
    }
    fn ldaxr(&mut self, args: ArmInstr) -> bool {
        defs::ldaxr(self, &args);
        return true;
    }
    fn stlxr(&mut self, args: ArmInstr) -> bool {
        defs::stlxr(self, &args);
        return true;
    }
    fn dmb(&mut self, args: ArmInstr) -> bool {
        defs::dmb(self, &args);
        return true;
    }
    fn and_log_imm(&mut self, args: ArmInstr) -> bool {
        defs::and_log_imm(self, &args);
        return true;
    }
    fn orr_log_imm(&mut self, args: ArmInstr) -> bool {
        defs::orr_log_imm(self, &args);
        return true;
    }
    fn eor_log_imm(&mut self, args: ArmInstr) -> bool {
        defs::eor_log_imm(self, &args);
        return true;
    }
    fn movn(&mut self, args: ArmInstr) -> bool {
        defs::movn(self, &args);
        return true;
    }
    fn movz(&mut self, args: ArmInstr) -> bool {
        defs::movz(self, &args);
        return true;
    }
    fn movk(&mut self, args: ArmInstr) -> bool {
        defs::movk(self, &args);
        return true;
    }
    fn sbfm(&mut self, args: ArmInstr) -> bool {
        defs::sbfm(self, &args);
        return true;
    }
    fn bfm(&mut self, args: ArmInstr) -> bool {
        defs::bfm(self, &args);
        return true;
    }
    fn ubfm(&mut self, args: ArmInstr) -> bool {
        defs::ubfm(self, &args);
        return true;
    }
    fn extr(&mut self, args: ArmInstr) -> bool {
        defs::extr(self, &args);
        return true;
    }
    fn b_uncond(&mut self, args: ArmInstr) -> bool {
        defs::b_uncond(self, &args);
        return true;
    }
    fn b_cond(&mut self, args: ArmInstr) -> bool {
        defs::b_cond(self, &args);
        return true;
    }
    fn msr_reg(&mut self, args: ArmInstr) -> bool {
        defs::msr_reg(self, &args);
        return true;
    }
    fn svc(&mut self, args: ArmInstr) -> bool {
        defs::svc(self, &args);
        return true;
    }
    fn bl(&mut self, args: ArmInstr) -> bool {
        defs::bl(self, &args);
        return true;
    }
    fn cbz(&mut self, args: ArmInstr) -> bool {
        defs::cbz(self, &args);
        return true;
    }
    fn cbnz(&mut self, args: ArmInstr) -> bool {
        defs::cbnz(self, &args);
        return true;
    }
    fn tbz(&mut self, args: ArmInstr) -> bool {
        defs::tbz(self, &args);
        return true;
    }
    fn tbnz(&mut self, args: ArmInstr) -> bool {
        defs::tbnz(self, &args);
        return true;
    }
    fn fmul_float(&mut self, args: ArmInstr) -> bool {
        defs::fmul_float(self, &args);
        return true;
    }
    fn fcvtms_float(&mut self, args: ArmInstr) -> bool {
        defs::fcvtms_float(self, &args);
        return true;
    }
    fn frintm_float(&mut self, args: ArmInstr) -> bool {
        defs::frintm_float(self, &args);
        return true;
    }
    fn fcvtau_float(&mut self, args: ArmInstr) -> bool {
        defs::fcvtau_float(self, &args);
        return true;
    }
    fn frintx_float(&mut self, args: ArmInstr) -> bool {
        defs::frintx_float(self, &args);
        return true;
    }
    fn fcvtzs_float_int(&mut self, args: ArmInstr) -> bool {
        defs::fcvtzs_float_int(self, &args);
        return true;
    }
    fn frinta_float(&mut self, args: ArmInstr) -> bool {
        defs::frinta_float(self, &args);
        return true;
    }
    fn fcvtns_float(&mut self, args: ArmInstr) -> bool {
        defs::fcvtns_float(self, &args);
        return true;
    }
    fn fcvtas_float(&mut self, args: ArmInstr) -> bool {
        defs::fcvtas_float(self, &args);
        return true;
    }
    fn fmin_float(&mut self, args: ArmInstr) -> bool {
        defs::fmin_float(self, &args);
        return true;
    }
    fn fmaxnm_float(&mut self, args: ArmInstr) -> bool {
        defs::fmaxnm_float(self, &args);
        return true;
    }
    fn fmov_float_imm(&mut self, args: ArmInstr) -> bool {
        defs::fmov_float_imm(self, &args);
        return true;
    }
    fn fcvtzs_advsimd_int(&mut self, args: ArmInstr) -> bool {
        defs::fcvtzs_advsimd_int(self, &args);
        return true;
    }
    fn fcvtzu_advsimd_int(&mut self, args: ArmInstr) -> bool {
        defs::fcvtzu_advsimd_int(self, &args);
        return true;
    }
    fn umov_advsimd(&mut self, args: ArmInstr) -> bool {
        defs::umov_advsimd(self, &args);
        return true;
    }
    fn nop(&mut self, args: ArmInstr) -> bool {
        return true;
    }
    fn fcvt_float(&mut self, args: ArmInstr) -> bool {
        defs::fcvt_float(self, &args);
        return true;
    }
    fn fmov_float(&mut self, args: ArmInstr) -> bool {
        defs::fmov_float(self, &args);
        return true;
    }
    fn fdiv_float(&mut self, args: ArmInstr) -> bool {
        defs::fdiv_float(self, &args);
        return true;
    }
    fn fmsub_float(&mut self, args: ArmInstr) -> bool {
        defs::fmsub_float(self, &args);
        return true;
    }
    fn fcvtps_float(&mut self, args: ArmInstr) -> bool {
        defs::fcvtps_float(self, &args);
        return true;
    }
    fn fccmpe_float(&mut self, args: ArmInstr) -> bool {
        defs::fccmpe_float(self, &args);
        return true;
    }
    fn fnmadd_float(&mut self, args: ArmInstr) -> bool {
        defs::fnmadd_float(self, &args);
        return true;
    }
    fn frinti_float(&mut self, args: ArmInstr) -> bool {
        defs::frinti_float(self, &args);
        return true;
    }
    fn fadd_float(&mut self, args: ArmInstr) -> bool {
        defs::fadd_float(self, &args);
        return true;
    }
    fn fcsel_float(&mut self, args: ArmInstr) -> bool {
        defs::fcsel_float(self, &args);
        return true;
    }
    fn ucvtf_float_int(&mut self, args: ArmInstr) -> bool {
        defs::ucvtf_float_int(self, &args);
        return true;
    }
    fn fcvtpu_float(&mut self, args: ArmInstr) -> bool {
        defs::fcvtpu_float(self, &args);
        return true;
    }
    fn fminnm_float(&mut self, args: ArmInstr) -> bool {
        defs::fminnm_float(self, &args);
        return true;
    }
    fn fcvtzu_float_int(&mut self, args: ArmInstr) -> bool {
        defs::fcvtzu_float_int(self, &args);
        return true;
    }
    fn fneg_float(&mut self, args: ArmInstr) -> bool {
        defs::fneg_float(self, &args);
        return true;
    }
    fn fcvtnu_float(&mut self, args: ArmInstr) -> bool {
        defs::fcvtnu_float(self, &args);
        return true;
    }
    fn fsub_float(&mut self, args: ArmInstr) -> bool {
        defs::fsub_float(self, &args);
        return true;
    }
    fn frintz_float(&mut self, args: ArmInstr) -> bool {
        defs::frintz_float(self, &args);
        return true;
    }
    fn scvtf_float_int(&mut self, args: ArmInstr) -> bool {
        defs::scvtf_float_int(self, &args);
        return true;
    }
    fn frintp_float(&mut self, args: ArmInstr) -> bool {
        defs::frintp_float(self, &args);
        return true;
    }
    fn fmax_float(&mut self, args: ArmInstr) -> bool {
        defs::fmax_float(self, &args);
        return true;
    }
    fn fcvtmu_float(&mut self, args: ArmInstr) -> bool {
        defs::fcvtmu_float(self, &args);
        return true;
    }
    fn fmadd_float(&mut self, args: ArmInstr) -> bool {
        defs::fmadd_float(self, &args);
        return true;
    }
    fn fnmul_float(&mut self, args: ArmInstr) -> bool {
        defs::fnmul_float(self, &args);
        return true;
    }
    fn fccmp_float(&mut self, args: ArmInstr) -> bool {
        defs::fccmp_float(self, &args);
        return true;
    }
    fn fabs_float(&mut self, args: ArmInstr) -> bool {
        defs::fabs_float(self, &args);
        return true;
    }
    fn fcmp_float(&mut self, args: ArmInstr) -> bool {
        defs::fcmp_float(self, &args);
        return true;
    }
    fn fnmsub_float(&mut self, args: ArmInstr) -> bool {
        defs::fnmsub_float(self, &args);
        return true;
    }
    fn fcmpe_float(&mut self, args: ArmInstr) -> bool {
        defs::fcmpe_float(self, &args);
        return true;
    }
    fn fsqrt_float(&mut self, args: ArmInstr) -> bool {
        defs::fsqrt_float(self, &args);
        return true;
    }
    fn frintn_float(&mut self, args: ArmInstr) -> bool {
        defs::frintn_float(self, &args);
        return true;
    }
    fn strh_imm(&mut self, args: ArmInstr) -> bool {
        defs::strh_imm(self, &args);
        return true;
    }
    fn ldursw(&mut self, args: ArmInstr) -> bool {
        defs::ldursw(self, &args);
        return true;
    }
    fn ldurb(&mut self, args: ArmInstr) -> bool {
        defs::ldurb(self, &args);
        return true;
    }
    fn str_reg_gen(&mut self, args: ArmInstr) -> bool {
        defs::str_reg_gen(self, &args);
        return true;
    }
    fn ldrsw_imm(&mut self, args: ArmInstr) -> bool {
        defs::ldrsw_imm(self, &args);
        return true;
    }
    fn fmov_float_gen(&mut self, args: ArmInstr) -> bool {
        defs::fmov_float_gen(self, &args);
        return true;
    }
    fn orr_advsimd_reg(&mut self, args: ArmInstr) -> bool {
        defs::orr_advsimd_reg(self, &args);
        return true;
    }
    fn movi_advsimd(&mut self, args: ArmInstr) -> bool {
        defs::movi_advsimd(self, &args);
        return true;
    }
    fn ldpsw(&mut self, args: ArmInstr) -> bool {
        defs::ldpsw(self, &args);
        return true;
    }
    fn ldrh_reg(&mut self, args: ArmInstr) -> bool {
        defs::ldrh_reg(self, &args);
        return true;
    }
    fn ldrsb_reg(&mut self, args: ArmInstr) -> bool {
        defs::ldrsb_reg(self, &args);
        return true;
    }
    fn ldp_gen(&mut self, args: ArmInstr) -> bool {
        defs::ldp_gen(self, &args);
        return true;
    }
    fn ldursb(&mut self, args: ArmInstr) -> bool {
        defs::ldursb(self, &args);
        return true;
    }
    fn strh_reg(&mut self, args: ArmInstr) -> bool {
        defs::strh_reg(self, &args);
        return true;
    }
    fn strb_imm(&mut self, args: ArmInstr) -> bool {
        defs::strb_imm(self, &args);
        return true;
    }
    fn sturh(&mut self, args: ArmInstr) -> bool {
        defs::sturh(self, &args);
        return true;
    }
    fn ldrb_imm(&mut self, args: ArmInstr) -> bool {
        defs::ldrb_imm(self, &args);
        return true;
    }
    fn ldrsh_reg(&mut self, args: ArmInstr) -> bool {
        defs::ldrsh_reg(self, &args);
        return true;
    }
    fn strb_reg(&mut self, args: ArmInstr) -> bool {
        defs::strb_reg(self, &args);
        return true;
    }
    fn stur_gen(&mut self, args: ArmInstr) -> bool {
        defs::stur_gen(self, &args);
        return true;
    }
    fn ldursh(&mut self, args: ArmInstr) -> bool {
        defs::ldursh(self, &args);
        return true;
    }
    fn sturb(&mut self, args: ArmInstr) -> bool {
        defs::sturb(self, &args);
        return true;
    }
    fn ldrh_imm(&mut self, args: ArmInstr) -> bool {
        defs::ldrh_imm(self, &args);
        return true;
    }
    fn ldur_gen(&mut self, args: ArmInstr) -> bool {
        defs::ldur_gen(self, &args);
        return true;
    }
    fn ldurh(&mut self, args: ArmInstr) -> bool {
        defs::ldurh(self, &args);
        return true;
    }
    fn ldr_reg_gen(&mut self, args: ArmInstr) -> bool {
        defs::ldr_reg_gen(self, &args);
        return true;
    }
    fn ldrb_reg(&mut self, args: ArmInstr) -> bool {
        defs::ldrb_reg(self, &args);
        return true;
    }
    fn stp_gen(&mut self, args: ArmInstr) -> bool {
        defs::stp_gen(self, &args);
        return true;
    }
    fn ldr_imm_gen(&mut self, args: ArmInstr) -> bool {
        defs::ldr_imm_gen(self, &args);
        return true;
    }
    fn hint(&mut self, args: ArmInstr) -> bool {
        defs::hint(self, &args);
        return true;
    }
    fn str_imm_gen(&mut self, args: ArmInstr) -> bool {
        defs::str_imm_gen(self, &args);
        return true;
    }
    fn ldrsw_reg(&mut self, args: ArmInstr) -> bool {
        defs::ldrsw_reg(self, &args);
        return true;
    }
    fn ldrsb_imm(&mut self, args: ArmInstr) -> bool {
        defs::ldrsb_imm(self, &args);
        return true;
    }
    fn ldrsh_imm(&mut self, args: ArmInstr) -> bool {
        defs::ldrsh_imm(self, &args);
        return true;
    }
    fn ld1_advsimd_mult(&mut self, args: ArmInstr) -> bool {
        defs::ld1_advsimd_mult(self, &args);
        return true;
    }
    fn st1_advsimd_mult(&mut self, args: ArmInstr) -> bool {
        defs::st1_advsimd_mult(self, &args);
        return true;
    }
    fn cmeq_advsimd_zero(&mut self, args: ArmInstr) -> bool {
        defs::cmeq_advsimd_zero(self, &args);
        return true;
    }

    fn and_advsimd(&mut self, args: ArmInstr) -> bool {
        defs::and_advsimd(self, &args);
        return true;
    }

    fn addp_advsimd_vec(&mut self, args: ArmInstr) -> bool {
        defs::addp_advsimd_vec(self, &args);
        return true;
    }

    fn umaxp_advsimd(&mut self, args: ArmInstr) -> bool {
        defs::umaxp_advsimd(self, &args);
        return true;
    }
    fn rev(&mut self, args: ArmInstr) -> bool {
        defs::rev(self, &args);
        return true;
    }
    fn stlr(&mut self, args: ArmInstr) -> bool {
        defs::stlr(self, &args);
        return true;
    }
    fn cmeq_advsimd_reg(&mut self, args: ArmInstr) -> bool {
        defs::cmeq_advsimd_reg(self, &args);
        return true;
    }
    fn bit_advsimd(&mut self, args: ArmInstr) -> bool {
        defs::bit_advsimd(self, &args);
        return true;
    }
    fn ldar(&mut self, args: ArmInstr) -> bool {
        defs::ldar(self, &args);
        return true;
    }
    fn cmhs_advsimd(&mut self, args: ArmInstr) -> bool {
        defs::cmhs_advsimd(self, &args);
        return true;
    }
    fn uminp_advsimd(&mut self, args: ArmInstr) -> bool {
        defs::uminp_advsimd(self, &args);
        return true;
    }
    fn bic_advsimd_imm(&mut self, args: ArmInstr) -> bool {
        defs::bic_advsimd_imm(self, &args);
        return true;
    }
    fn stxr(&mut self, args: ArmInstr) -> bool {
        defs::stxr(self, &args);
        return true;
    }
    fn ldxr(&mut self, args: ArmInstr) -> bool {
        defs::ldxr(self, &args);
        return true;
    }
}