use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use libc::{c_int, sigaddset, SIGHUP, SIGINT, sigset_t, SIGTERM, SIGALRM, SIGPIPE, SIGKILL, SIGSEGV, stat, SIGFPE, SIGABRT, SIGQUIT, SIGILL, sigaction, SA_NOCLDSTOP, sysinfo};
use num::Integer;
use crate::common::memory::{flat_mem, MemEndian};
// food for though: if host is i32, signext to i64 then u64.
// the type will already be i32 so we simply cast as i64

pub fn read64_advance_ptr(addr: &mut u64, end: MemEndian) -> u64 {
    let mut flatmem = flat_mem::new_usermode();
    let ret = flatmem.read_phys_64(*addr, end).unwrap();
    *addr += 8;
    ret
}
pub fn read32_advance_ptr(addr: &mut u64, end: MemEndian) -> u32 {
    let mut flatmem = flat_mem::new_usermode();
    let ret = flatmem.read_phys_32(*addr, end).unwrap();
    *addr += 4;
    ret
}
pub fn read16_advance_ptr(addr: &mut u64, end: MemEndian) -> u16 {
    let mut flatmem = flat_mem::new_usermode();
    let ret = flatmem.read_phys_16(*addr, end).unwrap();
    *addr += 2;
    ret
}
pub fn read8_advance_ptr(addr: &mut u64) -> u8 {
    let mut flatmem = flat_mem::new_usermode();
    let ret = flatmem.read_phys_8(*addr).unwrap();
    *addr += 1;
    ret
}
pub fn write16_advance_ptr(addr: &mut u64, val: u16, end: MemEndian) {
    let mut flatmem = flat_mem::new_usermode();
    flatmem.write_phys_16(*addr, val, end).unwrap();
    *addr += 2;
}
pub fn write64_advance_ptr(addr: &mut u64, val: u64, end: MemEndian) {
    let mut flatmem = flat_mem::new_usermode();
    flatmem.write_phys_64(*addr, val, end).unwrap();
    *addr += 8;
}
pub fn write32_advance_ptr(addr: &mut u64, val: u32, end: MemEndian) {
    let mut flatmem = flat_mem::new_usermode();
    flatmem.write_phys_32(*addr, val, end).unwrap();
    *addr += 4;
}
#[derive(Copy, Clone)]
pub struct GenericStat {
    pub st_dev: u64,
    pub st_ino: u64,
    pub st_mode: u64,
    pub st_nlink: u64,
    pub st_uid: u64,
    pub st_gid: u64,
    pub st_rdev: u64,
    pub st_size: i64,
    pub st_blksize: i64,
    pub st_blocks: i64,
    pub st_atime: i64,
    pub st_atime_nsec: i64,
    pub st_mtime: i64,
    pub st_mtime_nsec: i64,
    pub st_ctime: i64,
    pub st_ctime_nsec: i64,
}
pub fn plat2generic_stat(stats: stat) -> GenericStat {
    GenericStat {
        st_dev: stats.st_dev as u64,
        st_ino: stats.st_ino as u64,
        st_mode: stats.st_mode as u64,
        st_nlink: stats.st_nlink as u64,
        st_uid: stats.st_uid as u64,
        st_gid: stats.st_gid as u64,
        st_rdev: stats.st_rdev as u64,
        st_size: stats.st_size as i64,
        st_blksize: stats.st_blksize as i64,
        st_blocks: stats.st_blocks as i64,
        st_atime: stats.st_atime as i64,
        st_atime_nsec: stats.st_atime_nsec as i64,
        st_mtime: stats.st_mtime as i64,
        st_mtime_nsec: stats.st_mtime_nsec as i64,
        st_ctime: stats.st_ctime as i64,
        st_ctime_nsec: stats.st_ctime_nsec as i64,
    }
}
pub const SIG_FIRST_INVALID: i32 = 65; // an invalid value
#[derive(Clone)]
pub struct SigConstants {
    /*  sf_sigio: i32,
    pub sf_sigurg: i32,
    pub sf_sigchld: i32,
    pub sf_sigbus: i32,
    pub sf_sigttin: i32,
    pub sf_sigttou: i32,
    pub sf_sigxcpu: i32,
    pub sf_sigxfsz: i32,
    pub sf_sigvtalrm: i32,
    pub sf_sigprof: i32,
    pub sf_sigwinch: i32,
    pub sf_sigusr1: i32,
    pub sf_sigusr2: i32,
    pub sf_sigcont: i32,
    pub sf_sigstop: i32,
    pub sf_sigtstp: i32,
    pub sf_sigstkflt: i32,
    pub sf_sigsys: i32,
    pub sf_sigpoll: i32,
    pub sf_sigpwr: i32,

     */
    pub min_sig_stack: u64,
    pub guest_to_host_sigs: Vec<i32>,
    pub host_to_guest_sigs: Vec<i32>,
    pub guest_to_host_flags: HashMap<i32, i32>,
    pub host_to_guest_flags: HashMap<i32, i32>,

}
impl Default for SigConstants {
    fn default() -> Self {
        SigConstants {
            min_sig_stack: 0,
            guest_to_host_sigs: vec![],
            host_to_guest_sigs: vec![],
            guest_to_host_flags: Default::default(),
            host_to_guest_flags: Default::default()
        }
    }
}
impl SigConstants {
    pub fn check_host_flag_set(&self, guest_val: u64, host_flag: i32) -> bool {
        if let Some(maskval) = self.host_to_guest_flags.get(&host_flag) {
            if guest_val & (*maskval as u64) != 0 {
                true
            } else {
                false
            }
        } else {
            false
        }
    }
}
pub fn snyth_sigconst(h2gs: Vec<i32>, h2gf: HashMap<i32, i32>) -> SigConstants {
    let mut guest_to_host_sigs: Vec<i32> = vec![0; 64];
    for (i, v) in h2gs.iter().enumerate() {
        guest_to_host_sigs[*v as usize] = i as i32;
    }
    let mut ret = SigConstants {
        min_sig_stack: 0,
        guest_to_host_sigs,
        host_to_guest_sigs: h2gs,
        guest_to_host_flags: Default::default(),
        host_to_guest_flags: h2gf,
    };
    // panic!();
    ret
}
pub fn write_sysinfo_generic64(addr: u64, end: MemEndian, si: sysinfo) {
    let mut realaddr = addr;
    write64_advance_ptr(&mut realaddr, si.uptime as u64, end);
    for i in 0..3 {
        write64_advance_ptr(&mut realaddr, si.loads[i] as u64, end);
    }
    write64_advance_ptr(&mut realaddr, si.totalram as u64, end);
    write64_advance_ptr(&mut realaddr, si.freeram as u64, end);
    write64_advance_ptr(&mut realaddr, si.sharedram as u64, end);
    write64_advance_ptr(&mut realaddr, si.bufferram as u64, end);
    write64_advance_ptr(&mut realaddr, si.totalswap as u64, end);
    write64_advance_ptr(&mut realaddr, si.freeswap as u64, end);
    write16_advance_ptr(&mut realaddr, si.procs as u16, end);
    write16_advance_ptr(&mut realaddr, si.pad as u16, end);
    write64_advance_ptr(&mut realaddr, si.totalhigh as u64, end);
    write64_advance_ptr(&mut realaddr, si.freehigh as u64, end);
    write32_advance_ptr(&mut realaddr, si.mem_unit as u32, end);

}