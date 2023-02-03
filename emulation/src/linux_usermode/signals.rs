use std::borrow::BorrowMut;
use std::mem;
use std::ops::Range;
use std::ptr::null_mut;
use std::sync::{Arc, Mutex};
use base::block_signal;
use base::platform::kill;
use libc::{c_int, sigaddset, SIGHUP, SIGCHLD, SIGINT, sigset_t, SIGTERM, SIGALRM, SIGPIPE, SIGKILL, SIGSEGV, stat, SIGFPE, SIGABRT, SIGQUIT, SIGILL, sigaction, SA_NOCLDSTOP, sigfillset, pthread_sigmask, SIG_SETMASK, siginfo_t, SS_DISABLE, SS_ONSTACK, SIGRTMAX, SIGBUS, SA_SIGINFO, sighandler_t, SA_RESTART, SIGWINCH, SIGURG, SIGCONT, SIGSTOP, SIGTSTP, SIGTTIN, SIGTTOU, SIG_IGN, c_void, SIG_DFL, sigsuspend, EPERM, ENOMEM, EINVAL, CLD_EXITED, getpid, SIG_ERR, SA_NODEFER, sigismember, SA_ONSTACK, SIG_BLOCK, SIG_UNBLOCK, SA_RESETHAND, SA_NOCLDWAIT};
use num::Integer;
use crate::common::memory::{flat_mem, MemEndian};
use crate::elf::UserModeRuntime;
use crate::linux_usermode::defs::{SIG_FIRST_INVALID, SigConstants};
use crate::linux_usermode::main::{generic_error_handle, SyscallIn, SyscallOut, UsermodeCpu};

#[derive(Copy, Clone)]
pub struct SigEntry {
    pub handler_func: u64,
    pub is_valid: bool,
    // pub is_extended: bool, // if true, then sa_sigaction instead of sa_handler
    pub maskguest: Sigmask, // of guest
    pub flags: u64, // of guest, we can cvt to host if needed.
    pub sa_restorer: Option<u64>,
}

pub struct SigInfo {
    pub old_mask: sigset_t,
    pub entry: [SigEntry; 64], // by guest
    pub use_idx: Option<usize>,
    pub use_sig: SiginfoWrapper,
    pub ss_sp: u64,
    pub ss_size: u64,
    pub is_32: bool,
    pub cnsts: SigConstants,
    pub current_ss: Sigmask,
    pub sigsuspend_ss: Option<sigset_t>

}

// SS_ONSTACK and SS_DISABLE are same across all archs
pub fn sas_ss_flags(sp: u64, si: &SigInfo) -> i32 {
    if si.ss_size == 0 {
        SS_DISABLE
    } else {
        if on_sig_stack(sp, si) {
            SS_ONSTACK
        } else {
            0
        }
    }
}
pub fn on_sig_stack(sp: u64, si: &SigInfo) -> bool {
    if si.ss_size == 0 {
        false
    } else {
        let r: Range<u64> = si.ss_sp..(si.ss_sp+si.ss_size);
        r.contains(&sp)
    }
}
pub fn target_sigsp(sp: u64, sig_idx: usize, si: &SigInfo) -> u64 {
    let is_onstack_f_set = si.cnsts.check_host_flag_set(si.entry[sig_idx].flags, SA_ONSTACK);
    if is_onstack_f_set && (sas_ss_flags(sp, si) == 0){
        si.ss_sp + si.ss_size
    } else {
        sp
    }
}
impl SigInfo {
    pub fn new() -> SigInfo {
        let mut ss: sigset_t;
        panic!();
    }
}
use std::cell::{RefCell};
use std::collections::HashMap;
use std::cell::UnsafeCell;

use lazy_static::lazy_static;
use crate::common::{IS_LITTLE_ENDIAN, place_variable_guest_fmt_64};
/* lazy_static! {
    static ref SINFO: Mutex<SigInfo> = Mutex::new(SigInfo::new());
} */
thread_local! {
    static SINFO: RefCell<SigInfo> = RefCell::new(SigInfo::new());
    static CHECK_SIG: RefCell<bool> = RefCell::new(false);
  //  static SINFO: Mutex<SigInfo> = Mutex::new(SigInfo::new());
}
// static mut SINFO: Arc<Option<Mutex<SigInfo>>> = Arc::new(None);

