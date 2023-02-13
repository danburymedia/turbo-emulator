#![allow(dead_code, unused_variables)]

use jit::extract::*;
use crate::riscv::common::RiscvArgs;
// todo: convert to return false
pub trait DecodeTrait {
    fn ecall(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn ebreak(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn uret(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn sret(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn mret(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn wfi(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn sfence_vma(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn sfence_vm(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn lui(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn auipc(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn jal(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn jalr(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn beq(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn bne(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn blt(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn bge(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn bltu(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn bgeu(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn lb(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn lh(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn lw(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn lbu(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn lhu(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn sb(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn sh(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn sw(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn addi(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn slti(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn sltiu(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn xori(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn ori(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn andi(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn slli(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn srli(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn srai(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn add(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn sub(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn sll(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn slt(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn sltu(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn xor(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn srl(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn sra(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn or(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn and(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn pause(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn fence(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn fence_i(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn csrrw(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn csrrs(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn csrrc(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn csrrwi(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn csrrsi(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn csrrci(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn lwu(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn ld(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn sd(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn addiw(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn slliw(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn srliw(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn sraiw(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn addw(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn subw(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn sllw(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn srlw(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn sraw(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn ldu(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn lq(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn sq(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn addid(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn sllid(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn srlid(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn sraid(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn addd(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn subd(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn slld(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn srld(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn srad(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn mul(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn mulh(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn mulhsu(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn mulhu(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn div(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn divu(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn rem(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn remu(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn mulw(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn divw(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn divuw(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn remw(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn remuw(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn muld(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn divd(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn divud(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn remd(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn remud(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn lr_w(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn sc_w(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn amoswap_w(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn amoadd_w(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn amoxor_w(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn amoand_w(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn amoor_w(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn amomin_w(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn amomax_w(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn amominu_w(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn amomaxu_w(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn lr_d(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn sc_d(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn amoswap_d(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn amoadd_d(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn amoxor_d(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn amoand_d(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn amoor_d(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn amomin_d(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn amomax_d(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn amominu_d(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn amomaxu_d(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn flw(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn fsw(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn fmadd_s(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn fmsub_s(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn fnmsub_s(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn fnmadd_s(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn fadd_s(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn fsub_s(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn fmul_s(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn fdiv_s(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn fsqrt_s(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn fsgnj_s(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn fsgnjn_s(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn fsgnjx_s(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn fmin_s(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn fmax_s(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn fcvt_w_s(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn fcvt_wu_s(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn fmv_x_w(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn feq_s(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn flt_s(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn fle_s(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn fclass_s(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn fcvt_s_w(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn fcvt_s_wu(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn fmv_w_x(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn fcvt_l_s(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn fcvt_lu_s(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn fcvt_s_l(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn fcvt_s_lu(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn fld(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn fsd(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn fmadd_d(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn fmsub_d(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn fnmsub_d(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn fnmadd_d(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn fadd_d(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn fsub_d(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn fmul_d(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn fdiv_d(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn fsqrt_d(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn fsgnj_d(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn fsgnjn_d(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn fsgnjx_d(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn fmin_d(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn fmax_d(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn fcvt_s_d(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn fcvt_d_s(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn feq_d(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn flt_d(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn fle_d(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn fclass_d(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn fcvt_w_d(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn fcvt_wu_d(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn fcvt_d_w(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn fcvt_d_wu(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn fcvt_l_d(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn fcvt_lu_d(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn fmv_x_d(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn fcvt_d_l(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn fcvt_d_lu(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn fmv_d_x(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn hlv_b(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn hlv_bu(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn hlv_h(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn hlv_hu(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn hlvx_hu(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn hlv_w(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn hlvx_wu(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn hsv_b(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn hsv_h(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn hsv_w(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn hfence_gvma(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn hfence_vvma(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn hlv_wu(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn hlv_d(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn hsv_d(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vle8_v(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vle16_v(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vle32_v(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vle64_v(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vse8_v(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vse16_v(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vse32_v(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vse64_v(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vlm_v(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vsm_v(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vlse8_v(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vlse16_v(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vlse32_v(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vlse64_v(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vsse8_v(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vsse16_v(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vsse32_v(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vsse64_v(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vlxei8_v(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vlxei16_v(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vlxei32_v(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vlxei64_v(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vsxei8_v(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vsxei16_v(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vsxei32_v(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vsxei64_v(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vle8ff_v(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vle16ff_v(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vle32ff_v(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vle64ff_v(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vl1re8_v(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vl1re16_v(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vl1re32_v(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vl1re64_v(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vl2re8_v(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vl2re16_v(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vl2re32_v(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vl2re64_v(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vl4re8_v(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vl4re16_v(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vl4re32_v(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vl4re64_v(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vl8re8_v(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vl8re16_v(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vl8re32_v(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vl8re64_v(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vs1r_v(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vs2r_v(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vs4r_v(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vs8r_v(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vadd_vv(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vadd_vx(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vadd_vi(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vsub_vv(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vsub_vx(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vrsub_vx(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vrsub_vi(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vwaddu_vv(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vwaddu_vx(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vwadd_vv(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vwadd_vx(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vwsubu_vv(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vwsubu_vx(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vwsub_vv(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vwsub_vx(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vwaddu_wv(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vwaddu_wx(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vwadd_wv(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vwadd_wx(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vwsubu_wv(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vwsubu_wx(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vwsub_wv(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vwsub_wx(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vadc_vvm(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vadc_vxm(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vadc_vim(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vmadc_vvm(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vmadc_vxm(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vmadc_vim(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vsbc_vvm(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vsbc_vxm(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vmsbc_vvm(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vmsbc_vxm(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vand_vv(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vand_vx(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vand_vi(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vor_vv(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vor_vx(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vor_vi(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vxor_vv(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vxor_vx(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vxor_vi(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vsll_vv(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vsll_vx(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vsll_vi(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vsrl_vv(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vsrl_vx(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vsrl_vi(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vsra_vv(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vsra_vx(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vsra_vi(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vnsrl_wv(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vnsrl_wx(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vnsrl_wi(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vnsra_wv(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vnsra_wx(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vnsra_wi(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vmseq_vv(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vmseq_vx(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vmseq_vi(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vmsne_vv(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vmsne_vx(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vmsne_vi(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vmsltu_vv(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vmsltu_vx(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vmslt_vv(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vmslt_vx(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vmsleu_vv(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vmsleu_vx(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vmsleu_vi(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vmsle_vv(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vmsle_vx(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vmsle_vi(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vmsgtu_vx(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vmsgtu_vi(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vmsgt_vx(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vmsgt_vi(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vminu_vv(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vminu_vx(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vmin_vv(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vmin_vx(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vmaxu_vv(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vmaxu_vx(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vmax_vv(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vmax_vx(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vmul_vv(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vmul_vx(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vmulh_vv(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vmulh_vx(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vmulhu_vv(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vmulhu_vx(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vmulhsu_vv(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vmulhsu_vx(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vdivu_vv(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vdivu_vx(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vdiv_vv(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vdiv_vx(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vremu_vv(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vremu_vx(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vrem_vv(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vrem_vx(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vwmulu_vv(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vwmulu_vx(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vwmulsu_vv(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vwmulsu_vx(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vwmul_vv(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vwmul_vx(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vmacc_vv(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vmacc_vx(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vnmsac_vv(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vnmsac_vx(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vmadd_vv(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vmadd_vx(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vnmsub_vv(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vnmsub_vx(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vwmaccu_vv(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vwmaccu_vx(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vwmacc_vv(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vwmacc_vx(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vwmaccsu_vv(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vwmaccsu_vx(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vwmaccus_vx(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vmv_v_v(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vmv_v_x(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vmv_v_i(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vmerge_vvm(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vmerge_vxm(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vmerge_vim(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vsaddu_vv(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vsaddu_vx(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vsaddu_vi(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vsadd_vv(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vsadd_vx(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vsadd_vi(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vssubu_vv(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vssubu_vx(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vssub_vv(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vssub_vx(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vaadd_vv(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vaadd_vx(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vaaddu_vv(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vaaddu_vx(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vasub_vv(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vasub_vx(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vasubu_vv(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vasubu_vx(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vsmul_vv(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vsmul_vx(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vssrl_vv(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vssrl_vx(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vssrl_vi(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vssra_vv(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vssra_vx(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vssra_vi(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vnclipu_wv(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vnclipu_wx(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vnclipu_wi(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vnclip_wv(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vnclip_wx(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vnclip_wi(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vfadd_vv(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vfadd_vf(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vfsub_vv(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vfsub_vf(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vfrsub_vf(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vfwadd_vv(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vfwadd_vf(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vfwadd_wv(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vfwadd_wf(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vfwsub_vv(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vfwsub_vf(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vfwsub_wv(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vfwsub_wf(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vfmul_vv(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vfmul_vf(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vfdiv_vv(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vfdiv_vf(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vfrdiv_vf(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vfwmul_vv(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vfwmul_vf(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vfmacc_vv(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vfnmacc_vv(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vfnmacc_vf(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vfmacc_vf(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vfmsac_vv(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vfmsac_vf(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vfnmsac_vv(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vfnmsac_vf(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vfmadd_vv(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vfmadd_vf(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vfnmadd_vv(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vfnmadd_vf(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vfmsub_vv(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vfmsub_vf(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vfnmsub_vv(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vfnmsub_vf(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vfwmacc_vv(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vfwmacc_vf(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vfwnmacc_vv(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vfwnmacc_vf(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vfwmsac_vv(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vfwmsac_vf(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vfwnmsac_vv(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vfwnmsac_vf(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vfsqrt_v(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vfrsqrt7_v(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vfrec7_v(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vfmin_vv(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vfmin_vf(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vfmax_vv(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vfmax_vf(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vfsgnj_vv(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vfsgnj_vf(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vfsgnjn_vv(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vfsgnjn_vf(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vfsgnjx_vv(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vfsgnjx_vf(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vfslide1up_vf(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vfslide1down_vf(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vmfeq_vv(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vmfeq_vf(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vmfne_vv(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vmfne_vf(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vmflt_vv(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vmflt_vf(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vmfle_vv(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vmfle_vf(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vmfgt_vf(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vmfge_vf(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vfclass_v(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vfmerge_vfm(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vfmv_v_f(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vfcvt_xu_f_v(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vfcvt_x_f_v(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vfcvt_f_xu_v(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vfcvt_f_x_v(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vfcvt_rtz_xu_f_v(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vfcvt_rtz_x_f_v(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vfwcvt_xu_f_v(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vfwcvt_x_f_v(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vfwcvt_f_xu_v(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vfwcvt_f_x_v(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vfwcvt_f_f_v(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vfwcvt_rtz_xu_f_v(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vfwcvt_rtz_x_f_v(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vfncvt_xu_f_w(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vfncvt_x_f_w(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vfncvt_f_xu_w(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vfncvt_f_x_w(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vfncvt_f_f_w(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vfncvt_rod_f_f_w(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vfncvt_rtz_xu_f_w(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vfncvt_rtz_x_f_w(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vredsum_vs(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vredand_vs(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vredor_vs(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vredxor_vs(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vredminu_vs(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vredmin_vs(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vredmaxu_vs(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vredmax_vs(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vwredsumu_vs(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vwredsum_vs(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vfredusum_vs(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vfredosum_vs(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vfredmin_vs(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vfredmax_vs(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vfwredusum_vs(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vfwredosum_vs(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vmand_mm(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vmnand_mm(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vmandn_mm(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vmxor_mm(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vmor_mm(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vmnor_mm(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vmorn_mm(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vmxnor_mm(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vcpop_m(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vfirst_m(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vmsbf_m(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vmsif_m(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vmsof_m(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn viota_m(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vid_v(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vmv_x_s(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vmv_s_x(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vfmv_f_s(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vfmv_s_f(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vslideup_vx(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vslideup_vi(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vslide1up_vx(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vslidedown_vx(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vslidedown_vi(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vslide1down_vx(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vrgather_vv(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vrgatherei16_vv(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vrgather_vx(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vrgather_vi(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vcompress_vm(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vmv1r_v(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vmv2r_v(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vmv4r_v(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vmv8r_v(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vzext_vf2(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vzext_vf4(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vzext_vf8(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vsext_vf2(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vsext_vf4(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vsext_vf8(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vsetvli(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vsetivli(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn vsetvl(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn sh1add(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn sh2add(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn sh3add(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn add_uw(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn sh1add_uw(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn sh2add_uw(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn sh3add_uw(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn slli_uw(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn andn(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn rol(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn ror(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn rori(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn rev8_32(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn zext_h_32(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn pack(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn xnor(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn clz(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn cpop(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn ctz(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn max(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn maxu(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn min(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn minu(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn orc_b(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn orn(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn sext_b(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn sext_h(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn brev8(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn packh(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn unzip(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn zip(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn rev8_64(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn rolw(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn roriw(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn rorw(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn zext_h_64(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn packw(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn clzw(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn ctzw(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn cpopw(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn clmul(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn clmulh(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn clmulr(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn xperm4(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn xperm8(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn bclr(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn bclri(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn bext(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn bexti(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn binv(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn binvi(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn bset(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn bseti(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn flh(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn fsh(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn fmadd_h(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn fmsub_h(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn fnmsub_h(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn fnmadd_h(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn fadd_h(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn fsub_h(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn fmul_h(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn fdiv_h(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn fsqrt_h(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn fsgnj_h(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn fsgnjn_h(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn fsgnjx_h(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn fmin_h(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn fmax_h(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn fcvt_h_s(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn fcvt_s_h(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn fcvt_h_d(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn fcvt_d_h(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn fcvt_w_h(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn fcvt_wu_h(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn fmv_x_h(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn feq_h(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn flt_h(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn fle_h(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn fclass_h(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn fcvt_h_w(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn fcvt_h_wu(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn fmv_h_x(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn fcvt_l_h(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn fcvt_lu_h(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn fcvt_h_l(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn fcvt_h_lu(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn sinval_vma(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn sfence_w_inval(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn sfence_inval_ir(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn hinval_vvma(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn hinval_gvma(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn aes32dsmi(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn aes32dsi(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn aes64dsm(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn aes64ds(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn aes64im(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn aes32esmi(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn aes32esi(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn aes64es(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn aes64esm(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn aes64ks2(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn aes64ks1i(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn sha256sig0(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn sha256sig1(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn sha256sum0(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn sha256sum1(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn sha512sum0r(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn sha512sum1r(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn sha512sig0l(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn sha512sig0h(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn sha512sig1l(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn sha512sig1h(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn sha512sig0(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn sha512sig1(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn sha512sum0(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn sha512sum1(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn sm3p0(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn sm3p1(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn sm4ed(&mut self, args: RiscvArgs) -> bool { panic!(); }
    fn sm4ks(&mut self, args: RiscvArgs) -> bool { panic!(); }
}

fn decode_extract_atom_ld<T: DecodeTrait>(ctx: &T, a: &mut RiscvArgs, insn: u32)
{
    a.aq = extract32(insn, 26, 1);
    a.rl = extract32(insn, 25, 1);
    a.rs2 = 0;
    a.rs1 = extract32(insn, 15, 5);
    a.rd = extract32(insn, 7, 5);
}

fn decode_extract_atom_st<T: DecodeTrait>(ctx: &T, a: &mut RiscvArgs, insn: u32)
{
    a.aq = extract32(insn, 26, 1);
    a.rl = extract32(insn, 25, 1);
    a.rs2 = extract32(insn, 20, 5);
    a.rs1 = extract32(insn, 15, 5);
    a.rd = extract32(insn, 7, 5);
}

fn decode_extract_b<T: DecodeTrait>(ctx: &T, a: &mut RiscvArgs, insn: u32)
{
    a.imm = ex_shift_1(deposit32(deposit32(deposit32(extract32(insn, 8, 4), 4, 28, extract32(insn, 25, 6)), 10, 22, extract32(insn, 7, 1)), 11, 21, sextract32(insn, 31, 1)));
    a.rs2 = extract32(insn, 20, 5);
    a.rs1 = extract32(insn, 15, 5);
}

fn decode_extract_csr<T: DecodeTrait>(ctx: &T, a: &mut RiscvArgs, insn: u32)
{
    a.csr = extract32(insn, 20, 12);
    a.rs1 = extract32(insn, 15, 5);
    a.rd = extract32(insn, 7, 5);
}

fn decode_extract_decode_Fmt_33<T: DecodeTrait>(ctx: &T, a: &mut RiscvArgs, insn: u32)
{
}

fn decode_extract_decode_Fmt_34<T: DecodeTrait>(ctx: &T, a: &mut RiscvArgs, insn: u32)
{
    a.pred = extract32(insn, 24, 4);
    a.succ = extract32(insn, 20, 4);
}

fn decode_extract_hfence_gvma<T: DecodeTrait>(ctx: &T, a: &mut RiscvArgs, insn: u32)
{
    a.rs2 = extract32(insn, 20, 5);
    a.rs1 = extract32(insn, 15, 5);
}

fn decode_extract_hfence_vvma<T: DecodeTrait>(ctx: &T, a: &mut RiscvArgs, insn: u32)
{
    a.rs2 = extract32(insn, 20, 5);
    a.rs1 = extract32(insn, 15, 5);
}

fn decode_extract_i<T: DecodeTrait>(ctx: &T, a: &mut RiscvArgs, insn: u32)
{
    a.imm = sextract32(insn, 20, 12);
    a.rs1 = extract32(insn, 15, 5);
    a.rd = extract32(insn, 7, 5);
}

fn decode_extract_i_aes<T: DecodeTrait>(ctx: &T, a: &mut RiscvArgs, insn: u32)
{
    a.imm = extract32(insn, 20, 4);
    a.rs1 = extract32(insn, 15, 5);
    a.rd = extract32(insn, 7, 5);
}

fn decode_extract_j<T: DecodeTrait>(ctx: &T, a: &mut RiscvArgs, insn: u32)
{
    a.imm = ex_shift_1(deposit32(deposit32(deposit32(extract32(insn, 21, 10), 10, 22, extract32(insn, 20, 1)), 11, 21, extract32(insn, 12, 8)), 19, 13, sextract32(insn, 31, 1)));
    a.rd = extract32(insn, 7, 5);
}

fn decode_extract_k_aes<T: DecodeTrait>(ctx: &T, a: &mut RiscvArgs, insn: u32)
{
    a.shamt = ex_shift_3(extract32(insn, 30, 2));
    a.rs2 = extract32(insn, 20, 5);
    a.rs1 = extract32(insn, 15, 5);
    a.rd = extract32(insn, 7, 5);
}

fn decode_extract_r<T: DecodeTrait>(ctx: &T, a: &mut RiscvArgs, insn: u32)
{
    a.rs2 = extract32(insn, 20, 5);
    a.rs1 = extract32(insn, 15, 5);
    a.rd = extract32(insn, 7, 5);
}

fn decode_extract_r1_vm<T: DecodeTrait>(ctx: &T, a: &mut RiscvArgs, insn: u32)
{
    a.vm = extract32(insn, 25, 1);
    a.rd = extract32(insn, 7, 5);
}

fn decode_extract_r2<T: DecodeTrait>(ctx: &T, a: &mut RiscvArgs, insn: u32)
{
    a.rs1 = extract32(insn, 15, 5);
    a.rd = extract32(insn, 7, 5);
}

fn decode_extract_r2_nfvm<T: DecodeTrait>(ctx: &T, a: &mut RiscvArgs, insn: u32)
{
    a.vm = extract32(insn, 25, 1);
    a.nf = ex_plus_1(extract32(insn, 29, 3));
    a.rs1 = extract32(insn, 15, 5);
    a.rd = extract32(insn, 7, 5);
}

fn decode_extract_r2_rm<T: DecodeTrait>(ctx: &T, a: &mut RiscvArgs, insn: u32)
{
    a.rs1 = extract32(insn, 15, 5);
    a.rm = extract32(insn, 12, 3);
    a.rd = extract32(insn, 7, 5);
}

fn decode_extract_r2_s<T: DecodeTrait>(ctx: &T, a: &mut RiscvArgs, insn: u32)
{
    a.rs2 = extract32(insn, 20, 5);
    a.rs1 = extract32(insn, 15, 5);
}

fn decode_extract_r2_vm<T: DecodeTrait>(ctx: &T, a: &mut RiscvArgs, insn: u32)
{
    a.vm = extract32(insn, 25, 1);
    a.rs2 = extract32(insn, 20, 5);
    a.rd = extract32(insn, 7, 5);
}

fn decode_extract_r2_zimm10<T: DecodeTrait>(ctx: &T, a: &mut RiscvArgs, insn: u32)
{
    a.zimm = extract32(insn, 20, 10);
    a.rs1 = extract32(insn, 15, 5);
    a.rd = extract32(insn, 7, 5);
}

fn decode_extract_r2_zimm11<T: DecodeTrait>(ctx: &T, a: &mut RiscvArgs, insn: u32)
{
    a.zimm = extract32(insn, 20, 11);
    a.rs1 = extract32(insn, 15, 5);
    a.rd = extract32(insn, 7, 5);
}

fn decode_extract_r2rd<T: DecodeTrait>(ctx: &T, a: &mut RiscvArgs, insn: u32)
{
    a.rs2 = extract32(insn, 20, 5);
    a.rd = extract32(insn, 7, 5);
}

fn decode_extract_r4_rm<T: DecodeTrait>(ctx: &T, a: &mut RiscvArgs, insn: u32)
{
    a.rs3 = extract32(insn, 27, 5);
    a.rs2 = extract32(insn, 20, 5);
    a.rs1 = extract32(insn, 15, 5);
    a.rm = extract32(insn, 12, 3);
    a.rd = extract32(insn, 7, 5);
}

fn decode_extract_r_nfvm<T: DecodeTrait>(ctx: &T, a: &mut RiscvArgs, insn: u32)
{
    a.vm = extract32(insn, 25, 1);
    a.nf = ex_plus_1(extract32(insn, 29, 3));
    a.rs2 = extract32(insn, 20, 5);
    a.rs1 = extract32(insn, 15, 5);
    a.rd = extract32(insn, 7, 5);
}

fn decode_extract_r_rm<T: DecodeTrait>(ctx: &T, a: &mut RiscvArgs, insn: u32)
{
    a.rs2 = extract32(insn, 20, 5);
    a.rs1 = extract32(insn, 15, 5);
    a.rm = extract32(insn, 12, 3);
    a.rd = extract32(insn, 7, 5);
}

fn decode_extract_r_vm<T: DecodeTrait>(ctx: &T, a: &mut RiscvArgs, insn: u32)
{
    a.vm = extract32(insn, 25, 1);
    a.rs2 = extract32(insn, 20, 5);
    a.rs1 = extract32(insn, 15, 5);
    a.rd = extract32(insn, 7, 5);
}

fn decode_extract_r_vm_0<T: DecodeTrait>(ctx: &T, a: &mut RiscvArgs, insn: u32)
{
    a.vm = 0;
    a.rs2 = extract32(insn, 20, 5);
    a.rs1 = extract32(insn, 15, 5);
    a.rd = extract32(insn, 7, 5);
}

fn decode_extract_r_vm_1<T: DecodeTrait>(ctx: &T, a: &mut RiscvArgs, insn: u32)
{
    a.vm = 1;
    a.rs2 = extract32(insn, 20, 5);
    a.rs1 = extract32(insn, 15, 5);
    a.rd = extract32(insn, 7, 5);
}

fn decode_extract_s<T: DecodeTrait>(ctx: &T, a: &mut RiscvArgs, insn: u32)
{
    a.imm = deposit32(extract32(insn, 7, 5), 5, 27, sextract32(insn, 25, 7));
    a.rs2 = extract32(insn, 20, 5);
    a.rs1 = extract32(insn, 15, 5);
}

fn decode_extract_sfence_vm<T: DecodeTrait>(ctx: &T, a: &mut RiscvArgs, insn: u32)
{
    a.rs1 = extract32(insn, 15, 5);
}

fn decode_extract_sfence_vma<T: DecodeTrait>(ctx: &T, a: &mut RiscvArgs, insn: u32)
{
    a.rs2 = extract32(insn, 20, 5);
    a.rs1 = extract32(insn, 15, 5);
}

fn decode_extract_sh<T: DecodeTrait>(ctx: &T, a: &mut RiscvArgs, insn: u32)
{
    a.shamt = extract32(insn, 20, 7);
    a.rs1 = extract32(insn, 15, 5);
    a.rd = extract32(insn, 7, 5);
}

fn decode_extract_sh5<T: DecodeTrait>(ctx: &T, a: &mut RiscvArgs, insn: u32)
{
    a.shamt = extract32(insn, 20, 5);
    a.rs1 = extract32(insn, 15, 5);
    a.rd = extract32(insn, 7, 5);
}

fn decode_extract_sh6<T: DecodeTrait>(ctx: &T, a: &mut RiscvArgs, insn: u32)
{
    a.shamt = extract32(insn, 20, 6);
    a.rs1 = extract32(insn, 15, 5);
    a.rd = extract32(insn, 7, 5);
}

fn decode_extract_u<T: DecodeTrait>(ctx: &T, a: &mut RiscvArgs, insn: u32)
{
    a.imm = ex_shift_12(sextract32(insn, 12, 20));
    a.rd = extract32(insn, 7, 5);
}

pub fn decode<T: DecodeTrait>(transimpl: &mut T, insn: u32) -> bool
{

    let mut args: RiscvArgs = Default::default();
    match insn & 0x0000007f {
        0x00000003 => {
            /* ........ ........ ........ .0000011 */
            decode_extract_i(transimpl, &mut args, insn);
            match (insn >> 12) & 0x7 {
                0x0 => {
                    /* ........ ........ .000.... .0000011 */
                    if transimpl.lb(args) { return true; }
                },
                0x1 => {
                    /* ........ ........ .001.... .0000011 */
                    if transimpl.lh(args) { return true; }
                },
                0x2 => {
                    /* ........ ........ .010.... .0000011 */
                    if transimpl.lw(args) { return true; }
                },
                0x3 => {
                    /* ........ ........ .011.... .0000011 */
                    if transimpl.ld(args) { return true; }
                },
                0x4 => {
                    /* ........ ........ .100.... .0000011 */
                    if transimpl.lbu(args) { return true; }
                },
                0x5 => {
                    /* ........ ........ .101.... .0000011 */
                    if transimpl.lhu(args) { return true; }
                },
                0x6 => {
                    /* ........ ........ .110.... .0000011 */
                    if transimpl.lwu(args) { return true; }
                },
                0x7 => {
                    /* ........ ........ .111.... .0000011 */
                    if transimpl.ldu(args) { return true; }
                },
                _ => { },
            };
        },
        0x00000007 => {
            /* ........ ........ ........ .0000111 */
            match (insn >> 12) & 0x7 {
                0x0 => {
                    /* ........ ........ .000.... .0000111 */
                    match insn & 0x14000000 {
                        0x00000000 => {
                            /* ...0.0.. ........ .000.... .0000111 */
                            match (insn >> 27) & 0x1 {
                                0x0 => {
                                    /* ...000.. ........ .000.... .0000111 */
                                    match (insn >> 20) & 0x1f {
                                        0x0 => {
                                            /* ...000.0 0000.... .000.... .0000111 */
                                            decode_extract_r2_nfvm(transimpl, &mut args, insn);
                                            if transimpl.vle8_v(args) { return true; }
                                        },
                                        0x8 => {
                                            /* ...000.0 1000.... .000.... .0000111 */
                                            decode_extract_r2(transimpl, &mut args, insn);
                                            match insn & 0xe2000000 {
                                                0x02000000 => {
                                                    /* 00000010 1000.... .000.... .0000111 */
                                                    if transimpl.vl1re8_v(args) { return true; }
                                                },
                                                0x22000000 => {
                                                    /* 00100010 1000.... .000.... .0000111 */
                                                    if transimpl.vl2re8_v(args) { return true; }
                                                },
                                                0x62000000 => {
                                                    /* 01100010 1000.... .000.... .0000111 */
                                                    if transimpl.vl4re8_v(args) { return true; }
                                                },
                                                0xe2000000 => {
                                                    /* 11100010 1000.... .000.... .0000111 */
                                                    if transimpl.vl8re8_v(args) { return true; }
                                                },
                                                _ => { },
                                            };
                                        },
                                        0xb => {
                                            /* ...000.0 1011.... .000.... .0000111 */
                                            decode_extract_r2(transimpl, &mut args, insn);
                                            match insn & 0xe2000000 {
                                                0x02000000 => {
                                                    /* 00000010 1011.... .000.... .0000111 */
                                                    if transimpl.vlm_v(args) { return true; }
                                                },
                                                _ => { },
                                            };
                                        },
                                        0x10 => {
                                            /* ...000.1 0000.... .000.... .0000111 */
                                            decode_extract_r2_nfvm(transimpl, &mut args, insn);
                                            if transimpl.vle8ff_v(args) { return true; }
                                        },
                                        _ => { },
                                    };
                                },
                                0x1 => {
                                    /* ...010.. ........ .000.... .0000111 */
                                    decode_extract_r_nfvm(transimpl, &mut args, insn);
                                    if transimpl.vlse8_v(args) { return true; }
                                },
                                _ => { },
                            };
                        },
                        0x04000000 => {
                            /* ...0.1.. ........ .000.... .0000111 */
                            decode_extract_r_nfvm(transimpl, &mut args, insn);
                            if transimpl.vlxei8_v(args) { return true; }
                        },
                        _ => { },
                    };
                },
                0x1 => {
                    /* ........ ........ .001.... .0000111 */
                    decode_extract_i(transimpl, &mut args, insn);
                    if transimpl.flh(args) { return true; }
                },
                0x2 => {
                    /* ........ ........ .010.... .0000111 */
                    decode_extract_i(transimpl, &mut args, insn);
                    if transimpl.flw(args) { return true; }
                },
                0x3 => {
                    /* ........ ........ .011.... .0000111 */
                    decode_extract_i(transimpl, &mut args, insn);
                    if transimpl.fld(args) { return true; }
                },
                0x5 => {
                    /* ........ ........ .101.... .0000111 */
                    match insn & 0x14000000 {
                        0x00000000 => {
                            /* ...0.0.. ........ .101.... .0000111 */
                            match (insn >> 27) & 0x1 {
                                0x0 => {
                                    /* ...000.. ........ .101.... .0000111 */
                                    match (insn >> 20) & 0x1f {
                                        0x0 => {
                                            /* ...000.0 0000.... .101.... .0000111 */
                                            decode_extract_r2_nfvm(transimpl, &mut args, insn);
                                            if transimpl.vle16_v(args) { return true; }
                                        },
                                        0x8 => {
                                            /* ...000.0 1000.... .101.... .0000111 */
                                            decode_extract_r2(transimpl, &mut args, insn);
                                            match insn & 0xe2000000 {
                                                0x02000000 => {
                                                    /* 00000010 1000.... .101.... .0000111 */
                                                    if transimpl.vl1re16_v(args) { return true; }
                                                },
                                                0x22000000 => {
                                                    /* 00100010 1000.... .101.... .0000111 */
                                                    if transimpl.vl2re16_v(args) { return true; }
                                                },
                                                0x62000000 => {
                                                    /* 01100010 1000.... .101.... .0000111 */
                                                    if transimpl.vl4re16_v(args) { return true; }
                                                },
                                                0xe2000000 => {
                                                    /* 11100010 1000.... .101.... .0000111 */
                                                    if transimpl.vl8re16_v(args) { return true; }
                                                },
                                                _ => { },
                                            };
                                        },
                                        0x10 => {
                                            /* ...000.1 0000.... .101.... .0000111 */
                                            decode_extract_r2_nfvm(transimpl, &mut args, insn);
                                            if transimpl.vle16ff_v(args) { return true; }
                                        },
                                        _ => { },
                                    };
                                },
                                0x1 => {
                                    /* ...010.. ........ .101.... .0000111 */
                                    decode_extract_r_nfvm(transimpl, &mut args, insn);
                                    if transimpl.vlse16_v(args) { return true; }
                                },
                                _ => { },
                            };
                        },
                        0x04000000 => {
                            /* ...0.1.. ........ .101.... .0000111 */
                            decode_extract_r_nfvm(transimpl, &mut args, insn);
                            if transimpl.vlxei16_v(args) { return true; }
                        },
                        _ => { },
                    };
                },
                0x6 => {
                    /* ........ ........ .110.... .0000111 */
                    match insn & 0x14000000 {
                        0x00000000 => {
                            /* ...0.0.. ........ .110.... .0000111 */
                            match (insn >> 27) & 0x1 {
                                0x0 => {
                                    /* ...000.. ........ .110.... .0000111 */
                                    match (insn >> 20) & 0x1f {
                                        0x0 => {
                                            /* ...000.0 0000.... .110.... .0000111 */
                                            decode_extract_r2_nfvm(transimpl, &mut args, insn);
                                            if transimpl.vle32_v(args) { return true; }
                                        },
                                        0x8 => {
                                            /* ...000.0 1000.... .110.... .0000111 */
                                            decode_extract_r2(transimpl, &mut args, insn);
                                            match insn & 0xe2000000 {
                                                0x02000000 => {
                                                    /* 00000010 1000.... .110.... .0000111 */
                                                    if transimpl.vl1re32_v(args) { return true; }
                                                },
                                                0x22000000 => {
                                                    /* 00100010 1000.... .110.... .0000111 */
                                                    if transimpl.vl2re32_v(args) { return true; }
                                                },
                                                0x62000000 => {
                                                    /* 01100010 1000.... .110.... .0000111 */
                                                    if transimpl.vl4re32_v(args) { return true; }
                                                },
                                                0xe2000000 => {
                                                    /* 11100010 1000.... .110.... .0000111 */
                                                    if transimpl.vl8re32_v(args) { return true; }
                                                },
                                                _ => { },
                                            };
                                        },
                                        0x10 => {
                                            /* ...000.1 0000.... .110.... .0000111 */
                                            decode_extract_r2_nfvm(transimpl, &mut args, insn);
                                            if transimpl.vle32ff_v(args) { return true; }
                                        },
                                        _ => { },
                                    };
                                },
                                0x1 => {
                                    /* ...010.. ........ .110.... .0000111 */
                                    decode_extract_r_nfvm(transimpl, &mut args, insn);
                                    if transimpl.vlse32_v(args) { return true; }
                                },
                                _ => { },
                            };
                        },
                        0x04000000 => {
                            /* ...0.1.. ........ .110.... .0000111 */
                            decode_extract_r_nfvm(transimpl, &mut args, insn);
                            if transimpl.vlxei32_v(args) { return true; }
                        },
                        _ => { },
                    };
                },
                0x7 => {
                    /* ........ ........ .111.... .0000111 */
                    match insn & 0x14000000 {
                        0x00000000 => {
                            /* ...0.0.. ........ .111.... .0000111 */
                            match (insn >> 27) & 0x1 {
                                0x0 => {
                                    /* ...000.. ........ .111.... .0000111 */
                                    match (insn >> 20) & 0x1f {
                                        0x0 => {
                                            /* ...000.0 0000.... .111.... .0000111 */
                                            decode_extract_r2_nfvm(transimpl, &mut args, insn);
                                            if transimpl.vle64_v(args) { return true; }
                                        },
                                        0x8 => {
                                            /* ...000.0 1000.... .111.... .0000111 */
                                            decode_extract_r2(transimpl, &mut args, insn);
                                            match insn & 0xe2000000 {
                                                0x02000000 => {
                                                    /* 00000010 1000.... .111.... .0000111 */
                                                    if transimpl.vl1re64_v(args) { return true; }
                                                },
                                                0x22000000 => {
                                                    /* 00100010 1000.... .111.... .0000111 */
                                                    if transimpl.vl2re64_v(args) { return true; }
                                                },
                                                0x62000000 => {
                                                    /* 01100010 1000.... .111.... .0000111 */
                                                    if transimpl.vl4re64_v(args) { return true; }
                                                },
                                                0xe2000000 => {
                                                    /* 11100010 1000.... .111.... .0000111 */
                                                    if transimpl.vl8re64_v(args) { return true; }
                                                },
                                                _ => { },
                                            };
                                        },
                                        0x10 => {
                                            /* ...000.1 0000.... .111.... .0000111 */
                                            decode_extract_r2_nfvm(transimpl, &mut args, insn);
                                            if transimpl.vle64ff_v(args) { return true; }
                                        },
                                        _ => { },
                                    };
                                },
                                0x1 => {
                                    /* ...010.. ........ .111.... .0000111 */
                                    decode_extract_r_nfvm(transimpl, &mut args, insn);
                                    if transimpl.vlse64_v(args) { return true; }
                                },
                                _ => { },
                            };
                        },
                        0x04000000 => {
                            /* ...0.1.. ........ .111.... .0000111 */
                            decode_extract_r_nfvm(transimpl, &mut args, insn);
                            if transimpl.vlxei64_v(args) { return true; }
                        },
                        _ => { },
                    };
                },
                _ => { },
            };
        },
        0x0000000f => {
            /* ........ ........ ........ .0001111 */
            match (insn >> 12) & 0x7 {
                0x0 => {
                    /* ........ ........ .000.... .0001111 */
                    if (insn & 0xffff8f80) == 0x01000000 {
                        decode_extract_decode_Fmt_33(transimpl, &mut args, insn);
                        if transimpl.pause(args) { return true; }
                    }
                    decode_extract_decode_Fmt_34(transimpl, &mut args, insn);
                    if transimpl.fence(args) { return true; }
                },
                0x1 => {
                    /* ........ ........ .001.... .0001111 */
                    decode_extract_decode_Fmt_33(transimpl, &mut args, insn);
                    if transimpl.fence_i(args) { return true; }
                },
                0x2 => {
                    /* ........ ........ .010.... .0001111 */
                    decode_extract_i(transimpl, &mut args, insn);
                    if transimpl.lq(args) { return true; }
                },
                _ => { },
            };
        },
        0x00000013 => {
            /* ........ ........ ........ .0010011 */
            match (insn >> 12) & 0x7 {
                0x0 => {
                    /* ........ ........ .000.... .0010011 */
                    decode_extract_i(transimpl, &mut args, insn);
                    if transimpl.addi(args) { return true; }
                },
                0x1 => {
                    /* ........ ........ .001.... .0010011 */
                    match (insn >> 27) & 0x1f {
                        0x0 => {
                            /* 00000... ........ .001.... .0010011 */
                            decode_extract_sh(transimpl, &mut args, insn);
                            if transimpl.slli(args) { return true; }
                        },
                        0x1 => {
                            /* 00001... ........ .001.... .0010011 */
                            decode_extract_r2(transimpl, &mut args, insn);
                            match (insn >> 20) & 0x7f {
                                0xf => {
                                    /* 00001000 1111.... .001.... .0010011 */
                                    if transimpl.zip(args) { return true; }
                                },
                                _ => { },
                            };
                        },
                        0x2 => {
                            /* 00010... ........ .001.... .0010011 */
                            decode_extract_r2(transimpl, &mut args, insn);
                            match (insn >> 20) & 0x7f {
                                0x0 => {
                                    /* 00010000 0000.... .001.... .0010011 */
                                    if transimpl.sha256sum0(args) { return true; }
                                },
                                0x1 => {
                                    /* 00010000 0001.... .001.... .0010011 */
                                    if transimpl.sha256sum1(args) { return true; }
                                },
                                0x2 => {
                                    /* 00010000 0010.... .001.... .0010011 */
                                    if transimpl.sha256sig0(args) { return true; }
                                },
                                0x3 => {
                                    /* 00010000 0011.... .001.... .0010011 */
                                    if transimpl.sha256sig1(args) { return true; }
                                },
                                0x4 => {
                                    /* 00010000 0100.... .001.... .0010011 */
                                    if transimpl.sha512sum0(args) { return true; }
                                },
                                0x5 => {
                                    /* 00010000 0101.... .001.... .0010011 */
                                    if transimpl.sha512sum1(args) { return true; }
                                },
                                0x6 => {
                                    /* 00010000 0110.... .001.... .0010011 */
                                    if transimpl.sha512sig0(args) { return true; }
                                },
                                0x7 => {
                                    /* 00010000 0111.... .001.... .0010011 */
                                    if transimpl.sha512sig1(args) { return true; }
                                },
                                0x8 => {
                                    /* 00010000 1000.... .001.... .0010011 */
                                    if transimpl.sm3p0(args) { return true; }
                                },
                                0x9 => {
                                    /* 00010000 1001.... .001.... .0010011 */
                                    if transimpl.sm3p1(args) { return true; }
                                },
                                _ => { },
                            };
                        },
                        0x5 => {
                            /* 00101... ........ .001.... .0010011 */
                            decode_extract_sh(transimpl, &mut args, insn);
                            if transimpl.bseti(args) { return true; }
                        },
                        0x6 => {
                            /* 00110... ........ .001.... .0010011 */
                            match (insn >> 24) & 0x7 {
                                0x0 => {
                                    /* 00110000 ........ .001.... .0010011 */
                                    decode_extract_r2(transimpl, &mut args, insn);
                                    match (insn >> 20) & 0xf {
                                        0x0 => {
                                            /* 00110000 0000.... .001.... .0010011 */
                                            if transimpl.aes64im(args) { return true; }
                                        },
                                        _ => { },
                                    };
                                },
                                0x1 => {
                                    /* 00110001 ........ .001.... .0010011 */
                                    decode_extract_i_aes(transimpl, &mut args, insn);
                                    if transimpl.aes64ks1i(args) { return true; }
                                },
                                _ => { },
                            };
                        },
                        0x9 => {
                            /* 01001... ........ .001.... .0010011 */
                            decode_extract_sh(transimpl, &mut args, insn);
                            if transimpl.bclri(args) { return true; }
                        },
                        0xc => {
                            /* 01100... ........ .001.... .0010011 */
                            decode_extract_r2(transimpl, &mut args, insn);
                            match (insn >> 20) & 0x7f {
                                0x0 => {
                                    /* 01100000 0000.... .001.... .0010011 */
                                    if transimpl.clz(args) { return true; }
                                },
                                0x1 => {
                                    /* 01100000 0001.... .001.... .0010011 */
                                    if transimpl.ctz(args) { return true; }
                                },
                                0x2 => {
                                    /* 01100000 0010.... .001.... .0010011 */
                                    if transimpl.cpop(args) { return true; }
                                },
                                0x4 => {
                                    /* 01100000 0100.... .001.... .0010011 */
                                    if transimpl.sext_b(args) { return true; }
                                },
                                0x5 => {
                                    /* 01100000 0101.... .001.... .0010011 */
                                    if transimpl.sext_h(args) { return true; }
                                },
                                _ => { },
                            };
                        },
                        0xd => {
                            /* 01101... ........ .001.... .0010011 */
                            decode_extract_sh(transimpl, &mut args, insn);
                            if transimpl.binvi(args) { return true; }
                        },
                        _ => { },
                    };
                },
                0x2 => {
                    /* ........ ........ .010.... .0010011 */
                    decode_extract_i(transimpl, &mut args, insn);
                    if transimpl.slti(args) { return true; }
                },
                0x3 => {
                    /* ........ ........ .011.... .0010011 */
                    decode_extract_i(transimpl, &mut args, insn);
                    if transimpl.sltiu(args) { return true; }
                },
                0x4 => {
                    /* ........ ........ .100.... .0010011 */
                    decode_extract_i(transimpl, &mut args, insn);
                    if transimpl.xori(args) { return true; }
                },
                0x5 => {
                    /* ........ ........ .101.... .0010011 */
                    match (insn >> 27) & 0x1f {
                        0x0 => {
                            /* 00000... ........ .101.... .0010011 */
                            decode_extract_sh(transimpl, &mut args, insn);
                            if transimpl.srli(args) { return true; }
                        },
                        0x1 => {
                            /* 00001... ........ .101.... .0010011 */
                            decode_extract_r2(transimpl, &mut args, insn);
                            match (insn >> 20) & 0x7f {
                                0xf => {
                                    /* 00001000 1111.... .101.... .0010011 */
                                    if transimpl.unzip(args) { return true; }
                                },
                                _ => { },
                            };
                        },
                        0x5 => {
                            /* 00101... ........ .101.... .0010011 */
                            decode_extract_r2(transimpl, &mut args, insn);
                            match (insn >> 20) & 0x7f {
                                0x7 => {
                                    /* 00101000 0111.... .101.... .0010011 */
                                    if transimpl.orc_b(args) { return true; }
                                },
                                _ => { },
                            };
                        },
                        0x8 => {
                            /* 01000... ........ .101.... .0010011 */
                            decode_extract_sh(transimpl, &mut args, insn);
                            if transimpl.srai(args) { return true; }
                        },
                        0x9 => {
                            /* 01001... ........ .101.... .0010011 */
                            decode_extract_sh(transimpl, &mut args, insn);
                            if transimpl.bexti(args) { return true; }
                        },
                        0xc => {
                            /* 01100... ........ .101.... .0010011 */
                            decode_extract_sh(transimpl, &mut args, insn);
                            if transimpl.rori(args) { return true; }
                        },
                        0xd => {
                            /* 01101... ........ .101.... .0010011 */
                            decode_extract_r2(transimpl, &mut args, insn);
                            match (insn >> 20) & 0x7f {
                                0x7 => {
                                    /* 01101000 0111.... .101.... .0010011 */
                                    if transimpl.brev8(args) { return true; }
                                },
                                0x18 => {
                                    /* 01101001 1000.... .101.... .0010011 */
                                    if transimpl.rev8_32(args) { return true; }
                                },
                                0x38 => {
                                    /* 01101011 1000.... .101.... .0010011 */
                                    if transimpl.rev8_64(args) { return true; }
                                },
                                _ => { },
                            };
                        },
                        _ => { },
                    };
                },
                0x6 => {
                    /* ........ ........ .110.... .0010011 */
                    decode_extract_i(transimpl, &mut args, insn);
                    if transimpl.ori(args) { return true; }
                },
                0x7 => {
                    /* ........ ........ .111.... .0010011 */
                    decode_extract_i(transimpl, &mut args, insn);
                    if transimpl.andi(args) { return true; }
                },
                _ => { },
            };
        },
        0x00000017 => {
            /* ........ ........ ........ .0010111 */
            decode_extract_u(transimpl, &mut args, insn);
            if transimpl.auipc(args) { return true; }
        },
        0x0000001b => {
            /* ........ ........ ........ .0011011 */
            match (insn >> 12) & 0x7 {
                0x0 => {
                    /* ........ ........ .000.... .0011011 */
                    decode_extract_i(transimpl, &mut args, insn);
                    if transimpl.addiw(args) { return true; }
                },
                0x1 => {
                    /* ........ ........ .001.... .0011011 */
                    match (insn >> 27) & 0x1f {
                        0x0 => {
                            /* 00000... ........ .001.... .0011011 */
                            decode_extract_sh5(transimpl, &mut args, insn);
                            match (insn >> 25) & 0x3 {
                                0x0 => {
                                    /* 0000000. ........ .001.... .0011011 */
                                    if transimpl.slliw(args) { return true; }
                                },
                                _ => { },
                            };
                        },
                        0x1 => {
                            /* 00001... ........ .001.... .0011011 */
                            decode_extract_sh(transimpl, &mut args, insn);
                            if transimpl.slli_uw(args) { return true; }
                        },
                        0xc => {
                            /* 01100... ........ .001.... .0011011 */
                            decode_extract_r2(transimpl, &mut args, insn);
                            match (insn >> 20) & 0x7f {
                                0x0 => {
                                    /* 01100000 0000.... .001.... .0011011 */
                                    if transimpl.clzw(args) { return true; }
                                },
                                0x1 => {
                                    /* 01100000 0001.... .001.... .0011011 */
                                    if transimpl.ctzw(args) { return true; }
                                },
                                0x2 => {
                                    /* 01100000 0010.... .001.... .0011011 */
                                    if transimpl.cpopw(args) { return true; }
                                },
                                _ => { },
                            };
                        },
                        _ => { },
                    };
                },
                0x5 => {
                    /* ........ ........ .101.... .0011011 */
                    decode_extract_sh5(transimpl, &mut args, insn);
                    match (insn >> 25) & 0x7f {
                        0x0 => {
                            /* 0000000. ........ .101.... .0011011 */
                            if transimpl.srliw(args) { return true; }
                        },
                        0x20 => {
                            /* 0100000. ........ .101.... .0011011 */
                            if transimpl.sraiw(args) { return true; }
                        },
                        0x30 => {
                            /* 0110000. ........ .101.... .0011011 */
                            if transimpl.roriw(args) { return true; }
                        },
                        _ => { },
                    };
                },
                _ => { },
            };
        },
        0x00000023 => {
            /* ........ ........ ........ .0100011 */
            decode_extract_s(transimpl, &mut args, insn);
            match (insn >> 12) & 0x7 {
                0x0 => {
                    /* ........ ........ .000.... .0100011 */
                    if transimpl.sb(args) { return true; }
                },
                0x1 => {
                    /* ........ ........ .001.... .0100011 */
                    if transimpl.sh(args) { return true; }
                },
                0x2 => {
                    /* ........ ........ .010.... .0100011 */
                    if transimpl.sw(args) { return true; }
                },
                0x3 => {
                    /* ........ ........ .011.... .0100011 */
                    if transimpl.sd(args) { return true; }
                },
                0x4 => {
                    /* ........ ........ .100.... .0100011 */
                    if transimpl.sq(args) { return true; }
                },
                _ => { },
            };
        },
        0x00000027 => {
            /* ........ ........ ........ .0100111 */
            match (insn >> 12) & 0x7 {
                0x0 => {
                    /* ........ ........ .000.... .0100111 */
                    match insn & 0x14000000 {
                        0x00000000 => {
                            /* ...0.0.. ........ .000.... .0100111 */
                            match (insn >> 27) & 0x1 {
                                0x0 => {
                                    /* ...000.. ........ .000.... .0100111 */
                                    match (insn >> 20) & 0x1f {
                                        0x0 => {
                                            /* ...000.0 0000.... .000.... .0100111 */
                                            decode_extract_r2_nfvm(transimpl, &mut args, insn);
                                            if transimpl.vse8_v(args) { return true; }
                                        },
                                        0x8 => {
                                            /* ...000.0 1000.... .000.... .0100111 */
                                            decode_extract_r2(transimpl, &mut args, insn);
                                            match insn & 0xe2000000 {
                                                0x02000000 => {
                                                    /* 00000010 1000.... .000.... .0100111 */
                                                    if transimpl.vs1r_v(args) { return true; }
                                                },
                                                0x22000000 => {
                                                    /* 00100010 1000.... .000.... .0100111 */
                                                    if transimpl.vs2r_v(args) { return true; }
                                                },
                                                0x62000000 => {
                                                    /* 01100010 1000.... .000.... .0100111 */
                                                    if transimpl.vs4r_v(args) { return true; }
                                                },
                                                0xe2000000 => {
                                                    /* 11100010 1000.... .000.... .0100111 */
                                                    if transimpl.vs8r_v(args) { return true; }
                                                },
                                                _ => { },
                                            };
                                        },
                                        0xb => {
                                            /* ...000.0 1011.... .000.... .0100111 */
                                            decode_extract_r2(transimpl, &mut args, insn);
                                            match insn & 0xe2000000 {
                                                0x02000000 => {
                                                    /* 00000010 1011.... .000.... .0100111 */
                                                    if transimpl.vsm_v(args) { return true; }
                                                },
                                                _ => { },
                                            };
                                        },
                                        _ => { },
                                    };
                                },
                                0x1 => {
                                    /* ...010.. ........ .000.... .0100111 */
                                    decode_extract_r_nfvm(transimpl, &mut args, insn);
                                    if transimpl.vsse8_v(args) { return true; }
                                },
                                _ => { },
                            };
                        },
                        0x04000000 => {
                            /* ...0.1.. ........ .000.... .0100111 */
                            decode_extract_r_nfvm(transimpl, &mut args, insn);
                            if transimpl.vsxei8_v(args) { return true; }
                        },
                        _ => { },
                    };
                },
                0x1 => {
                    /* ........ ........ .001.... .0100111 */
                    decode_extract_s(transimpl, &mut args, insn);
                    if transimpl.fsh(args) { return true; }
                },
                0x2 => {
                    /* ........ ........ .010.... .0100111 */
                    decode_extract_s(transimpl, &mut args, insn);
                    if transimpl.fsw(args) { return true; }
                },
                0x3 => {
                    /* ........ ........ .011.... .0100111 */
                    decode_extract_s(transimpl, &mut args, insn);
                    if transimpl.fsd(args) { return true; }
                },
                0x5 => {
                    /* ........ ........ .101.... .0100111 */
                    match insn & 0x14000000 {
                        0x00000000 => {
                            /* ...0.0.. ........ .101.... .0100111 */
                            match (insn >> 27) & 0x1 {
                                0x0 => {
                                    /* ...000.. ........ .101.... .0100111 */
                                    decode_extract_r2_nfvm(transimpl, &mut args, insn);
                                    match (insn >> 20) & 0x1f {
                                        0x0 => {
                                            /* ...000.0 0000.... .101.... .0100111 */
                                            if transimpl.vse16_v(args) { return true; }
                                        },
                                        _ => { },
                                    };
                                },
                                0x1 => {
                                    /* ...010.. ........ .101.... .0100111 */
                                    decode_extract_r_nfvm(transimpl, &mut args, insn);
                                    if transimpl.vsse16_v(args) { return true; }
                                },
                                _ => { },
                            };
                        },
                        0x04000000 => {
                            /* ...0.1.. ........ .101.... .0100111 */
                            decode_extract_r_nfvm(transimpl, &mut args, insn);
                            if transimpl.vsxei16_v(args) { return true; }
                        },
                        _ => { },
                    };
                },
                0x6 => {
                    /* ........ ........ .110.... .0100111 */
                    match insn & 0x14000000 {
                        0x00000000 => {
                            /* ...0.0.. ........ .110.... .0100111 */
                            match (insn >> 27) & 0x1 {
                                0x0 => {
                                    /* ...000.. ........ .110.... .0100111 */
                                    decode_extract_r2_nfvm(transimpl, &mut args, insn);
                                    match (insn >> 20) & 0x1f {
                                        0x0 => {
                                            /* ...000.0 0000.... .110.... .0100111 */
                                            if transimpl.vse32_v(args) { return true; }
                                        },
                                        _ => { },
                                    };
                                },
                                0x1 => {
                                    /* ...010.. ........ .110.... .0100111 */
                                    decode_extract_r_nfvm(transimpl, &mut args, insn);
                                    if transimpl.vsse32_v(args) { return true; }
                                },
                                _ => { },
                            };
                        },
                        0x04000000 => {
                            /* ...0.1.. ........ .110.... .0100111 */
                            decode_extract_r_nfvm(transimpl, &mut args, insn);
                            if transimpl.vsxei32_v(args) { return true; }
                        },
                        _ => { },
                    };
                },
                0x7 => {
                    /* ........ ........ .111.... .0100111 */
                    match insn & 0x14000000 {
                        0x00000000 => {
                            /* ...0.0.. ........ .111.... .0100111 */
                            match (insn >> 27) & 0x1 {
                                0x0 => {
                                    /* ...000.. ........ .111.... .0100111 */
                                    decode_extract_r2_nfvm(transimpl, &mut args, insn);
                                    match (insn >> 20) & 0x1f {
                                        0x0 => {
                                            /* ...000.0 0000.... .111.... .0100111 */
                                            if transimpl.vse64_v(args) { return true; }
                                        },
                                        _ => { },
                                    };
                                },
                                0x1 => {
                                    /* ...010.. ........ .111.... .0100111 */
                                    decode_extract_r_nfvm(transimpl, &mut args, insn);
                                    if transimpl.vsse64_v(args) { return true; }
                                },
                                _ => { },
                            };
                        },
                        0x04000000 => {
                            /* ...0.1.. ........ .111.... .0100111 */
                            decode_extract_r_nfvm(transimpl, &mut args, insn);
                            if transimpl.vsxei64_v(args) { return true; }
                        },
                        _ => { },
                    };
                },
                _ => { },
            };
        },
        0x0000002f => {
            /* ........ ........ ........ .0101111 */
            match insn & 0xf8007000 {
                0x00002000 => {
                    /* 00000... ........ .010.... .0101111 */
                    decode_extract_atom_st(transimpl, &mut args, insn);
                    if transimpl.amoadd_w(args) { return true; }
                },
                0x00003000 => {
                    /* 00000... ........ .011.... .0101111 */
                    decode_extract_atom_st(transimpl, &mut args, insn);
                    if transimpl.amoadd_d(args) { return true; }
                },
                0x08002000 => {
                    /* 00001... ........ .010.... .0101111 */
                    decode_extract_atom_st(transimpl, &mut args, insn);
                    if transimpl.amoswap_w(args) { return true; }
                },
                0x08003000 => {
                    /* 00001... ........ .011.... .0101111 */
                    decode_extract_atom_st(transimpl, &mut args, insn);
                    if transimpl.amoswap_d(args) { return true; }
                },
                0x10002000 => {
                    /* 00010... ........ .010.... .0101111 */
                    decode_extract_atom_ld(transimpl, &mut args, insn);
                    match (insn >> 20) & 0x1f {
                        0x0 => {
                            /* 00010..0 0000.... .010.... .0101111 */
                            if transimpl.lr_w(args) { return true; }
                        },
                        _ => { },
                    };
                },
                0x10003000 => {
                    /* 00010... ........ .011.... .0101111 */
                    decode_extract_atom_ld(transimpl, &mut args, insn);
                    match (insn >> 20) & 0x1f {
                        0x0 => {
                            /* 00010..0 0000.... .011.... .0101111 */
                            if transimpl.lr_d(args) { return true; }
                        },
                        _ => { },
                    };
                },
                0x18002000 => {
                    /* 00011... ........ .010.... .0101111 */
                    decode_extract_atom_st(transimpl, &mut args, insn);
                    if transimpl.sc_w(args) { return true; }
                },
                0x18003000 => {
                    /* 00011... ........ .011.... .0101111 */
                    decode_extract_atom_st(transimpl, &mut args, insn);
                    if transimpl.sc_d(args) { return true; }
                },
                0x20002000 => {
                    /* 00100... ........ .010.... .0101111 */
                    decode_extract_atom_st(transimpl, &mut args, insn);
                    if transimpl.amoxor_w(args) { return true; }
                },
                0x20003000 => {
                    /* 00100... ........ .011.... .0101111 */
                    decode_extract_atom_st(transimpl, &mut args, insn);
                    if transimpl.amoxor_d(args) { return true; }
                },
                0x40002000 => {
                    /* 01000... ........ .010.... .0101111 */
                    decode_extract_atom_st(transimpl, &mut args, insn);
                    if transimpl.amoor_w(args) { return true; }
                },
                0x40003000 => {
                    /* 01000... ........ .011.... .0101111 */
                    decode_extract_atom_st(transimpl, &mut args, insn);
                    if transimpl.amoor_d(args) { return true; }
                },
                0x60002000 => {
                    /* 01100... ........ .010.... .0101111 */
                    decode_extract_atom_st(transimpl, &mut args, insn);
                    if transimpl.amoand_w(args) { return true; }
                },
                0x60003000 => {
                    /* 01100... ........ .011.... .0101111 */
                    decode_extract_atom_st(transimpl, &mut args, insn);
                    if transimpl.amoand_d(args) { return true; }
                },
                0x80002000 => {
                    /* 10000... ........ .010.... .0101111 */
                    decode_extract_atom_st(transimpl, &mut args, insn);
                    if transimpl.amomin_w(args) { return true; }
                },
                0x80003000 => {
                    /* 10000... ........ .011.... .0101111 */
                    decode_extract_atom_st(transimpl, &mut args, insn);
                    if transimpl.amomin_d(args) { return true; }
                },
                0xa0002000 => {
                    /* 10100... ........ .010.... .0101111 */
                    decode_extract_atom_st(transimpl, &mut args, insn);
                    if transimpl.amomax_w(args) { return true; }
                },
                0xa0003000 => {
                    /* 10100... ........ .011.... .0101111 */
                    decode_extract_atom_st(transimpl, &mut args, insn);
                    if transimpl.amomax_d(args) { return true; }
                },
                0xc0002000 => {
                    /* 11000... ........ .010.... .0101111 */
                    decode_extract_atom_st(transimpl, &mut args, insn);
                    if transimpl.amominu_w(args) { return true; }
                },
                0xc0003000 => {
                    /* 11000... ........ .011.... .0101111 */
                    decode_extract_atom_st(transimpl, &mut args, insn);
                    if transimpl.amominu_d(args) { return true; }
                },
                0xe0002000 => {
                    /* 11100... ........ .010.... .0101111 */
                    decode_extract_atom_st(transimpl, &mut args, insn);
                    if transimpl.amomaxu_w(args) { return true; }
                },
                0xe0003000 => {
                    /* 11100... ........ .011.... .0101111 */
                    decode_extract_atom_st(transimpl, &mut args, insn);
                    if transimpl.amomaxu_d(args) { return true; }
                },
                _ => { },
            };
        },
        0x00000033 => {
            /* ........ ........ ........ .0110011 */
            match insn & 0x3e007000 {
                0x00000000 => {
                    /* ..00000. ........ .000.... .0110011 */
                    decode_extract_r(transimpl, &mut args, insn);
                    match (insn >> 30) & 0x3 {
                        0x0 => {
                            /* 0000000. ........ .000.... .0110011 */
                            if transimpl.add(args) { return true; }
                        },
                        0x1 => {
                            /* 0100000. ........ .000.... .0110011 */
                            if transimpl.sub(args) { return true; }
                        },
                        _ => { },
                    };
                },
                0x00001000 => {
                    /* ..00000. ........ .001.... .0110011 */
                    decode_extract_r(transimpl, &mut args, insn);
                    match (insn >> 30) & 0x3 {
                        0x0 => {
                            /* 0000000. ........ .001.... .0110011 */
                            if transimpl.sll(args) { return true; }
                        },
                        _ => { },
                    };
                },
                0x00002000 => {
                    /* ..00000. ........ .010.... .0110011 */
                    decode_extract_r(transimpl, &mut args, insn);
                    match (insn >> 30) & 0x3 {
                        0x0 => {
                            /* 0000000. ........ .010.... .0110011 */
                            if transimpl.slt(args) { return true; }
                        },
                        _ => { },
                    };
                },
                0x00003000 => {
                    /* ..00000. ........ .011.... .0110011 */
                    decode_extract_r(transimpl, &mut args, insn);
                    match (insn >> 30) & 0x3 {
                        0x0 => {
                            /* 0000000. ........ .011.... .0110011 */
                            if transimpl.sltu(args) { return true; }
                        },
                        _ => { },
                    };
                },
                0x00004000 => {
                    /* ..00000. ........ .100.... .0110011 */
                    decode_extract_r(transimpl, &mut args, insn);
                    match (insn >> 30) & 0x3 {
                        0x0 => {
                            /* 0000000. ........ .100.... .0110011 */
                            if transimpl.xor(args) { return true; }
                        },
                        0x1 => {
                            /* 0100000. ........ .100.... .0110011 */
                            if transimpl.xnor(args) { return true; }
                        },
                        _ => { },
                    };
                },
                0x00005000 => {
                    /* ..00000. ........ .101.... .0110011 */
                    decode_extract_r(transimpl, &mut args, insn);
                    match (insn >> 30) & 0x3 {
                        0x0 => {
                            /* 0000000. ........ .101.... .0110011 */
                            if transimpl.srl(args) { return true; }
                        },
                        0x1 => {
                            /* 0100000. ........ .101.... .0110011 */
                            if transimpl.sra(args) { return true; }
                        },
                        _ => { },
                    };
                },
                0x00006000 => {
                    /* ..00000. ........ .110.... .0110011 */
                    decode_extract_r(transimpl, &mut args, insn);
                    match (insn >> 30) & 0x3 {
                        0x0 => {
                            /* 0000000. ........ .110.... .0110011 */
                            if transimpl.or(args) { return true; }
                        },
                        0x1 => {
                            /* 0100000. ........ .110.... .0110011 */
                            if transimpl.orn(args) { return true; }
                        },
                        _ => { },
                    };
                },
                0x00007000 => {
                    /* ..00000. ........ .111.... .0110011 */
                    decode_extract_r(transimpl, &mut args, insn);
                    match (insn >> 30) & 0x3 {
                        0x0 => {
                            /* 0000000. ........ .111.... .0110011 */
                            if transimpl.and(args) { return true; }
                        },
                        0x1 => {
                            /* 0100000. ........ .111.... .0110011 */
                            if transimpl.andn(args) { return true; }
                        },
                        _ => { },
                    };
                },
                0x02000000 => {
                    /* ..00001. ........ .000.... .0110011 */
                    decode_extract_r(transimpl, &mut args, insn);
                    match (insn >> 30) & 0x3 {
                        0x0 => {
                            /* 0000001. ........ .000.... .0110011 */
                            if transimpl.mul(args) { return true; }
                        },
                        _ => { },
                    };
                },
                0x02001000 => {
                    /* ..00001. ........ .001.... .0110011 */
                    decode_extract_r(transimpl, &mut args, insn);
                    match (insn >> 30) & 0x3 {
                        0x0 => {
                            /* 0000001. ........ .001.... .0110011 */
                            if transimpl.mulh(args) { return true; }
                        },
                        _ => { },
                    };
                },
                0x02002000 => {
                    /* ..00001. ........ .010.... .0110011 */
                    decode_extract_r(transimpl, &mut args, insn);
                    match (insn >> 30) & 0x3 {
                        0x0 => {
                            /* 0000001. ........ .010.... .0110011 */
                            if transimpl.mulhsu(args) { return true; }
                        },
                        _ => { },
                    };
                },
                0x02003000 => {
                    /* ..00001. ........ .011.... .0110011 */
                    decode_extract_r(transimpl, &mut args, insn);
                    match (insn >> 30) & 0x3 {
                        0x0 => {
                            /* 0000001. ........ .011.... .0110011 */
                            if transimpl.mulhu(args) { return true; }
                        },
                        _ => { },
                    };
                },
                0x02004000 => {
                    /* ..00001. ........ .100.... .0110011 */
                    decode_extract_r(transimpl, &mut args, insn);
                    match (insn >> 30) & 0x3 {
                        0x0 => {
                            /* 0000001. ........ .100.... .0110011 */
                            if transimpl.div(args) { return true; }
                        },
                        _ => { },
                    };
                },
                0x02005000 => {
                    /* ..00001. ........ .101.... .0110011 */
                    decode_extract_r(transimpl, &mut args, insn);
                    match (insn >> 30) & 0x3 {
                        0x0 => {
                            /* 0000001. ........ .101.... .0110011 */
                            if transimpl.divu(args) { return true; }
                        },
                        _ => { },
                    };
                },
                0x02006000 => {
                    /* ..00001. ........ .110.... .0110011 */
                    decode_extract_r(transimpl, &mut args, insn);
                    match (insn >> 30) & 0x3 {
                        0x0 => {
                            /* 0000001. ........ .110.... .0110011 */
                            if transimpl.rem(args) { return true; }
                        },
                        _ => { },
                    };
                },
                0x02007000 => {
                    /* ..00001. ........ .111.... .0110011 */
                    decode_extract_r(transimpl, &mut args, insn);
                    match (insn >> 30) & 0x3 {
                        0x0 => {
                            /* 0000001. ........ .111.... .0110011 */
                            if transimpl.remu(args) { return true; }
                        },
                        _ => { },
                    };
                },
                0x08001000 => {
                    /* ..00100. ........ .001.... .0110011 */
                    decode_extract_r(transimpl, &mut args, insn);
                    match (insn >> 30) & 0x3 {
                        0x1 => {
                            /* 0100100. ........ .001.... .0110011 */
                            if transimpl.bclr(args) { return true; }
                        },
                        _ => { },
                    };
                },
                0x08004000 => {
                    /* ..00100. ........ .100.... .0110011 */
                    match (insn >> 30) & 0x3 {
                        0x0 => {
                            /* 0000100. ........ .100.... .0110011 */
                            if (insn & 0x01f00000) == 0x00000000 {
                                decode_extract_r2(transimpl, &mut args, insn);
                                if transimpl.zext_h_32(args) { return true; }
                            }
                            decode_extract_r(transimpl, &mut args, insn);
                            if transimpl.pack(args) { return true; }
                        },
                        _ => { },
                    };
                },
                0x08005000 => {
                    /* ..00100. ........ .101.... .0110011 */
                    decode_extract_r(transimpl, &mut args, insn);
                    match (insn >> 30) & 0x3 {
                        0x1 => {
                            /* 0100100. ........ .101.... .0110011 */
                            if transimpl.bext(args) { return true; }
                        },
                        _ => { },
                    };
                },
                0x08007000 => {
                    /* ..00100. ........ .111.... .0110011 */
                    decode_extract_r(transimpl, &mut args, insn);
                    match (insn >> 30) & 0x3 {
                        0x0 => {
                            /* 0000100. ........ .111.... .0110011 */
                            if transimpl.packh(args) { return true; }
                        },
                        _ => { },
                    };
                },
                0x0a001000 => {
                    /* ..00101. ........ .001.... .0110011 */
                    decode_extract_r(transimpl, &mut args, insn);
                    match (insn >> 30) & 0x3 {
                        0x0 => {
                            /* 0000101. ........ .001.... .0110011 */
                            if transimpl.clmul(args) { return true; }
                        },
                        _ => { },
                    };
                },
                0x0a002000 => {
                    /* ..00101. ........ .010.... .0110011 */
                    decode_extract_r(transimpl, &mut args, insn);
                    match (insn >> 30) & 0x3 {
                        0x0 => {
                            /* 0000101. ........ .010.... .0110011 */
                            if transimpl.clmulr(args) { return true; }
                        },
                        _ => { },
                    };
                },
                0x0a003000 => {
                    /* ..00101. ........ .011.... .0110011 */
                    decode_extract_r(transimpl, &mut args, insn);
                    match (insn >> 30) & 0x3 {
                        0x0 => {
                            /* 0000101. ........ .011.... .0110011 */
                            if transimpl.clmulh(args) { return true; }
                        },
                        _ => { },
                    };
                },
                0x0a004000 => {
                    /* ..00101. ........ .100.... .0110011 */
                    decode_extract_r(transimpl, &mut args, insn);
                    match (insn >> 30) & 0x3 {
                        0x0 => {
                            /* 0000101. ........ .100.... .0110011 */
                            if transimpl.min(args) { return true; }
                        },
                        _ => { },
                    };
                },
                0x0a005000 => {
                    /* ..00101. ........ .101.... .0110011 */
                    decode_extract_r(transimpl, &mut args, insn);
                    match (insn >> 30) & 0x3 {
                        0x0 => {
                            /* 0000101. ........ .101.... .0110011 */
                            if transimpl.minu(args) { return true; }
                        },
                        _ => { },
                    };
                },
                0x0a006000 => {
                    /* ..00101. ........ .110.... .0110011 */
                    decode_extract_r(transimpl, &mut args, insn);
                    match (insn >> 30) & 0x3 {
                        0x0 => {
                            /* 0000101. ........ .110.... .0110011 */
                            if transimpl.max(args) { return true; }
                        },
                        _ => { },
                    };
                },
                0x0a007000 => {
                    /* ..00101. ........ .111.... .0110011 */
                    decode_extract_r(transimpl, &mut args, insn);
                    match (insn >> 30) & 0x3 {
                        0x0 => {
                            /* 0000101. ........ .111.... .0110011 */
                            if transimpl.maxu(args) { return true; }
                        },
                        _ => { },
                    };
                },
                0x10000000 => {
                    /* ..01000. ........ .000.... .0110011 */
                    decode_extract_r(transimpl, &mut args, insn);
                    match (insn >> 30) & 0x3 {
                        0x1 => {
                            /* 0101000. ........ .000.... .0110011 */
                            if transimpl.sha512sum0r(args) { return true; }
                        },
                        _ => { },
                    };
                },
                0x12000000 => {
                    /* ..01001. ........ .000.... .0110011 */
                    decode_extract_r(transimpl, &mut args, insn);
                    match (insn >> 30) & 0x3 {
                        0x1 => {
                            /* 0101001. ........ .000.... .0110011 */
                            if transimpl.sha512sum1r(args) { return true; }
                        },
                        _ => { },
                    };
                },
                0x14000000 => {
                    /* ..01010. ........ .000.... .0110011 */
                    decode_extract_r(transimpl, &mut args, insn);
                    match (insn >> 30) & 0x3 {
                        0x1 => {
                            /* 0101010. ........ .000.... .0110011 */
                            if transimpl.sha512sig0l(args) { return true; }
                        },
                        _ => { },
                    };
                },
                0x16000000 => {
                    /* ..01011. ........ .000.... .0110011 */
                    decode_extract_r(transimpl, &mut args, insn);
                    match (insn >> 30) & 0x3 {
                        0x1 => {
                            /* 0101011. ........ .000.... .0110011 */
                            if transimpl.sha512sig1l(args) { return true; }
                        },
                        _ => { },
                    };
                },
                0x1c000000 => {
                    /* ..01110. ........ .000.... .0110011 */
                    decode_extract_r(transimpl, &mut args, insn);
                    match (insn >> 30) & 0x3 {
                        0x1 => {
                            /* 0101110. ........ .000.... .0110011 */
                            if transimpl.sha512sig0h(args) { return true; }
                        },
                        _ => { },
                    };
                },
                0x1e000000 => {
                    /* ..01111. ........ .000.... .0110011 */
                    decode_extract_r(transimpl, &mut args, insn);
                    match (insn >> 30) & 0x3 {
                        0x1 => {
                            /* 0101111. ........ .000.... .0110011 */
                            if transimpl.sha512sig1h(args) { return true; }
                        },
                        _ => { },
                    };
                },
                0x20001000 => {
                    /* ..10000. ........ .001.... .0110011 */
                    decode_extract_r(transimpl, &mut args, insn);
                    match (insn >> 30) & 0x3 {
                        0x1 => {
                            /* 0110000. ........ .001.... .0110011 */
                            if transimpl.rol(args) { return true; }
                        },
                        _ => { },
                    };
                },
                0x20002000 => {
                    /* ..10000. ........ .010.... .0110011 */
                    decode_extract_r(transimpl, &mut args, insn);
                    match (insn >> 30) & 0x3 {
                        0x0 => {
                            /* 0010000. ........ .010.... .0110011 */
                            if transimpl.sh1add(args) { return true; }
                        },
                        _ => { },
                    };
                },
                0x20004000 => {
                    /* ..10000. ........ .100.... .0110011 */
                    decode_extract_r(transimpl, &mut args, insn);
                    match (insn >> 30) & 0x3 {
                        0x0 => {
                            /* 0010000. ........ .100.... .0110011 */
                            if transimpl.sh2add(args) { return true; }
                        },
                        _ => { },
                    };
                },
                0x20005000 => {
                    /* ..10000. ........ .101.... .0110011 */
                    decode_extract_r(transimpl, &mut args, insn);
                    match (insn >> 30) & 0x3 {
                        0x1 => {
                            /* 0110000. ........ .101.... .0110011 */
                            if transimpl.ror(args) { return true; }
                        },
                        _ => { },
                    };
                },
                0x20006000 => {
                    /* ..10000. ........ .110.... .0110011 */
                    decode_extract_r(transimpl, &mut args, insn);
                    match (insn >> 30) & 0x3 {
                        0x0 => {
                            /* 0010000. ........ .110.... .0110011 */
                            if transimpl.sh3add(args) { return true; }
                        },
                        _ => { },
                    };
                },
                0x22000000 => {
                    /* ..10001. ........ .000.... .0110011 */
                    decode_extract_k_aes(transimpl, &mut args, insn);
                    if transimpl.aes32esi(args) { return true; }
                },
                0x26000000 => {
                    /* ..10011. ........ .000.... .0110011 */
                    decode_extract_k_aes(transimpl, &mut args, insn);
                    if transimpl.aes32esmi(args) { return true; }
                },
                0x28001000 => {
                    /* ..10100. ........ .001.... .0110011 */
                    decode_extract_r(transimpl, &mut args, insn);
                    match (insn >> 30) & 0x3 {
                        0x0 => {
                            /* 0010100. ........ .001.... .0110011 */
                            if transimpl.bset(args) { return true; }
                        },
                        0x1 => {
                            /* 0110100. ........ .001.... .0110011 */
                            if transimpl.binv(args) { return true; }
                        },
                        _ => { },
                    };
                },
                0x28002000 => {
                    /* ..10100. ........ .010.... .0110011 */
                    decode_extract_r(transimpl, &mut args, insn);
                    match (insn >> 30) & 0x3 {
                        0x0 => {
                            /* 0010100. ........ .010.... .0110011 */
                            if transimpl.xperm4(args) { return true; }
                        },
                        _ => { },
                    };
                },
                0x28004000 => {
                    /* ..10100. ........ .100.... .0110011 */
                    decode_extract_r(transimpl, &mut args, insn);
                    match (insn >> 30) & 0x3 {
                        0x0 => {
                            /* 0010100. ........ .100.... .0110011 */
                            if transimpl.xperm8(args) { return true; }
                        },
                        _ => { },
                    };
                },
                0x2a000000 => {
                    /* ..10101. ........ .000.... .0110011 */
                    decode_extract_k_aes(transimpl, &mut args, insn);
                    if transimpl.aes32dsi(args) { return true; }
                },
                0x2e000000 => {
                    /* ..10111. ........ .000.... .0110011 */
                    decode_extract_k_aes(transimpl, &mut args, insn);
                    if transimpl.aes32dsmi(args) { return true; }
                },
                0x30000000 => {
                    /* ..11000. ........ .000.... .0110011 */
                    decode_extract_k_aes(transimpl, &mut args, insn);
                    if transimpl.sm4ed(args) { return true; }
                },
                0x32000000 => {
                    /* ..11001. ........ .000.... .0110011 */
                    decode_extract_r(transimpl, &mut args, insn);
                    match (insn >> 30) & 0x3 {
                        0x0 => {
                            /* 0011001. ........ .000.... .0110011 */
                            if transimpl.aes64es(args) { return true; }
                        },
                        _ => { },
                    };
                },
                0x34000000 => {
                    /* ..11010. ........ .000.... .0110011 */
                    decode_extract_k_aes(transimpl, &mut args, insn);
                    if transimpl.sm4ks(args) { return true; }
                },
                0x36000000 => {
                    /* ..11011. ........ .000.... .0110011 */
                    decode_extract_r(transimpl, &mut args, insn);
                    match (insn >> 30) & 0x3 {
                        0x0 => {
                            /* 0011011. ........ .000.... .0110011 */
                            if transimpl.aes64esm(args) { return true; }
                        },
                        _ => { },
                    };
                },
                0x3a000000 => {
                    /* ..11101. ........ .000.... .0110011 */
                    decode_extract_r(transimpl, &mut args, insn);
                    match (insn >> 30) & 0x3 {
                        0x0 => {
                            /* 0011101. ........ .000.... .0110011 */
                            if transimpl.aes64ds(args) { return true; }
                        },
                        _ => { },
                    };
                },
                0x3e000000 => {
                    /* ..11111. ........ .000.... .0110011 */
                    decode_extract_r(transimpl, &mut args, insn);
                    match (insn >> 30) & 0x3 {
                        0x0 => {
                            /* 0011111. ........ .000.... .0110011 */
                            if transimpl.aes64dsm(args) { return true; }
                        },
                        0x1 => {
                            /* 0111111. ........ .000.... .0110011 */
                            if transimpl.aes64ks2(args) { return true; }
                        },
                        _ => { },
                    };
                },
                _ => { },
            };
        },
        0x00000037 => {
            /* ........ ........ ........ .0110111 */
            decode_extract_u(transimpl, &mut args, insn);
            if transimpl.lui(args) { return true; }
        },
        0x0000003b => {
            /* ........ ........ ........ .0111011 */
            match insn & 0xfe007000 {
                0x00000000 => {
                    /* 0000000. ........ .000.... .0111011 */
                    decode_extract_r(transimpl, &mut args, insn);
                    if transimpl.addw(args) { return true; }
                },
                0x00001000 => {
                    /* 0000000. ........ .001.... .0111011 */
                    decode_extract_r(transimpl, &mut args, insn);
                    if transimpl.sllw(args) { return true; }
                },
                0x00005000 => {
                    /* 0000000. ........ .101.... .0111011 */
                    decode_extract_r(transimpl, &mut args, insn);
                    if transimpl.srlw(args) { return true; }
                },
                0x02000000 => {
                    /* 0000001. ........ .000.... .0111011 */
                    decode_extract_r(transimpl, &mut args, insn);
                    if transimpl.mulw(args) { return true; }
                },
                0x02004000 => {
                    /* 0000001. ........ .100.... .0111011 */
                    decode_extract_r(transimpl, &mut args, insn);
                    if transimpl.divw(args) { return true; }
                },
                0x02005000 => {
                    /* 0000001. ........ .101.... .0111011 */
                    decode_extract_r(transimpl, &mut args, insn);
                    if transimpl.divuw(args) { return true; }
                },
                0x02006000 => {
                    /* 0000001. ........ .110.... .0111011 */
                    decode_extract_r(transimpl, &mut args, insn);
                    if transimpl.remw(args) { return true; }
                },
                0x02007000 => {
                    /* 0000001. ........ .111.... .0111011 */
                    decode_extract_r(transimpl, &mut args, insn);
                    if transimpl.remuw(args) { return true; }
                },
                0x08000000 => {
                    /* 0000100. ........ .000.... .0111011 */
                    decode_extract_r(transimpl, &mut args, insn);
                    if transimpl.add_uw(args) { return true; }
                },
                0x08004000 => {
                    /* 0000100. ........ .100.... .0111011 */
                    if (insn & 0x01f00000) == 0x00000000 {
                        decode_extract_r2(transimpl, &mut args, insn);
                        if transimpl.zext_h_64(args) { return true; }
                    }
                    decode_extract_r(transimpl, &mut args, insn);
                    if transimpl.packw(args) { return true; }
                },
                0x20002000 => {
                    /* 0010000. ........ .010.... .0111011 */
                    decode_extract_r(transimpl, &mut args, insn);
                    if transimpl.sh1add_uw(args) { return true; }
                },
                0x20004000 => {
                    /* 0010000. ........ .100.... .0111011 */
                    decode_extract_r(transimpl, &mut args, insn);
                    if transimpl.sh2add_uw(args) { return true; }
                },
                0x20006000 => {
                    /* 0010000. ........ .110.... .0111011 */
                    decode_extract_r(transimpl, &mut args, insn);
                    if transimpl.sh3add_uw(args) { return true; }
                },
                0x40000000 => {
                    /* 0100000. ........ .000.... .0111011 */
                    decode_extract_r(transimpl, &mut args, insn);
                    if transimpl.subw(args) { return true; }
                },
                0x40005000 => {
                    /* 0100000. ........ .101.... .0111011 */
                    decode_extract_r(transimpl, &mut args, insn);
                    if transimpl.sraw(args) { return true; }
                },
                0x60001000 => {
                    /* 0110000. ........ .001.... .0111011 */
                    decode_extract_r(transimpl, &mut args, insn);
                    if transimpl.rolw(args) { return true; }
                },
                0x60005000 => {
                    /* 0110000. ........ .101.... .0111011 */
                    decode_extract_r(transimpl, &mut args, insn);
                    if transimpl.rorw(args) { return true; }
                },
                _ => { },
            };
        },
        0x00000043 => {
            /* ........ ........ ........ .1000011 */
            decode_extract_r4_rm(transimpl, &mut args, insn);
            match (insn >> 25) & 0x3 {
                0x0 => {
                    /* .....00. ........ ........ .1000011 */
                    if transimpl.fmadd_s(args) { return true; }
                },
                0x1 => {
                    /* .....01. ........ ........ .1000011 */
                    if transimpl.fmadd_d(args) { return true; }
                },
                0x2 => {
                    /* .....10. ........ ........ .1000011 */
                    if transimpl.fmadd_h(args) { return true; }
                },
                _ => { },
            };
        },
        0x00000047 => {
            /* ........ ........ ........ .1000111 */
            decode_extract_r4_rm(transimpl, &mut args, insn);
            match (insn >> 25) & 0x3 {
                0x0 => {
                    /* .....00. ........ ........ .1000111 */
                    if transimpl.fmsub_s(args) { return true; }
                },
                0x1 => {
                    /* .....01. ........ ........ .1000111 */
                    if transimpl.fmsub_d(args) { return true; }
                },
                0x2 => {
                    /* .....10. ........ ........ .1000111 */
                    if transimpl.fmsub_h(args) { return true; }
                },
                _ => { },
            };
        },
        0x0000004b => {
            /* ........ ........ ........ .1001011 */
            decode_extract_r4_rm(transimpl, &mut args, insn);
            match (insn >> 25) & 0x3 {
                0x0 => {
                    /* .....00. ........ ........ .1001011 */
                    if transimpl.fnmsub_s(args) { return true; }
                },
                0x1 => {
                    /* .....01. ........ ........ .1001011 */
                    if transimpl.fnmsub_d(args) { return true; }
                },
                0x2 => {
                    /* .....10. ........ ........ .1001011 */
                    if transimpl.fnmsub_h(args) { return true; }
                },
                _ => { },
            };
        },
        0x0000004f => {
            /* ........ ........ ........ .1001111 */
            decode_extract_r4_rm(transimpl, &mut args, insn);
            match (insn >> 25) & 0x3 {
                0x0 => {
                    /* .....00. ........ ........ .1001111 */
                    if transimpl.fnmadd_s(args) { return true; }
                },
                0x1 => {
                    /* .....01. ........ ........ .1001111 */
                    if transimpl.fnmadd_d(args) { return true; }
                },
                0x2 => {
                    /* .....10. ........ ........ .1001111 */
                    if transimpl.fnmadd_h(args) { return true; }
                },
                _ => { },
            };
        },
        0x00000053 => {
            /* ........ ........ ........ .1010011 */
            match (insn >> 25) & 0x7f {
                0x0 => {
                    /* 0000000. ........ ........ .1010011 */
                    decode_extract_r_rm(transimpl, &mut args, insn);
                    if transimpl.fadd_s(args) { return true; }
                },
                0x1 => {
                    /* 0000001. ........ ........ .1010011 */
                    decode_extract_r_rm(transimpl, &mut args, insn);
                    if transimpl.fadd_d(args) { return true; }
                },
                0x2 => {
                    /* 0000010. ........ ........ .1010011 */
                    decode_extract_r_rm(transimpl, &mut args, insn);
                    if transimpl.fadd_h(args) { return true; }
                },
                0x4 => {
                    /* 0000100. ........ ........ .1010011 */
                    decode_extract_r_rm(transimpl, &mut args, insn);
                    if transimpl.fsub_s(args) { return true; }
                },
                0x5 => {
                    /* 0000101. ........ ........ .1010011 */
                    decode_extract_r_rm(transimpl, &mut args, insn);
                    if transimpl.fsub_d(args) { return true; }
                },
                0x6 => {
                    /* 0000110. ........ ........ .1010011 */
                    decode_extract_r_rm(transimpl, &mut args, insn);
                    if transimpl.fsub_h(args) { return true; }
                },
                0x8 => {
                    /* 0001000. ........ ........ .1010011 */
                    decode_extract_r_rm(transimpl, &mut args, insn);
                    if transimpl.fmul_s(args) { return true; }
                },
                0x9 => {
                    /* 0001001. ........ ........ .1010011 */
                    decode_extract_r_rm(transimpl, &mut args, insn);
                    if transimpl.fmul_d(args) { return true; }
                },
                0xa => {
                    /* 0001010. ........ ........ .1010011 */
                    decode_extract_r_rm(transimpl, &mut args, insn);
                    if transimpl.fmul_h(args) { return true; }
                },
                0xc => {
                    /* 0001100. ........ ........ .1010011 */
                    decode_extract_r_rm(transimpl, &mut args, insn);
                    if transimpl.fdiv_s(args) { return true; }
                },
                0xd => {
                    /* 0001101. ........ ........ .1010011 */
                    decode_extract_r_rm(transimpl, &mut args, insn);
                    if transimpl.fdiv_d(args) { return true; }
                },
                0xe => {
                    /* 0001110. ........ ........ .1010011 */
                    decode_extract_r_rm(transimpl, &mut args, insn);
                    if transimpl.fdiv_h(args) { return true; }
                },
                0x10 => {
                    /* 0010000. ........ ........ .1010011 */
                    decode_extract_r(transimpl, &mut args, insn);
                    match (insn >> 12) & 0x7 {
                        0x0 => {
                            /* 0010000. ........ .000.... .1010011 */
                            if transimpl.fsgnj_s(args) { return true; }
                        },
                        0x1 => {
                            /* 0010000. ........ .001.... .1010011 */
                            if transimpl.fsgnjn_s(args) { return true; }
                        },
                        0x2 => {
                            /* 0010000. ........ .010.... .1010011 */
                            if transimpl.fsgnjx_s(args) { return true; }
                        },
                        _ => { },
                    };
                },
                0x11 => {
                    /* 0010001. ........ ........ .1010011 */
                    decode_extract_r(transimpl, &mut args, insn);
                    match (insn >> 12) & 0x7 {
                        0x0 => {
                            /* 0010001. ........ .000.... .1010011 */
                            if transimpl.fsgnj_d(args) { return true; }
                        },
                        0x1 => {
                            /* 0010001. ........ .001.... .1010011 */
                            if transimpl.fsgnjn_d(args) { return true; }
                        },
                        0x2 => {
                            /* 0010001. ........ .010.... .1010011 */
                            if transimpl.fsgnjx_d(args) { return true; }
                        },
                        _ => { },
                    };
                },
                0x12 => {
                    /* 0010010. ........ ........ .1010011 */
                    decode_extract_r(transimpl, &mut args, insn);
                    match (insn >> 12) & 0x7 {
                        0x0 => {
                            /* 0010010. ........ .000.... .1010011 */
                            if transimpl.fsgnj_h(args) { return true; }
                        },
                        0x1 => {
                            /* 0010010. ........ .001.... .1010011 */
                            if transimpl.fsgnjn_h(args) { return true; }
                        },
                        0x2 => {
                            /* 0010010. ........ .010.... .1010011 */
                            if transimpl.fsgnjx_h(args) { return true; }
                        },
                        _ => { },
                    };
                },
                0x14 => {
                    /* 0010100. ........ ........ .1010011 */
                    decode_extract_r(transimpl, &mut args, insn);
                    match (insn >> 12) & 0x7 {
                        0x0 => {
                            /* 0010100. ........ .000.... .1010011 */
                            if transimpl.fmin_s(args) { return true; }
                        },
                        0x1 => {
                            /* 0010100. ........ .001.... .1010011 */
                            if transimpl.fmax_s(args) { return true; }
                        },
                        _ => { },
                    };
                },
                0x15 => {
                    /* 0010101. ........ ........ .1010011 */
                    decode_extract_r(transimpl, &mut args, insn);
                    match (insn >> 12) & 0x7 {
                        0x0 => {
                            /* 0010101. ........ .000.... .1010011 */
                            if transimpl.fmin_d(args) { return true; }
                        },
                        0x1 => {
                            /* 0010101. ........ .001.... .1010011 */
                            if transimpl.fmax_d(args) { return true; }
                        },
                        _ => { },
                    };
                },
                0x16 => {
                    /* 0010110. ........ ........ .1010011 */
                    decode_extract_r(transimpl, &mut args, insn);
                    match (insn >> 12) & 0x7 {
                        0x0 => {
                            /* 0010110. ........ .000.... .1010011 */
                            if transimpl.fmin_h(args) { return true; }
                        },
                        0x1 => {
                            /* 0010110. ........ .001.... .1010011 */
                            if transimpl.fmax_h(args) { return true; }
                        },
                        _ => { },
                    };
                },
                0x20 => {
                    /* 0100000. ........ ........ .1010011 */
                    decode_extract_r2_rm(transimpl, &mut args, insn);
                    match (insn >> 20) & 0x1f {
                        0x1 => {
                            /* 01000000 0001.... ........ .1010011 */
                            if transimpl.fcvt_s_d(args) { return true; }
                        },
                        0x2 => {
                            /* 01000000 0010.... ........ .1010011 */
                            if transimpl.fcvt_s_h(args) { return true; }
                        },
                        _ => { },
                    };
                },
                0x21 => {
                    /* 0100001. ........ ........ .1010011 */
                    decode_extract_r2_rm(transimpl, &mut args, insn);
                    match (insn >> 20) & 0x1f {
                        0x0 => {
                            /* 01000010 0000.... ........ .1010011 */
                            if transimpl.fcvt_d_s(args) { return true; }
                        },
                        0x2 => {
                            /* 01000010 0010.... ........ .1010011 */
                            if transimpl.fcvt_d_h(args) { return true; }
                        },
                        _ => { },
                    };
                },
                0x22 => {
                    /* 0100010. ........ ........ .1010011 */
                    decode_extract_r2_rm(transimpl, &mut args, insn);
                    match (insn >> 20) & 0x1f {
                        0x0 => {
                            /* 01000100 0000.... ........ .1010011 */
                            if transimpl.fcvt_h_s(args) { return true; }
                        },
                        0x1 => {
                            /* 01000100 0001.... ........ .1010011 */
                            if transimpl.fcvt_h_d(args) { return true; }
                        },
                        _ => { },
                    };
                },
                0x2c => {
                    /* 0101100. ........ ........ .1010011 */
                    decode_extract_r2_rm(transimpl, &mut args, insn);
                    match (insn >> 20) & 0x1f {
                        0x0 => {
                            /* 01011000 0000.... ........ .1010011 */
                            if transimpl.fsqrt_s(args) { return true; }
                        },
                        _ => { },
                    };
                },
                0x2d => {
                    /* 0101101. ........ ........ .1010011 */
                    decode_extract_r2_rm(transimpl, &mut args, insn);
                    match (insn >> 20) & 0x1f {
                        0x0 => {
                            /* 01011010 0000.... ........ .1010011 */
                            if transimpl.fsqrt_d(args) { return true; }
                        },
                        _ => { },
                    };
                },
                0x2e => {
                    /* 0101110. ........ ........ .1010011 */
                    decode_extract_r2_rm(transimpl, &mut args, insn);
                    match (insn >> 20) & 0x1f {
                        0x0 => {
                            /* 01011100 0000.... ........ .1010011 */
                            if transimpl.fsqrt_h(args) { return true; }
                        },
                        _ => { },
                    };
                },
                0x50 => {
                    /* 1010000. ........ ........ .1010011 */
                    decode_extract_r(transimpl, &mut args, insn);
                    match (insn >> 12) & 0x7 {
                        0x0 => {
                            /* 1010000. ........ .000.... .1010011 */
                            if transimpl.fle_s(args) { return true; }
                        },
                        0x1 => {
                            /* 1010000. ........ .001.... .1010011 */
                            if transimpl.flt_s(args) { return true; }
                        },
                        0x2 => {
                            /* 1010000. ........ .010.... .1010011 */
                            if transimpl.feq_s(args) { return true; }
                        },
                        _ => { },
                    };
                },
                0x51 => {
                    /* 1010001. ........ ........ .1010011 */
                    decode_extract_r(transimpl, &mut args, insn);
                    match (insn >> 12) & 0x7 {
                        0x0 => {
                            /* 1010001. ........ .000.... .1010011 */
                            if transimpl.fle_d(args) { return true; }
                        },
                        0x1 => {
                            /* 1010001. ........ .001.... .1010011 */
                            if transimpl.flt_d(args) { return true; }
                        },
                        0x2 => {
                            /* 1010001. ........ .010.... .1010011 */
                            if transimpl.feq_d(args) { return true; }
                        },
                        _ => { },
                    };
                },
                0x52 => {
                    /* 1010010. ........ ........ .1010011 */
                    decode_extract_r(transimpl, &mut args, insn);
                    match (insn >> 12) & 0x7 {
                        0x0 => {
                            /* 1010010. ........ .000.... .1010011 */
                            if transimpl.fle_h(args) { return true; }
                        },
                        0x1 => {
                            /* 1010010. ........ .001.... .1010011 */
                            if transimpl.flt_h(args) { return true; }
                        },
                        0x2 => {
                            /* 1010010. ........ .010.... .1010011 */
                            if transimpl.feq_h(args) { return true; }
                        },
                        _ => { },
                    };
                },
                0x60 => {
                    /* 1100000. ........ ........ .1010011 */
                    decode_extract_r2_rm(transimpl, &mut args, insn);
                    match (insn >> 20) & 0x1f {
                        0x0 => {
                            /* 11000000 0000.... ........ .1010011 */
                            if transimpl.fcvt_w_s(args) { return true; }
                        },
                        0x1 => {
                            /* 11000000 0001.... ........ .1010011 */
                            if transimpl.fcvt_wu_s(args) { return true; }
                        },
                        0x2 => {
                            /* 11000000 0010.... ........ .1010011 */
                            if transimpl.fcvt_l_s(args) { return true; }
                        },
                        0x3 => {
                            /* 11000000 0011.... ........ .1010011 */
                            if transimpl.fcvt_lu_s(args) { return true; }
                        },
                        _ => { },
                    };
                },
                0x61 => {
                    /* 1100001. ........ ........ .1010011 */
                    decode_extract_r2_rm(transimpl, &mut args, insn);
                    match (insn >> 20) & 0x1f {
                        0x0 => {
                            /* 11000010 0000.... ........ .1010011 */
                            if transimpl.fcvt_w_d(args) { return true; }
                        },
                        0x1 => {
                            /* 11000010 0001.... ........ .1010011 */
                            if transimpl.fcvt_wu_d(args) { return true; }
                        },
                        0x2 => {
                            /* 11000010 0010.... ........ .1010011 */
                            if transimpl.fcvt_l_d(args) { return true; }
                        },
                        0x3 => {
                            /* 11000010 0011.... ........ .1010011 */
                            if transimpl.fcvt_lu_d(args) { return true; }
                        },
                        _ => { },
                    };
                },
                0x62 => {
                    /* 1100010. ........ ........ .1010011 */
                    decode_extract_r2_rm(transimpl, &mut args, insn);
                    match (insn >> 20) & 0x1f {
                        0x0 => {
                            /* 11000100 0000.... ........ .1010011 */
                            if transimpl.fcvt_w_h(args) { return true; }
                        },
                        0x1 => {
                            /* 11000100 0001.... ........ .1010011 */
                            if transimpl.fcvt_wu_h(args) { return true; }
                        },
                        0x2 => {
                            /* 11000100 0010.... ........ .1010011 */
                            if transimpl.fcvt_l_h(args) { return true; }
                        },
                        0x3 => {
                            /* 11000100 0011.... ........ .1010011 */
                            if transimpl.fcvt_lu_h(args) { return true; }
                        },
                        _ => { },
                    };
                },
                0x68 => {
                    /* 1101000. ........ ........ .1010011 */
                    decode_extract_r2_rm(transimpl, &mut args, insn);
                    match (insn >> 20) & 0x1f {
                        0x0 => {
                            /* 11010000 0000.... ........ .1010011 */
                            if transimpl.fcvt_s_w(args) { return true; }
                        },
                        0x1 => {
                            /* 11010000 0001.... ........ .1010011 */
                            if transimpl.fcvt_s_wu(args) { return true; }
                        },
                        0x2 => {
                            /* 11010000 0010.... ........ .1010011 */
                            if transimpl.fcvt_s_l(args) { return true; }
                        },
                        0x3 => {
                            /* 11010000 0011.... ........ .1010011 */
                            if transimpl.fcvt_s_lu(args) { return true; }
                        },
                        _ => { },
                    };
                },
                0x69 => {
                    /* 1101001. ........ ........ .1010011 */
                    decode_extract_r2_rm(transimpl, &mut args, insn);
                    match (insn >> 20) & 0x1f {
                        0x0 => {
                            /* 11010010 0000.... ........ .1010011 */
                            if transimpl.fcvt_d_w(args) { return true; }
                        },
                        0x1 => {
                            /* 11010010 0001.... ........ .1010011 */
                            if transimpl.fcvt_d_wu(args) { return true; }
                        },
                        0x2 => {
                            /* 11010010 0010.... ........ .1010011 */
                            if transimpl.fcvt_d_l(args) { return true; }
                        },
                        0x3 => {
                            /* 11010010 0011.... ........ .1010011 */
                            if transimpl.fcvt_d_lu(args) { return true; }
                        },
                        _ => { },
                    };
                },
                0x6a => {
                    /* 1101010. ........ ........ .1010011 */
                    decode_extract_r2_rm(transimpl, &mut args, insn);
                    match (insn >> 20) & 0x1f {
                        0x0 => {
                            /* 11010100 0000.... ........ .1010011 */
                            if transimpl.fcvt_h_w(args) { return true; }
                        },
                        0x1 => {
                            /* 11010100 0001.... ........ .1010011 */
                            if transimpl.fcvt_h_wu(args) { return true; }
                        },
                        0x2 => {
                            /* 11010100 0010.... ........ .1010011 */
                            if transimpl.fcvt_h_l(args) { return true; }
                        },
                        0x3 => {
                            /* 11010100 0011.... ........ .1010011 */
                            if transimpl.fcvt_h_lu(args) { return true; }
                        },
                        _ => { },
                    };
                },
                0x70 => {
                    /* 1110000. ........ ........ .1010011 */
                    decode_extract_r2(transimpl, &mut args, insn);
                    match insn & 0x01f07000 {
                        0x00000000 => {
                            /* 11100000 0000.... .000.... .1010011 */
                            if transimpl.fmv_x_w(args) { return true; }
                        },
                        0x00001000 => {
                            /* 11100000 0000.... .001.... .1010011 */
                            if transimpl.fclass_s(args) { return true; }
                        },
                        _ => { },
                    };
                },
                0x71 => {
                    /* 1110001. ........ ........ .1010011 */
                    decode_extract_r2(transimpl, &mut args, insn);
                    match insn & 0x01f07000 {
                        0x00000000 => {
                            /* 11100010 0000.... .000.... .1010011 */
                            if transimpl.fmv_x_d(args) { return true; }
                        },
                        0x00001000 => {
                            /* 11100010 0000.... .001.... .1010011 */
                            if transimpl.fclass_d(args) { return true; }
                        },
                        _ => { },
                    };
                },
                0x72 => {
                    /* 1110010. ........ ........ .1010011 */
                    decode_extract_r2(transimpl, &mut args, insn);
                    match insn & 0x01f07000 {
                        0x00000000 => {
                            /* 11100100 0000.... .000.... .1010011 */
                            if transimpl.fmv_x_h(args) { return true; }
                        },
                        0x00001000 => {
                            /* 11100100 0000.... .001.... .1010011 */
                            if transimpl.fclass_h(args) { return true; }
                        },
                        _ => { },
                    };
                },
                0x78 => {
                    /* 1111000. ........ ........ .1010011 */
                    decode_extract_r2(transimpl, &mut args, insn);
                    match insn & 0x01f07000 {
                        0x00000000 => {
                            /* 11110000 0000.... .000.... .1010011 */
                            if transimpl.fmv_w_x(args) { return true; }
                        },
                        _ => { },
                    };
                },
                0x79 => {
                    /* 1111001. ........ ........ .1010011 */
                    decode_extract_r2(transimpl, &mut args, insn);
                    match insn & 0x01f07000 {
                        0x00000000 => {
                            /* 11110010 0000.... .000.... .1010011 */
                            if transimpl.fmv_d_x(args) { return true; }
                        },
                        _ => { },
                    };
                },
                0x7a => {
                    /* 1111010. ........ ........ .1010011 */
                    decode_extract_r2(transimpl, &mut args, insn);
                    match insn & 0x01f07000 {
                        0x00000000 => {
                            /* 11110100 0000.... .000.... .1010011 */
                            if transimpl.fmv_h_x(args) { return true; }
                        },
                        _ => { },
                    };
                },
                _ => { },
            };
        },
        0x00000057 => {
            /* ........ ........ ........ .1010111 */
            match insn & 0x80007000 {
                0x00000000 => {
                    /* 0....... ........ .000.... .1010111 */
                    match (insn >> 26) & 0x1f {
                        0x0 => {
                            /* 000000.. ........ .000.... .1010111 */
                            decode_extract_r_vm(transimpl, &mut args, insn);
                            if transimpl.vadd_vv(args) { return true; }
                        },
                        0x2 => {
                            /* 000010.. ........ .000.... .1010111 */
                            decode_extract_r_vm(transimpl, &mut args, insn);
                            if transimpl.vsub_vv(args) { return true; }
                        },
                        0x4 => {
                            /* 000100.. ........ .000.... .1010111 */
                            decode_extract_r_vm(transimpl, &mut args, insn);
                            if transimpl.vminu_vv(args) { return true; }
                        },
                        0x5 => {
                            /* 000101.. ........ .000.... .1010111 */
                            decode_extract_r_vm(transimpl, &mut args, insn);
                            if transimpl.vmin_vv(args) { return true; }
                        },
                        0x6 => {
                            /* 000110.. ........ .000.... .1010111 */
                            decode_extract_r_vm(transimpl, &mut args, insn);
                            if transimpl.vmaxu_vv(args) { return true; }
                        },
                        0x7 => {
                            /* 000111.. ........ .000.... .1010111 */
                            decode_extract_r_vm(transimpl, &mut args, insn);
                            if transimpl.vmax_vv(args) { return true; }
                        },
                        0x9 => {
                            /* 001001.. ........ .000.... .1010111 */
                            decode_extract_r_vm(transimpl, &mut args, insn);
                            if transimpl.vand_vv(args) { return true; }
                        },
                        0xa => {
                            /* 001010.. ........ .000.... .1010111 */
                            decode_extract_r_vm(transimpl, &mut args, insn);
                            if transimpl.vor_vv(args) { return true; }
                        },
                        0xb => {
                            /* 001011.. ........ .000.... .1010111 */
                            decode_extract_r_vm(transimpl, &mut args, insn);
                            if transimpl.vxor_vv(args) { return true; }
                        },
                        0xc => {
                            /* 001100.. ........ .000.... .1010111 */
                            decode_extract_r_vm(transimpl, &mut args, insn);
                            if transimpl.vrgather_vv(args) { return true; }
                        },
                        0xe => {
                            /* 001110.. ........ .000.... .1010111 */
                            decode_extract_r_vm(transimpl, &mut args, insn);
                            if transimpl.vrgatherei16_vv(args) { return true; }
                        },
                        0x10 => {
                            /* 010000.. ........ .000.... .1010111 */
                            decode_extract_r_vm_1(transimpl, &mut args, insn);
                            match (insn >> 25) & 0x1 {
                                0x0 => {
                                    /* 0100000. ........ .000.... .1010111 */
                                    if transimpl.vadc_vvm(args) { return true; }
                                },
                                _ => { },
                            };
                        },
                        0x11 => {
                            /* 010001.. ........ .000.... .1010111 */
                            decode_extract_r_vm(transimpl, &mut args, insn);
                            if transimpl.vmadc_vvm(args) { return true; }
                        },
                        0x12 => {
                            /* 010010.. ........ .000.... .1010111 */
                            decode_extract_r_vm_1(transimpl, &mut args, insn);
                            match (insn >> 25) & 0x1 {
                                0x0 => {
                                    /* 0100100. ........ .000.... .1010111 */
                                    if transimpl.vsbc_vvm(args) { return true; }
                                },
                                _ => { },
                            };
                        },
                        0x13 => {
                            /* 010011.. ........ .000.... .1010111 */
                            decode_extract_r_vm(transimpl, &mut args, insn);
                            if transimpl.vmsbc_vvm(args) { return true; }
                        },
                        0x17 => {
                            /* 010111.. ........ .000.... .1010111 */
                            match (insn >> 25) & 0x1 {
                                0x0 => {
                                    /* 0101110. ........ .000.... .1010111 */
                                    decode_extract_r_vm_0(transimpl, &mut args, insn);
                                    if transimpl.vmerge_vvm(args) { return true; }
                                },
                                0x1 => {
                                    /* 0101111. ........ .000.... .1010111 */
                                    decode_extract_r2(transimpl, &mut args, insn);
                                    match (insn >> 20) & 0x1f {
                                        0x0 => {
                                            /* 01011110 0000.... .000.... .1010111 */
                                            if transimpl.vmv_v_v(args) { return true; }
                                        },
                                        _ => { },
                                    };
                                },
                                _ => { },
                            };
                        },
                        0x18 => {
                            /* 011000.. ........ .000.... .1010111 */
                            decode_extract_r_vm(transimpl, &mut args, insn);
                            if transimpl.vmseq_vv(args) { return true; }
                        },
                        0x19 => {
                            /* 011001.. ........ .000.... .1010111 */
                            decode_extract_r_vm(transimpl, &mut args, insn);
                            if transimpl.vmsne_vv(args) { return true; }
                        },
                        0x1a => {
                            /* 011010.. ........ .000.... .1010111 */
                            decode_extract_r_vm(transimpl, &mut args, insn);
                            if transimpl.vmsltu_vv(args) { return true; }
                        },
                        0x1b => {
                            /* 011011.. ........ .000.... .1010111 */
                            decode_extract_r_vm(transimpl, &mut args, insn);
                            if transimpl.vmslt_vv(args) { return true; }
                        },
                        0x1c => {
                            /* 011100.. ........ .000.... .1010111 */
                            decode_extract_r_vm(transimpl, &mut args, insn);
                            if transimpl.vmsleu_vv(args) { return true; }
                        },
                        0x1d => {
                            /* 011101.. ........ .000.... .1010111 */
                            decode_extract_r_vm(transimpl, &mut args, insn);
                            if transimpl.vmsle_vv(args) { return true; }
                        },
                        _ => { },
                    };
                },
                0x00001000 => {
                    /* 0....... ........ .001.... .1010111 */
                    match (insn >> 26) & 0x1f {
                        0x0 => {
                            /* 000000.. ........ .001.... .1010111 */
                            decode_extract_r_vm(transimpl, &mut args, insn);
                            if transimpl.vfadd_vv(args) { return true; }
                        },
                        0x1 => {
                            /* 000001.. ........ .001.... .1010111 */
                            decode_extract_r_vm(transimpl, &mut args, insn);
                            if transimpl.vfredusum_vs(args) { return true; }
                        },
                        0x2 => {
                            /* 000010.. ........ .001.... .1010111 */
                            decode_extract_r_vm(transimpl, &mut args, insn);
                            if transimpl.vfsub_vv(args) { return true; }
                        },
                        0x3 => {
                            /* 000011.. ........ .001.... .1010111 */
                            decode_extract_r_vm(transimpl, &mut args, insn);
                            if transimpl.vfredosum_vs(args) { return true; }
                        },
                        0x4 => {
                            /* 000100.. ........ .001.... .1010111 */
                            decode_extract_r_vm(transimpl, &mut args, insn);
                            if transimpl.vfmin_vv(args) { return true; }
                        },
                        0x5 => {
                            /* 000101.. ........ .001.... .1010111 */
                            decode_extract_r_vm(transimpl, &mut args, insn);
                            if transimpl.vfredmin_vs(args) { return true; }
                        },
                        0x6 => {
                            /* 000110.. ........ .001.... .1010111 */
                            decode_extract_r_vm(transimpl, &mut args, insn);
                            if transimpl.vfmax_vv(args) { return true; }
                        },
                        0x7 => {
                            /* 000111.. ........ .001.... .1010111 */
                            decode_extract_r_vm(transimpl, &mut args, insn);
                            if transimpl.vfredmax_vs(args) { return true; }
                        },
                        0x8 => {
                            /* 001000.. ........ .001.... .1010111 */
                            decode_extract_r_vm(transimpl, &mut args, insn);
                            if transimpl.vfsgnj_vv(args) { return true; }
                        },
                        0x9 => {
                            /* 001001.. ........ .001.... .1010111 */
                            decode_extract_r_vm(transimpl, &mut args, insn);
                            if transimpl.vfsgnjn_vv(args) { return true; }
                        },
                        0xa => {
                            /* 001010.. ........ .001.... .1010111 */
                            decode_extract_r_vm(transimpl, &mut args, insn);
                            if transimpl.vfsgnjx_vv(args) { return true; }
                        },
                        0x10 => {
                            /* 010000.. ........ .001.... .1010111 */
                            decode_extract_r2rd(transimpl, &mut args, insn);
                            match insn & 0x020f8000 {
                                0x02000000 => {
                                    /* 0100001. ....0000 0001.... .1010111 */
                                    if transimpl.vfmv_f_s(args) { return true; }
                                },
                                _ => { },
                            };
                        },
                        0x12 => {
                            /* 010010.. ........ .001.... .1010111 */
                            decode_extract_r2_vm(transimpl, &mut args, insn);
                            match (insn >> 15) & 0x1f {
                                0x0 => {
                                    /* 010010.. ....0000 0001.... .1010111 */
                                    if transimpl.vfcvt_xu_f_v(args) { return true; }
                                },
                                0x1 => {
                                    /* 010010.. ....0000 1001.... .1010111 */
                                    if transimpl.vfcvt_x_f_v(args) { return true; }
                                },
                                0x2 => {
                                    /* 010010.. ....0001 0001.... .1010111 */
                                    if transimpl.vfcvt_f_xu_v(args) { return true; }
                                },
                                0x3 => {
                                    /* 010010.. ....0001 1001.... .1010111 */
                                    if transimpl.vfcvt_f_x_v(args) { return true; }
                                },
                                0x6 => {
                                    /* 010010.. ....0011 0001.... .1010111 */
                                    if transimpl.vfcvt_rtz_xu_f_v(args) { return true; }
                                },
                                0x7 => {
                                    /* 010010.. ....0011 1001.... .1010111 */
                                    if transimpl.vfcvt_rtz_x_f_v(args) { return true; }
                                },
                                0x8 => {
                                    /* 010010.. ....0100 0001.... .1010111 */
                                    if transimpl.vfwcvt_xu_f_v(args) { return true; }
                                },
                                0x9 => {
                                    /* 010010.. ....0100 1001.... .1010111 */
                                    if transimpl.vfwcvt_x_f_v(args) { return true; }
                                },
                                0xa => {
                                    /* 010010.. ....0101 0001.... .1010111 */
                                    if transimpl.vfwcvt_f_xu_v(args) { return true; }
                                },
                                0xb => {
                                    /* 010010.. ....0101 1001.... .1010111 */
                                    if transimpl.vfwcvt_f_x_v(args) { return true; }
                                },
                                0xc => {
                                    /* 010010.. ....0110 0001.... .1010111 */
                                    if transimpl.vfwcvt_f_f_v(args) { return true; }
                                },
                                0xe => {
                                    /* 010010.. ....0111 0001.... .1010111 */
                                    if transimpl.vfwcvt_rtz_xu_f_v(args) { return true; }
                                },
                                0xf => {
                                    /* 010010.. ....0111 1001.... .1010111 */
                                    if transimpl.vfwcvt_rtz_x_f_v(args) { return true; }
                                },
                                0x10 => {
                                    /* 010010.. ....1000 0001.... .1010111 */
                                    if transimpl.vfncvt_xu_f_w(args) { return true; }
                                },
                                0x11 => {
                                    /* 010010.. ....1000 1001.... .1010111 */
                                    if transimpl.vfncvt_x_f_w(args) { return true; }
                                },
                                0x12 => {
                                    /* 010010.. ....1001 0001.... .1010111 */
                                    if transimpl.vfncvt_f_xu_w(args) { return true; }
                                },
                                0x13 => {
                                    /* 010010.. ....1001 1001.... .1010111 */
                                    if transimpl.vfncvt_f_x_w(args) { return true; }
                                },
                                0x14 => {
                                    /* 010010.. ....1010 0001.... .1010111 */
                                    if transimpl.vfncvt_f_f_w(args) { return true; }
                                },
                                0x15 => {
                                    /* 010010.. ....1010 1001.... .1010111 */
                                    if transimpl.vfncvt_rod_f_f_w(args) { return true; }
                                },
                                0x16 => {
                                    /* 010010.. ....1011 0001.... .1010111 */
                                    if transimpl.vfncvt_rtz_xu_f_w(args) { return true; }
                                },
                                0x17 => {
                                    /* 010010.. ....1011 1001.... .1010111 */
                                    if transimpl.vfncvt_rtz_x_f_w(args) { return true; }
                                },
                                _ => { },
                            };
                        },
                        0x13 => {
                            /* 010011.. ........ .001.... .1010111 */
                            decode_extract_r2_vm(transimpl, &mut args, insn);
                            match (insn >> 15) & 0x1f {
                                0x0 => {
                                    /* 010011.. ....0000 0001.... .1010111 */
                                    if transimpl.vfsqrt_v(args) { return true; }
                                },
                                0x4 => {
                                    /* 010011.. ....0010 0001.... .1010111 */
                                    if transimpl.vfrsqrt7_v(args) { return true; }
                                },
                                0x5 => {
                                    /* 010011.. ....0010 1001.... .1010111 */
                                    if transimpl.vfrec7_v(args) { return true; }
                                },
                                0x10 => {
                                    /* 010011.. ....1000 0001.... .1010111 */
                                    if transimpl.vfclass_v(args) { return true; }
                                },
                                _ => { },
                            };
                        },
                        0x18 => {
                            /* 011000.. ........ .001.... .1010111 */
                            decode_extract_r_vm(transimpl, &mut args, insn);
                            if transimpl.vmfeq_vv(args) { return true; }
                        },
                        0x19 => {
                            /* 011001.. ........ .001.... .1010111 */
                            decode_extract_r_vm(transimpl, &mut args, insn);
                            if transimpl.vmfle_vv(args) { return true; }
                        },
                        0x1b => {
                            /* 011011.. ........ .001.... .1010111 */
                            decode_extract_r_vm(transimpl, &mut args, insn);
                            if transimpl.vmflt_vv(args) { return true; }
                        },
                        0x1c => {
                            /* 011100.. ........ .001.... .1010111 */
                            decode_extract_r_vm(transimpl, &mut args, insn);
                            if transimpl.vmfne_vv(args) { return true; }
                        },
                        _ => { },
                    };
                },
                0x00002000 => {
                    /* 0....... ........ .010.... .1010111 */
                    match (insn >> 26) & 0x1f {
                        0x0 => {
                            /* 000000.. ........ .010.... .1010111 */
                            decode_extract_r_vm(transimpl, &mut args, insn);
                            if transimpl.vredsum_vs(args) { return true; }
                        },
                        0x1 => {
                            /* 000001.. ........ .010.... .1010111 */
                            decode_extract_r_vm(transimpl, &mut args, insn);
                            if transimpl.vredand_vs(args) { return true; }
                        },
                        0x2 => {
                            /* 000010.. ........ .010.... .1010111 */
                            decode_extract_r_vm(transimpl, &mut args, insn);
                            if transimpl.vredor_vs(args) { return true; }
                        },
                        0x3 => {
                            /* 000011.. ........ .010.... .1010111 */
                            decode_extract_r_vm(transimpl, &mut args, insn);
                            if transimpl.vredxor_vs(args) { return true; }
                        },
                        0x4 => {
                            /* 000100.. ........ .010.... .1010111 */
                            decode_extract_r_vm(transimpl, &mut args, insn);
                            if transimpl.vredminu_vs(args) { return true; }
                        },
                        0x5 => {
                            /* 000101.. ........ .010.... .1010111 */
                            decode_extract_r_vm(transimpl, &mut args, insn);
                            if transimpl.vredmin_vs(args) { return true; }
                        },
                        0x6 => {
                            /* 000110.. ........ .010.... .1010111 */
                            decode_extract_r_vm(transimpl, &mut args, insn);
                            if transimpl.vredmaxu_vs(args) { return true; }
                        },
                        0x7 => {
                            /* 000111.. ........ .010.... .1010111 */
                            decode_extract_r_vm(transimpl, &mut args, insn);
                            if transimpl.vredmax_vs(args) { return true; }
                        },
                        0x8 => {
                            /* 001000.. ........ .010.... .1010111 */
                            decode_extract_r_vm(transimpl, &mut args, insn);
                            if transimpl.vaaddu_vv(args) { return true; }
                        },
                        0x9 => {
                            /* 001001.. ........ .010.... .1010111 */
                            decode_extract_r_vm(transimpl, &mut args, insn);
                            if transimpl.vaadd_vv(args) { return true; }
                        },
                        0xa => {
                            /* 001010.. ........ .010.... .1010111 */
                            decode_extract_r_vm(transimpl, &mut args, insn);
                            if transimpl.vasubu_vv(args) { return true; }
                        },
                        0xb => {
                            /* 001011.. ........ .010.... .1010111 */
                            decode_extract_r_vm(transimpl, &mut args, insn);
                            if transimpl.vasub_vv(args) { return true; }
                        },
                        0x10 => {
                            /* 010000.. ........ .010.... .1010111 */
                            match (insn >> 15) & 0x1f {
                                0x0 => {
                                    /* 010000.. ....0000 0010.... .1010111 */
                                    decode_extract_r2rd(transimpl, &mut args, insn);
                                    match (insn >> 25) & 0x1 {
                                        0x1 => {
                                            /* 0100001. ....0000 0010.... .1010111 */
                                            if transimpl.vmv_x_s(args) { return true; }
                                        },
                                        _ => { },
                                    };
                                },
                                0x10 => {
                                    /* 010000.. ....1000 0010.... .1010111 */
                                    decode_extract_r2_vm(transimpl, &mut args, insn);
                                    if transimpl.vcpop_m(args) { return true; }
                                },
                                0x11 => {
                                    /* 010000.. ....1000 1010.... .1010111 */
                                    decode_extract_r2_vm(transimpl, &mut args, insn);
                                    if transimpl.vfirst_m(args) { return true; }
                                },
                                _ => { },
                            };
                        },
                        0x12 => {
                            /* 010010.. ........ .010.... .1010111 */
                            decode_extract_r2_vm(transimpl, &mut args, insn);
                            match (insn >> 15) & 0x1f {
                                0x2 => {
                                    /* 010010.. ....0001 0010.... .1010111 */
                                    if transimpl.vzext_vf8(args) { return true; }
                                },
                                0x3 => {
                                    /* 010010.. ....0001 1010.... .1010111 */
                                    if transimpl.vsext_vf8(args) { return true; }
                                },
                                0x4 => {
                                    /* 010010.. ....0010 0010.... .1010111 */
                                    if transimpl.vzext_vf4(args) { return true; }
                                },
                                0x5 => {
                                    /* 010010.. ....0010 1010.... .1010111 */
                                    if transimpl.vsext_vf4(args) { return true; }
                                },
                                0x6 => {
                                    /* 010010.. ....0011 0010.... .1010111 */
                                    if transimpl.vzext_vf2(args) { return true; }
                                },
                                0x7 => {
                                    /* 010010.. ....0011 1010.... .1010111 */
                                    if transimpl.vsext_vf2(args) { return true; }
                                },
                                _ => { },
                            };
                        },
                        0x14 => {
                            /* 010100.. ........ .010.... .1010111 */
                            match (insn >> 15) & 0x1f {
                                0x1 => {
                                    /* 010100.. ....0000 1010.... .1010111 */
                                    decode_extract_r2_vm(transimpl, &mut args, insn);
                                    if transimpl.vmsbf_m(args) { return true; }
                                },
                                0x2 => {
                                    /* 010100.. ....0001 0010.... .1010111 */
                                    decode_extract_r2_vm(transimpl, &mut args, insn);
                                    if transimpl.vmsof_m(args) { return true; }
                                },
                                0x3 => {
                                    /* 010100.. ....0001 1010.... .1010111 */
                                    decode_extract_r2_vm(transimpl, &mut args, insn);
                                    if transimpl.vmsif_m(args) { return true; }
                                },
                                0x10 => {
                                    /* 010100.. ....1000 0010.... .1010111 */
                                    decode_extract_r2_vm(transimpl, &mut args, insn);
                                    if transimpl.viota_m(args) { return true; }
                                },
                                0x11 => {
                                    /* 010100.. ....1000 1010.... .1010111 */
                                    decode_extract_r1_vm(transimpl, &mut args, insn);
                                    match (insn >> 20) & 0x1f {
                                        0x0 => {
                                            /* 010100.0 00001000 1010.... .1010111 */
                                            if transimpl.vid_v(args) { return true; }
                                        },
                                        _ => { },
                                    };
                                },
                                _ => { },
                            };
                        },
                        0x17 => {
                            /* 010111.. ........ .010.... .1010111 */
                            decode_extract_r(transimpl, &mut args, insn);
                            if transimpl.vcompress_vm(args) { return true; }
                        },
                        0x18 => {
                            /* 011000.. ........ .010.... .1010111 */
                            decode_extract_r(transimpl, &mut args, insn);
                            if transimpl.vmandn_mm(args) { return true; }
                        },
                        0x19 => {
                            /* 011001.. ........ .010.... .1010111 */
                            decode_extract_r(transimpl, &mut args, insn);
                            if transimpl.vmand_mm(args) { return true; }
                        },
                        0x1a => {
                            /* 011010.. ........ .010.... .1010111 */
                            decode_extract_r(transimpl, &mut args, insn);
                            if transimpl.vmor_mm(args) { return true; }
                        },
                        0x1b => {
                            /* 011011.. ........ .010.... .1010111 */
                            decode_extract_r(transimpl, &mut args, insn);
                            if transimpl.vmxor_mm(args) { return true; }
                        },
                        0x1c => {
                            /* 011100.. ........ .010.... .1010111 */
                            decode_extract_r(transimpl, &mut args, insn);
                            if transimpl.vmorn_mm(args) { return true; }
                        },
                        0x1d => {
                            /* 011101.. ........ .010.... .1010111 */
                            decode_extract_r(transimpl, &mut args, insn);
                            if transimpl.vmnand_mm(args) { return true; }
                        },
                        0x1e => {
                            /* 011110.. ........ .010.... .1010111 */
                            decode_extract_r(transimpl, &mut args, insn);
                            if transimpl.vmnor_mm(args) { return true; }
                        },
                        0x1f => {
                            /* 011111.. ........ .010.... .1010111 */
                            decode_extract_r(transimpl, &mut args, insn);
                            if transimpl.vmxnor_mm(args) { return true; }
                        },
                        _ => { },
                    };
                },
                0x00003000 => {
                    /* 0....... ........ .011.... .1010111 */
                    match (insn >> 26) & 0x1f {
                        0x0 => {
                            /* 000000.. ........ .011.... .1010111 */
                            decode_extract_r_vm(transimpl, &mut args, insn);
                            if transimpl.vadd_vi(args) { return true; }
                        },
                        0x3 => {
                            /* 000011.. ........ .011.... .1010111 */
                            decode_extract_r_vm(transimpl, &mut args, insn);
                            if transimpl.vrsub_vi(args) { return true; }
                        },
                        0x9 => {
                            /* 001001.. ........ .011.... .1010111 */
                            decode_extract_r_vm(transimpl, &mut args, insn);
                            if transimpl.vand_vi(args) { return true; }
                        },
                        0xa => {
                            /* 001010.. ........ .011.... .1010111 */
                            decode_extract_r_vm(transimpl, &mut args, insn);
                            if transimpl.vor_vi(args) { return true; }
                        },
                        0xb => {
                            /* 001011.. ........ .011.... .1010111 */
                            decode_extract_r_vm(transimpl, &mut args, insn);
                            if transimpl.vxor_vi(args) { return true; }
                        },
                        0xc => {
                            /* 001100.. ........ .011.... .1010111 */
                            decode_extract_r_vm(transimpl, &mut args, insn);
                            if transimpl.vrgather_vi(args) { return true; }
                        },
                        0xe => {
                            /* 001110.. ........ .011.... .1010111 */
                            decode_extract_r_vm(transimpl, &mut args, insn);
                            if transimpl.vslideup_vi(args) { return true; }
                        },
                        0xf => {
                            /* 001111.. ........ .011.... .1010111 */
                            decode_extract_r_vm(transimpl, &mut args, insn);
                            if transimpl.vslidedown_vi(args) { return true; }
                        },
                        0x10 => {
                            /* 010000.. ........ .011.... .1010111 */
                            decode_extract_r_vm_1(transimpl, &mut args, insn);
                            match (insn >> 25) & 0x1 {
                                0x0 => {
                                    /* 0100000. ........ .011.... .1010111 */
                                    if transimpl.vadc_vim(args) { return true; }
                                },
                                _ => { },
                            };
                        },
                        0x11 => {
                            /* 010001.. ........ .011.... .1010111 */
                            decode_extract_r_vm(transimpl, &mut args, insn);
                            if transimpl.vmadc_vim(args) { return true; }
                        },
                        0x17 => {
                            /* 010111.. ........ .011.... .1010111 */
                            match (insn >> 25) & 0x1 {
                                0x0 => {
                                    /* 0101110. ........ .011.... .1010111 */
                                    decode_extract_r_vm_0(transimpl, &mut args, insn);
                                    if transimpl.vmerge_vim(args) { return true; }
                                },
                                0x1 => {
                                    /* 0101111. ........ .011.... .1010111 */
                                    decode_extract_r2(transimpl, &mut args, insn);
                                    match (insn >> 20) & 0x1f {
                                        0x0 => {
                                            /* 01011110 0000.... .011.... .1010111 */
                                            if transimpl.vmv_v_i(args) { return true; }
                                        },
                                        _ => { },
                                    };
                                },
                                _ => { },
                            };
                        },
                        0x18 => {
                            /* 011000.. ........ .011.... .1010111 */
                            decode_extract_r_vm(transimpl, &mut args, insn);
                            if transimpl.vmseq_vi(args) { return true; }
                        },
                        0x19 => {
                            /* 011001.. ........ .011.... .1010111 */
                            decode_extract_r_vm(transimpl, &mut args, insn);
                            if transimpl.vmsne_vi(args) { return true; }
                        },
                        0x1c => {
                            /* 011100.. ........ .011.... .1010111 */
                            decode_extract_r_vm(transimpl, &mut args, insn);
                            if transimpl.vmsleu_vi(args) { return true; }
                        },
                        0x1d => {
                            /* 011101.. ........ .011.... .1010111 */
                            decode_extract_r_vm(transimpl, &mut args, insn);
                            if transimpl.vmsle_vi(args) { return true; }
                        },
                        0x1e => {
                            /* 011110.. ........ .011.... .1010111 */
                            decode_extract_r_vm(transimpl, &mut args, insn);
                            if transimpl.vmsgtu_vi(args) { return true; }
                        },
                        0x1f => {
                            /* 011111.. ........ .011.... .1010111 */
                            decode_extract_r_vm(transimpl, &mut args, insn);
                            if transimpl.vmsgt_vi(args) { return true; }
                        },
                        _ => { },
                    };
                },
                0x00004000 => {
                    /* 0....... ........ .100.... .1010111 */
                    match (insn >> 26) & 0x1f {
                        0x0 => {
                            /* 000000.. ........ .100.... .1010111 */
                            decode_extract_r_vm(transimpl, &mut args, insn);
                            if transimpl.vadd_vx(args) { return true; }
                        },
                        0x2 => {
                            /* 000010.. ........ .100.... .1010111 */
                            decode_extract_r_vm(transimpl, &mut args, insn);
                            if transimpl.vsub_vx(args) { return true; }
                        },
                        0x3 => {
                            /* 000011.. ........ .100.... .1010111 */
                            decode_extract_r_vm(transimpl, &mut args, insn);
                            if transimpl.vrsub_vx(args) { return true; }
                        },
                        0x4 => {
                            /* 000100.. ........ .100.... .1010111 */
                            decode_extract_r_vm(transimpl, &mut args, insn);
                            if transimpl.vminu_vx(args) { return true; }
                        },
                        0x5 => {
                            /* 000101.. ........ .100.... .1010111 */
                            decode_extract_r_vm(transimpl, &mut args, insn);
                            if transimpl.vmin_vx(args) { return true; }
                        },
                        0x6 => {
                            /* 000110.. ........ .100.... .1010111 */
                            decode_extract_r_vm(transimpl, &mut args, insn);
                            if transimpl.vmaxu_vx(args) { return true; }
                        },
                        0x7 => {
                            /* 000111.. ........ .100.... .1010111 */
                            decode_extract_r_vm(transimpl, &mut args, insn);
                            if transimpl.vmax_vx(args) { return true; }
                        },
                        0x9 => {
                            /* 001001.. ........ .100.... .1010111 */
                            decode_extract_r_vm(transimpl, &mut args, insn);
                            if transimpl.vand_vx(args) { return true; }
                        },
                        0xa => {
                            /* 001010.. ........ .100.... .1010111 */
                            decode_extract_r_vm(transimpl, &mut args, insn);
                            if transimpl.vor_vx(args) { return true; }
                        },
                        0xb => {
                            /* 001011.. ........ .100.... .1010111 */
                            decode_extract_r_vm(transimpl, &mut args, insn);
                            if transimpl.vxor_vx(args) { return true; }
                        },
                        0xc => {
                            /* 001100.. ........ .100.... .1010111 */
                            decode_extract_r_vm(transimpl, &mut args, insn);
                            if transimpl.vrgather_vx(args) { return true; }
                        },
                        0xe => {
                            /* 001110.. ........ .100.... .1010111 */
                            decode_extract_r_vm(transimpl, &mut args, insn);
                            if transimpl.vslideup_vx(args) { return true; }
                        },
                        0xf => {
                            /* 001111.. ........ .100.... .1010111 */
                            decode_extract_r_vm(transimpl, &mut args, insn);
                            if transimpl.vslidedown_vx(args) { return true; }
                        },
                        0x10 => {
                            /* 010000.. ........ .100.... .1010111 */
                            decode_extract_r_vm_1(transimpl, &mut args, insn);
                            match (insn >> 25) & 0x1 {
                                0x0 => {
                                    /* 0100000. ........ .100.... .1010111 */
                                    if transimpl.vadc_vxm(args) { return true; }
                                },
                                _ => { },
                            };
                        },
                        0x11 => {
                            /* 010001.. ........ .100.... .1010111 */
                            decode_extract_r_vm(transimpl, &mut args, insn);
                            if transimpl.vmadc_vxm(args) { return true; }
                        },
                        0x12 => {
                            /* 010010.. ........ .100.... .1010111 */
                            decode_extract_r_vm_1(transimpl, &mut args, insn);
                            match (insn >> 25) & 0x1 {
                                0x0 => {
                                    /* 0100100. ........ .100.... .1010111 */
                                    if transimpl.vsbc_vxm(args) { return true; }
                                },
                                _ => { },
                            };
                        },
                        0x13 => {
                            /* 010011.. ........ .100.... .1010111 */
                            decode_extract_r_vm(transimpl, &mut args, insn);
                            if transimpl.vmsbc_vxm(args) { return true; }
                        },
                        0x17 => {
                            /* 010111.. ........ .100.... .1010111 */
                            match (insn >> 25) & 0x1 {
                                0x0 => {
                                    /* 0101110. ........ .100.... .1010111 */
                                    decode_extract_r_vm_0(transimpl, &mut args, insn);
                                    if transimpl.vmerge_vxm(args) { return true; }
                                },
                                0x1 => {
                                    /* 0101111. ........ .100.... .1010111 */
                                    decode_extract_r2(transimpl, &mut args, insn);
                                    match (insn >> 20) & 0x1f {
                                        0x0 => {
                                            /* 01011110 0000.... .100.... .1010111 */
                                            if transimpl.vmv_v_x(args) { return true; }
                                        },
                                        _ => { },
                                    };
                                },
                                _ => { },
                            };
                        },
                        0x18 => {
                            /* 011000.. ........ .100.... .1010111 */
                            decode_extract_r_vm(transimpl, &mut args, insn);
                            if transimpl.vmseq_vx(args) { return true; }
                        },
                        0x19 => {
                            /* 011001.. ........ .100.... .1010111 */
                            decode_extract_r_vm(transimpl, &mut args, insn);
                            if transimpl.vmsne_vx(args) { return true; }
                        },
                        0x1a => {
                            /* 011010.. ........ .100.... .1010111 */
                            decode_extract_r_vm(transimpl, &mut args, insn);
                            if transimpl.vmsltu_vx(args) { return true; }
                        },
                        0x1b => {
                            /* 011011.. ........ .100.... .1010111 */
                            decode_extract_r_vm(transimpl, &mut args, insn);
                            if transimpl.vmslt_vx(args) { return true; }
                        },
                        0x1c => {
                            /* 011100.. ........ .100.... .1010111 */
                            decode_extract_r_vm(transimpl, &mut args, insn);
                            if transimpl.vmsleu_vx(args) { return true; }
                        },
                        0x1d => {
                            /* 011101.. ........ .100.... .1010111 */
                            decode_extract_r_vm(transimpl, &mut args, insn);
                            if transimpl.vmsle_vx(args) { return true; }
                        },
                        0x1e => {
                            /* 011110.. ........ .100.... .1010111 */
                            decode_extract_r_vm(transimpl, &mut args, insn);
                            if transimpl.vmsgtu_vx(args) { return true; }
                        },
                        0x1f => {
                            /* 011111.. ........ .100.... .1010111 */
                            decode_extract_r_vm(transimpl, &mut args, insn);
                            if transimpl.vmsgt_vx(args) { return true; }
                        },
                        _ => { },
                    };
                },
                0x00005000 => {
                    /* 0....... ........ .101.... .1010111 */
                    match (insn >> 26) & 0x1f {
                        0x0 => {
                            /* 000000.. ........ .101.... .1010111 */
                            decode_extract_r_vm(transimpl, &mut args, insn);
                            if transimpl.vfadd_vf(args) { return true; }
                        },
                        0x2 => {
                            /* 000010.. ........ .101.... .1010111 */
                            decode_extract_r_vm(transimpl, &mut args, insn);
                            if transimpl.vfsub_vf(args) { return true; }
                        },
                        0x4 => {
                            /* 000100.. ........ .101.... .1010111 */
                            decode_extract_r_vm(transimpl, &mut args, insn);
                            if transimpl.vfmin_vf(args) { return true; }
                        },
                        0x6 => {
                            /* 000110.. ........ .101.... .1010111 */
                            decode_extract_r_vm(transimpl, &mut args, insn);
                            if transimpl.vfmax_vf(args) { return true; }
                        },
                        0x8 => {
                            /* 001000.. ........ .101.... .1010111 */
                            decode_extract_r_vm(transimpl, &mut args, insn);
                            if transimpl.vfsgnj_vf(args) { return true; }
                        },
                        0x9 => {
                            /* 001001.. ........ .101.... .1010111 */
                            decode_extract_r_vm(transimpl, &mut args, insn);
                            if transimpl.vfsgnjn_vf(args) { return true; }
                        },
                        0xa => {
                            /* 001010.. ........ .101.... .1010111 */
                            decode_extract_r_vm(transimpl, &mut args, insn);
                            if transimpl.vfsgnjx_vf(args) { return true; }
                        },
                        0xe => {
                            /* 001110.. ........ .101.... .1010111 */
                            decode_extract_r_vm(transimpl, &mut args, insn);
                            if transimpl.vfslide1up_vf(args) { return true; }
                        },
                        0xf => {
                            /* 001111.. ........ .101.... .1010111 */
                            decode_extract_r_vm(transimpl, &mut args, insn);
                            if transimpl.vfslide1down_vf(args) { return true; }
                        },
                        0x10 => {
                            /* 010000.. ........ .101.... .1010111 */
                            decode_extract_r2(transimpl, &mut args, insn);
                            match (insn >> 20) & 0x3f {
                                0x20 => {
                                    /* 01000010 0000.... .101.... .1010111 */
                                    if transimpl.vfmv_s_f(args) { return true; }
                                },
                                _ => { },
                            };
                        },
                        0x17 => {
                            /* 010111.. ........ .101.... .1010111 */
                            match (insn >> 25) & 0x1 {
                                0x0 => {
                                    /* 0101110. ........ .101.... .1010111 */
                                    decode_extract_r_vm_0(transimpl, &mut args, insn);
                                    if transimpl.vfmerge_vfm(args) { return true; }
                                },
                                0x1 => {
                                    /* 0101111. ........ .101.... .1010111 */
                                    decode_extract_r2(transimpl, &mut args, insn);
                                    match (insn >> 20) & 0x1f {
                                        0x0 => {
                                            /* 01011110 0000.... .101.... .1010111 */
                                            if transimpl.vfmv_v_f(args) { return true; }
                                        },
                                        _ => { },
                                    };
                                },
                                _ => { },
                            };
                        },
                        0x18 => {
                            /* 011000.. ........ .101.... .1010111 */
                            decode_extract_r_vm(transimpl, &mut args, insn);
                            if transimpl.vmfeq_vf(args) { return true; }
                        },
                        0x19 => {
                            /* 011001.. ........ .101.... .1010111 */
                            decode_extract_r_vm(transimpl, &mut args, insn);
                            if transimpl.vmfle_vf(args) { return true; }
                        },
                        0x1b => {
                            /* 011011.. ........ .101.... .1010111 */
                            decode_extract_r_vm(transimpl, &mut args, insn);
                            if transimpl.vmflt_vf(args) { return true; }
                        },
                        0x1c => {
                            /* 011100.. ........ .101.... .1010111 */
                            decode_extract_r_vm(transimpl, &mut args, insn);
                            if transimpl.vmfne_vf(args) { return true; }
                        },
                        0x1d => {
                            /* 011101.. ........ .101.... .1010111 */
                            decode_extract_r_vm(transimpl, &mut args, insn);
                            if transimpl.vmfgt_vf(args) { return true; }
                        },
                        0x1f => {
                            /* 011111.. ........ .101.... .1010111 */
                            decode_extract_r_vm(transimpl, &mut args, insn);
                            if transimpl.vmfge_vf(args) { return true; }
                        },
                        _ => { },
                    };
                },
                0x00006000 => {
                    /* 0....... ........ .110.... .1010111 */
                    match (insn >> 26) & 0x1f {
                        0x8 => {
                            /* 001000.. ........ .110.... .1010111 */
                            decode_extract_r_vm(transimpl, &mut args, insn);
                            if transimpl.vaaddu_vx(args) { return true; }
                        },
                        0x9 => {
                            /* 001001.. ........ .110.... .1010111 */
                            decode_extract_r_vm(transimpl, &mut args, insn);
                            if transimpl.vaadd_vx(args) { return true; }
                        },
                        0xa => {
                            /* 001010.. ........ .110.... .1010111 */
                            decode_extract_r_vm(transimpl, &mut args, insn);
                            if transimpl.vasubu_vx(args) { return true; }
                        },
                        0xb => {
                            /* 001011.. ........ .110.... .1010111 */
                            decode_extract_r_vm(transimpl, &mut args, insn);
                            if transimpl.vasub_vx(args) { return true; }
                        },
                        0xe => {
                            /* 001110.. ........ .110.... .1010111 */
                            decode_extract_r_vm(transimpl, &mut args, insn);
                            if transimpl.vslide1up_vx(args) { return true; }
                        },
                        0xf => {
                            /* 001111.. ........ .110.... .1010111 */
                            decode_extract_r_vm(transimpl, &mut args, insn);
                            if transimpl.vslide1down_vx(args) { return true; }
                        },
                        0x10 => {
                            /* 010000.. ........ .110.... .1010111 */
                            decode_extract_r2(transimpl, &mut args, insn);
                            match (insn >> 20) & 0x3f {
                                0x20 => {
                                    /* 01000010 0000.... .110.... .1010111 */
                                    if transimpl.vmv_s_x(args) { return true; }
                                },
                                _ => { },
                            };
                        },
                        _ => { },
                    };
                },
                0x00007000 => {
                    /* 0....... ........ .111.... .1010111 */
                    decode_extract_r2_zimm11(transimpl, &mut args, insn);
                    if transimpl.vsetvli(args) { return true; }
                },
                0x80000000 => {
                    /* 1....... ........ .000.... .1010111 */
                    decode_extract_r_vm(transimpl, &mut args, insn);
                    match (insn >> 26) & 0x1f {
                        0x0 => {
                            /* 100000.. ........ .000.... .1010111 */
                            if transimpl.vsaddu_vv(args) { return true; }
                        },
                        0x1 => {
                            /* 100001.. ........ .000.... .1010111 */
                            if transimpl.vsadd_vv(args) { return true; }
                        },
                        0x2 => {
                            /* 100010.. ........ .000.... .1010111 */
                            if transimpl.vssubu_vv(args) { return true; }
                        },
                        0x3 => {
                            /* 100011.. ........ .000.... .1010111 */
                            if transimpl.vssub_vv(args) { return true; }
                        },
                        0x5 => {
                            /* 100101.. ........ .000.... .1010111 */
                            if transimpl.vsll_vv(args) { return true; }
                        },
                        0x7 => {
                            /* 100111.. ........ .000.... .1010111 */
                            if transimpl.vsmul_vv(args) { return true; }
                        },
                        0x8 => {
                            /* 101000.. ........ .000.... .1010111 */
                            if transimpl.vsrl_vv(args) { return true; }
                        },
                        0x9 => {
                            /* 101001.. ........ .000.... .1010111 */
                            if transimpl.vsra_vv(args) { return true; }
                        },
                        0xa => {
                            /* 101010.. ........ .000.... .1010111 */
                            if transimpl.vssrl_vv(args) { return true; }
                        },
                        0xb => {
                            /* 101011.. ........ .000.... .1010111 */
                            if transimpl.vssra_vv(args) { return true; }
                        },
                        0xc => {
                            /* 101100.. ........ .000.... .1010111 */
                            if transimpl.vnsrl_wv(args) { return true; }
                        },
                        0xd => {
                            /* 101101.. ........ .000.... .1010111 */
                            if transimpl.vnsra_wv(args) { return true; }
                        },
                        0xe => {
                            /* 101110.. ........ .000.... .1010111 */
                            if transimpl.vnclipu_wv(args) { return true; }
                        },
                        0xf => {
                            /* 101111.. ........ .000.... .1010111 */
                            if transimpl.vnclip_wv(args) { return true; }
                        },
                        0x10 => {
                            /* 110000.. ........ .000.... .1010111 */
                            if transimpl.vwredsumu_vs(args) { return true; }
                        },
                        0x11 => {
                            /* 110001.. ........ .000.... .1010111 */
                            if transimpl.vwredsum_vs(args) { return true; }
                        },
                        _ => { },
                    };
                },
                0x80001000 => {
                    /* 1....... ........ .001.... .1010111 */
                    decode_extract_r_vm(transimpl, &mut args, insn);
                    match (insn >> 26) & 0x1f {
                        0x0 => {
                            /* 100000.. ........ .001.... .1010111 */
                            if transimpl.vfdiv_vv(args) { return true; }
                        },
                        0x4 => {
                            /* 100100.. ........ .001.... .1010111 */
                            if transimpl.vfmul_vv(args) { return true; }
                        },
                        0x8 => {
                            /* 101000.. ........ .001.... .1010111 */
                            if transimpl.vfmadd_vv(args) { return true; }
                        },
                        0x9 => {
                            /* 101001.. ........ .001.... .1010111 */
                            if transimpl.vfnmadd_vv(args) { return true; }
                        },
                        0xa => {
                            /* 101010.. ........ .001.... .1010111 */
                            if transimpl.vfmsub_vv(args) { return true; }
                        },
                        0xb => {
                            /* 101011.. ........ .001.... .1010111 */
                            if transimpl.vfnmsub_vv(args) { return true; }
                        },
                        0xc => {
                            /* 101100.. ........ .001.... .1010111 */
                            if transimpl.vfmacc_vv(args) { return true; }
                        },
                        0xd => {
                            /* 101101.. ........ .001.... .1010111 */
                            if transimpl.vfnmacc_vv(args) { return true; }
                        },
                        0xe => {
                            /* 101110.. ........ .001.... .1010111 */
                            if transimpl.vfmsac_vv(args) { return true; }
                        },
                        0xf => {
                            /* 101111.. ........ .001.... .1010111 */
                            if transimpl.vfnmsac_vv(args) { return true; }
                        },
                        0x10 => {
                            /* 110000.. ........ .001.... .1010111 */
                            if transimpl.vfwadd_vv(args) { return true; }
                        },
                        0x11 => {
                            /* 110001.. ........ .001.... .1010111 */
                            if transimpl.vfwredusum_vs(args) { return true; }
                        },
                        0x12 => {
                            /* 110010.. ........ .001.... .1010111 */
                            if transimpl.vfwsub_vv(args) { return true; }
                        },
                        0x13 => {
                            /* 110011.. ........ .001.... .1010111 */
                            if transimpl.vfwredosum_vs(args) { return true; }
                        },
                        0x14 => {
                            /* 110100.. ........ .001.... .1010111 */
                            if transimpl.vfwadd_wv(args) { return true; }
                        },
                        0x16 => {
                            /* 110110.. ........ .001.... .1010111 */
                            if transimpl.vfwsub_wv(args) { return true; }
                        },
                        0x18 => {
                            /* 111000.. ........ .001.... .1010111 */
                            if transimpl.vfwmul_vv(args) { return true; }
                        },
                        0x1c => {
                            /* 111100.. ........ .001.... .1010111 */
                            if transimpl.vfwmacc_vv(args) { return true; }
                        },
                        0x1d => {
                            /* 111101.. ........ .001.... .1010111 */
                            if transimpl.vfwnmacc_vv(args) { return true; }
                        },
                        0x1e => {
                            /* 111110.. ........ .001.... .1010111 */
                            if transimpl.vfwmsac_vv(args) { return true; }
                        },
                        0x1f => {
                            /* 111111.. ........ .001.... .1010111 */
                            if transimpl.vfwnmsac_vv(args) { return true; }
                        },
                        _ => { },
                    };
                },
                0x80002000 => {
                    /* 1....... ........ .010.... .1010111 */
                    decode_extract_r_vm(transimpl, &mut args, insn);
                    match (insn >> 26) & 0x1f {
                        0x0 => {
                            /* 100000.. ........ .010.... .1010111 */
                            if transimpl.vdivu_vv(args) { return true; }
                        },
                        0x1 => {
                            /* 100001.. ........ .010.... .1010111 */
                            if transimpl.vdiv_vv(args) { return true; }
                        },
                        0x2 => {
                            /* 100010.. ........ .010.... .1010111 */
                            if transimpl.vremu_vv(args) { return true; }
                        },
                        0x3 => {
                            /* 100011.. ........ .010.... .1010111 */
                            if transimpl.vrem_vv(args) { return true; }
                        },
                        0x4 => {
                            /* 100100.. ........ .010.... .1010111 */
                            if transimpl.vmulhu_vv(args) { return true; }
                        },
                        0x5 => {
                            /* 100101.. ........ .010.... .1010111 */
                            if transimpl.vmul_vv(args) { return true; }
                        },
                        0x6 => {
                            /* 100110.. ........ .010.... .1010111 */
                            if transimpl.vmulhsu_vv(args) { return true; }
                        },
                        0x7 => {
                            /* 100111.. ........ .010.... .1010111 */
                            if transimpl.vmulh_vv(args) { return true; }
                        },
                        0x9 => {
                            /* 101001.. ........ .010.... .1010111 */
                            if transimpl.vmadd_vv(args) { return true; }
                        },
                        0xb => {
                            /* 101011.. ........ .010.... .1010111 */
                            if transimpl.vnmsub_vv(args) { return true; }
                        },
                        0xd => {
                            /* 101101.. ........ .010.... .1010111 */
                            if transimpl.vmacc_vv(args) { return true; }
                        },
                        0xf => {
                            /* 101111.. ........ .010.... .1010111 */
                            if transimpl.vnmsac_vv(args) { return true; }
                        },
                        0x10 => {
                            /* 110000.. ........ .010.... .1010111 */
                            if transimpl.vwaddu_vv(args) { return true; }
                        },
                        0x11 => {
                            /* 110001.. ........ .010.... .1010111 */
                            if transimpl.vwadd_vv(args) { return true; }
                        },
                        0x12 => {
                            /* 110010.. ........ .010.... .1010111 */
                            if transimpl.vwsubu_vv(args) { return true; }
                        },
                        0x13 => {
                            /* 110011.. ........ .010.... .1010111 */
                            if transimpl.vwsub_vv(args) { return true; }
                        },
                        0x14 => {
                            /* 110100.. ........ .010.... .1010111 */
                            if transimpl.vwaddu_wv(args) { return true; }
                        },
                        0x15 => {
                            /* 110101.. ........ .010.... .1010111 */
                            if transimpl.vwadd_wv(args) { return true; }
                        },
                        0x16 => {
                            /* 110110.. ........ .010.... .1010111 */
                            if transimpl.vwsubu_wv(args) { return true; }
                        },
                        0x17 => {
                            /* 110111.. ........ .010.... .1010111 */
                            if transimpl.vwsub_wv(args) { return true; }
                        },
                        0x18 => {
                            /* 111000.. ........ .010.... .1010111 */
                            if transimpl.vwmulu_vv(args) { return true; }
                        },
                        0x1a => {
                            /* 111010.. ........ .010.... .1010111 */
                            if transimpl.vwmulsu_vv(args) { return true; }
                        },
                        0x1b => {
                            /* 111011.. ........ .010.... .1010111 */
                            if transimpl.vwmul_vv(args) { return true; }
                        },
                        0x1c => {
                            /* 111100.. ........ .010.... .1010111 */
                            if transimpl.vwmaccu_vv(args) { return true; }
                        },
                        0x1d => {
                            /* 111101.. ........ .010.... .1010111 */
                            if transimpl.vwmacc_vv(args) { return true; }
                        },
                        0x1f => {
                            /* 111111.. ........ .010.... .1010111 */
                            if transimpl.vwmaccsu_vv(args) { return true; }
                        },
                        _ => { },
                    };
                },
                0x80003000 => {
                    /* 1....... ........ .011.... .1010111 */
                    match (insn >> 26) & 0x1f {
                        0x0 => {
                            /* 100000.. ........ .011.... .1010111 */
                            decode_extract_r_vm(transimpl, &mut args, insn);
                            if transimpl.vsaddu_vi(args) { return true; }
                        },
                        0x1 => {
                            /* 100001.. ........ .011.... .1010111 */
                            decode_extract_r_vm(transimpl, &mut args, insn);
                            if transimpl.vsadd_vi(args) { return true; }
                        },
                        0x5 => {
                            /* 100101.. ........ .011.... .1010111 */
                            decode_extract_r_vm(transimpl, &mut args, insn);
                            if transimpl.vsll_vi(args) { return true; }
                        },
                        0x7 => {
                            /* 100111.. ........ .011.... .1010111 */
                            decode_extract_r2rd(transimpl, &mut args, insn);
                            match insn & 0x020f8000 {
                                0x02000000 => {
                                    /* 1001111. ....0000 0011.... .1010111 */
                                    if transimpl.vmv1r_v(args) { return true; }
                                },
                                0x02008000 => {
                                    /* 1001111. ....0000 1011.... .1010111 */
                                    if transimpl.vmv2r_v(args) { return true; }
                                },
                                0x02018000 => {
                                    /* 1001111. ....0001 1011.... .1010111 */
                                    if transimpl.vmv4r_v(args) { return true; }
                                },
                                0x02038000 => {
                                    /* 1001111. ....0011 1011.... .1010111 */
                                    if transimpl.vmv8r_v(args) { return true; }
                                },
                                _ => { },
                            };
                        },
                        0x8 => {
                            /* 101000.. ........ .011.... .1010111 */
                            decode_extract_r_vm(transimpl, &mut args, insn);
                            if transimpl.vsrl_vi(args) { return true; }
                        },
                        0x9 => {
                            /* 101001.. ........ .011.... .1010111 */
                            decode_extract_r_vm(transimpl, &mut args, insn);
                            if transimpl.vsra_vi(args) { return true; }
                        },
                        0xa => {
                            /* 101010.. ........ .011.... .1010111 */
                            decode_extract_r_vm(transimpl, &mut args, insn);
                            if transimpl.vssrl_vi(args) { return true; }
                        },
                        0xb => {
                            /* 101011.. ........ .011.... .1010111 */
                            decode_extract_r_vm(transimpl, &mut args, insn);
                            if transimpl.vssra_vi(args) { return true; }
                        },
                        0xc => {
                            /* 101100.. ........ .011.... .1010111 */
                            decode_extract_r_vm(transimpl, &mut args, insn);
                            if transimpl.vnsrl_wi(args) { return true; }
                        },
                        0xd => {
                            /* 101101.. ........ .011.... .1010111 */
                            decode_extract_r_vm(transimpl, &mut args, insn);
                            if transimpl.vnsra_wi(args) { return true; }
                        },
                        0xe => {
                            /* 101110.. ........ .011.... .1010111 */
                            decode_extract_r_vm(transimpl, &mut args, insn);
                            if transimpl.vnclipu_wi(args) { return true; }
                        },
                        0xf => {
                            /* 101111.. ........ .011.... .1010111 */
                            decode_extract_r_vm(transimpl, &mut args, insn);
                            if transimpl.vnclip_wi(args) { return true; }
                        },
                        _ => { },
                    };
                },
                0x80004000 => {
                    /* 1....... ........ .100.... .1010111 */
                    decode_extract_r_vm(transimpl, &mut args, insn);
                    match (insn >> 26) & 0x1f {
                        0x0 => {
                            /* 100000.. ........ .100.... .1010111 */
                            if transimpl.vsaddu_vx(args) { return true; }
                        },
                        0x1 => {
                            /* 100001.. ........ .100.... .1010111 */
                            if transimpl.vsadd_vx(args) { return true; }
                        },
                        0x2 => {
                            /* 100010.. ........ .100.... .1010111 */
                            if transimpl.vssubu_vx(args) { return true; }
                        },
                        0x3 => {
                            /* 100011.. ........ .100.... .1010111 */
                            if transimpl.vssub_vx(args) { return true; }
                        },
                        0x5 => {
                            /* 100101.. ........ .100.... .1010111 */
                            if transimpl.vsll_vx(args) { return true; }
                        },
                        0x7 => {
                            /* 100111.. ........ .100.... .1010111 */
                            if transimpl.vsmul_vx(args) { return true; }
                        },
                        0x8 => {
                            /* 101000.. ........ .100.... .1010111 */
                            if transimpl.vsrl_vx(args) { return true; }
                        },
                        0x9 => {
                            /* 101001.. ........ .100.... .1010111 */
                            if transimpl.vsra_vx(args) { return true; }
                        },
                        0xa => {
                            /* 101010.. ........ .100.... .1010111 */
                            if transimpl.vssrl_vx(args) { return true; }
                        },
                        0xb => {
                            /* 101011.. ........ .100.... .1010111 */
                            if transimpl.vssra_vx(args) { return true; }
                        },
                        0xc => {
                            /* 101100.. ........ .100.... .1010111 */
                            if transimpl.vnsrl_wx(args) { return true; }
                        },
                        0xd => {
                            /* 101101.. ........ .100.... .1010111 */
                            if transimpl.vnsra_wx(args) { return true; }
                        },
                        0xe => {
                            /* 101110.. ........ .100.... .1010111 */
                            if transimpl.vnclipu_wx(args) { return true; }
                        },
                        0xf => {
                            /* 101111.. ........ .100.... .1010111 */
                            if transimpl.vnclip_wx(args) { return true; }
                        },
                        _ => { },
                    };
                },
                0x80005000 => {
                    /* 1....... ........ .101.... .1010111 */
                    decode_extract_r_vm(transimpl, &mut args, insn);
                    match (insn >> 26) & 0x1f {
                        0x0 => {
                            /* 100000.. ........ .101.... .1010111 */
                            if transimpl.vfdiv_vf(args) { return true; }
                        },
                        0x1 => {
                            /* 100001.. ........ .101.... .1010111 */
                            if transimpl.vfrdiv_vf(args) { return true; }
                        },
                        0x4 => {
                            /* 100100.. ........ .101.... .1010111 */
                            if transimpl.vfmul_vf(args) { return true; }
                        },
                        0x7 => {
                            /* 100111.. ........ .101.... .1010111 */
                            if transimpl.vfrsub_vf(args) { return true; }
                        },
                        0x8 => {
                            /* 101000.. ........ .101.... .1010111 */
                            if transimpl.vfmadd_vf(args) { return true; }
                        },
                        0x9 => {
                            /* 101001.. ........ .101.... .1010111 */
                            if transimpl.vfnmadd_vf(args) { return true; }
                        },
                        0xa => {
                            /* 101010.. ........ .101.... .1010111 */
                            if transimpl.vfmsub_vf(args) { return true; }
                        },
                        0xb => {
                            /* 101011.. ........ .101.... .1010111 */
                            if transimpl.vfnmsub_vf(args) { return true; }
                        },
                        0xc => {
                            /* 101100.. ........ .101.... .1010111 */
                            if transimpl.vfmacc_vf(args) { return true; }
                        },
                        0xd => {
                            /* 101101.. ........ .101.... .1010111 */
                            if transimpl.vfnmacc_vf(args) { return true; }
                        },
                        0xe => {
                            /* 101110.. ........ .101.... .1010111 */
                            if transimpl.vfmsac_vf(args) { return true; }
                        },
                        0xf => {
                            /* 101111.. ........ .101.... .1010111 */
                            if transimpl.vfnmsac_vf(args) { return true; }
                        },
                        0x10 => {
                            /* 110000.. ........ .101.... .1010111 */
                            if transimpl.vfwadd_vf(args) { return true; }
                        },
                        0x12 => {
                            /* 110010.. ........ .101.... .1010111 */
                            if transimpl.vfwsub_vf(args) { return true; }
                        },
                        0x14 => {
                            /* 110100.. ........ .101.... .1010111 */
                            if transimpl.vfwadd_wf(args) { return true; }
                        },
                        0x16 => {
                            /* 110110.. ........ .101.... .1010111 */
                            if transimpl.vfwsub_wf(args) { return true; }
                        },
                        0x18 => {
                            /* 111000.. ........ .101.... .1010111 */
                            if transimpl.vfwmul_vf(args) { return true; }
                        },
                        0x1c => {
                            /* 111100.. ........ .101.... .1010111 */
                            if transimpl.vfwmacc_vf(args) { return true; }
                        },
                        0x1d => {
                            /* 111101.. ........ .101.... .1010111 */
                            if transimpl.vfwnmacc_vf(args) { return true; }
                        },
                        0x1e => {
                            /* 111110.. ........ .101.... .1010111 */
                            if transimpl.vfwmsac_vf(args) { return true; }
                        },
                        0x1f => {
                            /* 111111.. ........ .101.... .1010111 */
                            if transimpl.vfwnmsac_vf(args) { return true; }
                        },
                        _ => { },
                    };
                },
                0x80006000 => {
                    /* 1....... ........ .110.... .1010111 */
                    decode_extract_r_vm(transimpl, &mut args, insn);
                    match (insn >> 26) & 0x1f {
                        0x0 => {
                            /* 100000.. ........ .110.... .1010111 */
                            if transimpl.vdivu_vx(args) { return true; }
                        },
                        0x1 => {
                            /* 100001.. ........ .110.... .1010111 */
                            if transimpl.vdiv_vx(args) { return true; }
                        },
                        0x2 => {
                            /* 100010.. ........ .110.... .1010111 */
                            if transimpl.vremu_vx(args) { return true; }
                        },
                        0x3 => {
                            /* 100011.. ........ .110.... .1010111 */
                            if transimpl.vrem_vx(args) { return true; }
                        },
                        0x4 => {
                            /* 100100.. ........ .110.... .1010111 */
                            if transimpl.vmulhu_vx(args) { return true; }
                        },
                        0x5 => {
                            /* 100101.. ........ .110.... .1010111 */
                            if transimpl.vmul_vx(args) { return true; }
                        },
                        0x6 => {
                            /* 100110.. ........ .110.... .1010111 */
                            if transimpl.vmulhsu_vx(args) { return true; }
                        },
                        0x7 => {
                            /* 100111.. ........ .110.... .1010111 */
                            if transimpl.vmulh_vx(args) { return true; }
                        },
                        0x9 => {
                            /* 101001.. ........ .110.... .1010111 */
                            if transimpl.vmadd_vx(args) { return true; }
                        },
                        0xb => {
                            /* 101011.. ........ .110.... .1010111 */
                            if transimpl.vnmsub_vx(args) { return true; }
                        },
                        0xd => {
                            /* 101101.. ........ .110.... .1010111 */
                            if transimpl.vmacc_vx(args) { return true; }
                        },
                        0xf => {
                            /* 101111.. ........ .110.... .1010111 */
                            if transimpl.vnmsac_vx(args) { return true; }
                        },
                        0x10 => {
                            /* 110000.. ........ .110.... .1010111 */
                            if transimpl.vwaddu_vx(args) { return true; }
                        },
                        0x11 => {
                            /* 110001.. ........ .110.... .1010111 */
                            if transimpl.vwadd_vx(args) { return true; }
                        },
                        0x12 => {
                            /* 110010.. ........ .110.... .1010111 */
                            if transimpl.vwsubu_vx(args) { return true; }
                        },
                        0x13 => {
                            /* 110011.. ........ .110.... .1010111 */
                            if transimpl.vwsub_vx(args) { return true; }
                        },
                        0x14 => {
                            /* 110100.. ........ .110.... .1010111 */
                            if transimpl.vwaddu_wx(args) { return true; }
                        },
                        0x15 => {
                            /* 110101.. ........ .110.... .1010111 */
                            if transimpl.vwadd_wx(args) { return true; }
                        },
                        0x16 => {
                            /* 110110.. ........ .110.... .1010111 */
                            if transimpl.vwsubu_wx(args) { return true; }
                        },
                        0x17 => {
                            /* 110111.. ........ .110.... .1010111 */
                            if transimpl.vwsub_wx(args) { return true; }
                        },
                        0x18 => {
                            /* 111000.. ........ .110.... .1010111 */
                            if transimpl.vwmulu_vx(args) { return true; }
                        },
                        0x1a => {
                            /* 111010.. ........ .110.... .1010111 */
                            if transimpl.vwmulsu_vx(args) { return true; }
                        },
                        0x1b => {
                            /* 111011.. ........ .110.... .1010111 */
                            if transimpl.vwmul_vx(args) { return true; }
                        },
                        0x1c => {
                            /* 111100.. ........ .110.... .1010111 */
                            if transimpl.vwmaccu_vx(args) { return true; }
                        },
                        0x1d => {
                            /* 111101.. ........ .110.... .1010111 */
                            if transimpl.vwmacc_vx(args) { return true; }
                        },
                        0x1e => {
                            /* 111110.. ........ .110.... .1010111 */
                            if transimpl.vwmaccus_vx(args) { return true; }
                        },
                        0x1f => {
                            /* 111111.. ........ .110.... .1010111 */
                            if transimpl.vwmaccsu_vx(args) { return true; }
                        },
                        _ => { },
                    };
                },
                0x80007000 => {
                    /* 1....... ........ .111.... .1010111 */
                    match (insn >> 30) & 0x1 {
                        0x0 => {
                            /* 10...... ........ .111.... .1010111 */
                            decode_extract_r(transimpl, &mut args, insn);
                            match (insn >> 25) & 0x1f {
                                0x0 => {
                                    /* 1000000. ........ .111.... .1010111 */
                                    if transimpl.vsetvl(args) { return true; }
                                },
                                _ => { },
                            };
                        },
                        0x1 => {
                            /* 11...... ........ .111.... .1010111 */
                            decode_extract_r2_zimm10(transimpl, &mut args, insn);
                            if transimpl.vsetivli(args) { return true; }
                        },
                        _ => { },
                    };
                },
                _ => { },
            };
        },
        0x0000005b => {
            /* ........ ........ ........ .1011011 */
            match (insn >> 12) & 0x7 {
                0x0 => {
                    /* ........ ........ .000.... .1011011 */
                    decode_extract_i(transimpl, &mut args, insn);
                    if transimpl.addid(args) { return true; }
                },
                0x1 => {
                    /* ........ ........ .001.... .1011011 */
                    decode_extract_sh6(transimpl, &mut args, insn);
                    match (insn >> 26) & 0x3f {
                        0x0 => {
                            /* 000000.. ........ .001.... .1011011 */
                            if transimpl.sllid(args) { return true; }
                        },
                        _ => { },
                    };
                },
                0x5 => {
                    /* ........ ........ .101.... .1011011 */
                    decode_extract_sh6(transimpl, &mut args, insn);
                    match (insn >> 26) & 0x3f {
                        0x0 => {
                            /* 000000.. ........ .101.... .1011011 */
                            if transimpl.srlid(args) { return true; }
                        },
                        0x10 => {
                            /* 010000.. ........ .101.... .1011011 */
                            if transimpl.sraid(args) { return true; }
                        },
                        _ => { },
                    };
                },
                _ => { },
            };
        },
        0x00000063 => {
            /* ........ ........ ........ .1100011 */
            decode_extract_b(transimpl, &mut args, insn);
            match (insn >> 12) & 0x7 {
                0x0 => {
                    /* ........ ........ .000.... .1100011 */
                    if transimpl.beq(args) { return true; }
                },
                0x1 => {
                    /* ........ ........ .001.... .1100011 */
                    if transimpl.bne(args) { return true; }
                },
                0x4 => {
                    /* ........ ........ .100.... .1100011 */
                    if transimpl.blt(args) { return true; }
                },
                0x5 => {
                    /* ........ ........ .101.... .1100011 */
                    if transimpl.bge(args) { return true; }
                },
                0x6 => {
                    /* ........ ........ .110.... .1100011 */
                    if transimpl.bltu(args) { return true; }
                },
                0x7 => {
                    /* ........ ........ .111.... .1100011 */
                    if transimpl.bgeu(args) { return true; }
                },
                _ => { },
            };
        },
        0x00000067 => {
            /* ........ ........ ........ .1100111 */
            decode_extract_i(transimpl, &mut args, insn);
            match (insn >> 12) & 0x7 {
                0x0 => {
                    /* ........ ........ .000.... .1100111 */
                    if transimpl.jalr(args) { return true; }
                },
                _ => { },
            };
        },
        0x0000006f => {
            /* ........ ........ ........ .1101111 */
            decode_extract_j(transimpl, &mut args, insn);
            if transimpl.jal(args) { return true; }
        },
        0x00000073 => {
            /* ........ ........ ........ .1110011 */
            match (insn >> 12) & 0x7 {
                0x0 => {
                    /* ........ ........ .000.... .1110011 */
                    match insn & 0xfe000f80 {
                        0x00000000 => {
                            /* 0000000. ........ .0000000 01110011 */
                            decode_extract_decode_Fmt_33(transimpl, &mut args, insn);
                            match (insn >> 15) & 0x3ff {
                                0x0 => {
                                    /* 00000000 00000000 00000000 01110011 */
                                    if transimpl.ecall(args) { return true; }
                                },
                                0x20 => {
                                    /* 00000000 00010000 00000000 01110011 */
                                    if transimpl.ebreak(args) { return true; }
                                },
                                0x40 => {
                                    /* 00000000 00100000 00000000 01110011 */
                                    if transimpl.uret(args) { return true; }
                                },
                                _ => { },
                            };
                        },
                        0x10000000 => {
                            /* 0001000. ........ .0000000 01110011 */
                            match (insn >> 20) & 0x1f {
                                0x2 => {
                                    /* 00010000 0010.... .0000000 01110011 */
                                    decode_extract_decode_Fmt_33(transimpl, &mut args, insn);
                                    match (insn >> 15) & 0x1f {
                                        0x0 => {
                                            /* 00010000 00100000 00000000 01110011 */
                                            if transimpl.sret(args) { return true; }
                                        },
                                        _ => { },
                                    };
                                },
                                0x4 => {
                                    /* 00010000 0100.... .0000000 01110011 */
                                    decode_extract_sfence_vm(transimpl, &mut args, insn);
                                    if transimpl.sfence_vm(args) { return true; }
                                },
                                0x5 => {
                                    /* 00010000 0101.... .0000000 01110011 */
                                    decode_extract_decode_Fmt_33(transimpl, &mut args, insn);
                                    match (insn >> 15) & 0x1f {
                                        0x0 => {
                                            /* 00010000 01010000 00000000 01110011 */
                                            if transimpl.wfi(args) { return true; }
                                        },
                                        _ => { },
                                    };
                                },
                                _ => { },
                            };
                        },
                        0x12000000 => {
                            /* 0001001. ........ .0000000 01110011 */
                            decode_extract_sfence_vma(transimpl, &mut args, insn);
                            if transimpl.sfence_vma(args) { return true; }
                        },
                        0x16000000 => {
                            /* 0001011. ........ .0000000 01110011 */
                            decode_extract_sfence_vma(transimpl, &mut args, insn);
                            if transimpl.sinval_vma(args) { return true; }
                        },
                        0x18000000 => {
                            /* 0001100. ........ .0000000 01110011 */
                            decode_extract_decode_Fmt_33(transimpl, &mut args, insn);
                            match (insn >> 15) & 0x3ff {
                                0x0 => {
                                    /* 00011000 00000000 00000000 01110011 */
                                    if transimpl.sfence_w_inval(args) { return true; }
                                },
                                0x20 => {
                                    /* 00011000 00010000 00000000 01110011 */
                                    if transimpl.sfence_inval_ir(args) { return true; }
                                },
                                _ => { },
                            };
                        },
                        0x22000000 => {
                            /* 0010001. ........ .0000000 01110011 */
                            decode_extract_hfence_vvma(transimpl, &mut args, insn);
                            if transimpl.hfence_vvma(args) { return true; }
                        },
                        0x26000000 => {
                            /* 0010011. ........ .0000000 01110011 */
                            decode_extract_hfence_vvma(transimpl, &mut args, insn);
                            if transimpl.hinval_vvma(args) { return true; }
                        },
                        0x30000000 => {
                            /* 0011000. ........ .0000000 01110011 */
                            decode_extract_decode_Fmt_33(transimpl, &mut args, insn);
                            match (insn >> 15) & 0x3ff {
                                0x40 => {
                                    /* 00110000 00100000 00000000 01110011 */
                                    if transimpl.mret(args) { return true; }
                                },
                                _ => { },
                            };
                        },
                        0x62000000 => {
                            /* 0110001. ........ .0000000 01110011 */
                            decode_extract_hfence_gvma(transimpl, &mut args, insn);
                            if transimpl.hfence_gvma(args) { return true; }
                        },
                        0x66000000 => {
                            /* 0110011. ........ .0000000 01110011 */
                            decode_extract_hfence_gvma(transimpl, &mut args, insn);
                            if transimpl.hinval_gvma(args) { return true; }
                        },
                        _ => { },
                    };
                },
                0x1 => {
                    /* ........ ........ .001.... .1110011 */
                    decode_extract_csr(transimpl, &mut args, insn);
                    if transimpl.csrrw(args) { return true; }
                },
                0x2 => {
                    /* ........ ........ .010.... .1110011 */
                    decode_extract_csr(transimpl, &mut args, insn);
                    if transimpl.csrrs(args) { return true; }
                },
                0x3 => {
                    /* ........ ........ .011.... .1110011 */
                    decode_extract_csr(transimpl, &mut args, insn);
                    if transimpl.csrrc(args) { return true; }
                },
                0x4 => {
                    /* ........ ........ .100.... .1110011 */
                    match (insn >> 25) & 0x7f {
                        0x30 => {
                            /* 0110000. ........ .100.... .1110011 */
                            decode_extract_r2(transimpl, &mut args, insn);
                            match (insn >> 20) & 0x1f {
                                0x0 => {
                                    /* 01100000 0000.... .100.... .1110011 */
                                    if transimpl.hlv_b(args) { return true; }
                                },
                                0x1 => {
                                    /* 01100000 0001.... .100.... .1110011 */
                                    if transimpl.hlv_bu(args) { return true; }
                                },
                                _ => { },
                            };
                        },
                        0x31 => {
                            /* 0110001. ........ .100.... .1110011 */
                            decode_extract_r2_s(transimpl, &mut args, insn);
                            match (insn >> 7) & 0x1f {
                                0x0 => {
                                    /* 0110001. ........ .1000000 01110011 */
                                    if transimpl.hsv_b(args) { return true; }
                                },
                                _ => { },
                            };
                        },
                        0x32 => {
                            /* 0110010. ........ .100.... .1110011 */
                            decode_extract_r2(transimpl, &mut args, insn);
                            match (insn >> 20) & 0x1f {
                                0x0 => {
                                    /* 01100100 0000.... .100.... .1110011 */
                                    if transimpl.hlv_h(args) { return true; }
                                },
                                0x1 => {
                                    /* 01100100 0001.... .100.... .1110011 */
                                    if transimpl.hlv_hu(args) { return true; }
                                },
                                0x3 => {
                                    /* 01100100 0011.... .100.... .1110011 */
                                    if transimpl.hlvx_hu(args) { return true; }
                                },
                                _ => { },
                            };
                        },
                        0x33 => {
                            /* 0110011. ........ .100.... .1110011 */
                            decode_extract_r2_s(transimpl, &mut args, insn);
                            match (insn >> 7) & 0x1f {
                                0x0 => {
                                    /* 0110011. ........ .1000000 01110011 */
                                    if transimpl.hsv_h(args) { return true; }
                                },
                                _ => { },
                            };
                        },
                        0x34 => {
                            /* 0110100. ........ .100.... .1110011 */
                            decode_extract_r2(transimpl, &mut args, insn);
                            match (insn >> 20) & 0x1f {
                                0x0 => {
                                    /* 01101000 0000.... .100.... .1110011 */
                                    if transimpl.hlv_w(args) { return true; }
                                },
                                0x1 => {
                                    /* 01101000 0001.... .100.... .1110011 */
                                    if transimpl.hlv_wu(args) { return true; }
                                },
                                0x3 => {
                                    /* 01101000 0011.... .100.... .1110011 */
                                    if transimpl.hlvx_wu(args) { return true; }
                                },
                                _ => { },
                            };
                        },
                        0x35 => {
                            /* 0110101. ........ .100.... .1110011 */
                            decode_extract_r2_s(transimpl, &mut args, insn);
                            match (insn >> 7) & 0x1f {
                                0x0 => {
                                    /* 0110101. ........ .1000000 01110011 */
                                    if transimpl.hsv_w(args) { return true; }
                                },
                                _ => { },
                            };
                        },
                        0x36 => {
                            /* 0110110. ........ .100.... .1110011 */
                            decode_extract_r2(transimpl, &mut args, insn);
                            match (insn >> 20) & 0x1f {
                                0x0 => {
                                    /* 01101100 0000.... .100.... .1110011 */
                                    if transimpl.hlv_d(args) { return true; }
                                },
                                _ => { },
                            };
                        },
                        0x37 => {
                            /* 0110111. ........ .100.... .1110011 */
                            decode_extract_r2_s(transimpl, &mut args, insn);
                            match (insn >> 7) & 0x1f {
                                0x0 => {
                                    /* 0110111. ........ .1000000 01110011 */
                                    if transimpl.hsv_d(args) { return true; }
                                },
                                _ => { },
                            };
                        },
                        _ => { },
                    };
                },
                0x5 => {
                    /* ........ ........ .101.... .1110011 */
                    decode_extract_csr(transimpl, &mut args, insn);
                    if transimpl.csrrwi(args) { return true; }
                },
                0x6 => {
                    /* ........ ........ .110.... .1110011 */
                    decode_extract_csr(transimpl, &mut args, insn);
                    if transimpl.csrrsi(args) { return true; }
                },
                0x7 => {
                    /* ........ ........ .111.... .1110011 */
                    decode_extract_csr(transimpl, &mut args, insn);
                    if transimpl.csrrci(args) { return true; }
                },
                _ => { },
            };
        },
        0x0000007b => {
            /* ........ ........ ........ .1111011 */
            decode_extract_r(transimpl, &mut args, insn);
            match insn & 0xfe007000 {
                0x00000000 => {
                    /* 0000000. ........ .000.... .1111011 */
                    if transimpl.addd(args) { return true; }
                },
                0x00001000 => {
                    /* 0000000. ........ .001.... .1111011 */
                    if transimpl.slld(args) { return true; }
                },
                0x00005000 => {
                    /* 0000000. ........ .101.... .1111011 */
                    if transimpl.srld(args) { return true; }
                },
                0x02000000 => {
                    /* 0000001. ........ .000.... .1111011 */
                    if transimpl.muld(args) { return true; }
                },
                0x02004000 => {
                    /* 0000001. ........ .100.... .1111011 */
                    if transimpl.divd(args) { return true; }
                },
                0x02005000 => {
                    /* 0000001. ........ .101.... .1111011 */
                    if transimpl.divud(args) { return true; }
                },
                0x02006000 => {
                    /* 0000001. ........ .110.... .1111011 */
                    if transimpl.remd(args) { return true; }
                },
                0x02007000 => {
                    /* 0000001. ........ .111.... .1111011 */
                    if transimpl.remud(args) { return true; }
                },
                0x40000000 => {
                    /* 0100000. ........ .000.... .1111011 */
                    if transimpl.subd(args) { return true; }
                },
                0x40005000 => {
                    /* 0100000. ........ .101.... .1111011 */
                    if transimpl.srad(args) { return true; }
                },
                _ => { },
            };
        },
        _ => { },
    };
    panic!();
}
