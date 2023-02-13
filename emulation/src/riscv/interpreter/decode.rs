use crate::riscv::common::RiscvArgs;
use crate::riscv::interpreter;
use crate::riscv::interpreter::core::nop;
use crate::riscv::interpreter::main::{RiscvInstr, RiscvInt};


impl crate::riscv::decoder::DecodeTrait for RiscvInt {
    fn ecall(&mut self, args: RiscvArgs) -> bool {
        if self.cache_enabled {
            self.stop_translating = true;
            self.insert_insn_current(RiscvInstr {
                args,
                inc_by: 0,
                func: interpreter::defs::ecall
            });
        } else {
            interpreter::defs::ecall(self, &args);
        }
        return true;
    }
    fn mret(&mut self, args: RiscvArgs) -> bool {
        if self.cache_enabled {
            self.stop_translating = true;
            self.insert_insn_current(RiscvInstr {
                args,
                inc_by: 0,
                func: interpreter::defs::mret
            });
        } else {
            interpreter::defs::mret(self, &args);
        }
        return true;
    }
    fn lui(&mut self, args: RiscvArgs) -> bool {
        if self.cache_enabled {
            self.insert_insn_current(RiscvInstr {
                args,
                inc_by: 0,
                func: interpreter::defs::lui
            });
        } else {
            interpreter::defs::lui(self, &args);
        }
        return true;
    }
    fn auipc(&mut self, args: RiscvArgs) -> bool {
        if self.cache_enabled {
            self.insert_insn_current(RiscvInstr {
                args,
                inc_by: 0,
                func: interpreter::defs::auipc
            });
        } else {
            interpreter::defs::auipc(self, &args);
        }
        return true;
    }
    fn jal(&mut self, args: RiscvArgs) -> bool {
        if self.cache_enabled {
            self.stop_translating = true;
            self.insert_insn_current(RiscvInstr {
                args,
                inc_by: 0,
                func: interpreter::defs::jal
            });
        } else {
            interpreter::defs::jal(self, &args);
        }
        return true;
    }
    fn jalr(&mut self, args: RiscvArgs) -> bool {
        if self.cache_enabled {
            self.stop_translating = true;
            self.insert_insn_current(RiscvInstr {
                args,
                inc_by: 0,
                func: interpreter::defs::jalr
            });
        } else {
            interpreter::defs::jalr(self, &args);
        }
        return true;
    }
    fn beq(&mut self, args: RiscvArgs) -> bool {
        if self.cache_enabled {
            self.stop_translating = true;
            self.insert_insn_current(RiscvInstr {
                args,
                inc_by: 0,
                func: interpreter::defs::beq
            });
        } else {
            interpreter::defs::beq(self, &args);
        }
        return true;
    }
    fn bne(&mut self, args: RiscvArgs) -> bool {
        if self.cache_enabled {
            self.stop_translating = true;
            self.insert_insn_current(RiscvInstr {
                args,
                inc_by: 0,
                func: interpreter::defs::bne
            });
        } else {
            interpreter::defs::bne(self, &args);
        }
        return true;
    }
    fn blt(&mut self, args: RiscvArgs) -> bool {
        if self.cache_enabled {
            self.stop_translating = true;
            self.insert_insn_current(RiscvInstr {
                args,
                inc_by: 0,
                func: interpreter::defs::blt
            });
        } else {
            interpreter::defs::blt(self, &args);
        }
        return true;
    }
    fn bge(&mut self, args: RiscvArgs) -> bool {
        if self.cache_enabled {
            self.stop_translating = true;
            self.insert_insn_current(RiscvInstr {
                args,
                inc_by: 0,
                func: interpreter::defs::bge
            });
        } else {
            interpreter::defs::bge(self, &args);
        }
        return true;
    }
    fn bltu(&mut self, args: RiscvArgs) -> bool {
        if self.cache_enabled {
            self.stop_translating = true;
            self.insert_insn_current(RiscvInstr {
                args,
                inc_by: 0,
                func: interpreter::defs::bltu
            });
        } else {
            interpreter::defs::bltu(self, &args);
        }
        return true;
    }
    fn bgeu(&mut self, args: RiscvArgs) -> bool {
        if self.cache_enabled {
            self.stop_translating = true;
            self.insert_insn_current(RiscvInstr {
                args,
                inc_by: 0,
                func: interpreter::defs::bgeu
            });
        } else {
            interpreter::defs::bgeu(self, &args);
        }
        return true;
    }
    fn lb(&mut self, args: RiscvArgs) -> bool {
        if self.cache_enabled {
            self.insert_insn_current(RiscvInstr {
                args,
                inc_by: 0,
                func: interpreter::defs::lb
            });
        } else {
            interpreter::defs::lb(self, &args);
        }
        return true;
    }
    fn lh(&mut self, args: RiscvArgs) -> bool {
        if self.cache_enabled {
            self.insert_insn_current(RiscvInstr {
                args,
                inc_by: 0,
                func: interpreter::defs::lh
            });
        } else {
            interpreter::defs::lh(self, &args);
        }
        return true;
    }
    fn lw(&mut self, args: RiscvArgs) -> bool {
        if self.cache_enabled {
            self.insert_insn_current(RiscvInstr {
                args,
                inc_by: 0,
                func: interpreter::defs::lw
            });
        } else {
            interpreter::defs::lw(self, &args);
        }
        return true;
    }
    fn lbu(&mut self, args: RiscvArgs) -> bool {
        if self.cache_enabled {
            self.insert_insn_current(RiscvInstr {
                args,
                inc_by: 0,
                func: interpreter::defs::lbu
            });
        } else {
            interpreter::defs::lbu(self, &args);
        }
        return true;
    }
    fn lhu(&mut self, args: RiscvArgs) -> bool {
        if self.cache_enabled {
            self.insert_insn_current(RiscvInstr {
                args,
                inc_by: 0,
                func: interpreter::defs::lhu
            });
        } else {
            interpreter::defs::lhu(self, &args);
        }
        return true;
    }
    fn sb(&mut self, args: RiscvArgs) -> bool {
        if self.cache_enabled {
            self.insert_insn_current(RiscvInstr {
                args,
                inc_by: 0,
                func: interpreter::defs::sb
            });
        } else {
            interpreter::defs::sb(self, &args);
        }
        return true;
    }
    fn sh(&mut self, args: RiscvArgs) -> bool {
        if self.cache_enabled {
            self.insert_insn_current(RiscvInstr {
                args,
                inc_by: 0,
                func: interpreter::defs::sh
            });
        } else {
            interpreter::defs::sh(self, &args);
        }
        return true;
    }
    fn sw(&mut self, args: RiscvArgs) -> bool {
        if self.cache_enabled {
            self.insert_insn_current(RiscvInstr {
                args,
                inc_by: 0,
                func: interpreter::defs::sw
            });
        } else {
            interpreter::defs::sw(self, &args);
        }
        return true;
    }
    fn addi(&mut self, args: RiscvArgs) -> bool {
        if self.cache_enabled {
            self.insert_insn_current(RiscvInstr {
                args,
                inc_by: 0,
                func: interpreter::defs::addi
            });
        } else {
            interpreter::defs::addi(self, &args);
        }
        return true;
    }
    fn slti(&mut self, args: RiscvArgs) -> bool {
        if self.cache_enabled {
            self.insert_insn_current(RiscvInstr {
                args,
                inc_by: 0,
                func: interpreter::defs::slti
            });
        } else {
            interpreter::defs::slti(self, &args);
        }
        return true;
    }
    fn sltiu(&mut self, args: RiscvArgs) -> bool {
        if self.cache_enabled {
            self.insert_insn_current(RiscvInstr {
                args,
                inc_by: 0,
                func: interpreter::defs::sltiu
            });
        } else {
            interpreter::defs::sltiu(self, &args);
        }
        return true;
    }
    fn xori(&mut self, args: RiscvArgs) -> bool {
        if self.cache_enabled {
            self.insert_insn_current(RiscvInstr {
                args,
                inc_by: 0,
                func: interpreter::defs::xori
            });
        } else {
            interpreter::defs::xori(self, &args);
        }
        return true;
    }
    fn ori(&mut self, args: RiscvArgs) -> bool {
        if self.cache_enabled {
            self.insert_insn_current(RiscvInstr {
                args,
                inc_by: 0,
                func: interpreter::defs::ori
            });
        } else {
            interpreter::defs::ori(self, &args);
        }
        return true;
    }
    fn andi(&mut self, args: RiscvArgs) -> bool {
        if self.cache_enabled {
            self.insert_insn_current(RiscvInstr {
                args,
                inc_by: 0,
                func: interpreter::defs::andi
            });
        } else {
            interpreter::defs::andi(self, &args);
        }
        return true;
    }
    fn slli(&mut self, args: RiscvArgs) -> bool {
        if self.cache_enabled {
            self.insert_insn_current(RiscvInstr {
                args,
                inc_by: 0,
                func: interpreter::defs::slli
            });
        } else {
            interpreter::defs::slli(self, &args);
        }
        return true;
    }
    fn srli(&mut self, args: RiscvArgs) -> bool {
        if self.cache_enabled {
            self.insert_insn_current(RiscvInstr {
                args,
                inc_by: 0,
                func: interpreter::defs::srli
            });
        } else {
            interpreter::defs::srli(self, &args);
        }
        return true;
    }
    fn srai(&mut self, args: RiscvArgs) -> bool {
        if self.cache_enabled {
            self.insert_insn_current(RiscvInstr {
                args,
                inc_by: 0,
                func: interpreter::defs::srai
            });
        } else {
            interpreter::defs::srai(self, &args);
        }
        return true;
    }
    fn add(&mut self, args: RiscvArgs) -> bool {
        if self.cache_enabled {
            self.insert_insn_current(RiscvInstr {
                args,
                inc_by: 0,
                func: interpreter::defs::add
            });
        } else {
            interpreter::defs::add(self, &args);
        }
        return true;
    }
    fn sub(&mut self, args: RiscvArgs) -> bool {
        if self.cache_enabled {
            self.insert_insn_current(RiscvInstr {
                args,
                inc_by: 0,
                func: interpreter::defs::sub
            });
        } else {
            interpreter::defs::sub(self, &args);
        }
        return true;
    }
    fn sll(&mut self, args: RiscvArgs) -> bool {
        if self.cache_enabled {
            self.insert_insn_current(RiscvInstr {
                args,
                inc_by: 0,
                func: interpreter::defs::sll
            });
        } else {
            interpreter::defs::sll(self, &args);
        }
        return true;
    }
    fn slt(&mut self, args: RiscvArgs) -> bool {
        if self.cache_enabled {
            self.insert_insn_current(RiscvInstr {
                args,
                inc_by: 0,
                func: interpreter::defs::slt
            });
        } else {
            interpreter::defs::slt(self, &args);
        }
        return true;
    }
    fn sltu(&mut self, args: RiscvArgs) -> bool {
        if self.cache_enabled {
            self.insert_insn_current(RiscvInstr {
                args,
                inc_by: 0,
                func: interpreter::defs::sltu
            });
        } else {
            interpreter::defs::sltu(self, &args);
        }
        return true;
    }
    fn xor(&mut self, args: RiscvArgs) -> bool {
        if self.cache_enabled {
            self.insert_insn_current(RiscvInstr {
                args,
                inc_by: 0,
                func: interpreter::defs::xor
            });
        } else {
            interpreter::defs::xor(self, &args);
        }
        return true;
    }
    fn srl(&mut self, args: RiscvArgs) -> bool {
        if self.cache_enabled {
            self.insert_insn_current(RiscvInstr {
                args,
                inc_by: 0,
                func: interpreter::defs::srl
            });
        } else {
            interpreter::defs::srl(self, &args);
        }
        return true;
    }
    fn sra(&mut self, args: RiscvArgs) -> bool {
        if self.cache_enabled {
            self.insert_insn_current(RiscvInstr {
                args,
                inc_by: 0,
                func: interpreter::defs::sra
            });
        } else {
            interpreter::defs::sra(self, &args);
        }
        return true;
    }