#[derive(Copy, Clone,Default)]
pub struct Sigmask {
    vals: [u64; 32],
    real_size: usize // 4 for 32 bit,8 for 64 bit
}
impl Sigmask {
    pub fn read32(addr: u64, end: MemEndian) -> Sigmask {
        let mut flatmem = flat_mem::new_usermode();
        let mut sigret = Sigmask::default();
        let mut realaddr = addr;
        sigret.real_size = 4;
        for i in 0..32 {
            let val = flatmem.read_phys_32(realaddr, end);
            sigret.vals[i] = val as u64;
            realaddr += 4;
        }
        sigret
    }
    pub fn read64(addr: u64, end: MemEndian) -> Sigmask {
        let mut flatmem = flat_mem::new_usermode();
        let mut sigret = Sigmask::default();
        let mut realaddr = addr;
        sigret.real_size = 8;
        for i in 0..16 {
            let val = flatmem.read_phys_64(realaddr, end);
            sigret.vals[i] = val as u64;
            realaddr += 8;
        }
        sigret
    }
    pub fn normalize_to_u32(&self) -> [u32; 32] {
        let mut ret: [u32; 32] = [0; 32];
        for i in 0..32 {
            ret[i] = self.vals[i] as u32; // we do not pack ints so this works
        }
        ret
    }
    pub fn normalize_to_u64(&self) -> [u64; 16] {
        let mut ret: [u64; 16] = [0; 16];
        for i in 0..16 {
            ret[i] = self.vals[i];
        }
        ret
    }
    pub fn is_bit_set(&self, bit: usize) -> bool {

        let real_bits = self.real_size * 4;
        let (amt, idx) = bit.div_rem(&real_bits);
        if (self.vals[idx] & (1 << amt)) != 0 {
            true
        } else {
            false
        }
    }
    pub fn set_bit(&mut self, bit: usize, val: bool) {
        let uval = if val {1} else {0};
        let real_bits = self.real_size * 4;
        let (idx, amt) = bit.div_rem(&real_bits);
        self.vals[idx] &= !(1 << amt);
        self.vals[idx] |= (uval << amt);
    }
    pub fn host_to_guest_sigbits(ss: &sigset_t, cnsts: SigConstants) -> Sigmask {
        let mut ret: Sigmask = Sigmask::default();
        unsafe {
            for i in 1..SIG_FIRST_INVALID {
                let val = sigismember(ss, i);
                if val == 1 {
                    let usethis = cnsts.host_to_guest_sigs[i as usize];
                    ret.set_bit(usethis as usize, true);
                }
                if val < 0 {
                    panic!();
                }
            }

        }
        ret
    }
    pub fn guest_to_host_sigbits(&self, cnsts: SigConstants) -> sigset_t {
        let mut ssret: sigset_t = unsafe { mem::zeroed() };

        for i in 1..SIG_FIRST_INVALID {
            if self.is_bit_set(i as usize) {
                let usethis = cnsts.guest_to_host_sigs[i as usize];
                let ret = unsafe { sigaddset(&mut ssret, usethis) };
                if ret < 0 {
                    panic!();
                }
            }
        }
        ssret
    }
    pub fn transfer_common_sigbits(&self, exist: &mut sigset_t) {
        let arr: [c_int; 11] = [SIGHUP, SIGINT,
            SIGQUIT, SIGILL,
            SIGABRT, SIGFPE,
            SIGKILL, SIGSEGV,
            SIGPIPE, SIGALRM,
            SIGTERM ];
        for i in arr {
            if self.is_bit_set(i as usize) {
                let ret = unsafe { sigaddset(exist, i) };
                if ret < 0 {
                    panic!();
                }
            }
        }
    }

}
pub fn block_all_signals() -> sigset_t {

    let mut old_sigset: sigset_t = unsafe { mem::zeroed() } ;
    let mut all_sig_set: sigset_t = unsafe { mem::zeroed() } ;
    unsafe {
        sigfillset(&mut all_sig_set);
        let ret = pthread_sigmask(SIG_SETMASK, &all_sig_set, &mut old_sigset as *mut sigset_t);
        if ret < 0 {
            panic!(); // for now
        }
    }
    old_sigset

}
pub fn restore_orig_blocked_sigs(s: sigset_t) {
    unsafe {
        let ret = pthread_sigmask(SIG_SETMASK, &s, null_mut());
        if ret < 0 {
            panic!(); // for now
        }
    }
}
pub unsafe extern "C" fn generic_handler(sig: c_int, siginfo: *mut siginfo_t, uctx: *mut c_void ) {
    // On host, kernel signal handling is atomic. But here, it's not, so we need to block all signals
    // we can return to mask defined by signal entry as soon are done with signal init.
    // let sseg = block_all_signals(); we block all signals from the get go, neither one of us knows why
    //let mut sunwrapped = unsafe { SINFO };
    // sunwrapped.with_borrow()
    SINFO.with(|z| {
        let mut val = z.borrow_mut();

        let guestsig = val.cnsts.host_to_guest_sigs[sig as usize];
        // To handle sync symbols (SIGSEIV, SIGBUS) we need to either make use
        // of siglongjmp (undefined on rust) or fiddle with the program counter
        // manually to redirect to arch specific code
        if sig == SIGSEGV || sig == SIGBUS {
            panic!();
        }
        let gensinfo = cvt_host_to_guest_siginfo(&val.cnsts, *siginfo.clone(), val.is_32);
        // val.old_mask = sseg;
        if val.use_idx.is_some() {
            panic!(); // we block signals, how possible?
        }
        val.use_idx = Some(sig as usize);
        val.use_sig = gensinfo;
    });
    //sunwrapped.unwrap().entry
    //let mut val = unsafe { sunwrapped.with() };
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct GenericSIKill {
    pid: i32,
    uid: i32,
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct GenericSISigchld32 {
    pid: i32,
    uid: i32,
    status: i32,
    utime: i32,
    stime: i32,
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct GenericSISigchld64 {
    pid: i32,
    uid: i32,
    status: i32,
    utime: i64,
    stime: i64,

}
#[repr(C)]
#[derive(Copy, Clone)]
pub union GenericSiginfoUnion {
    pub _pad: [i32; 29],
    pub kill: GenericSIKill,
    pub sigchld32: GenericSISigchld32,
    pub sigchld64: GenericSISigchld64
}
// the standard 29 byte pad, suitable for most guests. MIPS (and others?) is different
#[repr(C)]
#[derive(Copy, Clone)]
pub struct GenericSiginfo {
    pub si_signo: i32,
    pub si_errno: i32,
    pub si_code: i32,
    pub aux: GenericSiginfoUnion
}
#[derive(Copy, Clone)]
pub enum SigType {
    None,
    UserKill,
    Sigchld
}
pub const SI_USER: i32 = 0;
pub const SI_TKILL: i32 = -6;

fn cvt_host_to_guest_siginfo(cnsts: &SigConstants, host_siginfo: siginfo_t, is_32bit_guest: bool) -> SiginfoWrapper {
    let mut gen: GenericSiginfo = unsafe { mem::zeroed() };
    let hostsig = host_siginfo.si_signo;
    let guestsig = cnsts.host_to_guest_sigs[hostsig as usize];
    gen.si_code = host_siginfo.si_code;
    gen.si_signo = guestsig;
    gen.si_errno = host_siginfo.si_errno;
    let mut stype = SigType::None;
    unsafe {
        if host_siginfo.si_code == SI_USER || host_siginfo.si_code == SI_TKILL {
            // sig sent on purpose by kill
            stype = SigType::UserKill;
            gen.aux.kill.pid = host_siginfo.si_pid();
            gen.aux.kill.uid = host_siginfo.si_uid() as i32; // both 32 bit
        } else {
            match hostsig {
                SIGCHLD => {
                    stype = SigType::Sigchld;
                    if is_32bit_guest {
                        unimplemented!();
                    } else {
                        gen.aux.sigchld64.pid = host_siginfo.si_pid();
                        gen.aux.sigchld64.uid = host_siginfo.si_uid() as i32;
                        gen.aux.sigchld64.utime = host_siginfo.si_utime();
                        gen.aux.sigchld64.stime = host_siginfo.si_stime();
                        if gen.si_code == CLD_EXITED {
                            gen.aux.sigchld64.status = host_siginfo.si_status();
                        } else {
                            let status = host_siginfo.si_status();
                            let vallower = cnsts.host_to_guest_sigs[(status & 0x7f) as usize];
                            gen.aux.sigchld64.status = vallower | (status & !0x7f);
                        }

                    }
                },
                _ => {
                    unimplemented!();
                }
            }
        }
    }
    SiginfoWrapper {
        stype,
        sinfo: gen
    }

}

pub struct SiginfoWrapper {
    pub stype: SigType,
    pub sinfo: GenericSiginfo
}
pub struct GenericSigactionArg {
    pub handler: u64,
    pub mask: Sigmask, // of guest
    pub flags: u64, // of guest
    pub restorer: Option<u64>
}
// each architecture has their own, but this works as a filler
pub struct GenericStackt {
    pub ss_sp: u64,
    pub ss_flags: i32,
    pub ss_size: u64
}
pub fn fill_generic_stackt(sp: u64, si: &SigInfo) -> GenericStackt {
    let nuflags = sas_ss_flags(sp, si);
    GenericStackt {
        ss_sp: si.ss_sp,
        ss_flags: nuflags,
        ss_size: si.ss_size
    }
}
pub fn inject_signal<T: UsermodeCpu>(cpu: &mut T, sig: i32, si: &mut SigInfo) {
    /*
    if sig == 0 {
        return;
    }
    let ent = si.entry[sig as usize];
    let realsig = si.cnsts.guest_to_host_sigs[sig as usize];
    if ent.handler_func == SIG_DFL as u64 {
        if realsig == SIGTSTP || realsig == SIGTTIN || realsig == SIGTTOU {
            unsafe {
                libc::kill(getpid(), SIGSTOP);
            }
        }
    } else if ent.handler_func == SIG_ERR as u64 { // todo: check if need extend for 32 bit
        panic!();
    } else {
        let mut initalmask = ent.maskguest.guest_to_host_sigbits(si.cnsts.clone());
        let deferbit = si.cnsts.host_to_guest_flags.get(&SA_NODEFER).unwrap();
        if (ent.flags & *deferbit) == 0 {
            unsafe {
                sigaddset(&mut initalmask, realsig);
            }
        }
        let cursig_guest = Sigmask::host_to_guest_sigbits(&si.current_ss, si.cnsts.clone());
        /* let handler_set = if let Some(ss) = si.sigsuspend_ss {
            ss
        } else {
            cursig_guest
        };

         */
        // sigorset
        si.sigsuspend_ss = None;
        if si.cnsts.check_host_flag_set(ent.flags, SA_SIGINFO) {
            unimplemented!();
        } else {
            panic!();
        }
    }

     */
}
pub fn fatal_signal(host_sig: i32) -> bool {
    // true if fatal
    match host_sig {
        SIGWINCH | SIGCHLD |  SIGURG | SIGCONT
        | SIGSTOP | SIGTSTP | SIGTTIN | SIGTTOU => {
            false
        }
        _ => true
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct SigActionRaw64 {
    sa_handler: u64,
    sa_flags: u64,
    sa_restorer: u64,
    sa_mask: u64,

}
pub fn generic_sigaction_write_64(addr: u64, end: MemEndian, se: SigEntry) {
    let mut realstruct: *mut SigActionRaw64 = addr as *mut SigActionRaw64;
    /* place_variable_guest_fmt_64(se.handler_func, &mut realstruct.sa_handler, end);
    place_variable_guest_fmt_64(se.flags as u64, &mut realstruct.sa_flags, end);
    if let Some(x) = se.sa_restorer {
        place_variable_guest_fmt_64(x, &mut realstruct.sa_restorer, end);

    }
    place_variable_guest_fmt_64(se.maskguest.normalize_to_u64()[0],
                                &mut realstruct.sa_mask, end); */

}
pub fn u_sigaction<T: UsermodeCpu>(cpu: &mut T, sysin: SyscallIn,
                                   umr: &mut UserModeRuntime, si: &mut SigInfo) -> SyscallOut {
    let signum = sysin.args[0];
    let newact = sysin.args[1];
    let oldact = sysin.args[2];
    let mut sout: SyscallOut = Default::default();
    let sseg = block_all_signals();
    if oldact != 0 {
        cpu.set_old_sigaction(oldact, si.entry[signum as usize]);
    }
    if newact != 0 {
        let host_sig = si.cnsts.guest_to_host_sigs[signum as usize];
        if host_sig > SIGRTMAX() {
            sout.ret1 = 0;
            generic_error_handle(&mut sout, -EINVAL);
            restore_orig_blocked_sigs(sseg);
            return sout;
        }
        let args = cpu.get_sigaction(newact);
        if host_sig != SIGSEGV && host_sig != SIGBUS {
            let mut hostact: sigaction = unsafe { mem::zeroed() };
            // always use extended handler.
            // Linux puts args in backend anyway regardless of flag
            hostact.sa_flags |= SA_SIGINFO;
            hostact.sa_sigaction = generic_handler as sighandler_t;
            if si.cnsts.check_host_flag_set(args.flags, SA_RESTART) {
                hostact.sa_flags |= SA_RESTART;
            }
            if si.cnsts.check_host_flag_set(args.flags, SA_ONSTACK) {
                unimplemented!();
                hostact.sa_flags |= SA_ONSTACK;
            }
            if si.cnsts.check_host_flag_set(args.flags, SA_RESETHAND) {
                unimplemented!();
                hostact.sa_flags |= SA_RESETHAND; // can we do this here?
            }
            if si.cnsts.check_host_flag_set(args.flags, SA_NOCLDSTOP) {
                unimplemented!();
                hostact.sa_flags |= SA_NOCLDSTOP;
            }
            if si.cnsts.check_host_flag_set(args.flags, SA_NOCLDWAIT) {
                unimplemented!();
                hostact.sa_flags |= SA_NOCLDWAIT;
            }
            if (args.handler == SIG_DFL as u64) && !(fatal_signal(host_sig)){
                hostact.sa_sigaction = SIG_DFL;
                // if fatal, we need to use handler
            }
            // we don't passthough sa_resethand, sa_restorer,
            // because we take care of that manually

            let ret  = unsafe {
                sigaction(host_sig, &hostact, null_mut())
            };
            si.entry[signum as usize] = SigEntry {
                handler_func: args.handler,
                is_valid: true,
                maskguest: args.mask,
                flags: args.flags,
                sa_restorer: args.restorer
            };

            generic_error_handle(&mut sout, ret);
        }
    }
    restore_orig_blocked_sigs(sseg);
    sout
}
pub fn u_sigaltstack<T: UsermodeCpu>(cpu: &mut T, sysin: SyscallIn, umr: &mut UserModeRuntime) -> SyscallOut {
    let ss = sysin.args[0];
    let old_ss = sysin.args[1];
    let mut sout: SyscallOut = Default::default();
    let sseg = block_all_signals();
    SINFO.with(|z| {
        let mut val = z.borrow_mut();
        if old_ss != 0 {
            cpu.set_altstack(old_ss, &val);
        }
        if ss != 0 {
            let gs = cpu.get_altstack(ss);
            let sp = cpu.get_stack_reg();
            if on_sig_stack(sp, &val) {
                sout.is_error = true;
                sout.ret1 = -EPERM as i32 as i64 as u64;
                restore_orig_blocked_sigs(sseg);
                return sout;
            }
            match gs.ss_flags {
                SS_DISABLE => {
                    val.ss_size = 0;
                    val.ss_sp = 0;
                },
                0 | SS_ONSTACK => {
                    if gs.ss_size < val.cnsts.min_sig_stack {
                        sout.is_error = true;
                        sout.ret1 = -ENOMEM as i32 as i64 as u64;
                    } else {
                        val.ss_size = gs.ss_size;
                        val.ss_sp = gs.ss_sp;
                    }
                },
                _ => {
                    sout.is_error = true;
                    sout.ret1 = -EINVAL as i32 as i64 as u64;
                }
            }
        }
        restore_orig_blocked_sigs(sseg);
        sout
    })
    // let mut val = unsafe { sunwrapped.unwrap() };

}
pub fn u_sigsuspend<T: UsermodeCpu>(cpu: &mut T, sysin: SyscallIn, umr: &mut UserModeRuntime) -> SyscallOut {
    let addr = sysin.args[0];
    let msk = cpu.get_mask(addr);
    let sigmsk = msk.guest_to_host_sigbits(umr.sigcnst.lock().clone());
    // if this is called, then signal handler will call, set future direction, then this is released
    // very important that pc for ret addr on sigframe is NEXT instruction, not this one
    let ret  = unsafe {
        sigsuspend(&sigmsk)
    };
    let mut sout: SyscallOut = Default::default();
    generic_error_handle(&mut sout, ret);
    sout
}
// flow: sig handler (or f

pub fn u_rt_sigprocmask(sysin: SyscallIn, ume: &mut UserModeRuntime, sig: &mut SigInfo) -> SyscallOut {
    if ume.is_64 == false {
        unimplemented!();
    }
    let how = sysin.args[0] as c_int; // sig(block//unblock/setmask) is same except for mips, sparc
    let set = sysin.args[1];
    let oldset = sysin.args[2];
    let endian = if ume.is_little_endian { MemEndian::Little } else { MemEndian::Big };
    let oldmask = sig.current_ss;
    let sm = Sigmask::read64(set, endian);
    if how == SIG_SETMASK {
        sig.current_ss = sm;
    } else {
        for i in 0..128 {
            if sm.is_bit_set(i) {
                if how == SIG_BLOCK {
                    sig.current_ss.set_bit(i, true);
                } else if how == SIG_UNBLOCK {
                    sig.current_ss.set_bit(i, false);
                } else {
                    panic!();
                }
            }
        }
    }
    if oldset != 0 {
        let write = oldmask.normalize_to_u64();
        unimplemented!();
    }
    let mut sout: SyscallOut = Default::default();
    sout


}
// for restart, save cpu state befire int. syscall, then execute coode in handler, then returb. flush jit too