use std::cell::UnsafeCell;
use std::collections::HashMap;
use std::sync::{Arc};
use base::{debug, gettid};
use libc::sysinfo;
use sync::Mutex;
use vm_memory::GuestMemory;
use rustc_hash::FxHashMap;
use crate::common::memory::{flat_mem, MemEndian};
use crate::elf::UserModeRuntime;
use crate::linux_usermode::defs::{GenericStat, read32_advance_ptr, read64_advance_ptr};
use crate::linux_usermode::main::{dispatch, SyscallIn, SyscallOut, UsermodeCpu};
use crate::linux_usermode::signals::{block_all_signals, GenericSigactionArg, GenericStackt, get_generic_sigaction_64, SigEntry, SigInfo, Sigmask, SIGNAL_AVAIL, SINFO};
use crate::riscv::common::{Exception, get_privilege_encoding, get_trap_cause, Priv, RISCV_STACKPOINTER_REG, RiscvArgs, Trap, Xlen, xlen2bits, xlen2misa};
use crate::riscv::common::Exception::{EnvironmentCallFromMMode, EnvironmentCallFromUMode};
use crate::riscv::decoder;
use crate::riscv::interpreter::consts::*;
use crate::riscv::mem::{get_read_access_type, MemAccessCircumstances, MemAccessType, RISCV_PAGE_OFFSET, RISCV_PAGE_SHIFT, RISCV_PAGE_SIZE, RiscVMem};
//use crate::riscv::vector::vect_state;
use crate::riscv::interpreter::core::illegal_instr;
use crate::riscv::interpreter::defs::or;
use crate::riscv::ume::defs::{riscv_translate_syscall, write_riscv_stat, write_riscv_sysinfo};
use crate::riscv::ume::signals::setup_rt_frame;
// use crate::riscv::vector::VectState;

#[derive(Clone)]
pub struct RiscvInstr {
    pub inc_by: u64, // compressed = 2, normal = 4
    pub args: RiscvArgs,
   // func: Box<dyn Fn(&mut RiscvInt, &[u64]) -> bool>
    pub func: fn(&mut RiscvInt, &RiscvArgs),
}
#[derive(Debug,Copy, Clone,Eq, PartialEq)]
pub enum ExitReason {
    ExitTrap(Trap)
}
#[derive(Clone, Default)]
pub struct RiscvBlock {
    pub begin: u64, // for invalid, set to zero
    pub end: u64,
    pub instrs: Vec<RiscvInstr>
}
#[derive(Default)]
pub struct RiscvBlockCollection {
    pub ainstr: [RiscvBlock; 5],
    pub idx: usize,
}

