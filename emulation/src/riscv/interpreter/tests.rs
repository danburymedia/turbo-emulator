use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use vm_memory::GuestAddress;
use crate::riscv::common::{DRAM_BASE, Trap, Xlen};
use crate::riscv::interpreter::main::RiscvInt;
use kernel_loader::*;
use crate::common::memory::MemEndian;
use std::result::Result;
impl RiscvInt {
    fn run_test(&mut self, to_host: u64) -> Result<u32, u32> {
        loop {
            self.exec_once(); // traps are already set
            self.regs[0] = 0;
            let val = self.memsource.guest_mem.read_phys_32(to_host, MemEndian::Little);
            match val {
                0 => {},
                1 => return Ok(1),
                n => return Err(n),
            }
            if self.trap.is_some() {
                if self.usermode {
                    panic!("Protection error  - Suffered RISCV trap in user mode: {:?}", self.trap.unwrap())
                } else {
                    self.handle_trap(self.trap.unwrap(), self.trap_pc);
                    self.trap_pc = 0;
                    self.trap = None;
                    self.want_pc = None;
                    self.wfi = false;
                    self.stop_exec = false;
                    continue;
                }

            }
            if let Some(f) = self.want_pc {
                self.pc = f;
                self.want_pc = None;
            }
            if self.wfi {
                unimplemented!();
            }
            self.stop_exec = false;
        }
    }
    fn exec_once(&mut self) -> Result<(), Trap> {
        let instr = self.read32(self.pc, true, true)?;
        if (instr & 0x3) != 0x3 {
                // compressed
            self.is_compressed = true;
            if !crate::riscv::decoder16::decode(self, instr as u16) {
                self.illegal_instr();
            }
            self.pc += 2;
        } else {
            self.is_compressed = false;
            if !crate::riscv::decoder::decode(self, instr) {
                self.illegal_instr(); // this will set stop_exec = true
            }
            self.pc += 4;
        }
        Ok(())
    }
}
fn cint_init_test(fs: &'static str) -> u32 {
    let mut root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    root.push("testrom/riscv");
    root.push(fs);
    let mut f = File::open(root).unwrap();
    let mut data = Vec::new();
    f.read_to_end(&mut data).unwrap();
    let ef = goblin::elf::Elf::parse(&data).unwrap();
    let vmmem = vm_memory::GuestMemory::new(&[(GuestAddress(DRAM_BASE), 512 * 1024)]).unwrap();
    let lk = load_elf(&vmmem, GuestAddress(DRAM_BASE), &mut f).unwrap();
    let mut to_host: Option<u64> = None;
    for sect in &ef.section_headers {
        let s = ef.shdr_strtab.get_at(sect.sh_name);
        if let Some(st) = s {
            if st == ".tohost" {
                to_host = Some(sect.sh_addr);
                break;
            }
        } else {
            continue;
        }
    }
    if to_host.is_none() {
        panic!("No to_host section");
    }
    let is64bit: bool = if ef.is_64 {
        true
    } else {
        false
    };
    let mut rcpu = RiscvInt::init_systemmode(if is64bit {Xlen::X64} else {Xlen::X32}, vmmem.clone());
    rcpu.cache_enabled = true;
    rcpu.pc = ef.entry;
    let res = match rcpu.run_test(to_host.unwrap()) {
        Ok(z) => {
            z
        },
        Err(z) => {
            z
        }
    };
    return res;
}
fn init_test(fs: &'static str) -> u32 {
    let mut root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    root.push("testrom/riscv");
    root.push(fs);
    let mut f = File::open(root).unwrap();
    let mut data = Vec::new();
    f.read_to_end(&mut data).unwrap();
    let ef = goblin::elf::Elf::parse(&data).unwrap();
    let vmmem = vm_memory::GuestMemory::new(&[(GuestAddress(DRAM_BASE), 512 * 1024)]).unwrap();
    let lk = load_elf(&vmmem, GuestAddress(DRAM_BASE), &mut f).unwrap();
    let mut to_host: Option<u64> = None;
    for sect in &ef.section_headers {
        let s = ef.shdr_strtab.get_at(sect.sh_name);
        if let Some(st) = s {
            if st == ".tohost" {
                to_host = Some(sect.sh_addr);
                break;
            }
        } else {
            continue;
        }
    }
    if to_host.is_none() {
        panic!("No to_host section");
    }
    let is64bit: bool = if ef.is_64 {
        true
    } else {
        false
    };
    let mut rcpu = RiscvInt::init_systemmode(if is64bit {Xlen::X64} else {Xlen::X32}, vmmem.clone());
    rcpu.pc = ef.entry;
   // let res = match rcpu.run_test(to_host.unwrap()) {
    let res = match RiscvInt::run_test(&mut rcpu, to_host.unwrap()) {
        Ok(z) => {
            z
        },
        Err(z) => {
            z
        }
    };
    return res;



}
mod test {
    use crate::riscv::interpreter::tests::init_test;
    // integer, 32-bit mode, physical addressing
    #[test]
    fn rv32ui_p_add() {
        assert_eq!(1, init_test("rv32ui-p-add"));
    }
    #[test]
    fn rv32ui_v_add() {
        assert_eq!(1, init_test("rv32ui-v-add"));
    }
    #[test]
    fn rv32ui_p_addi() {
        assert_eq!(1, init_test("rv32ui-p-addi"));
    }

