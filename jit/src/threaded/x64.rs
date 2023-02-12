use std::collections::BTreeMap;
use std::num::Wrapping;
use std::{mem, ptr};
use base::MappedRegion;
use base::{MemoryMapping, Protection};
use iced_x86::code_asm::*;
use iced_x86::*;
use iced_x86::Mnemonic::Str;
use std::cell::UnsafeCell;
use crate::main::{DataDesc, DataDescType, ExtVarDesc, JitBackend, JitEndian, JitError, JitOps};
use crate::threaded::x64_stubs::*;
pub struct X64JitBlock {
    addr: u64,
    code: Vec<u8>,
    exec_code: MemoryMapping

}
pub struct ThreadJit {
    endianused: JitEndian,
    //temps: BTreeMap<u16, Variable>,
    // these are temps. We only do ops on temps, and then bring back forth to regs (ext reg var)
    pub regs: [u64; 32],
    pub free_reg: [bool; 32],
    pub temp_regs: [u64; 32],
    pub current_addr: u64,
    asm: CodeAssembler,
    labels: Vec<CodeLabel>,
    blocks: BTreeMap<u64, X64JitBlock>, // todo; make into array where index overwrites earliest
    // regs: BTreeMap<u16, Variable>,
}

impl  ThreadJit {
    pub fn try_get_free_reg(&mut self) -> Option<u64> {
        for i in 0..self.regs.len() {
            if self.free_reg[i] == true {
                self.free_reg[i] = false;
                return Some(i as u64);
            }
        }
        return None;
    }
    pub fn get_free_reg(&mut self) -> u64 {
        self.try_get_free_reg().unwrap()
    }
    pub fn release_reg(&mut self, v: u64) {
        assert_eq!(self.free_reg[v as usize], false);
        self.free_reg[v as usize] = true;
    }
    pub fn init() -> ThreadJit {
        ThreadJit {
            endianused: JitEndian::Little,
            regs: [0 ;32],
            free_reg: [true; 32],
            temp_regs: [0; 32],
            current_addr: 0,
            asm: CodeAssembler::new(64).unwrap(),
            labels: vec![],
            blocks: Default::default()
        }
    }
}
thread_local! {
    pub static global_thread_ptr: *mut ThreadJit = ptr::null_mut();
}
// pub static mut global_thread_ptr: *mut ThreadJit = ptr::null_mut();
type twoarg_fn = extern "win64" fn(u64, u64);
type threearg_fn = extern "win64" fn(u64, u64, u64);
// temps can be register or varialbe
impl ThreadJit {
    pub fn emit_3arg_fn(&mut self, tf: threearg_fn, arg1: u64, arg2: u64, arg3: u64) {
        // we are not using actual x86 regs, so we just use eax
        /* self.asm.mov(gpr64::rax, arg3).unwrap();
        self.asm.push(gpr64::rax).unwrap();
        self.asm.mov(gpr64::rax, arg2).unwrap();
        self.asm.push(gpr64::rax).unwrap();
        self.asm.mov(gpr64::rax, arg1).unwrap();
        self.asm.push(gpr64::rax).unwrap();

         */
        self.asm.mov(gpr64::rcx, arg1).unwrap();
        self.asm.mov(gpr64::rdx, arg2).unwrap();
        self.asm.mov(gpr64::r8, arg3).unwrap();

        let functionaddr = tf as *const u64;
        let rawval = functionaddr as u64;
        self.asm.mov(gpr64::rax, rawval).unwrap();
        self.asm.call(gpr64::rax).unwrap();
    }
    pub fn emit_2arg_fn(&mut self, tf: twoarg_fn, arg1: u64, arg2: u64) {
        // we are not using actual x86 regs, so we just use eax
        //self.asm.mov(gpr64::rax, arg2).unwrap();
        ///self.asm.push(gpr64::rax).unwrap();
        //self.asm.mov(gpr64::rax, arg1).unwrap();
        //.asm.push(gpr64::rax).unwrap();
        self.asm.mov(gpr64::rcx, arg1).unwrap();
        self.asm.mov(gpr64::rdx, arg2).unwrap();

        let functionaddr = tf as *const u64;
        let rawval = functionaddr as u64;
        self.asm.mov(gpr64::rax, rawval).unwrap();
        self.asm.call(gpr64::rax).unwrap();
    }
    pub fn free_reg_from_desc(&mut self, dd: DataDesc, regnum: u64, flush: bool) {
        match dd.dttype {
            DataDescType::Imm => {
                self.release_reg(regnum);
            }
            DataDescType::PermReg => {
                // we didn't make it just now, so let it be
            },
            DataDescType::TempReg => {
                if flush {
                    let src = DataDesc::new_permreg(regnum as u64);
                    self.mov(dd, src);
                }
                self.release_reg(regnum);
            }
        }
    }
    pub fn gen_reg_from_desc(&mut self, dd: DataDesc) -> u64 {
        match dd.dttype {
            DataDescType::Imm => {
                let rn = self.get_free_reg();
                let rnd = DataDesc::new_tempreg(rn as u64);
                self.mov(rnd, dd);
                rn
            }
            DataDescType::PermReg => {
                return dd.num as u64;
            }
            DataDescType::TempReg => {
                let rn = self.get_free_reg();
                let rnd = DataDesc::new_tempreg(rn as u64);
                self.mov(rnd, dd);
                rn
            }
        }
    }
    pub fn emit_src_dst_2arg(&mut self, fnptr: twoarg_fn, dst: DataDesc, src: DataDesc) {
        assert_ne!(dst.dttype, DataDescType::Imm);
        let dstreg = self.gen_reg_from_desc(dst);
        let srcreg = self.gen_reg_from_desc(src);
        self.emit_2arg_fn(fnptr, dstreg, srcreg);
        self.free_reg_from_desc(src, srcreg, false);
        self.free_reg_from_desc(dst, dstreg, true);
    }
    pub fn emit_dst_2ops_3arg(&mut self, fnptr: threearg_fn,
                              dst: DataDesc, op1: DataDesc, op2: DataDesc) {
        assert_ne!(dst.dttype, DataDescType::Imm);
        let r1 = self.gen_reg_from_desc(op1);
        let r2 = self.gen_reg_from_desc(op2);
        let r3 = self.gen_reg_from_desc(dst);
        self.emit_3arg_fn(fnptr, r3, r1, r2);
        self.free_reg_from_desc(dst, r3, true);
        self.free_reg_from_desc(op2, r2, false);
        self.free_reg_from_desc(op1, r1, false);
    }
}

