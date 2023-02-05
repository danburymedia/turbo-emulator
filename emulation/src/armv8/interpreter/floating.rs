use num::ToPrimitive;
use simple_soft_float::{DynamicFloat, F16, F16Traits, F32, F32Traits,
                        F64, F64Traits, FPState, RoundingMode};
use crate::armv8::decodedefs::*;
use crate::armv8::interpreter::floating_helper::{FloatMode, fp32_process_NaNs,
                                                 fp_cvt_2_float_raw, fp_cvt_2_int,
                                                 fp_process_nans_gen, FPCompare, imm8_to_fp16,
                                                 imm8_to_fp32, imm8_to_fp64};
use crate::armv8::interpreter::floating_jumpers::*;
use crate::armv8::interpreter::main::Arm64Cpu;
use crate::common::vect::*;
use crate::armv8::interpreter::vect_helper::VectorReg;
use crate::common::arm_fp_defs::{apply_fpstate, cond_holds, FPSR};
use crate::common::floating_wrappers::{fp_cvt_2_16, fp_cvt_2_32, fp_cvt_2_64, fp_muladd};

pub fn fp_fixed_convert(ai: &mut Arm64Cpu, arg: &ArmInstr) {
    let ftype_raw = ((arg.insn >> 22) & 0b11) as u8;
    let mut fltsize = if ftype_raw == 0 {
        32
    } else if ftype_raw == 0b1 {
        64
    } else if ftype_raw == 0b11 {
        16
    } else {
        panic!()
    };
    todo!(); // later

}
pub fn fp_data_processing_3(ai: &mut Arm64Cpu, arg: &ArmInstr) {
    let fpsize = arg.get_float_type(); // ftype

    let a = ai.vreg[arg.get_ra()].vect;
    let op1 = ai.vreg[arg.get_rn()].vect;
    let op2 = ai.vreg[arg.get_rm()].vect;
    let rm = ai.get_fpscr_rounding_mode();

    let o0 = (arg.insn >> 15) & 1;
    let o1 = (arg.insn >> 21) & 1;
    let opa_neg = o1 == 1;
    let op1_neg = o0 != o1;
    let (res, state) = match fpsize {
        FloatMode::Half => fp_muladd::<u16, F16Traits>(a as u16, op1 as u16,
                                                       op2 as u16, opa_neg, op1_neg,
                                                       false, rm),
        FloatMode::Single => fp_muladd::<u32, F32Traits>(a as u32, op1 as u32,
                                                         op2 as u32, opa_neg, op1_neg,
                                                         false, rm),
        FloatMode::Double => fp_muladd::<u64, F64Traits>(a as u64, op1 as u64,
                                                         op2 as u64, opa_neg, op1_neg,
                                                         false, rm),
    };
    if handle_fpstate(ai, state) {
        return;
    }
    ai.vreg[arg.get_rd()].vect = res as u128;
}
pub fn handle_fpstate(ai: &mut Arm64Cpu, fpstate: FPState) -> bool {
    // If we wanted to support fastexiting on exceptions, we can do that here.
    // For now, we just accumulate into one place. Then we will decide to except after instruction is done
    let mut fps: FPSR = FPSR::default();
    apply_fpstate(&mut fps, &fpstate);
    ai.accumlate_fpsr_errors(fps);
    false
}
pub fn gen_fp_proc_nans(ai: &mut Arm64Cpu, op1: u64, op2: u64, size: usize) -> (bool, u64) {
    // we handle fp trapping (if desiered by guest) after instruction is done, not during
    let mut state: FPSR = FPSR::default();
    let ret = if size == 32 {
        let a = F32::from_bits(op1 as u32);
        let b = F32::from_bits(op2 as u32);
        fp_process_nans_gen(ai, a,b, &mut state)
    } else if size == 64 {
        let a = F64::from_bits(op1);
        let b = F64::from_bits(op2);
        fp_process_nans_gen(ai, a,b, &mut state)
    } else if size == 16 {
        let a = F16::from_bits(op1 as u16);
        let b = F16::from_bits(op2 as u16);
        fp_process_nans_gen(ai, a,b, &mut state)
    } else {
        panic!();
    };
    ai.accumlate_fpsr_errors(state);
    ret
}
pub fn fpcr_2_fpsr(ai: &mut Arm64Cpu, boolval: bool, offset: u32) {
    if (ai.fpcr & (1 << (8 + offset))) != 0 {
        unimplemented!(); // trap
    } else {
        if boolval {
            ai.fpsr |= (1 << (offset));
        }
    }
}