    #[test]
    fn rv32ui_p_and() {
        assert_eq!(1, init_test("rv32ui-p-and"));
    }
    #[test]
    fn rv32ui_p_andi() {
        assert_eq!(1, init_test("rv32ui-p-andi"));
    }
    #[test]
    fn rv32ui_p_auipc() {
        assert_eq!(1, init_test("rv32ui-p-auipc"));
    }
    #[test]
    fn rv32ui_p_beq() {
        assert_eq!(1, init_test("rv32ui-p-beq"));
    }
    #[test]
    fn rv32ui_p_bge() {
        assert_eq!(1, init_test("rv32ui-p-bge"));
    }
    #[test]
    fn rv32ui_p_bgeu() {
        assert_eq!(1, init_test("rv32ui-p-bgeu"));
    }
    #[test]
    fn rv32ui_p_blt() {
        assert_eq!(1, init_test("rv32ui-p-blt"));
    }
    #[test]
    fn rv32ui_p_bltu() {
        assert_eq!(1, init_test("rv32ui-p-bltu"));
    }
    #[test]
    fn rv32ui_p_bne() {
        assert_eq!(1, init_test("rv32ui-p-bne"));
    }
    #[test]
    fn rv32ui_p_jal() {
        assert_eq!(1, init_test("rv32ui-p-jal"));
    }
    #[test]
    fn rv32ui_p_jalr() {
        assert_eq!(1, init_test("rv32ui-p-jalr"));
    }
    #[test]
    fn rv32ui_p_lb() {
        assert_eq!(1, init_test("rv32ui-p-lb"));
    }
    #[test]
    fn rv32ui_p_lbu() {
        assert_eq!(1, init_test("rv32ui-p-lbu"));
    }
    #[test]
    fn rv32ui_p_lh() {
        assert_eq!(1, init_test("rv32ui-p-lh"));
    }
    #[test]
    fn rv32ui_p_lhu() {
        assert_eq!(1, init_test("rv32ui-p-lhu"));
    }
    #[test]
    fn rv32ui_p_lui() {
        assert_eq!(1, init_test("rv32ui-p-lui"));
    }
    #[test]
    fn rv32ui_p_lw() {
        assert_eq!(1, init_test("rv32ui-p-lw"));
    }
    #[test]
    fn rv32ui_p_or() {
        assert_eq!(1, init_test("rv32ui-p-or"));
    }
    #[test]
    fn rv32ui_p_ori() {
        assert_eq!(1, init_test("rv32ui-p-ori"));
    }
    #[test]
    fn rv32ui_p_sb() {
        assert_eq!(1, init_test("rv32ui-p-sb"));
    }
    #[test]
    fn rv32ui_p_sh() {
        assert_eq!(1, init_test("rv32ui-p-sh"));
    }
    #[test]
    fn rv32ui_p_simple() {
        assert_eq!(1, init_test("rv32ui-p-simple"));
    }
    #[test]
    fn rv32ui_p_sll() {
        assert_eq!(1, init_test("rv32ui-p-sll"));
    }
    #[test]
    fn rv32ui_p_slli() {
        assert_eq!(1, init_test("rv32ui-p-slli"));
    }
    #[test]
    fn rv32ui_p_slt() {
        assert_eq!(1, init_test("rv32ui-p-slt"));
    }
    #[test]
    fn rv32ui_p_slti() {
        assert_eq!(1, init_test("rv32ui-p-slti"));
    }
    #[test]
    fn rv32ui_p_sltiu() {
        assert_eq!(1, init_test("rv32ui-p-sltiu"));
    }
    #[test]
    fn rv32ui_p_sltu() {
        assert_eq!(1, init_test("rv32ui-p-sltu"));
    }
    #[test]
    fn rv32ui_p_sra() {
        assert_eq!(1, init_test("rv32ui-p-sra"));
    }
    #[test]
    fn rv32ui_p_srai() {
        assert_eq!(1, init_test("rv32ui-p-srai"));
    }
    #[test]
    fn rv32ui_p_srl() {
        assert_eq!(1, init_test("rv32ui-p-srl"));
    }
    #[test]
    fn rv32ui_p_srli() {
        assert_eq!(1, init_test("rv32ui-p-srli"));
    }