impl JitBackend for ThreadJit {
    fn new_block(&mut self, addr: u64) -> Result<(), JitError> {
        self.asm = CodeAssembler::new(64).unwrap();
        self.labels = Vec::new();
        self.asm.push(rax).unwrap();
        self.asm.push(rbx).unwrap();
        self.asm.push(rcx).unwrap();
        self.asm.push(rdx).unwrap();
        self.asm.push(r11).unwrap();
        self.free_reg = [true; 32];
        self.current_addr = addr;
        Ok(())
    }

    fn end_block(&mut self) -> Result<(), JitError> {
        self.asm.pop(r11).unwrap();
        self.asm.pop(rdx).unwrap();
        self.asm.pop(rcx).unwrap();
        self.asm.pop(rbx).unwrap();
        self.asm.pop(rax).unwrap();
        self.asm.ret().unwrap();
        let mut vect = self.asm.assemble(0).unwrap();
        self.labels.clear();
        let memloc = base::platform::
        MemoryMapping::new_protection(vect.len(),
                                      Protection::read_write_execute())
            .unwrap();
        let memloc2 = base::MemoryMapping {
            mapping: memloc,
            _file_descriptor: None
        };
        memloc2.write_slice(vect.as_slice(), 0).unwrap();
        let block = X64JitBlock {
            addr: self.current_addr,
            code: vect,
            exec_code: memloc2
        };

        self.blocks.insert(self.current_addr, block);

        Ok(())
    }

