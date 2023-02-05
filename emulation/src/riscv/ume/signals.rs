use std::collections::HashMap;
use std::mem;
use libc::{SA_NODEFER, SA_RESTART, SIGABRT, SIGALRM, SIGBUS, SIGCHLD, SIGCONT, SIGFPE, SIGHUP, SIGILL, SIGINT, SIGIO, SIGKILL, SIGPIPE, SIGPROF, SIGPWR, SIGQUIT, SIGSEGV, SIGSTKFLT, SIGSTOP, SIGSYS, SIGTRAP, SIGTSTP, SIGTTIN, SIGTTOU, SIGURG, SIGUSR1, SIGUSR2, SIGVTALRM, SIGWINCH, SIGXCPU, SIGXFSZ};
use crate::common::memory::{flat_mem, MemEndian};
use crate::linux_usermode::defs::{SigConstants, snyth_sigconst};
use crate::linux_usermode::signals::{fill_generic_stackt, GenericSigactionArg, GenericSiginfo, on_sig_stack, SigInfo, target_sigsp};
use crate::riscv::interpreter::consts::CSR_FCSR_ADDRESS;
use crate::riscv::interpreter::main::RiscvInt;

#[repr(C)]
#[derive(Copy, Clone)]
pub union RiscvDFloatCtx {
    pub fpr: [u64; 32],
    pub fcsr: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union Riscv64FprCtx {
    bytes: [u8; 528],
    pub dfloat: RiscvDFloatCtx,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Riscv64SigCtx {
    pub pc: u64,
    pub gpr: [u64; 31], // no zero reg
    pub fpr: Riscv64FprCtx

}
impl Default for Riscv64SigCtx {
    fn default() -> Self {
        Riscv64SigCtx {
            pc: 0,
            gpr: [0; 31],
            fpr: unsafe {mem::zeroed() }
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Riscv64RtSigframe {
    info: GenericSiginfo,
    uctx: Riscv64Uctx
}
fn get_sigframe_64(ri: &mut RiscvInt, si: &SigInfo, framesize: u64, sig: i32) -> u64 {
    // guest sig
    let sp = ri.get_stack_reg();
    if on_sig_stack(sp, si) && !on_sig_stack(sp - framesize, si) {
        return u64::max_value();
    }
    let mut nsp = target_sigsp(sp, sig as usize, si) - framesize;
    nsp &= !0xf;
    nsp
}
pub fn riscv64_setup_sigctx(ri: &mut RiscvInt) -> Riscv64SigCtx {
    let mut rsc: Riscv64SigCtx = Default::default();
    rsc.pc = if let Some(ss) = ri.want_pc {
        ri.want_pc = None; // we will overwrite
        ss
    } else {
        ri.get_pc_of_current_instr()
    };
    for i in 1..32 {
        rsc.gpr[i - 1] = ri.regs[i];
    }
    for i in 0..32 {
        unsafe {
            rsc.fpr.dfloat.fpr[i] = ri.fregs[i];
        }
    }
    let fscr = ri.get_csr_raw(CSR_FCSR_ADDRESS) as u32;
    rsc.fpr.dfloat.fcsr = fscr;
    rsc
}
pub fn riscv64_setup_uctx(ri: &mut RiscvInt, si: &SigInfo, idx: usize) -> Riscv64Uctx {
    let mut uctx: Riscv64Uctx = Default::default();
    uctx.stackt = riscv64_setup_stackt(ri, si);
    let arr = si.entry[idx].maskguest.normalize_to_u64();
    uctx.sigset =arr;
    uctx.sctx = riscv64_setup_sigctx(ri);
    uctx

}
impl RiscvInt {

}
pub fn riscv64_setup_stackt(ri: &mut RiscvInt, si: &SigInfo) -> RiscvStackt {
    let gen = fill_generic_stackt(ri.get_stack_reg(), si);
    RiscvStackt {
        ss_sp: gen.ss_sp,
        ss_flags: gen.ss_flags,
        ss_size: gen.ss_size
    }
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct RiscvStackt {
    ss_sp: u64,
    ss_flags: i32,
    ss_size: u64
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct Riscv64Uctx {
    flags: u64,
    link: u64,
    stackt: RiscvStackt,
    sigset: [u64; 16],
    pad: [u8; 8],
    sctx: Riscv64SigCtx

}
pub fn setup_rt_frame(ri: &mut RiscvInt, sig: i32, si: &mut SigInfo) {
    let fsize = mem::size_of::<Riscv64RtSigframe>();
    let isize = mem::size_of::<GenericSiginfo>() as u64;

    let addr = get_sigframe_64(ri, si, fsize as u64, sig);
    let mut rt: *mut Riscv64RtSigframe = addr as *mut Riscv64RtSigframe;
    unsafe {
        (*rt).uctx = riscv64_setup_uctx(ri, si, sig as usize);
        (*rt).info = si.use_sig.as_mut().unwrap().sinfo;
    }
    ri.stop_exec = true;
    ri.want_pc = Some(si.entry[sig as usize].handler_func);
    // todo: write stryct
    ri.regs[2] = addr as u64; // sp
    ri.regs[10] = sig as u64; // a0
    ri.regs[11] = addr as u64; // a1 for siginfo, which is at begiinning
    ri.regs[12] = (addr + isize) as u64;
    ri.regs[1] = ri.user_struct.sig_tramp; // ra
    panic!();

}
pub fn riscv64_init_sigconstant() -> SigConstants {
    // 2048 min
    let mut host_to_guest_sigs: Vec<i32> = vec![0; 64];
    host_to_guest_sigs[SIGHUP as usize] = 1;
    host_to_guest_sigs[SIGINT as usize] = 2;
    host_to_guest_sigs[SIGQUIT as usize] = 3;
    host_to_guest_sigs[SIGILL as usize] = 4;
    host_to_guest_sigs[SIGTRAP as usize] = 5;
    host_to_guest_sigs[SIGABRT as usize] = 6;
    host_to_guest_sigs[SIGBUS as usize] = 7;
    host_to_guest_sigs[SIGFPE as usize] = 8;
    host_to_guest_sigs[SIGKILL as usize] = 9;
    host_to_guest_sigs[SIGUSR1 as usize] = 0xA;
    host_to_guest_sigs[SIGSEGV as usize] = 0xB;
    host_to_guest_sigs[SIGUSR2 as usize] = 0xc;
    host_to_guest_sigs[SIGPIPE as usize] = 0xd;
    host_to_guest_sigs[SIGALRM as usize] = 0xe;
    host_to_guest_sigs[SIGSTKFLT as usize] = 0x10;
    host_to_guest_sigs[SIGCHLD as usize] = 0x11;
    host_to_guest_sigs[SIGCONT as usize] = 0x12;
    host_to_guest_sigs[SIGSTOP as usize] = 0x13;
    host_to_guest_sigs[SIGTSTP as usize] = 0x14;
    host_to_guest_sigs[SIGTTIN as usize] = 0x15;
    host_to_guest_sigs[SIGTTOU as usize] = 0x16;
    host_to_guest_sigs[SIGURG as usize] = 0x17;
    host_to_guest_sigs[SIGXCPU as usize] = 0x18;
    host_to_guest_sigs[SIGXFSZ as usize] = 0x19;
    host_to_guest_sigs[SIGVTALRM as usize] = 0x1a;
    host_to_guest_sigs[SIGPROF as usize] = 0x1b;
    host_to_guest_sigs[SIGWINCH as usize] = 0x1c;
    host_to_guest_sigs[SIGIO as usize] = 0x1d;
    host_to_guest_sigs[SIGPWR as usize] = 0x1e;
    host_to_guest_sigs[SIGSYS as usize] = 0x1f;
    let mut host_to_guest_flags: HashMap<i32, i32> = HashMap::new();
    host_to_guest_flags.insert(SA_RESTART, 0x10000000);
    host_to_guest_flags.insert(SA_NODEFER, 0x40000000);
    let mut ret = snyth_sigconst(host_to_guest_sigs, host_to_guest_flags);
    ret.min_sig_stack = 2048;
    ret
}