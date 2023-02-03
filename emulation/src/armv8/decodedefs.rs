use jit::extract::extract32;
use crate::armv8::interpreter::floating_helper::FloatMode;
use crate::common::arm_fp_defs::Flags;
use crate::common::signext_arbpos;
#[derive(Clone, Copy, Debug, Default)]
pub struct ArmInstr {
    pub insn: u32,
}
impl ArmInstr {
    pub fn get_float_type(&self) -> FloatMode {
        let ftype = (self.insn >> 22) & 0b11;
        match ftype {
            0 => FloatMode::Single,
            1 => FloatMode::Double,
            3 => FloatMode::Half,
            _ => panic!()
        }
    }
    pub fn get_nzcv(&self) -> Flags {
        Flags {
            n: (self.insn & 0b1000) != 0,
            z: (self.insn & 0b100) != 0,
            c: (self.insn & 0b10) != 0,
            v: (self.insn & 1) != 0
        }
    }
    pub fn get_loadstore_v(&self) -> bool {
        ((self.insn >> 26) & 1) != 0
    }
    pub fn get_loadstore_is_sign_ext(&self) -> bool {
        ((self.insn >> 24) & 1) == 0
    }
    pub fn get_loadstore_reg_imm12(&self) -> u64 {
        ((self.insn >> 10) & 0xfff) as u64

    }

    pub fn get_loadstore_reg_imm9(&self) -> u64 {
        let mut val: u64 = ((self.insn >> 12) & 0x1ff) as u64;
        signext_arbpos(val, 9)
    }
    pub fn get_loadstore_pair_imm7(&self) -> u64 {
        let mut val: u64 = ((self.insn >> 15) & 0b1111111) as u64;
        signext_arbpos(val, 7)
    }
    pub fn get_branch_imm26(&self) -> u64 {
        let mut val: u64 = (self.insn & 0x3ffffff) as u64;
        val <<= 2;
        signext_arbpos(val, 28)
    }
    pub fn get_branch_imm19(&self) -> u64 {
        let mut val: u64 = ((self.insn >> 5) & 0x7ffff) as u64; // todo: right?
        val <<= 2;
        signext_arbpos(val, 21)
    }
    pub fn get_branch_imm14(&self) -> u64 {
        let mut val: u64 = ((self.insn >> 5) & 0x3fff) as u64; // todo: right?
        val <<= 2;
        signext_arbpos(val, 16)
    }
    pub fn get_loadstore_opc(&self) -> u8 {
        ((self.insn >> 22) & 0b11) as u8
    }
    pub fn flags_update(&self) -> bool {
        ((self.insn >> 29) & 1) != 0
    }
    pub fn is_64bit_set(&self) -> bool {
        (self.insn & (1 << 31)) != 0
        // bit 31
    }
    pub fn get_rd(&self) -> usize {
        (self.insn & 0x1f) as usize
    }
    pub fn get_rn(&self) -> usize {
        ((self.insn >> 5) & 0x1f) as usize
    }
    pub fn get_ra(&self) -> usize {
        ((self.insn >> 10) & 0x1f) as usize
    }
    pub fn get_ls_size(&self) -> u64 {
        ((self.insn >> 30) & 0b11) as u64
    }
    pub fn get_simd_size(&self) -> u64 {
        ((self.insn >> 22) & 0b11) as u64
    }
    pub fn get_rm(&self) -> usize {
        ((self.insn >> 16) & 0x1f) as usize

    }
    pub fn get_rt(&self) -> usize {
        (self.insn & 0x1f) as usize
    }
    pub fn get_rt2(&self) -> usize {
        ((self.insn >> 10) & 0x1f) as usize
    }

    pub fn get_condition(&self) -> u8 {
        ((self.insn >> 12) & 0xf) as u8
    }
    pub fn is_bit_set(&self, bitnum: usize) -> bool {
        ((self.insn >> bitnum) & 1) != 0 // non natural, come straight out of manual
    }
    pub fn get_bits(&self, start: u32, len: u32) -> u32 {
        extract32(self.insn, start, len)
    }
}