    fn drop_specific_block_page(&mut self, addr: u64, pagesize: u64) -> Result<(), JitError> {
        todo!()
    }

    fn exec_block(&mut self, addr: u64) -> Result<(), JitError> {
        let blk =  self.blocks.get(&addr).unwrap();
        let addr = blk.exec_code.as_ptr();
        let exec_fn: extern "win64" fn() -> bool = unsafe { mem::transmute(addr as *const u8)};
        exec_fn();
        Ok(())
    }

    fn init_guest_registers(&mut self, regs: Vec<ExtVarDesc>) -> Result<(), JitError> {
        todo!()
    }

    fn get_guest_reg_desc(&mut self, idx: u64) -> Result<DataDesc, JitError> {
        todo!()
    }

    fn get_guest_reg(&mut self, idx: u64) -> Result<u64, JitError> {
        todo!()
    }
}

impl JitOps for ThreadJit {
    fn mov(&mut self, dst: DataDesc, src: DataDesc) {
        if src.dttype == DataDescType::Imm {
            if dst.dttype == DataDescType::PermReg {
                self.emit_2arg_fn(x64_load_imm64, dst.num, src.num);
            } else if dst.dttype == DataDescType::TempReg {
                self.emit_2arg_fn(x64_temp_load_imm64, dst.num, src.num);
            } else {
                panic!(); // can't move into an imm
            }
        } else if src.dttype == DataDescType::PermReg {
            if dst.dttype == DataDescType::PermReg {
                self.emit_2arg_fn(x64_mov_reg, dst.num, src.num);
            } else if dst.dttype == DataDescType::TempReg {
                self.emit_2arg_fn(x64_store_ext_reg, dst.num, src.num);
            } else {
                panic!();
            }
        } else if src.dttype == DataDescType::TempReg {
            if dst.dttype == DataDescType::PermReg {
                self.emit_2arg_fn(x64_load_ext_reg, dst.num, src.num);
            } else if dst.dttype == DataDescType::TempReg {
                self.emit_2arg_fn(x64_mov_temp, dst.num, src.num);
            } else {
                panic!();
            }
        } else {
            unreachable!();
        }
    }
    fn add(&mut self, dst: DataDesc, op1: DataDesc, op2: DataDesc) {
        self.emit_dst_2ops_3arg(x64_add64, dst, op1, op2);

    }

    fn sub(&mut self, dst: DataDesc, op1: DataDesc, op2: DataDesc) {
        self.emit_dst_2ops_3arg(x64_sub, dst, op1, op2);
    }

    fn extsw(&mut self, dst: DataDesc, op1: DataDesc) {
        self.emit_src_dst_2arg(x64_extsw, dst, op1);
    }

    fn extuw(&mut self, dst: DataDesc, op1: DataDesc) {
        self.emit_src_dst_2arg(x64_extuw, dst, op1);
    }

    fn extsh(&mut self, dst: DataDesc, op1: DataDesc) {
        self.emit_src_dst_2arg(x64_extsh, dst, op1);
    }

    fn extuh(&mut self, dst: DataDesc, op1: DataDesc) {
        self.emit_src_dst_2arg(x64_extuh, dst, op1);
    }

    fn extsb(&mut self, dst: DataDesc, op1: DataDesc) {
        self.emit_src_dst_2arg(x64_extsb, dst, op1);
    }

    fn extub(&mut self, dst: DataDesc, op1: DataDesc) {
        self.emit_src_dst_2arg(x64_extub, dst, op1);
    }

    fn ceq(&mut self, dst: DataDesc, op1: DataDesc, op2: DataDesc) {
        self.emit_dst_2ops_3arg(x64_ceq, dst, op1, op2);

    }

    fn cne(&mut self, dst: DataDesc, op1: DataDesc, op2: DataDesc) {
        self.emit_dst_2ops_3arg(x64_cne, dst, op1, op2);
    }