    #[test]
    fn rv32ui_p_sub() {
        assert_eq!(1, init_test("rv32ui-p-sub"));
    }
    #[test]
    fn rv32ui_p_sw() {
        assert_eq!(1, init_test("rv32ui-p-sw"));
    }
    #[test]
    fn rv32ui_p_xor() {
        assert_eq!(1, init_test("rv32ui-p-xor"));
    }
    #[test]
    fn rv32ui_p_xori() {
        assert_eq!(1, init_test("rv32ui-p-xori"));
    }
    // integer, 64-bit mode, physical addressing

    #[test]
    fn rv64ui_p_add() {
        assert_eq!(1, init_test("rv64ui-p-add"));
    }
    #[test]
    fn rv64ui_p_addi() {
        assert_eq!(1, init_test("rv64ui-p-addi"));
    }
    #[test]
    fn rv64ui_p_addiw() {
        assert_eq!(1, init_test("rv64ui-p-addiw"));
    }
    #[test]
    fn rv64ui_p_and() {
        assert_eq!(1, init_test("rv64ui-p-and"));
    }
    #[test]
    fn rv64ui_p_andi() {
        assert_eq!(1, init_test("rv64ui-p-andi"));
    }
    #[test]
    fn rv64ui_p_auipc() {
        assert_eq!(1, init_test("rv64ui-p-auipc"));
    }
    #[test]
    fn rv64ui_p_beq() {
        assert_eq!(1, init_test("rv64ui-p-beq"));
    }
    #[test]
    fn rv64ui_p_bge() {
        assert_eq!(1, init_test("rv64ui-p-bge"));
    }
    #[test]
    fn rv64ui_p_bgeu() {
        assert_eq!(1, init_test("rv64ui-p-bgeu"));
    }
    #[test]
    fn rv64ui_p_blt() {
        assert_eq!(1, init_test("rv64ui-p-blt"));
    }
    #[test]
    fn rv64ui_p_bltu() {
        assert_eq!(1, init_test("rv64ui-p-bltu"));
    }
    #[test]
    fn rv64ui_p_bne() {
        assert_eq!(1, init_test("rv64ui-p-bne"));
    }
    #[test]
    fn rv64ui_p_sllw() {
        assert_eq!(1, init_test("rv64ui-p-sllw"));
    }
    #[test]
    fn rv64ui_p_srlw() {
        assert_eq!(1, init_test("rv64ui-p-srlw"));
    }
    #[test]
    fn rv64ui_p_subw() {
        assert_eq!(1, init_test("rv64ui-p-subw"));
    }
    #[test]
    fn rv64ui_p_jal() {
        assert_eq!(1, init_test("rv64ui-p-jal"));
    }
    #[test]
    fn rv64ui_p_jalr() {
        assert_eq!(1, init_test("rv64ui-p-jalr"));
    }
    #[test]
    fn rv64ui_p_lb() {
        assert_eq!(1, init_test("rv64ui-p-lb"));
    }
    #[test]
    fn rv64ui_p_lbu() {
        assert_eq!(1, init_test("rv64ui-p-lbu"));
    }
    #[test]
    fn rv64ui_p_lh() {
        assert_eq!(1, init_test("rv64ui-p-lh"));
    }
    #[test]
    fn rv64ui_p_lhu() {
        assert_eq!(1, init_test("rv64ui-p-lhu"));
    }
    #[test]
    fn rv64ui_p_lwu() {
        assert_eq!(1, init_test("rv64ui-p-lwu"));
    }
    #[test]
    fn rv64ui_p_lui() {
        assert_eq!(1, init_test("rv64ui-p-lui"));
    }
    #[test]
    fn rv64ui_p_lw() {
        assert_eq!(1, init_test("rv64ui-p-lw"));
    }
    #[test]
    fn rv64ui_p_or() {
        assert_eq!(1, init_test("rv64ui-p-or"));
    }
    #[test]
    fn rv64ui_p_ori() {
        assert_eq!(1, init_test("rv64ui-p-ori"));
    }
    #[test]
    fn rv64ui_p_sb() {
        assert_eq!(1, init_test("rv64ui-p-sb"));
    }
    #[test]
    fn rv64ui_p_sh() {
        assert_eq!(1, init_test("rv64ui-p-sh"));
    }
    #[test]
    fn rv64ui_p_simple() {
        assert_eq!(1, init_test("rv64ui-p-simple"));
    }
    #[test]
    fn rv64ui_p_sll() {
        assert_eq!(1, init_test("rv64ui-p-sll"));
    }
    #[test]
    fn rv64ui_p_slli() {
        assert_eq!(1, init_test("rv64ui-p-slli"));
    }
    #[test]
    fn rv64ui_p_slliw() {
        assert_eq!(1, init_test("rv64ui-p-slliw"));
    }
    #[test]
    fn rv64ui_p_slt() {
        assert_eq!(1, init_test("rv64ui-p-slt"));
    }
    #[test]
    fn rv64ui_p_slti() {
        assert_eq!(1, init_test("rv64ui-p-slti"));
    }
    #[test]
    fn rv64ui_p_sltiu() {
        assert_eq!(1, init_test("rv64ui-p-sltiu"));
    }
    #[test]
    fn rv64ui_p_sltu() {
        assert_eq!(1, init_test("rv64ui-p-sltu"));
    }
    #[test]
    fn rv64ui_p_sra() {
        assert_eq!(1, init_test("rv64ui-p-sra"));
    }
    #[test]
    fn rv64ui_p_srai() {
        assert_eq!(1, init_test("rv64ui-p-srai"));
    }
    #[test]
    fn rv64ui_p_sraiw() {
        assert_eq!(1, init_test("rv64ui-p-sraiw"));
    }
    #[test]
    fn rv64ui_p_srl() {
        assert_eq!(1, init_test("rv64ui-p-srl"));
    }
    #[test]
    fn rv64ui_p_srli() {
        assert_eq!(1, init_test("rv64ui-p-srli"));
    }
    #[test]
    fn rv64ui_p_srliw() {
        assert_eq!(1, init_test("rv64ui-p-srliw"));
    }
    #[test]
    fn rv64ui_p_sraw() {
        assert_eq!(1, init_test("rv64ui-p-sraw"));
    }
    #[test]
    fn rv64ui_p_sub() {
        assert_eq!(1, init_test("rv64ui-p-sub"));
    }
    #[test]
    fn rv64ui_p_sw() {
        assert_eq!(1, init_test("rv64ui-p-sw"));
    }
    #[test]
    fn rv64ui_p_xor() {
        assert_eq!(1, init_test("rv64ui-p-xor"));
    }
    #[test]
    fn rv64ui_p_xori() {
        assert_eq!(1, init_test("rv64ui-p-xori"));
    }
    #[test]
    fn rv64ui_p_addw() {
        assert_eq!(1, init_test("rv64ui-p-addw"));
    }
    // 32-bit float, 32-bit mode, physical addressing
    #[test]
    fn rv32uf_p_fadd() {
        assert_eq!(1, init_test("rv32uf-p-fadd"));
    }
    #[test]
    fn rv32uf_p_fcmp() {
        assert_eq!(1, init_test("rv32uf-p-fcmp"));
    }
    #[test]
    fn rv32uf_p_fcvt() {
        assert_eq!(1, init_test("rv32uf-p-fcvt"));
    }
    #[test]
    fn rv32uf_p_fdiv() {
        assert_eq!(1, init_test("rv32uf-p-fdiv"));
    }
    #[test]
    fn rv32uf_p_fmin() {
        assert_eq!(1, init_test("rv32uf-p-fmin"));
    }
    #[test]
    fn rv32uf_p_ldst() {
        assert_eq!(1, init_test("rv32uf-p-ldst"));
    }
    #[test]
    fn rv32uf_p_fmadd() {
        assert_eq!(1, init_test("rv32uf-p-fmadd"));
    }
    #[test]
    fn rv32uf_p_recoding() {
        assert_eq!(1, init_test("rv32uf-p-recoding"));
    }
    // 64-bit float, 32-bit mode, physical addressing
    #[test]
    fn rv64uf_p_fadd() {
        assert_eq!(1, init_test("rv64uf-p-fadd"));
    }
    #[test]
    fn rv64uf_p_fcmp() {
        assert_eq!(1, init_test("rv64uf-p-fcmp"));
    }
    #[test]
    fn rv64uf_p_fcvt() {
        assert_eq!(1, init_test("rv64uf-p-fcvt"));
    }
    #[test]
    fn rv64uf_p_fcvt_w() {
        assert_eq!(1, init_test("rv64uf-p-fcvt_w"));
    }
    #[test]
    fn rv64uf_p_fdiv() {
        assert_eq!(1, init_test("rv64uf-p-fdiv"));
    }
    #[test]
    fn rv64uf_p_fmin() {
        assert_eq!(1, init_test("rv64uf-p-fmin"));
    }
    #[test]
    fn rv64uf_p_ldst() {
        assert_eq!(1, init_test("rv64uf-p-ldst"));
    }
    #[test]
    fn rv64uf_p_fmadd() {
        assert_eq!(1, init_test("rv64uf-p-fmadd"));
    }
    #[test]
    fn rv64uf_p_fclass() {
        assert_eq!(1, init_test("rv64uf-p-fclass"));
    }
    #[test]
    fn rv64uf_p_move() {
        assert_eq!(1, init_test("rv64uf-p-move"));
    }
    #[test]
    fn rv64uf_p_recoding() {
        assert_eq!(1, init_test("rv64uf-p-recoding"));
    }
    // multiplication, 32-bit mode, physical addressing
    #[test]
    fn rv32um_p_mul() {
        assert_eq!(1, init_test("rv32um-p-mul"));
    }
    #[test]
    fn rv32um_p_mulh() {
        assert_eq!(1, init_test("rv32um-p-mulh"));
    }
    #[test]
    fn rv32um_p_mulhu() {
        assert_eq!(1, init_test("rv32um-p-mulhu"));
    }
    #[test]
    fn rv32um_p_mulhsu() {
        assert_eq!(1, init_test("rv32um-p-mulhsu"));
    }
    #[test]
    fn rv32um_p_div() {
        assert_eq!(1, init_test("rv32um-p-div"));
    }
    #[test]
    fn rv32um_p_divu() {
        assert_eq!(1, init_test("rv32um-p-divu"));
    }
    #[test]
    fn rv32um_p_rem() {
        assert_eq!(1, init_test("rv32um-p-rem"));
    }
    #[test]
    fn rv32um_p_remu() {
        assert_eq!(1, init_test("rv32um-p-remu"));
    }
    // multiplication, 64-bit mode, physical addressing
    #[test]
    fn rv64um_p_mul() {
        assert_eq!(1, init_test("rv64um-p-mul"));
    }
    #[test]
    fn rv64um_p_mulh() {
        assert_eq!(1, init_test("rv64um-p-mulh"));
    }
    #[test]
    fn rv64um_p_mulw() {
        assert_eq!(1, init_test("rv64um-p-mulw"));
    }
    #[test]
    fn rv64um_p_mulhu() {
        assert_eq!(1, init_test("rv64um-p-mulhu"));
    }
    #[test]
    fn rv64um_p_mulhsu() {
        assert_eq!(1, init_test("rv64um-p-mulhsu"));
    }
    #[test]
    fn rv64um_p_div() {
        assert_eq!(1, init_test("rv64um-p-div"));
    }
    #[test]
    fn rv64um_p_divu() {
        assert_eq!(1, init_test("rv64um-p-divu"));
    }
    #[test]
    fn rv64um_p_divw() {
        assert_eq!(1, init_test("rv64um-p-divw"));
    }
    #[test]
    fn rv64um_p_divuw() {
        assert_eq!(1, init_test("rv64um-p-divuw"));
    }
    #[test]
    fn rv64um_p_rem() {
        assert_eq!(1, init_test("rv64um-p-rem"));
    }
    #[test]
    fn rv64um_p_remu() {
        assert_eq!(1, init_test("rv64um-p-remu"));
    }
    #[test]
    fn rv64um_p_remw() {
        assert_eq!(1, init_test("rv64um-p-remw"));
    }
    #[test]
    fn rv64um_p_remuw() {
        assert_eq!(1, init_test("rv64um-p-remuw"));
    }
    #[test]
    fn rv64ua_p_amoswap_w() {
        assert_eq!(1, init_test("rv64ua-p-amoswap_w"));
    }

}
