use std::os::unix::thread::JoinHandleExt;
use std::sync::Arc;
use base::{EventFd, get_blocked_signals, gettid};
use libc::{CLONE_CHILD_CLEARTID, CLONE_CHILD_SETTID, CLONE_PARENT_SETTID, CLONE_SETTLS, sysinfo};
use sync::Mutex;
use crate::common::memory::MemEndian;
use crate::elf::UserModeRuntime;
use crate::linux_usermode::defs::GenericStat;
use crate::linux_usermode::main::{SyscallIn, SyscallOut, UsermodeCpu};
use crate::linux_usermode::signals::{block_all_signals, GenericSigactionArg, GenericStackt, get_generic_sigaction_64, set_mask_block, SigEntry, SigInfo, Sigmask};
use crate::riscv::common::{RISCV_STACKPOINTER_REG, Xlen};
use crate::riscv::interpreter::main::RiscvInt;
use crate::riscv::ume::defs::write_riscv_stat;
use crate::riscv::ume::defs::{riscv_translate_syscall, write_riscv_sysinfo};
use crate::riscv::ume::signals::setup_rt_frame;
pub mod load;
pub mod defs;
pub mod signals;

impl UsermodeCpu for RiscvInt {
    fn push_stack_natural(&mut self, val: u64) {
        todo!()
    }

    fn pop_stack_natural(&mut self) -> u64 {
        todo!()
    }

    fn get_stack_reg(&mut self) -> u64 {
        todo!()
    }

    fn get_ume(&mut self) -> &mut UserModeRuntime {
        &mut self.user_struct
    }
    fn write_stat_t(&mut self, addr: u64, stat_t: GenericStat) {
        // risc-v user mode always little endian
        write_riscv_stat(addr, MemEndian::Little, stat_t);
    }

    fn get_sigaction(&mut self, addr: u64) -> GenericSigactionArg {
        get_generic_sigaction_64(addr, MemEndian::Little, 0x04000000)
    }

    fn get_mask(&mut self, addr: u64) -> Sigmask {
        todo!()
    }

    fn set_old_sigaction(&mut self, addr: u64, se: SigEntry) {
        todo!()
    }
    fn write_sysinfo_t(&mut self, addr: u64, si: sysinfo) {
        write_riscv_sysinfo(addr, MemEndian::Little, si);
    }
    fn set_altstack(&mut self, addr: u64, si: &SigInfo) {
        todo!()
    }

    fn get_altstack(&mut self, addr: u64) -> GenericStackt {
        todo!()
    }

    fn rt_frame_setup(&mut self, sig: i32, si: &mut SigInfo) {
        todo!()
    }
    fn clone_thread(&mut self, sysin: SyscallIn) -> SyscallOut {
        let flags = sysin.args[0] as i32;
        let stack_addr = sysin.args[1];
        let parent_tid_addr = sysin.args[2];
        let ss_old = block_all_signals();
        let ss_old2 = ss_old.clone();
        let umec = self.user_struct.clone();

        let xlen = self.xlen;
        let regs = self.regs.clone();
        let fregs = self.fregs.clone();
        let pc = self.pc;
        let new_tls = sysin.args[3];
        //let mut ar: Arc<Mutex<u64>> = Arc::new(Mutex::new(0));
        //let mut ar2 = ar.clone();
        let child_tid_addr = sysin.args[4];

        let evt = EventFd::new().unwrap();
        let evt_clone = evt.try_clone().unwrap();
        let k = std::thread::Builder::new()
            .spawn(move || {
                let mut rv = RiscvInt::init_usermode(xlen, umec);
                rv.user_struct.tid_val = gettid() as u64;
                rv.user_struct.flags = flags;
                evt_clone.write(rv.user_struct.tid_val).unwrap();
                //let mut s = ar2.lock();
                //*s = rv.user_struct.tid_val;
                for i in 0..regs.len() {
                    rv.regs[i] = regs[i];
                }
                for i in 0..fregs.len() {
                    rv.fregs[i] = fregs[i];
                }
                rv.pc = pc;
                // 4 is thread pointer
                if flags & CLONE_SETTLS != 0 {
                    rv.regs[4] = new_tls;
                }
                if flags & CLONE_CHILD_SETTID != 0 {
                    panic!();
                }
                if flags & CLONE_CHILD_CLEARTID != 0 {
                    rv.user_struct.ctid_val = child_tid_addr;
                   // panic!();
                }
                rv.regs[RISCV_STACKPOINTER_REG] = stack_addr;
                rv.regs[10] = 0;
                set_mask_block(ss_old2);
                rv.run();

            }).unwrap();
        //let p = k.as_pthread_t() as *mut u64; // todo fix
        let p = evt.read().unwrap();
        if p == 0 {
            panic!();
        }
        let mut sout: SyscallOut = Default::default();
        sout.ret1 = p as u64;
        if flags & CLONE_PARENT_SETTID != 0 {
            self.write32(parent_tid_addr, p as u32, false).unwrap();
        }
        set_mask_block(ss_old);
        return sout;
    }
}