    fn cslel(&mut self, dst: DataDesc, op1: DataDesc, op2: DataDesc) {
        self.emit_dst_2ops_3arg(x64_cslel, dst, op1, op2);
    }

    fn cslew(&mut self, dst: DataDesc, op1: DataDesc, op2: DataDesc) {
        self.emit_dst_2ops_3arg(x64_cslew, dst, op1, op2);
    }

    fn csltl(&mut self, dst: DataDesc, op1: DataDesc, op2: DataDesc) {
        self.emit_dst_2ops_3arg(x64_csltl, dst, op1, op2);
    }

    fn csltw(&mut self, dst: DataDesc, op1: DataDesc, op2: DataDesc) {
        self.emit_dst_2ops_3arg(x64_csltw, dst, op1, op2);
    }

    fn csgel(&mut self, dst: DataDesc, op1: DataDesc, op2: DataDesc) {
        self.emit_dst_2ops_3arg(x64_csgel, dst, op1, op2);
    }

    fn csgew(&mut self, dst: DataDesc, op1: DataDesc, op2: DataDesc) {
        self.emit_dst_2ops_3arg(x64_csgew, dst, op1, op2);
    }

    fn csgtl(&mut self, dst: DataDesc, op1: DataDesc, op2: DataDesc) {
        self.emit_dst_2ops_3arg(x64_csgtl, dst, op1, op2);
    }

    fn csgtw(&mut self, dst: DataDesc, op1: DataDesc, op2: DataDesc) {
        self.emit_dst_2ops_3arg(x64_csgtw, dst, op1, op2);
    }

    fn culel(&mut self, dst: DataDesc, op1: DataDesc, op2: DataDesc) {
        self.emit_dst_2ops_3arg(x64_culel, dst, op1, op2);
    }

    fn culew(&mut self, dst: DataDesc, op1: DataDesc, op2: DataDesc) {
        self.emit_dst_2ops_3arg(x64_culew, dst, op1, op2);
    }

    fn cultl(&mut self, dst: DataDesc, op1: DataDesc, op2: DataDesc) {
        self.emit_dst_2ops_3arg(x64_cultl, dst, op1, op2);
    }

    fn cultw(&mut self, dst: DataDesc, op1: DataDesc, op2: DataDesc) {
        self.emit_dst_2ops_3arg(x64_cultw, dst, op1, op2);
    }

    fn cugel(&mut self, dst: DataDesc, op1: DataDesc, op2: DataDesc) {
        self.emit_dst_2ops_3arg(x64_cugel, dst, op1, op2);
    }

    fn cugew(&mut self, dst: DataDesc, op1: DataDesc, op2: DataDesc) {
        self.emit_dst_2ops_3arg(x64_cugew, dst, op1, op2);
    }

    fn cugtl(&mut self, dst: DataDesc, op1: DataDesc, op2: DataDesc) {
        self.emit_dst_2ops_3arg(x64_cugtl, dst, op1, op2);
    }

    fn cugtw(&mut self, dst: DataDesc, op1: DataDesc, op2: DataDesc) {
        self.emit_dst_2ops_3arg(x64_cugtw, dst, op1, op2);
    }

    fn udiv64(&mut self, dst: DataDesc, op1: DataDesc, op2: DataDesc) {
        self.emit_dst_2ops_3arg(x64_udiv64, dst, op1, op2);
    }

    fn udiv32(&mut self, dst: DataDesc, op1: DataDesc, op2: DataDesc) {
        self.emit_dst_2ops_3arg(x64_udiv32, dst, op1, op2);
    }

    fn sdiv64(&mut self, dst: DataDesc, op1: DataDesc, op2: DataDesc) {
        self.emit_dst_2ops_3arg(x64_sdiv64, dst, op1, op2);
    }

    fn sdiv32(&mut self, dst: DataDesc, op1: DataDesc, op2: DataDesc) {
        self.emit_dst_2ops_3arg(x64_sdiv32, dst, op1, op2);
    }