pub struct RiscvInt {
    pub regs: [u64; 32], // registeres can be smaller than this, but we do biggest for somplicity,
    pub fregs: [u64; 32],
    pub pc: u64,
    pub want_pc: Option<u64>, // basically next pc
    pub xlen: Xlen,
    pub csr: [u64; 4096],
    pub trap_pc: u64,
    // todo: no need for mutex, memsource is a per hart/cpu structure
    pub memsource: RiscVMem,
    pub ainstr: UnsafeCell<RiscvBlockCollection>,
   // pub instr: UnsafeCell<FxHashMap<u64, Vec<RiscvBlock>>>,
    pub trap: Option<Trap>,
    pub current_block: RiscvBlock,
    pub user_struct: UserModeRuntime,
    pub is_compressed: bool,
    // pub vector_state: VectState,
  //  pub vect_ctx: vect_state,
    pub changed_pc: bool,
    pub prvmode: Priv,
    pub stop_translating: bool, // could be due to it being jump, etc. Only due this for branches, not errors
    pub stop_exec: bool,
    pub cache_enabled: bool, // if disabled then just exec as it comes along,
    pub wfi: bool, // equiv to x86 hlt
    pub usermode: bool,
    pub is_reservation: bool,
    pub res_val: u64,
    pub res_len: u8

}
pub enum ExtensionSearchMode {
    AtLeastOne,
    All,
}
impl RiscvInt {
    pub fn init_systemmode(xlen: Xlen, vm_mem: GuestMemory) -> RiscvInt {
        RiscvInt {
            regs: [0; 32],
            fregs: [0; 32],
            pc: 0,
            trap_pc: 0,
            want_pc: None,
            xlen,
            csr: [0; 4096],
            memsource: RiscVMem::new_system(xlen, vm_mem),
            ainstr: Default::default(),
            trap: None,
            current_block: RiscvBlock::default(),
            changed_pc: false,
            prvmode: Priv::Machine,
            stop_translating: false,
            stop_exec: false,
            cache_enabled: false,
            wfi: false,
            user_struct: UserModeRuntime::default(),
            usermode: false,
            is_reservation: false,
            res_val: 0,
            is_compressed: false,
            res_len: 0
        }
    }
    pub fn init_usermode(xlen: Xlen, ume: UserModeRuntime) -> RiscvInt {
        RiscvInt {
            regs: [0; 32],
            fregs: [0; 32],
            pc: 0,
            want_pc: None,
            xlen,
            trap_pc: 0,
            csr: [0; 4096],
            memsource: RiscVMem::new_usermode(xlen),
            ainstr: Default::default(),
            trap: None,
            current_block: RiscvBlock::default(),
            changed_pc: false,
            prvmode: Priv::Machine,
            stop_translating: false,
            stop_exec: false,
            cache_enabled: false,
            wfi: false,
            user_struct: ume,
            usermode: true,
            is_reservation: false,
            res_val: 0,
            is_compressed: false,
            res_len: 0
        }
    }
    pub fn extension_verify(&mut self, exts: &[usize], mode: ExtensionSearchMode) -> bool {
        panic!();
        //true
    }
    pub fn extension_verify_trap_if_false(&mut self, exts: &[usize], mode: ExtensionSearchMode) -> bool {
        panic!();
        //true
    }
    pub fn get_stack_reg(&self) -> u64 {
        self.regs[RISCV_STACKPOINTER_REG]
    }
    pub fn set_stack_reg(&mut self, val: u64) {
        self.regs[RISCV_STACKPOINTER_REG] = val;
    }