    fn or(&mut self, args: RiscvArgs) -> bool {
        if self.cache_enabled {
            self.insert_insn_current(RiscvInstr {
                args,
                inc_by: 0,
                func: interpreter::defs::or
            });
        } else {
            interpreter::defs::or(self, &args);
        }
        return true;
    }
    fn and(&mut self, args: RiscvArgs) -> bool {
        if self.cache_enabled {
            self.insert_insn_current(RiscvInstr {
                args,
                inc_by: 0,
                func: interpreter::defs::and
            });
        } else {
            interpreter::defs::and(self, &args);
        }
        return true;
    }
    fn fence(&mut self, args: RiscvArgs) -> bool {
        if self.cache_enabled {
            self.insert_insn_current(RiscvInstr {
                args,
                inc_by: 0,
                func: interpreter::defs::fence
            });
        } else {
            interpreter::defs::fence(self, &args);
        }
        return true;
    }
    fn csrrw(&mut self, args: RiscvArgs) -> bool {
        if self.cache_enabled {
            self.stop_translating = true;
            self.insert_insn_current(RiscvInstr {
                args,
                inc_by: 0,
                func: interpreter::defs::csrrw
            });
        } else {
            interpreter::defs::csrrw(self, &args);
        }
        return true;
    }
    fn csrrs(&mut self, args: RiscvArgs) -> bool {
        if self.cache_enabled {
            self.stop_translating = true;
            self.insert_insn_current(RiscvInstr {
                args,
                inc_by: 0,
                func: interpreter::defs::csrrs
            });
        } else {
            interpreter::defs::csrrs(self, &args);
        }
        return true;
    }
    fn csrrc(&mut self, args: RiscvArgs) -> bool {
        if self.cache_enabled {
            self.stop_translating = true;
            self.insert_insn_current(RiscvInstr {
                args,
                inc_by: 0,
                func: interpreter::defs::csrrc
            });
        } else {
            interpreter::defs::csrrc(self, &args);
        }
        return true;
    }
    fn csrrwi(&mut self, args: RiscvArgs) -> bool {
        if self.cache_enabled {
            self.stop_translating = true;
            self.insert_insn_current(RiscvInstr {
                args,
                inc_by: 0,
                func: interpreter::defs::csrrwi
            });
        } else {
            interpreter::defs::csrrwi(self, &args);
        }
        return true;
    }
    fn csrrsi(&mut self, args: RiscvArgs) -> bool {
        if self.cache_enabled {
            self.stop_translating = true;
            self.insert_insn_current(RiscvInstr {
                args,
                inc_by: 0,
                func: interpreter::defs::csrrsi
            });
        } else {
            interpreter::defs::csrrsi(self, &args);
        }
        return true;
    }
    fn csrrci(&mut self, args: RiscvArgs) -> bool {
        if self.cache_enabled {
            self.stop_translating = true;
            self.insert_insn_current(RiscvInstr {
                args,
                inc_by: 0,
                func: interpreter::defs::csrrci
            });
        } else {
            interpreter::defs::csrrci(self, &args);
        }
        return true;
    }
    fn lwu(&mut self, args: RiscvArgs) -> bool {
        if self.cache_enabled {
            self.insert_insn_current(RiscvInstr {
                args,
                inc_by: 0,
                func: interpreter::defs::lwu
            });
        } else {
            interpreter::defs::lwu(self, &args);
        }
        return true;
    }
    fn ld(&mut self, args: RiscvArgs) -> bool {
        if self.cache_enabled {
            self.insert_insn_current(RiscvInstr {
                args,
                inc_by: 0,
                func: interpreter::defs::ld
            });
        } else {
            interpreter::defs::ld(self, &args);
        }
        return true;
    }
    fn sd(&mut self, args: RiscvArgs) -> bool {
        if self.cache_enabled {
            self.insert_insn_current(RiscvInstr {
                args,
                inc_by: 0,
                func: interpreter::defs::sd
            });
        } else {
            interpreter::defs::sd(self, &args);
        }
        return true;
    }
    fn addiw(&mut self, args: RiscvArgs) -> bool {
        if self.cache_enabled {
            self.insert_insn_current(RiscvInstr {
                args,
                inc_by: 0,
                func: interpreter::defs::addiw
            });
        } else {
            interpreter::defs::addiw(self, &args);
        }
        return true;
    }
    fn slliw(&mut self, args: RiscvArgs) -> bool {
        if self.cache_enabled {
            self.insert_insn_current(RiscvInstr {
                args,
                inc_by: 0,
                func: interpreter::defs::slliw
            });
        } else {
            interpreter::defs::slliw(self, &args);
        }
        return true;
    }
    fn srliw(&mut self, args: RiscvArgs) -> bool {
        if self.cache_enabled {
            self.insert_insn_current(RiscvInstr {
                args,
                inc_by: 0,
                func: interpreter::defs::srliw
            });
        } else {
            interpreter::defs::srliw(self, &args);
        }
        return true;
    }
    fn sraiw(&mut self, args: RiscvArgs) -> bool {
        if self.cache_enabled {
            self.insert_insn_current(RiscvInstr {
                args,
                inc_by: 0,
                func: interpreter::defs::sraiw
            });
        } else {
            interpreter::defs::sraiw(self, &args);
        }
        return true;
    }
    fn addw(&mut self, args: RiscvArgs) -> bool {
        if self.cache_enabled {
            self.insert_insn_current(RiscvInstr {
                args,
                inc_by: 0,
                func: interpreter::defs::addw
            });
        } else {
            interpreter::defs::addw(self, &args);
        }
        return true;
    }
    fn subw(&mut self, args: RiscvArgs) -> bool {
        if self.cache_enabled {
            self.insert_insn_current(RiscvInstr {
                args,
                inc_by: 0,
                func: interpreter::defs::subw
            });
        } else {
            interpreter::defs::subw(self, &args);
        }
        return true;
    }
    fn sllw(&mut self, args: RiscvArgs) -> bool {
        if self.cache_enabled {
            self.insert_insn_current(RiscvInstr {
                args,
                inc_by: 0,
                func: interpreter::defs::sllw
            });
        } else {
            interpreter::defs::sllw(self, &args);
        }
        return true;
    }
    fn srlw(&mut self, args: RiscvArgs) -> bool {
        if self.cache_enabled {
            self.insert_insn_current(RiscvInstr {
                args,
                inc_by: 0,
                func: interpreter::defs::srlw
            });
        } else {
            interpreter::defs::srlw(self, &args);
        }
        return true;
    }
    fn sraw(&mut self, args: RiscvArgs) -> bool {
        if self.cache_enabled {
            self.insert_insn_current(RiscvInstr {
                args,
                inc_by: 0,
                func: interpreter::defs::sraw
            });
        } else {
            interpreter::defs::sraw(self, &args);
        }
        return true;
    }
    fn mul(&mut self, args: RiscvArgs) -> bool {
        if self.cache_enabled {
            self.insert_insn_current(RiscvInstr {
                args,
                inc_by: 0,
                func: interpreter::defs::mul
            });
        } else {
            interpreter::defs::mul(self, &args);
        }
        return true;
    }
    fn mulh(&mut self, args: RiscvArgs) -> bool {
        if self.cache_enabled {
            self.insert_insn_current(RiscvInstr {
                args,
                inc_by: 0,
                func: interpreter::defs::mulh
            });
        } else {
            interpreter::defs::mulh(self, &args);
        }
        return true;
    }
    fn mulhsu(&mut self, args: RiscvArgs) -> bool {
        if self.cache_enabled {
            self.insert_insn_current(RiscvInstr {
                args,
                inc_by: 0,
                func: interpreter::defs::mulhsu
            });
        } else {
            interpreter::defs::mulhsu(self, &args);
        }
        return true;
    }
    fn mulhu(&mut self, args: RiscvArgs) -> bool {
        if self.cache_enabled {
            self.insert_insn_current(RiscvInstr {
                args,
                inc_by: 0,
                func: interpreter::defs::mulhu
            });
        } else {
            interpreter::defs::mulhu(self, &args);
        }
        return true;
    }
    fn div(&mut self, args: RiscvArgs) -> bool {
        if self.cache_enabled {
            self.insert_insn_current(RiscvInstr {
                args,
                inc_by: 0,
                func: interpreter::defs::div
            });
        } else {
            interpreter::defs::div(self, &args);
        }
        return true;
    }
    fn divu(&mut self, args: RiscvArgs) -> bool {
        if self.cache_enabled {
            self.insert_insn_current(RiscvInstr {
                args,
                inc_by: 0,
                func: interpreter::defs::divu
            });
        } else {
            interpreter::defs::divu(self, &args);
        }
        return true;
    }
    fn rem(&mut self, args: RiscvArgs) -> bool {
        if self.cache_enabled {
            self.insert_insn_current(RiscvInstr {
                args,
                inc_by: 0,
                func: interpreter::defs::rem
            });
        } else {
            interpreter::defs::rem(self, &args);
        }
        return true;
    }
    fn remu(&mut self, args: RiscvArgs) -> bool {
        if self.cache_enabled {
            self.insert_insn_current(RiscvInstr {
                args,
                inc_by: 0,
                func: interpreter::defs::remu
            });
        } else {
            interpreter::defs::remu(self, &args);
        }
        return true;
    }
    fn mulw(&mut self, args: RiscvArgs) -> bool {
        if self.cache_enabled {
            self.insert_insn_current(RiscvInstr {
                args,
                inc_by: 0,
                func: interpreter::defs::mulw
            });
        } else {
            interpreter::defs::mulw(self, &args);
        }
        return true;
    }
    fn divw(&mut self, args: RiscvArgs) -> bool {
        if self.cache_enabled {
            self.insert_insn_current(RiscvInstr {
                args,
                inc_by: 0,
                func: interpreter::defs::divw
            });
        } else {
            interpreter::defs::divw(self, &args);
        }
        return true;
    }
    fn divuw(&mut self, args: RiscvArgs) -> bool {
        if self.cache_enabled {
            self.insert_insn_current(RiscvInstr {
                args,
                inc_by: 0,
                func: interpreter::defs::divuw
            });
        } else {
            interpreter::defs::divuw(self, &args);
        }
        return true;
    }
    fn remw(&mut self, args: RiscvArgs) -> bool {
        if self.cache_enabled {
            self.insert_insn_current(RiscvInstr {
                args,
                inc_by: 0,
                func: interpreter::defs::remw
            });
        } else {
            interpreter::defs::remw(self, &args);
        }
        return true;
    }
    fn remuw(&mut self, args: RiscvArgs) -> bool {
        if self.cache_enabled {
            self.insert_insn_current(RiscvInstr {
                args,
                inc_by: 0,
                func: interpreter::defs::remuw
            });
        } else {
            interpreter::defs::remuw(self, &args);
        }
        return true;
    }
    fn lr_w(&mut self, args: RiscvArgs) -> bool {
        if self.cache_enabled {
            self.insert_insn_current(RiscvInstr {
                args,
                inc_by: 0,
                func: interpreter::defs::lr_w
            });
        } else {
            interpreter::defs::lr_w(self, &args);
        }
        return true;
    }
    fn sc_w(&mut self, args: RiscvArgs) -> bool {
        if self.cache_enabled {
            self.insert_insn_current(RiscvInstr {
                args,
                inc_by: 0,
                func: interpreter::defs::sc_w
            });
        } else {
            interpreter::defs::sc_w(self, &args);
        }
        return true;
    }
    fn amoswap_w(&mut self, args: RiscvArgs) -> bool {
        if self.cache_enabled {
            self.insert_insn_current(RiscvInstr {
                args,
                inc_by: 0,
                func: interpreter::defs::amoswap_w
            });
        } else {
            interpreter::defs::amoswap_w(self, &args);
        }
        return true;
    }
    fn flw(&mut self, args: RiscvArgs) -> bool {
        if self.cache_enabled {
            self.insert_insn_current(RiscvInstr {
                args,
                inc_by: 0,
                func: interpreter::defs::flw
            });
        } else {
            interpreter::defs::flw(self, &args);
        }
        return true;
    }
    fn fsw(&mut self, args: RiscvArgs) -> bool {
        if self.cache_enabled {
            self.insert_insn_current(RiscvInstr {
                args,
                inc_by: 0,
                func: interpreter::defs::fsw
            });
        } else {
            interpreter::defs::fsw(self, &args);
        }
        return true;
    }
    fn fmadd_s(&mut self, args: RiscvArgs) -> bool {
        if self.cache_enabled {
            self.insert_insn_current(RiscvInstr {
                args,
                inc_by: 0,
                func: interpreter::defs::fmadd_s
            });
        } else {
            interpreter::defs::fmadd_s(self, &args);
        }
        return true;
    }
    fn fmsub_s(&mut self, args: RiscvArgs) -> bool {
        if self.cache_enabled {
            self.insert_insn_current(RiscvInstr {
                args,
                inc_by: 0,
                func: interpreter::defs::fmsub_s
            });
        } else {
            interpreter::defs::fmsub_s(self, &args);
        }
        return true;
    }
    fn fnmsub_s(&mut self, args: RiscvArgs) -> bool {
        if self.cache_enabled {
            self.insert_insn_current(RiscvInstr {
                args,
                inc_by: 0,
                func: interpreter::defs::fnmsub_s
            });
        } else {
            interpreter::defs::fnmsub_s(self, &args);
        }
        return true;
    }
    fn fnmadd_s(&mut self, args: RiscvArgs) -> bool {
        if self.cache_enabled {
            self.insert_insn_current(RiscvInstr {
                args,
                inc_by: 0,
                func: interpreter::defs::fnmadd_s
            });
        } else {
            interpreter::defs::fnmadd_s(self, &args);
        }
        return true;
    }
    fn fadd_s(&mut self, args: RiscvArgs) -> bool {
        if self.cache_enabled {
            self.insert_insn_current(RiscvInstr {
                args,
                inc_by: 0,
                func: interpreter::defs::fadd_s
            });
        } else {
            interpreter::defs::fadd_s(self, &args);
        }
        return true;
    }
    fn fsub_s(&mut self, args: RiscvArgs) -> bool {
        if self.cache_enabled {
            self.insert_insn_current(RiscvInstr {
                args,
                inc_by: 0,
                func: interpreter::defs::fsub_s
            });
        } else {
            interpreter::defs::fsub_s(self, &args);
        }
        return true;
    }
    fn fmul_s(&mut self, args: RiscvArgs) -> bool {
        if self.cache_enabled {
            self.insert_insn_current(RiscvInstr {
                args,
                inc_by: 0,
                func: interpreter::defs::fmul_s
            });
        } else {
            interpreter::defs::fmul_s(self, &args);
        }
        return true;
    }
    fn fdiv_s(&mut self, args: RiscvArgs) -> bool {
        if self.cache_enabled {
            self.insert_insn_current(RiscvInstr {
                args,
                inc_by: 0,
                func: interpreter::defs::fdiv_s
            });
        } else {
            interpreter::defs::fdiv_s(self, &args);
        }
        return true;
    }
    fn fsqrt_s(&mut self, args: RiscvArgs) -> bool {
        if self.cache_enabled {
            self.insert_insn_current(RiscvInstr {
                args,
                inc_by: 0,
                func: interpreter::defs::fsqrt_s
            });
        } else {
            interpreter::defs::fsqrt_s(self, &args);
        }
        return true;
    }
    fn fsgnj_s(&mut self, args: RiscvArgs) -> bool {
        if self.cache_enabled {
            self.insert_insn_current(RiscvInstr {
                args,
                inc_by: 0,
                func: interpreter::defs::fsgnj_s
            });
        } else {
            interpreter::defs::fsgnj_s(self, &args);
        }
        return true;
    }
    fn fsgnjn_s(&mut self, args: RiscvArgs) -> bool {
        if self.cache_enabled {
            self.insert_insn_current(RiscvInstr {
                args,
                inc_by: 0,
                func: interpreter::defs::fsgnjn_s
            });
        } else {
            interpreter::defs::fsgnjn_s(self, &args);
        }
        return true;
    }
    fn fmin_s(&mut self, args: RiscvArgs) -> bool {
        if self.cache_enabled {
            self.insert_insn_current(RiscvInstr {
                args,
                inc_by: 0,
                func: interpreter::defs::fmin_s
            });
        } else {
            interpreter::defs::fmin_s(self, &args);
        }
        return true;
    }
    fn fmax_s(&mut self, args: RiscvArgs) -> bool {
        if self.cache_enabled {
            self.insert_insn_current(RiscvInstr {
                args,
                inc_by: 0,
                func: interpreter::defs::fmax_s
            });
        } else {
            interpreter::defs::fmax_s(self, &args);
        }
        return true;
    }
    fn fcvt_w_s(&mut self, args: RiscvArgs) -> bool {
        if self.cache_enabled {
            self.insert_insn_current(RiscvInstr {
                args,
                inc_by: 0,
                func: interpreter::defs::fcvt_w_s
            });
        } else {
            interpreter::defs::fcvt_w_s(self, &args);
        }
        return true;
    }
    fn fcvt_wu_s(&mut self, args: RiscvArgs) -> bool {
        if self.cache_enabled {
            self.insert_insn_current(RiscvInstr {
                args,
                inc_by: 0,
                func: interpreter::defs::fcvt_wu_s
            });
        } else {
            interpreter::defs::fcvt_wu_s(self, &args);
        }
        return true;
    }
    fn fmv_x_w(&mut self, args: RiscvArgs) -> bool {
        if self.cache_enabled {
            self.insert_insn_current(RiscvInstr {
                args,
                inc_by: 0,
                func: interpreter::defs::fmv_x_w
            });
        } else {
            interpreter::defs::fmv_x_w(self, &args);
        }
        return true;
    }
    fn feq_s(&mut self, args: RiscvArgs) -> bool {
        if self.cache_enabled {
            self.insert_insn_current(RiscvInstr {
                args,
                inc_by: 0,
                func: interpreter::defs::feq_s
            });
        } else {
            interpreter::defs::feq_s(self, &args);
        }
        return true;
    }
    fn flt_s(&mut self, args: RiscvArgs) -> bool {
        if self.cache_enabled {
            self.insert_insn_current(RiscvInstr {
                args,
                inc_by: 0,
                func: interpreter::defs::flt_s
            });
        } else {
            interpreter::defs::flt_s(self, &args);
        }
        return true;
    }
    fn fle_s(&mut self, args: RiscvArgs) -> bool {
        if self.cache_enabled {
            self.insert_insn_current(RiscvInstr {
                args,
                inc_by: 0,
                func: interpreter::defs::fle_s
            });
        } else {
            interpreter::defs::fle_s(self, &args);
        }
        return true;
    }
    fn fcvt_s_w(&mut self, args: RiscvArgs) -> bool {
        if self.cache_enabled {
            self.insert_insn_current(RiscvInstr {
                args,
                inc_by: 0,
                func: interpreter::defs::fcvt_s_w
            });
        } else {
            interpreter::defs::fcvt_s_w(self, &args);
        }
        return true;
    }
    fn fcvt_s_wu(&mut self, args: RiscvArgs) -> bool {
        if self.cache_enabled {
            self.insert_insn_current(RiscvInstr {
                args,
                inc_by: 0,
                func: interpreter::defs::fcvt_s_wu
            });
        } else {
            interpreter::defs::fcvt_s_wu(self, &args);
        }
        return true;
    }
    fn fmv_w_x(&mut self, args: RiscvArgs) -> bool {
        if self.cache_enabled {
            self.insert_insn_current(RiscvInstr {
                args,
                inc_by: 0,
                func: interpreter::defs::fmv_w_x
            });
        } else {
            interpreter::defs::fmv_w_x(self, &args);
        }
        return true;
    }
    fn fcvt_l_s(&mut self, args: RiscvArgs) -> bool {
        if self.cache_enabled {
            self.insert_insn_current(RiscvInstr {
                args,
                inc_by: 0,
                func: interpreter::defs::fcvt_l_s
            });
        } else {
            interpreter::defs::fcvt_l_s(self, &args);
        }
        return true;
    }
    fn fld(&mut self, args: RiscvArgs) -> bool {
        if self.cache_enabled {
            self.insert_insn_current(RiscvInstr {
                args,
                inc_by: 0,
                func: interpreter::defs::fld
            });
        } else {
            interpreter::defs::fld(self, &args);
        }
        return true;
    }
    fn fsd(&mut self, args: RiscvArgs) -> bool {
        if self.cache_enabled {
            self.insert_insn_current(RiscvInstr {
                args,
                inc_by: 0,
                func: interpreter::defs::fsd
            });
        } else {
            interpreter::defs::fsd(self, &args);
        }
        return true;
    }
    fn fmul_d(&mut self, args: RiscvArgs) -> bool {
        if self.cache_enabled {
            self.insert_insn_current(RiscvInstr {
                args,
                inc_by: 0,
                func: interpreter::defs::fmul_d
            });
        } else {
            interpreter::defs::fmul_d(self, &args);
        }
        return true;
    }
    fn fdiv_d(&mut self, args: RiscvArgs) -> bool {
        if self.cache_enabled {
            self.insert_insn_current(RiscvInstr {
                args,
                inc_by: 0,
                func: interpreter::defs::fdiv_d
            });
        } else {
            interpreter::defs::fdiv_d(self, &args);
        }
        return true;
    }
    fn fsgnj_d(&mut self, args: RiscvArgs) -> bool {
        if self.cache_enabled {
            self.insert_insn_current(RiscvInstr {
                args,
                inc_by: 0,
                func: interpreter::defs::fsgnj_d
            });
        } else {
            interpreter::defs::fsgnj_d(self, &args);
        }
        return true;
    }
    fn fsgnjx_d(&mut self, args: RiscvArgs) -> bool {
        if self.cache_enabled {
            self.insert_insn_current(RiscvInstr {
                args,
                inc_by: 0,
                func: interpreter::defs::fsgnjx_d
            });
        } else {
            interpreter::defs::fsgnjx_d(self, &args);
        }
        return true;
    }
    fn fcvt_d_s(&mut self, args: RiscvArgs) -> bool {
        if self.cache_enabled {
            self.insert_insn_current(RiscvInstr {
                args,
                inc_by: 0,
                func: interpreter::defs::fcvt_d_s
            });
        } else {
            interpreter::defs::fcvt_d_s(self, &args);
        }
        return true;
    }
    fn feq_d(&mut self, args: RiscvArgs) -> bool {
        if self.cache_enabled {
            self.insert_insn_current(RiscvInstr {
                args,
                inc_by: 0,
                func: interpreter::defs::feq_d
            });
        } else {
            interpreter::defs::feq_d(self, &args);
        }
        return true;
    }
    fn flt_d(&mut self, args: RiscvArgs) -> bool {
        if self.cache_enabled {
            self.insert_insn_current(RiscvInstr {
                args,
                inc_by: 0,
                func: interpreter::defs::flt_d
            });
        } else {
            interpreter::defs::flt_d(self, &args);
        }
        return true;
    }
    fn fcvt_w_d(&mut self, args: RiscvArgs) -> bool {
        if self.cache_enabled {
            self.insert_insn_current(RiscvInstr {
                args,
                inc_by: 0,
                func: interpreter::defs::fcvt_w_d
            });
        } else {
            interpreter::defs::fcvt_w_d(self, &args);
        }
        return true;
    }
    fn fcvt_l_d(&mut self, args: RiscvArgs) -> bool {
        if self.cache_enabled {
            self.insert_insn_current(RiscvInstr {
                args,
                inc_by: 0,
                func: interpreter::defs::fcvt_l_d
            });
        } else {
            interpreter::defs::fcvt_l_d(self, &args);
        }
        return true;
    }
    fn fmv_x_d(&mut self, args: RiscvArgs) -> bool {
        if self.cache_enabled {
            self.insert_insn_current(RiscvInstr {
                args,
                inc_by: 0,
                func: interpreter::defs::fmv_x_d
            });
        } else {
            interpreter::defs::fmv_x_d(self, &args);
        }
        return true;
    }
    fn fmv_d_x(&mut self, args: RiscvArgs) -> bool {
        if self.cache_enabled {
            self.insert_insn_current(RiscvInstr {
                args,
                inc_by: 0,
                func: interpreter::defs::fmv_d_x
            });
        } else {
            interpreter::defs::fmv_d_x(self, &args);
        }
        return true;
    }
    fn sh1add(&mut self, args: RiscvArgs) -> bool {
        if self.cache_enabled {
            self.insert_insn_current(RiscvInstr {
                args,
                inc_by: 0,
                func: interpreter::defs::sh1add
            });
        } else {
            interpreter::defs::sh1add(self, &args);
        }
        return true;
    }
    fn sh2add(&mut self, args: RiscvArgs) -> bool {
        if self.cache_enabled {
            self.insert_insn_current(RiscvInstr {
                args,
                inc_by: 0,
                func: interpreter::defs::sh2add
            });
        } else {
            interpreter::defs::sh2add(self, &args);
        }
        return true;
    }
    fn sh3add(&mut self, args: RiscvArgs) -> bool {
        if self.cache_enabled {
            self.insert_insn_current(RiscvInstr {
                args,
                inc_by: 0,
                func: interpreter::defs::sh3add
            });
        } else {
            interpreter::defs::sh3add(self, &args);
        }
        return true;
    }
    fn add_uw(&mut self, args: RiscvArgs) -> bool {
        if self.cache_enabled {
            self.insert_insn_current(RiscvInstr {
                args,
                inc_by: 0,
                func: interpreter::defs::add_uw
            });
        } else {
            interpreter::defs::add_uw(self, &args);
        }
        return true;
    }
    fn sh1add_uw(&mut self, args: RiscvArgs) -> bool {
        if self.cache_enabled {
            self.insert_insn_current(RiscvInstr {
                args,
                inc_by: 0,
                func: interpreter::defs::sh1add_uw
            });
        } else {
            interpreter::defs::sh1add_uw(self, &args);
        }
        return true;
    }
    fn sh2add_uw(&mut self, args: RiscvArgs) -> bool {
        if self.cache_enabled {
            self.insert_insn_current(RiscvInstr {
                args,
                inc_by: 0,
                func: interpreter::defs::sh2add_uw
            });
        } else {
            interpreter::defs::sh2add_uw(self, &args);
        }
        return true;
    }
    fn sh3add_uw(&mut self, args: RiscvArgs) -> bool {
        if self.cache_enabled {
            self.insert_insn_current(RiscvInstr {
                args,
                inc_by: 0,
                func: interpreter::defs::sh3add_uw
            });
        } else {
            interpreter::defs::sh3add_uw(self, &args);
        }
        return true;
    }
    fn andn(&mut self, args: RiscvArgs) -> bool {
        if self.cache_enabled {
            self.insert_insn_current(RiscvInstr {
                args,
                inc_by: 0,
                func: interpreter::defs::andn
            });
        } else {
            interpreter::defs::andn(self, &args);
        }
        return true;
    }
    fn rol(&mut self, args: RiscvArgs) -> bool {
        if self.cache_enabled {
            self.insert_insn_current(RiscvInstr {
                args,
                inc_by: 0,
                func: interpreter::defs::rol
            });
        } else {
            interpreter::defs::rol(self, &args);
        }
        return true;
    }
    fn ror(&mut self, args: RiscvArgs) -> bool {
        if self.cache_enabled {
            self.insert_insn_current(RiscvInstr {
                args,
                inc_by: 0,
                func: interpreter::defs::ror
            });
        } else {
            interpreter::defs::ror(self, &args);
        }
        return true;
    }
    fn rori(&mut self, args: RiscvArgs) -> bool {
        if self.cache_enabled {
            self.insert_insn_current(RiscvInstr {
                args,
                inc_by: 0,
                func: interpreter::defs::rori
            });
        } else {
            interpreter::defs::rori(self, &args);
        }
        return true;
    }
    fn zext_h_32(&mut self, args: RiscvArgs) -> bool {
        if self.cache_enabled {
            self.insert_insn_current(RiscvInstr {
                args,
                inc_by: 0,
                func: interpreter::defs::zext_h_32
            });
        } else {
            interpreter::defs::zext_h_32(self, &args);
        }
        return true;
    }
    fn xnor(&mut self, args: RiscvArgs) -> bool {
        if self.cache_enabled {
            self.insert_insn_current(RiscvInstr {
                args,
                inc_by: 0,
                func: interpreter::defs::xnor
            });
        } else {
            interpreter::defs::xnor(self, &args);
        }
        return true;
    }
    fn clz(&mut self, args: RiscvArgs) -> bool {
        if self.cache_enabled {
            self.insert_insn_current(RiscvInstr {
                args,
                inc_by: 0,
                func: interpreter::defs::clz
            });
        } else {
            interpreter::defs::clz(self, &args);
        }
        return true;
    }
    fn cpop(&mut self, args: RiscvArgs) -> bool {
        if self.cache_enabled {
            self.insert_insn_current(RiscvInstr {
                args,
                inc_by: 0,
                func: interpreter::defs::cpop
            });
        } else {
            interpreter::defs::cpop(self, &args);
        }
        return true;
    }
    fn ctz(&mut self, args: RiscvArgs) -> bool {
        if self.cache_enabled {
            self.insert_insn_current(RiscvInstr {
                args,
                inc_by: 0,
                func: interpreter::defs::ctz
            });
        } else {
            interpreter::defs::ctz(self, &args);
        }
        return true;
    }
    fn max(&mut self, args: RiscvArgs) -> bool {
        if self.cache_enabled {
            self.insert_insn_current(RiscvInstr {
                args,
                inc_by: 0,
                func: interpreter::defs::max
            });
        } else {
            interpreter::defs::max(self, &args);
        }
        return true;
    }
    fn maxu(&mut self, args: RiscvArgs) -> bool {
        if self.cache_enabled {
            self.insert_insn_current(RiscvInstr {
                args,
                inc_by: 0,
                func: interpreter::defs::maxu
            });
        } else {
            interpreter::defs::maxu(self, &args);
        }
        return true;
    }
    fn min(&mut self, args: RiscvArgs) -> bool {
        if self.cache_enabled {
            self.insert_insn_current(RiscvInstr {
                args,
                inc_by: 0,
                func: interpreter::defs::min
            });
        } else {
            interpreter::defs::min(self, &args);
        }
        return true;
    }
    fn minu(&mut self, args: RiscvArgs) -> bool {
        if self.cache_enabled {
            self.insert_insn_current(RiscvInstr {
                args,
                inc_by: 0,
                func: interpreter::defs::minu
            });
        } else {
            interpreter::defs::minu(self, &args);
        }
        return true;
    }
    fn orc_b(&mut self, args: RiscvArgs) -> bool {
        if self.cache_enabled {
            self.insert_insn_current(RiscvInstr {
                args,
                inc_by: 0,
                func: interpreter::defs::orc_b
            });
        } else {
            interpreter::defs::orc_b(self, &args);
        }
        return true;
    }
    fn orn(&mut self, args: RiscvArgs) -> bool {
        if self.cache_enabled {
            self.insert_insn_current(RiscvInstr {
                args,
                inc_by: 0,
                func: interpreter::defs::orn
            });
        } else {
            interpreter::defs::orn(self, &args);
        }
        return true;
    }
    fn sext_b(&mut self, args: RiscvArgs) -> bool {
        if self.cache_enabled {
            self.insert_insn_current(RiscvInstr {
                args,
                inc_by: 0,
                func: interpreter::defs::sext_b
            });
        } else {
            interpreter::defs::sext_b(self, &args);
        }
        return true;
    }
    fn sext_h(&mut self, args: RiscvArgs) -> bool {
        if self.cache_enabled {
            self.insert_insn_current(RiscvInstr {
                args,
                inc_by: 0,
                func: interpreter::defs::sext_h
            });
        } else {
            interpreter::defs::sext_h(self, &args);
        }
        return true;
    }
    fn rolw(&mut self, args: RiscvArgs) -> bool {
        if self.cache_enabled {
            self.insert_insn_current(RiscvInstr {
                args,
                inc_by: 0,
                func: interpreter::defs::rolw
            });
        } else {
            interpreter::defs::rolw(self, &args);
        }
        return true;
    }
    fn roriw(&mut self, args: RiscvArgs) -> bool {
        if self.cache_enabled {
            self.insert_insn_current(RiscvInstr {
                args,
                inc_by: 0,
                func: interpreter::defs::roriw
            });
        } else {
            interpreter::defs::roriw(self, &args);
        }
        return true;
    }
    fn rorw(&mut self, args: RiscvArgs) -> bool {
        if self.cache_enabled {
            self.insert_insn_current(RiscvInstr {
                args,
                inc_by: 0,
                func: interpreter::defs::rorw
            });
        } else {
            interpreter::defs::rorw(self, &args);
        }
        return true;
    }
    fn zext_h_64(&mut self, args: RiscvArgs) -> bool {
        if self.cache_enabled {
            self.insert_insn_current(RiscvInstr {
                args,
                inc_by: 0,
                func: interpreter::defs::zext_h_64
            });
        } else {
            interpreter::defs::zext_h_64(self, &args);
        }
        return true;
    }
    fn clzw(&mut self, args: RiscvArgs) -> bool {
        if self.cache_enabled {
            self.insert_insn_current(RiscvInstr {
                args,
                inc_by: 0,
                func: interpreter::defs::clzw
            });
        } else {
            interpreter::defs::clzw(self, &args);
        }
        return true;
    }
    fn ctzw(&mut self, args: RiscvArgs) -> bool {
        if self.cache_enabled {
            self.insert_insn_current(RiscvInstr {
                args,
                inc_by: 0,
                func: interpreter::defs::ctzw
            });
        } else {
            interpreter::defs::ctzw(self, &args);
        }
        return true;
    }
    fn cpopw(&mut self, args: RiscvArgs) -> bool {
        if self.cache_enabled {
            self.insert_insn_current(RiscvInstr {
                args,
                inc_by: 0,
                func: interpreter::defs::cpopw
            });
        } else {
            interpreter::defs::cpopw(self, &args);
        }
        return true;
    }
    fn clmul(&mut self, args: RiscvArgs) -> bool {
        if self.cache_enabled {
            self.insert_insn_current(RiscvInstr {
                args,
                inc_by: 0,
                func: interpreter::defs::clmul
            });
        } else {
            interpreter::defs::clmul(self, &args);
        }
        return true;
    }
    fn clmulh(&mut self, args: RiscvArgs) -> bool {
        if self.cache_enabled {
            self.insert_insn_current(RiscvInstr {
                args,
                inc_by: 0,
                func: interpreter::defs::clmulh
            });
        } else {
            interpreter::defs::clmulh(self, &args);
        }
        return true;
    }
    fn clmulr(&mut self, args: RiscvArgs) -> bool {
        if self.cache_enabled {
            self.insert_insn_current(RiscvInstr {
                args,
                inc_by: 0,
                func: interpreter::defs::clmulr
            });
        } else {
            interpreter::defs::clmulr(self, &args);
        }
        return true;
    }
    fn bclr(&mut self, args: RiscvArgs) -> bool {
        if self.cache_enabled {
            self.insert_insn_current(RiscvInstr {
                args,
                inc_by: 0,
                func: interpreter::defs::bclr
            });
        } else {
            interpreter::defs::bclr(self, &args);
        }
        return true;
    }
    fn bclri(&mut self, args: RiscvArgs) -> bool {
        if self.cache_enabled {
            self.insert_insn_current(RiscvInstr {
                args,
                inc_by: 0,
                func: interpreter::defs::bclri
            });
        } else {
            interpreter::defs::bclri(self, &args);
        }
        return true;
    }
    fn bext(&mut self, args: RiscvArgs) -> bool {
        if self.cache_enabled {
            self.insert_insn_current(RiscvInstr {
                args,
                inc_by: 0,
                func: interpreter::defs::bext
            });
        } else {
            interpreter::defs::bext(self, &args);
        }
        return true;
    }
    fn bexti(&mut self, args: RiscvArgs) -> bool {
        if self.cache_enabled {
            self.insert_insn_current(RiscvInstr {
                args,
                inc_by: 0,
                func: interpreter::defs::bexti
            });
        } else {
            interpreter::defs::bexti(self, &args);
        }
        return true;
    }
    fn binv(&mut self, args: RiscvArgs) -> bool {
        if self.cache_enabled {
            self.insert_insn_current(RiscvInstr {
                args,
                inc_by: 0,
                func: interpreter::defs::binv
            });
        } else {
            interpreter::defs::binv(self, &args);
        }
        return true;
    }
    fn binvi(&mut self, args: RiscvArgs) -> bool {
        if self.cache_enabled {
            self.insert_insn_current(RiscvInstr {
                args,
                inc_by: 0,
                func: interpreter::defs::binvi
            });
        } else {
            interpreter::defs::binvi(self, &args);
        }
        return true;
    }
    fn bset(&mut self, args: RiscvArgs) -> bool {
        if self.cache_enabled {
            self.insert_insn_current(RiscvInstr {
                args,
                inc_by: 0,
                func: interpreter::defs::bset
            });
        } else {
            interpreter::defs::bset(self, &args);
        }
        return true;
    }
    fn bseti(&mut self, args: RiscvArgs) -> bool {
        if self.cache_enabled {
            self.insert_insn_current(RiscvInstr {
                args,
                inc_by: 0,
                func: interpreter::defs::bseti
            });
        } else {
            interpreter::defs::bseti(self, &args);
        }
        return true;
    }
    fn aes32dsmi(&mut self, args: RiscvArgs) -> bool {
        if self.cache_enabled {
            self.insert_insn_current(RiscvInstr {
                args,
                inc_by: 0,
                func: interpreter::defs::aes32dsmi
            });
        } else {
            interpreter::defs::aes32dsmi(self, &args);
        }
        return true;
    }
    fn aes32dsi(&mut self, args: RiscvArgs) -> bool {
        if self.cache_enabled {
            self.insert_insn_current(RiscvInstr {
                args,
                inc_by: 0,
                func: interpreter::defs::aes32dsi
            });
        } else {
            interpreter::defs::aes32dsi(self, &args);
        }
        return true;
    }
    fn aes64dsm(&mut self, args: RiscvArgs) -> bool {
        if self.cache_enabled {
            self.insert_insn_current(RiscvInstr {
                args,
                inc_by: 0,
                func: interpreter::defs::aes64dsm
            });
        } else {
            interpreter::defs::aes64dsm(self, &args);
        }
        return true;
    }
    fn aes64ds(&mut self, args: RiscvArgs) -> bool {
        if self.cache_enabled {
            self.insert_insn_current(RiscvInstr {
                args,
                inc_by: 0,
                func: interpreter::defs::aes64ds
            });
        } else {
            interpreter::defs::aes64ds(self, &args);
        }
        return true;
    }
    fn aes64im(&mut self, args: RiscvArgs) -> bool {
        if self.cache_enabled {
            self.insert_insn_current(RiscvInstr {
                args,
                inc_by: 0,
                func: interpreter::defs::aes64im
            });
        } else {
            interpreter::defs::aes64im(self, &args);
        }
        return true;
    }
    fn aes32esmi(&mut self, args: RiscvArgs) -> bool {
        if self.cache_enabled {
            self.insert_insn_current(RiscvInstr {
                args,
                inc_by: 0,
                func: interpreter::defs::aes32esmi
            });
        } else {
            interpreter::defs::aes32esmi(self, &args);
        }
        return true;
    }
    fn aes32esi(&mut self, args: RiscvArgs) -> bool {
        if self.cache_enabled {
            self.insert_insn_current(RiscvInstr {
                args,
                inc_by: 0,
                func: interpreter::defs::aes32esi
            });
        } else {
            interpreter::defs::aes32esi(self, &args);
        }
        return true;
    }
    fn aes64ks2(&mut self, args: RiscvArgs) -> bool {
        if self.cache_enabled {
            self.insert_insn_current(RiscvInstr {
                args,
                inc_by: 0,
                func: interpreter::defs::aes64ks2
            });
        } else {
            interpreter::defs::aes64ks2(self, &args);
        }
        return true;
    }
    fn aes64ks1i(&mut self, args: RiscvArgs) -> bool {
        if self.cache_enabled {
            self.insert_insn_current(RiscvInstr {
                args,
                inc_by: 0,
                func: interpreter::defs::aes64ks1i
            });
        } else {
            interpreter::defs::aes64ks1i(self, &args);
        }
        return true;
    }
    fn sha256sig0(&mut self, args: RiscvArgs) -> bool {
        if self.cache_enabled {
            self.insert_insn_current(RiscvInstr {
                args,
                inc_by: 0,
                func: interpreter::defs::sha256sig0
            });
        } else {
            interpreter::defs::sha256sig0(self, &args);
        }
        return true;
    }
    fn sha256sig1(&mut self, args: RiscvArgs) -> bool {
        if self.cache_enabled {
            self.insert_insn_current(RiscvInstr {
                args,
                inc_by: 0,
                func: interpreter::defs::sha256sig1
            });
        } else {
            interpreter::defs::sha256sig1(self, &args);
        }
        return true;
    }
    fn sha256sum0(&mut self, args: RiscvArgs) -> bool {
        if self.cache_enabled {
            self.insert_insn_current(RiscvInstr {
                args,
                inc_by: 0,
                func: interpreter::defs::sha256sum0
            });
        } else {
            interpreter::defs::sha256sum0(self, &args);
        }
        return true;
    }
    fn sha256sum1(&mut self, args: RiscvArgs) -> bool {
        if self.cache_enabled {
            self.insert_insn_current(RiscvInstr {
                args,
                inc_by: 0,
                func: interpreter::defs::sha256sum1
            });
        } else {
            interpreter::defs::sha256sum1(self, &args);
        }
        return true;
    }
    fn sha512sum0r(&mut self, args: RiscvArgs) -> bool {
        if self.cache_enabled {
            self.insert_insn_current(RiscvInstr {
                args,
                inc_by: 0,
                func: interpreter::defs::sha512sum0r
            });
        } else {
            interpreter::defs::sha512sum0r(self, &args);
        }
        return true;
    }
    fn sha512sum1r(&mut self, args: RiscvArgs) -> bool {
        if self.cache_enabled {
            self.insert_insn_current(RiscvInstr {
                args,
                inc_by: 0,
                func: interpreter::defs::sha512sum1r
            });
        } else {
            interpreter::defs::sha512sum1r(self, &args);
        }
        return true;
    }
    fn sha512sig0l(&mut self, args: RiscvArgs) -> bool {
        if self.cache_enabled {
            self.insert_insn_current(RiscvInstr {
                args,
                inc_by: 0,
                func: interpreter::defs::sha512sig0l
            });
        } else {
            interpreter::defs::sha512sig0l(self, &args);
        }
        return true;
    }
    fn sha512sig0h(&mut self, args: RiscvArgs) -> bool {
        if self.cache_enabled {
            self.insert_insn_current(RiscvInstr {
                args,
                inc_by: 0,
                func: interpreter::defs::sha512sig0h
            });
        } else {
            interpreter::defs::sha512sig0h(self, &args);
        }
        return true;
    }
    fn sha512sig1l(&mut self, args: RiscvArgs) -> bool {
        if self.cache_enabled {
            self.insert_insn_current(RiscvInstr {
                args,
                inc_by: 0,
                func: interpreter::defs::sha512sig1l
            });
        } else {
            interpreter::defs::sha512sig1l(self, &args);
        }
        return true;
    }
    fn sha512sig1h(&mut self, args: RiscvArgs) -> bool {
        if self.cache_enabled {
            self.insert_insn_current(RiscvInstr {
                args,
                inc_by: 0,
                func: interpreter::defs::sha512sig1h
            });
        } else {
            interpreter::defs::sha512sig1h(self, &args);
        }
        return true;
    }
    fn sha512sig0(&mut self, args: RiscvArgs) -> bool {
        if self.cache_enabled {
            self.insert_insn_current(RiscvInstr {
                args,
                inc_by: 0,
                func: interpreter::defs::sha512sig0
            });
        } else {
            interpreter::defs::sha512sig0(self, &args);
        }
        return true;
    }
    fn sha512sig1(&mut self, args: RiscvArgs) -> bool {
        if self.cache_enabled {
            self.insert_insn_current(RiscvInstr {
                args,
                inc_by: 0,
                func: interpreter::defs::sha512sig1
            });
        } else {
            interpreter::defs::sha512sig1(self, &args);
        }
        return true;
    }
    fn sha512sum0(&mut self, args: RiscvArgs) -> bool {
        if self.cache_enabled {
            self.insert_insn_current(RiscvInstr {
                args,
                inc_by: 0,
                func: interpreter::defs::sha512sum0
            });
        } else {
            interpreter::defs::sha512sum0(self, &args);
        }
        return true;
    }
    fn sha512sum1(&mut self, args: RiscvArgs) -> bool {
        if self.cache_enabled {
            self.insert_insn_current(RiscvInstr {
                args,
                inc_by: 0,
                func: interpreter::defs::sha512sum1
            });
        } else {
            interpreter::defs::sha512sum1(self, &args);
        }
        return true;
    }
    fn sm3p0(&mut self, args: RiscvArgs) -> bool {
        if self.cache_enabled {
            self.insert_insn_current(RiscvInstr {
                args,
                inc_by: 0,
                func: interpreter::defs::sm3p0
            });
        } else {
            interpreter::defs::sm3p0(self, &args);
        }
        return true;
    }
    fn sm3p1(&mut self, args: RiscvArgs) -> bool {
        if self.cache_enabled {
            self.insert_insn_current(RiscvInstr {
                args,
                inc_by: 0,
                func: interpreter::defs::sm3p1
            });
        } else {
            interpreter::defs::sm3p1(self, &args);
        }
        return true;
    }
    fn sm4ed(&mut self, args: RiscvArgs) -> bool {
        if self.cache_enabled {
            self.insert_insn_current(RiscvInstr {
                args,
                inc_by: 0,
                func: interpreter::defs::sm4ed
            });
        } else {
            interpreter::defs::sm4ed(self, &args);
        }
        return true;
    }
    fn sm4ks(&mut self, args: RiscvArgs) -> bool {
        if self.cache_enabled {
            self.insert_insn_current(RiscvInstr {
                args,
                inc_by: 0,
                func: interpreter::defs::sm4ks
            });
        } else {
            interpreter::defs::sm4ks(self, &args);
        }
        return true;
    }
    fn lr_d(&mut self, args: RiscvArgs) -> bool {
        if self.cache_enabled {
            self.insert_insn_current(RiscvInstr {
                args,
                inc_by: 0,
                func: interpreter::defs::lr_d
            });
        } else {
            interpreter::defs::lr_d(self, &args);
        }
        return true;
    }
    fn sc_d(&mut self, args: RiscvArgs) -> bool {
        if self.cache_enabled {
            self.insert_insn_current(RiscvInstr {
                args,
                inc_by: 0,
                func: interpreter::defs::sc_d
            });
        } else {
            interpreter::defs::sc_d(self, &args);
        }
        return true;
    }
}