    fn mul64l(&mut self, dst: DataDesc, op1: DataDesc, op2: DataDesc) {
        self.emit_dst_2ops_3arg(x64_mul64l, dst, op1, op2);
    }

    fn umul64h(&mut self, dst: DataDesc, op1: DataDesc, op2: DataDesc) {
        self.emit_dst_2ops_3arg(x64_umul64h, dst, op1, op2);
    }

    fn smul64h(&mut self, dst: DataDesc, op1: DataDesc, op2: DataDesc) {
        self.emit_dst_2ops_3arg(x64_smul64h, dst, op1, op2);
    }

    fn mul32l(&mut self, dst: DataDesc, op1: DataDesc, op2: DataDesc) {
        self.emit_dst_2ops_3arg(x64_mul32l, dst, op1, op2);
    }

    fn umul32h(&mut self, dst: DataDesc, op1: DataDesc, op2: DataDesc) {
        self.emit_dst_2ops_3arg(x64_umul32h, dst, op1, op2);
    }

    fn smul32h(&mut self, dst: DataDesc, op1: DataDesc, op2: DataDesc) {
        self.emit_dst_2ops_3arg(x64_smul32h, dst, op1, op2);
    }

    fn neg32(&mut self, dst: DataDesc, op1: DataDesc) {
        self.emit_src_dst_2arg(x64_neg32, dst, op1);

    }

    fn neg64(&mut self, dst: DataDesc, op1: DataDesc) {
        self.emit_src_dst_2arg(x64_neg64, dst, op1);
    }

    fn urem32(&mut self, dst: DataDesc, op1: DataDesc, op2: DataDesc) {
        self.emit_dst_2ops_3arg(x64_urem32, dst, op1, op2);
    }

    fn srem64(&mut self, dst: DataDesc, op1: DataDesc, op2: DataDesc) {
        self.emit_dst_2ops_3arg(x64_srem64, dst, op1, op2);
    }

    fn srem32(&mut self, dst: DataDesc, op1: DataDesc, op2: DataDesc) {
        self.emit_dst_2ops_3arg(x64_srem32, dst, op1, op2);
    }

    fn urem64(&mut self, dst: DataDesc, op1: DataDesc, op2: DataDesc) {
        self.emit_dst_2ops_3arg(x64_urem64, dst, op1, op2);
    }

    fn or(&mut self, dst: DataDesc, op1: DataDesc, op2: DataDesc) {
        self.emit_dst_2ops_3arg(x64_or, dst, op1, op2);
    }

    fn and(&mut self, dst: DataDesc, op1: DataDesc, op2: DataDesc) {
        self.emit_dst_2ops_3arg(x64_and, dst, op1, op2);
    }

    fn xor(&mut self, dst: DataDesc, op1: DataDesc, op2: DataDesc) {
        self.emit_dst_2ops_3arg(x64_xor, dst, op1, op2);
    }

    fn sar(&mut self, dst: DataDesc, op1: DataDesc, op2: DataDesc) {
        self.emit_dst_2ops_3arg(x64_sar, dst, op1, op2);
    }

    fn shr(&mut self, dst: DataDesc, op1: DataDesc, op2: DataDesc) {
        self.emit_dst_2ops_3arg(x64_shr, dst, op1, op2);
    }

    fn shl(&mut self, dst: DataDesc, op1: DataDesc, op2: DataDesc) {
        self.emit_dst_2ops_3arg(x64_shl, dst, op1, op2);
    }

    fn free_temp(&mut self, var: DataDesc) {
        if var.dttype == DataDescType::TempReg {
            self.release_reg(var.num);
        } else {
            unimplemented!();
        }
    }

    fn create_temp(&mut self) -> Result<DataDesc, JitError> {
        if let Some(s) = self.try_get_free_reg() {
            Ok(DataDesc::new_tempreg(s))
        } else {
            // todo: create space in varialbe space
            unimplemented!();

        }
    }
}