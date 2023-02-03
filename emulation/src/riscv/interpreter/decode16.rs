use crate::riscv::common::{RiscvArgs, Xlen};
use crate::riscv::interpreter::main::RiscvInt;
use crate::riscv::decoder::DecodeTrait;
impl crate::riscv::decoder16::DecodeTrait for RiscvInt {
    fn c_illegal(&mut self, args: RiscvArgs) -> bool {
        self.illegal_instr();
        true
    }
    fn c_addi(&mut self, args: RiscvArgs) -> bool {
        self.addi(args)
    }
    fn c_lq(&mut self, args: RiscvArgs) -> bool {
        return false; // no 128 bit support

    }
    fn c_fld(&mut self, args: RiscvArgs) -> bool {
        self.fld(args)
    }
    fn c_lw(&mut self, args: RiscvArgs) -> bool {
        self.lw(args)
    }
    fn c_sq(&mut self, args: RiscvArgs) -> bool {
        return false; // we dont support 128 bit yet
    }
    fn c_fsd(&mut self, args: RiscvArgs) -> bool {
        self.fsd(args)
    }
    fn c_sw(&mut self, args: RiscvArgs) -> bool {
        self.sw(args)
    }
    fn c_ld(&mut self, args: RiscvArgs) -> bool {
        if self.xlen == Xlen::X32 {
            return false;
        }
        self.ld(args)
    }
    fn c_flw(&mut self, args: RiscvArgs) -> bool {
        if self.xlen != Xlen::X32 {
            return false;
        }
        self.flw(args)
    }
    fn c_sd(&mut self, args: RiscvArgs) -> bool {
        if self.xlen == Xlen::X32 {
            return false;
        }
        self.sd(args)
    }
    fn c_fsw(&mut self, args: RiscvArgs) -> bool {
        if self.xlen != Xlen::X32 {
            return false;
        }
        self.fsw(args)
    }
    fn c_lui(&mut self, args: RiscvArgs) -> bool {
        self.lui(args)
    }
    fn c_srli(&mut self, args: RiscvArgs) -> bool {
        self.srli(args)
    }
    fn c_srai(&mut self, args: RiscvArgs) -> bool {
        self.srai(args)
    }
    fn c_andi(&mut self, args: RiscvArgs) -> bool {
        self.andi(args)
    }
    fn c_sub(&mut self, args: RiscvArgs) -> bool {
        self.sub(args)
    }
    fn c_xor(&mut self, args: RiscvArgs) -> bool {
        self.xor(args)
    }
    fn c_or(&mut self, args: RiscvArgs) -> bool {
        self.or(args)
    }
    fn c_and(&mut self, args: RiscvArgs) -> bool {
        self.and(args)
    }
    fn c_jal(&mut self, args: RiscvArgs) -> bool {
        self.jal(args)
    }
    fn c_beq(&mut self, args: RiscvArgs) -> bool {
        self.beq(args)
    }
    fn c_bne(&mut self, args: RiscvArgs) -> bool {
        self.bne(args)
    }
    fn c64_illegal(&mut self, args: RiscvArgs) -> bool {
        todo!()
    }
    fn c_addiw(&mut self, args: RiscvArgs) -> bool {
        if self.xlen == Xlen::X32 {
            return false;
        }
        self.addiw(args)
    }
    fn c_subw(&mut self, args: RiscvArgs) -> bool {
        self.subw(args)
    }
    fn c_addw(&mut self, args: RiscvArgs) -> bool {
        self.addw(args)
    }
    fn c_slli(&mut self, args: RiscvArgs) -> bool {
        self.slli(args)
    }
    fn c_jalr(&mut self, args: RiscvArgs) -> bool {

        self.jalr(args)
    }
    fn c_ebreak(&mut self, args: RiscvArgs) -> bool {
        self.ebreak(args)
    }
    fn c_add(&mut self, args: RiscvArgs) -> bool {
        self.add(args)
    }
    fn is_128_bit(&self) -> bool {
        false // dont support 128-bit yet
    }


}