    pub fn get_csr_raw(&self, idx: usize) -> u64 {
        self.csr[idx]
    }
    pub fn set_csr_raw(&mut self, idx: usize, val: u64) {
        unimplemented!()
    }
    pub fn change_priv(&mut self, privs: Priv) {
        self.memsource.clear_cache();
        self.prvmode = privs;
    }
    pub fn handle_trap(&mut self, trp: Trap, trapped_pc: u64) {
        let mut reason = get_trap_cause(trp, self.xlen);
        let mut hsdeleg = 0;
        let intr = if 1 << (xlen2bits(self.xlen) - 1) & reason != 0 {
            true
        } else {
            false
        };
        if intr {
            if get_privilege_encoding(self.prvmode) <= Priv::Supervisor as u64 {
                hsdeleg = self.csr[CSR_MIDELEG_ADDRESS as usize];
            }
            reason &= !(1 << (xlen2bits(self.xlen) - 1));
        } else {
            if get_privilege_encoding(self.prvmode) <= Priv::Supervisor as u64 {
                hsdeleg = self.csr[CSR_MEDELEG_ADDRESS as usize];
            }
        }
        if (get_privilege_encoding(self.prvmode) <= Priv::Supervisor as u64) &&
            (reason < xlen2bits(self.xlen)) &&
            (((hsdeleg >> reason) & 1) != 0) {
            // supervisor mode
            let stvec = self.csr[CSR_STVEC_ADDRESS as usize];
            let vector = if ((stvec & 1) != 0) && intr {
                4 * reason
            } else {
                0
            };
            self.pc = (stvec & !1) + vector;
            self.csr[CSR_SCAUSE_ADDRESS as usize] = reason;
            self.csr[CSR_SEPC_ADDRESS as usize] = trapped_pc;
            self.csr[CSR_STVAL_ADDRESS as usize] = trp.val;
            let mut status = self.csr[CSR_SSTATUS_ADDRESS as usize];
            let sie = (status >> 1) & 1;
            // privlege: its either 0 or 1, because we checked before
            status = (status & !0x122) | (sie << 5) | ((get_privilege_encoding(self.prvmode) & 1) << 8);
            self.csr[CSR_SSTATUS_ADDRESS as usize] = status;
            self.change_priv(Priv::Supervisor); // or user?
        } else {
            let mtvec = self.csr[CSR_MTVEC_ADDRESS as usize];
            let vector = if ((mtvec & 1) != 0) && intr {
                4 * reason
            } else {
                0
            };
            self.pc = (mtvec & !1) + vector;
            self.csr[CSR_MCAUSE_ADDRESS as usize] = reason;
            self.csr[CSR_MEPC_ADDRESS as usize] = trapped_pc;
            self.csr[CSR_MTVAL_ADDRESS as usize] = trp.val;
            let mut status = self.csr[CSR_MSTATUS_ADDRESS as usize];
            let mie = (status >> 3) & 1;
            // privlege: its either 0 or 1, because we checked before
            status = (status & !0x1888) | (mie << 7) | ((get_privilege_encoding(self.prvmode)) << 11);
            // todo: mstatush
            self.csr[CSR_MSTATUS_ADDRESS as usize] = status;
            self.change_priv(Priv::Machine);


        }
    }
    fn mstatus_fixup(&mut self, m: u64) -> u64 {
        let mut mstatus = m;
        // little endian only (for now)
        mstatus &= !(1 << 6);
        if self.xlen == Xlen::X64 {
            mstatus &= !(1 << 36);
            mstatus &= !(1 << 37);

        }
        // sxl and uxl should be equal to the same thing (mxl)
        if self.xlen == Xlen::X64 {
            let s = xlen2misa(self.xlen);
            mstatus &= !((0b11) << 32);
            mstatus |= ((s & 0b11) << 32);
            mstatus &= !((0b11) << 34);
            mstatus |= ((s & 0b11) << 34);
        }
        mstatus
    }
    pub fn flush_mstatus(&mut self) {
        // todo: sum bit
    }
    pub fn sign_ext(&self, value: u64) -> u64 {
        match self.xlen {
            Xlen::X32 => value as i32 as i64 as u64,
            Xlen::X64 => value
        }
    }
    pub fn most_negative(&self) -> i64 {
        match self.xlen {
            Xlen::X32 => std::i32::MIN as i64,
            Xlen::X64 => std::i64::MIN
        }
    }
    pub fn cull_reg(&self, val: u64) -> u64 {
        if self.xlen == Xlen::X32 {
            val & 0xffffffff
        } else {
            val & 0xffffffffffffffff
        }
    }

    pub fn get_pc_of_current_instr(&mut self) -> u64 {
        // for when we want to do lazy pc optimization
        self.pc
    }
    pub fn get_pc_of_next_instr(&mut self) -> u64 {
        if self.is_compressed {
            self.pc + 2
        } else {
            self.pc + 4
        }
    }
    pub fn insert_insn_current(&mut self, instr: RiscvInstr) {
        self.current_block.instrs.push(instr);
    }

    pub fn set_trap(&mut self, trp: Trap) {
        // todo piority, i think its if one is already there
        if self.usermode {
        }
        self.trap = Some(trp);
        self.trap_pc = self.get_pc_of_current_instr();
        self.stop_exec = true;

    }

    pub fn illegal_instr(&mut self) {
        panic!(); // for now
        let current_pc = self.get_pc_of_current_instr();
        self.set_trap(Trap {
            ttype: Exception::IllegalInstruction,
            val: current_pc
        });
        self.stop_exec = true;
        self.stop_translating = true;
    }
    fn check_exec(&mut self) -> bool {
        // false, continue exec. true, stop exec and go to gen handler
        if self.stop_exec == false {
            panic!();
        }
        return true

    }
    fn exec_cached_int(&mut self) -> Result<(), Trap> {
        loop {
            let curpc = self.get_pc_of_current_instr();
            let mut max_count: i64 = (RISCV_PAGE_SIZE - (curpc & RISCV_PAGE_OFFSET)) as i64; // i64 for underflow
            if max_count < 4 {
                // this is so we don't make one line blocks
                // in case of page fault, mem will set parameters and next pc
                // instruction crosses a page boundary, so execute manually
                self.stop_exec = true; // instaquit after one instruction
                self.cache_enabled = false;
                self.exec_one_by_one()?; // dont worry if fail, we set it back to true on reentry
                // we don't know if another error happened while executing that instruction,
                // so go to outer loop
                return Ok(());
            }
            let macc = self.gen_mem_cirum(MemAccessType::Execute);
            // if we can access one page, we can access the rest. If not, then we need to fault
            let physpc = if let Ok(s) = self.memsource.virt2phys(curpc, macc) {
                s
            } else {
                self.stop_exec = true;
                return Err(self.mem_trap(MemAccessType::Execute, curpc));
            };
            /* 'outer: loop {
                let retaddr = physpc >> RISCV_PAGE_SHIFT;

                if let Some(zz) =  self.instr.get(&retaddr) {
                    for i in zz {
                        if i.begin == physpc {
                            if (i.begin & !RISCV_PAGE_OFFSET) ^ (i.end & !RISCV_PAGE_OFFSET) != 0 {
                                panic!(); // bug check
                            }
                            self.exec_block_inner(i);
                            if self.stop_exec {
                                return Ok(());
                            }
                            break 'outer;
                        }
                    }
                }
                // no "else" clause, because what if we found other in address space but no match?
                self.build_exec(physpc);
                continue;
            }

             */
            unsafe {
                if self.check_run_block(physpc) {
                    self.build_exec(physpc).unwrap();
                    if self.check_run_block(physpc) {
                        panic!();
                    }
                }
                if self.stop_exec {
                    return Ok(());
                }

            }
            /*if let Some(blk) = self.check_block(physpc) {
                // already exists
                self.exec_block_inner(&blk);
                if self.stop_exec {
                    return Ok(());
                }
            } else {
                self.build_exec(physpc);
                // unwrap cuz we already added it to block. If not there then something wrong;
                let blk = self.check_block(physpc).unwrap();
                self.exec_block_inner(&blk);
                if self.stop_exec {
                    return Ok(());
                }
            }

             */


        }
    }
    fn build_exec(&mut self, addr: u64) -> Result<(), Trap> {
        self.stop_translating = false;
        let mut iaddr = addr;
        self.current_block.begin = addr;
        self.current_block.instrs.clear();
        assert_eq!(self.cache_enabled, true);
        let mut max_count: i64 = (RISCV_PAGE_SIZE - (addr & RISCV_PAGE_OFFSET)) as i64; // i64 for underflow
       // let val = self.memsource.lock().guest_mem.guest_mem.get_host_address_range(GuestAddress(addr), max_count).unwrap();
        // potential optimizaion for system mode, get host page, and read from there
        // since we only go up to the end of a page, and the pagetable doesnt change during the
        // translation, we know the 4096 bytes in real mem corresponds to the. usage of read32 wrong too
        let mut inc_by = 0;
        while max_count >= 2 {
            let instr_lower = self.read16(iaddr, true, false)?; // this should be physical
            if (instr_lower & 0x3) != 0x3 {
                self.is_compressed = true;
                // compressed
                if !crate::riscv::decoder16::decode(self, instr_lower as u16) {
                    self.illegal_instr();
                }
                inc_by = 2;
            } else {
                if max_count < 4 {
                    break;
                }
                let instr_high = self.read16(iaddr + 2, true, false)?; // this should be physical
                let realinstr = ((instr_high as u32) << 16) | (instr_lower as u32);
                self.is_compressed = false;
                if !crate::riscv::decoder::decode(self, realinstr) {
                    self.illegal_instr(); // this will set stop_exec = true
                }
                inc_by = 4;
            }
            self.current_block.instrs.last_mut().unwrap().inc_by = inc_by;
            iaddr += inc_by;
            max_count -= (inc_by as i64);
            if self.stop_translating {
                // usually after branch
                // Runtime we will determine if we need to get out of loop via stop_exec
                break;
            }

        }
        self.current_block.end = iaddr - inc_by; // end would be the last pc the block world cover
        unsafe {
            let z = (self.ainstr.get());
            let newidx = ((*z).idx + 1) % (*z).ainstr.len();
            (*z).ainstr[newidx] = self.current_block.clone(); // should be z.idx
            (*z).idx = newidx;
        }
        Ok(())
    }
    unsafe fn check_run_block(&mut self, addr: u64) -> bool {
        // block if there, None if otherwise
        for i in (*self.ainstr.get()).ainstr.iter() {
            if i.begin == addr {
                if (i.begin & !RISCV_PAGE_OFFSET) ^ (i.end & !RISCV_PAGE_OFFSET) != 0 {
                    panic!(); // bug check
                }
                self.exec_block_inner(i);
                return false;
            }
        }
        return true;
    }
    fn exec_block_inner(&mut self, blk: &RiscvBlock) {
        self.stop_exec = false;
        for  z in &blk.instrs {
            self.is_compressed = if z.inc_by == 2 {
                true
            } else {
                false
            };
            (z.func)(self, &z.args);
            self.pc += z.inc_by;
            self.regs[0] = 0;
            if self.stop_exec {
                // for usual reasons, or maybe this cache has been invalidated 10e4e
                return;
            }
        }
        return;
    }
    pub fn handle_syscall(&mut self) {
        let syscallnum = self.regs[17]; // a7
        let systype = if let Some(s) = riscv_translate_syscall(syscallnum as u16) {
            debug!("Going to execute syscall {:?} (number {:}, on thread id {:x})",
                s, syscallnum, self.user_struct.tid_val);
            s
        } else {
            debug!("Failed to execute syscall number {:}", syscallnum);
            panic!();
        };
        let arg1 = self.regs[10]; // a0
        let arg2 = self.regs[11];
        let arg3 = self.regs[12];
        let arg4 = self.regs[13];
        let arg5 = self.regs[14];
        let arg6 = self.regs[15];
        let sysin: SyscallIn = SyscallIn {
            syscall: systype,
            args: [arg1, arg2, arg3, arg4, arg5, arg6, 0]
        };
        let out = dispatch(self, sysin);
        self.regs[10] = out.ret1;
        if let Some(xx) = out.ret2 {
            self.regs[11] = xx;
        }
    }
    pub fn run(&mut self) {
        loop {
            if self.cache_enabled {
                match self.exec_cached_int() {
                    Ok(()) => { },
                    Err(z) => {
                        self.trap = Some(z);
                    }
                }
                self.cache_enabled = true;
            } else {
                match self.exec_one_by_one() {
                    Ok(()) => { },
                    Err(z) => {
                        self.trap = Some(z);
                    }
                }
            }
            if self.trap.is_some() {
                if self.usermode {
                    let trp = self.trap.unwrap();
                    if trp.ttype == EnvironmentCallFromMMode {
                        self.handle_syscall();
                        self.stop_exec = false;
                        self.trap = None;

                    } else {
                        panic!("Protection error  - Suffered RISCV trap in user mode: {:?}", self.trap.unwrap())
                    }
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
            if self.usermode {
                SIGNAL_AVAIL.with(|z| {
                    let mut zz = z.borrow_mut();
                    if *zz == true {
                        // signal
                        SINFO.with(|a| {
                            let mut aa = a.borrow_mut();
                            let signum = aa.use_idx.unwrap();
                            setup_rt_frame(self, signum as i32, &mut aa);
                        });
                        *zz = false; // we will unblock signals later
                    }
                });
            }
            if let Some(f) = self.want_pc {
                // todo: any checks?
                self.pc = f;
                self.want_pc = None;
            }
            if self.wfi {
                unimplemented!();
            }
            self.stop_exec = false;
        }

    }
    pub(crate) fn exec_one_by_one(&mut self) -> Result<(), Trap> {
        loop {
            // 0x7effc001397e
            let instr = self.read32(self.pc, true, true)?;
            if (instr & 0x3) != 0x3 {
                self.is_compressed = true;
                // compressed
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
            self.regs[0] = 0;

            if self.stop_exec {
                return Ok(());
                // could be a trap for instr, request to jump, etc... We return err on
                // fetch error only
            }
            // todo/: "bias" optimization: don't even go through pagewalker, have hard addr


        }
    }
}
