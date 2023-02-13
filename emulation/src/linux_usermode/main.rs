use std::ffi::{CStr, CString};
use std::{mem, ptr};
use std::mem::MaybeUninit;
use std::ops::Add;
use std::sync::Arc;
use base::{debug, errno_result, pagesize, sys};
use base::platform::MemoryMapping;
use libc::{c_char, c_int, c_void, clock_gettime, clock_settime, clockid_t, close, EINVAL, ENOMEM, faccessat, fcntl, fd_set, fstatat, getuid, geteuid, iovec, lseek, MAP_ANON, MAP_ANONYMOUS, MAP_FAILED, MAP_FIXED, MAP_PRIVATE, MAP_SHARED, mprotect, off_t, open, openat, PROT_EXEC, PROT_READ, PROT_WRITE, read, readv, sigaction, sigset_t, size_t, ssize_t, SYS_exit_group, SYS_set_tid_address, syscall, time_t, timespec, timeval, uname, TCGETS, utsname, write, writev, TIOCGPGRP, TIOCGWINSZ, winsize, ioctl, SOCK_NONBLOCK, socketpair, ppoll, pollfd, c_short, c_long, socket, clone, SYS_clone, CLONE_VM, pipe2, sysinfo, fstat, posix_fadvise64, off64_t, fchown, uid_t, gid_t, mode_t, fchmod, utimensat, SYS_lookup_dcookie, dup3, O_CLOEXEC, getgid, setgid, setuid, sendfile, bind, sockaddr, socklen_t, sendto, recvfrom, ITIMER_REAL, itimerval, SYS_setitimer, SYS_getitimer, connect, listen, ftruncate, getpid, getppid, pid_t, getpgid, getsid, kill, SYS_getdents64, dirent64, truncate, statx, c_uint, F_SETLK, F_GETFL, F_SETFL, F_GETFD, F_SETFD, rlimit, getrlimit, __rlimit_resource_t, readlink, getrandom, prlimit64, rlimit64, readlinkat, SYS_futex, termios, ETIMEDOUT, sched_getaffinity, cpu_set_t};
use crate::common::genfunc::round_up;
use crate::elf::{MachineType, UserModeRuntime};
use libc::mmap;
use resources::{AddressRange, Alloc};
use sync::Mutex;
use crate::common::{host_guest_endian_mismatch, IS_LITTLE_ENDIAN};
use crate::common::memory::MemEndian;
use crate::linux_usermode::defs::{GenericStat, plat2generic_stat};
use crate::linux_usermode::signals::{GenericSigactionArg, GenericStackt, SigEntry, SigInfo, Sigmask, SINFO, u_sigaction};

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum SyscallType {
    Access,
    Brk,
    Tid,
    Writev,
    ExitGroup,
    Uname,
    Faccessat,
    Openat,
    Open,
    Fstatat,
    Read,
    Prlimit64,
    Mmap,
    Close,
    Mprotect,
    Munmap,
    Write,
    SetTidAddr,
    Fcntl,
    Fcntl64,
    Readv,
    Sigaction,
    Lseek,
    ClockGetTime,
    ClockSetTime,
    Getuid,
    Geteuid,
    Ioctl,
    Socketpair,
    Ppoll,
    Socket,
    RtSigprocmask,
    Sigprocmask,
    Clone,
    Pipe2,
    Sysinfo,
    Fstat,
    Fadvise64,
    Fchown,
    Fchmod,
    Utimensat,
    LookupDcookie,
    Dup3,
    Getgid,
    Setuid,
    Setgid,
    Sendfile,
    Bind,
    Sendto,
    Recvfrom,
    Setitimer,
    Getitimer,
    Connect,
    Listen,
    Ftruncate,
    Truncate,
    Getpid,
    Getppid,
    Getpgid,
    Getsid,
    Kill,
    Getdents64,
    ArmSetTls,
    Mmap2,
    Statx,
    SetRobustList,
    Rseq,
    Getrlimit,
    Readlink,
    Readlinkat,
    Getrandom,
    Futex,
    Gettid,
    Getaffinity,
}
#[derive(Copy, Clone, PartialEq)]
pub struct SyscallIn {
    pub syscall: SyscallType,
    pub args: [u64; 7],
}
#[derive(Copy, Clone, PartialEq, Default)]
pub struct SyscallOut {
    pub ret1: u64,
    pub ret2: Option<u64>,
    pub is_error: bool,

}
fn fix_path(path: &str, ptr: *const c_char) -> String {
    let oldpath = unsafe {
        CStr::from_ptr(ptr).to_string_lossy().clone().to_string()
    };
    let mut newstr: String = String::new();
    if oldpath.starts_with("/etc")
        || oldpath.starts_with("/usr")
        || oldpath.starts_with("/var")
        || oldpath.starts_with("/lib")
        || oldpath.starts_with("/sbin") {
        newstr.push_str(path);
    }
    newstr.push_str(oldpath.as_str());
    newstr

}
fn generic_error_handle_maxarch_int(sysout: &mut SyscallOut, res: i64, is_64: bool) {
    // if the value return from a syscall is the highest size int, this is the function to handle
    if res < 0 {
        sysout.is_error = true;
        let err = base::Error::last();
        sysout.ret1 = -err.errno() as i64 as u64; // we can cut off lower 32 bits if 32 bit guest
    } else {
        sysout.ret1 = if is_64 {
            res as i64 as u64
        } else {
            res as i32 as i64 as u64
        };
    }
}
pub fn generic_error_handle(sysout: &mut SyscallOut, res: c_int) {
    if res < 0 {
        sysout.is_error = true;
        let err = base::Error::last();
        sysout.ret1 = -err.errno() as i64 as u64;
    } else {
        sysout.ret1 = res as i32 as i64 as u64;
    }
}
pub fn u_faccess_at(sysin: SyscallIn, umr: &mut UserModeRuntime) -> SyscallOut {
    let fd = sysin.args[0];
    let path = sysin.args[1] as *const c_char;
    let amode = sysin.args[2];
    let flags = sysin.args[3];
    let mut sout: SyscallOut = Default::default();
    let newpath = CString::new(fix_path(umr.str_path.as_str(), path)).unwrap();
    let res = unsafe {
        faccessat(fd as c_int, newpath.as_ptr(), amode as c_int, flags as c_int)
    };
    generic_error_handle(&mut sout, res);
    sout
}
pub fn u_fchown(sysin: SyscallIn, umr: &mut UserModeRuntime) -> SyscallOut {
    let fd = sysin.args[0];
    let owner = sysin.args[1];
    let group = sysin.args[2];
    let res = unsafe {
        fchown(fd as c_int, owner as uid_t, group as gid_t)
    };
    let mut sysout: SyscallOut = Default::default();
    generic_error_handle(&mut sysout, res);
    sysout
}
pub fn u_futex(sysin: SyscallIn, umr: &mut UserModeRuntime) -> SyscallOut {
    // todo: assuming 64-bit little endian on both
    let fduaddr = sysin.args[0];
    let fop = sysin.args[1];
    let val = sysin.args[2];
    let timeout = sysin.args[3];
    let uaddr2 = sysin.args[4];
    let val3 = sysin.args[5];
    if val == 2 {
        // glibc sometimes calls this even in single threaded program
        let changeval: *mut u32 = fduaddr as *mut u32;
        unsafe {
            *changeval = 0;
        }
    }
    let res = unsafe {
        syscall(SYS_futex, fduaddr as u32, fop as u32,
                val as u32, timeout, uaddr2 as u32, val3 as u32)
    };
    let mut sysout: SyscallOut = Default::default();
    generic_error_handle_maxarch_int(&mut sysout, res, true);
    sysout
}
pub fn u_getaffinity(sysin: SyscallIn, umr: &mut UserModeRuntime) -> SyscallOut {
    /// todo: diff endian/bitsize
    let pid = sysin.args[0];
    let cpusetsize = sysin.args[1];
    let mask = sysin.args[2];

    let res = unsafe {
        sched_getaffinity(pid as pid_t,
                          cpusetsize as size_t, mask as *mut cpu_set_t)
    };
    let mut sysout: SyscallOut = Default::default();
    generic_error_handle(&mut sysout, res);
    sysout
}
pub fn u_fchmod(sysin: SyscallIn, umr: &mut UserModeRuntime) -> SyscallOut {
    let fd = sysin.args[0];
    let mode = sysin.args[1];
    let res = unsafe {
        fchmod(fd as c_int, mode as mode_t)
    };
    let mut sysout: SyscallOut = Default::default();
    generic_error_handle(&mut sysout, res);
    sysout
}
pub fn u_fstat<T: UsermodeCpu>(sysin: SyscallIn, cpu: &mut T) -> SyscallOut {
    // let umr = cpu.get_ume();
    let fd = sysin.args[0];
    let bufptr = sysin.args[1];
    let mut pstat  = MaybeUninit::<libc::stat>::zeroed();

    let res = unsafe {
        fstat(fd as c_int, pstat.as_mut_ptr())
    };
    let mut sysout: SyscallOut = Default::default();

    generic_error_handle(&mut sysout, res);
    if sysout.is_error {
        return sysout;
    }
    let prstat = unsafe {
        pstat.assume_init()
    };
    let gstat = plat2generic_stat(prstat);
    cpu.write_stat_t(bufptr, gstat);
    sysout
}
pub fn u_fstat_at<T: UsermodeCpu>(sysin: SyscallIn, cpu: &mut T) -> SyscallOut {
    let umr = cpu.get_ume();
    let fd = sysin.args[0];
    let path = sysin.args[1] as *const c_char;
    let bufptr = sysin.args[2];
    let flags = sysin.args[3];
    let mut sysout: SyscallOut = Default::default();
    let newpath = CString::new(fix_path(umr.str_path.as_str(), path)).unwrap();
    let mut pstat  = MaybeUninit::<libc::stat>::zeroed();
    let res = unsafe {
        fstatat(fd as c_int, newpath.as_ptr(), pstat.as_mut_ptr(), flags as c_int)
    };
    if res < 0 {
        sysout.is_error = true;
        let err = base::Error::last();
        sysout.ret1 = -err.errno() as i64 as u64;
        return sysout;
    } else {
        sysout.ret1 = res as i32 as i64 as u64;
    }
    let prstat = unsafe {
        pstat.assume_init()
    };
    let gstat = plat2generic_stat(prstat);
    cpu.write_stat_t(bufptr, gstat);
    sysout
}
pub fn u_clone<T: UsermodeCpu>(sysin: SyscallIn, cpu: &mut T) -> SyscallOut {
   // clone()
    // we will not support cris/s390x for the forseeable future
    let umr = cpu.get_ume();
    let flags = sysin.args[0] as i32;
    let stack_addr = sysin.args[1];
    let parent_tid_addr = sysin.args[2];
    let (tls, child_tid) = match umr.machine_type {
        MachineType::Riscv => {
            (sysin.args[3], sysin.args[4])
        }
      _ => panic!()
    };
    if flags & CLONE_VM != 0 {
        //we are sharing mem space. This means that changes in new process will effect
        // old ones. We have to make a copy of the CPU context so the workings
        // of the new process don't override the old

    }
    panic!();
    //syscall(SYS_clone)

}
pub fn u_fadvise64(sysin: SyscallIn, umr: &mut UserModeRuntime) -> SyscallOut {
    // todo: make sure we filter top 32 bits on 32 bit guests at syscall call time
    let fd = sysin.args[0];
    let off = sysin.args[1];
    let len = sysin.args[2];
    let advice = sysin.args[3];
    let res = unsafe {
        posix_fadvise64(fd as c_int, off as off64_t, len as off64_t, advice as c_int)
    };
    let mut sout: SyscallOut = Default::default();
    generic_error_handle(&mut sout, res);
    sout
}
pub fn u_access(sysin: SyscallIn, umr: &mut UserModeRuntime) -> SyscallOut {
    let path = sysin.args[0];
    let mode = sysin.args[1];
    let newpath = CString::new(
        fix_path(umr.str_path.as_str(), path as *const c_char)
    ).unwrap();
    let mut sout: SyscallOut = Default::default();
    let res = unsafe {
        libc::access(newpath.as_ptr(), mode as c_int)
    };
    generic_error_handle(&mut sout, res);
    sout
}
pub fn u_open(sysin: SyscallIn, umr: &mut UserModeRuntime) -> SyscallOut {
    let path = sysin.args[0];
    let flags = sysin.args[1];
    let amode = sysin.args[2];
    let newpath = CString::new(
        fix_path(umr.str_path.as_str(), path as *const c_char)
    ).unwrap();
    let mut sout: SyscallOut = Default::default();

    let res = unsafe {
        libc::open(newpath.as_ptr(),  flags as c_int, amode as c_int)
    };
    generic_error_handle(&mut sout, res);
    sout
}
pub fn u_openat(sysin: SyscallIn, umr: &mut UserModeRuntime) -> SyscallOut {
    let dirfd = sysin.args[0];
    let path = sysin.args[1];
    let flags = sysin.args[2];
    let amode = sysin.args[3];
    let newpath = CString::new(
        fix_path(umr.str_path.as_str(), path as *const c_char)
    ).unwrap();
    let mut sout: SyscallOut = Default::default();
    let res = unsafe {
        openat(dirfd as c_int, newpath.as_ptr(), flags as c_int, amode as c_int)
    };
    generic_error_handle(&mut sout, res);
    sout
}
pub fn u_statx(sysin: SyscallIn, umr: &mut UserModeRuntime) -> SyscallOut {
    let endian = if umr.is_little_endian { MemEndian::Little } else { MemEndian::Big };
    if host_guest_endian_mismatch(endian) {
        panic!();
    }
    let dirfd = sysin.args[0];
    let path = sysin.args[1];
    let flags = sysin.args[2];
    let mask = sysin.args[3];
    let statsbux = sysin.args[4];
    let newpath = CString::new(
        fix_path(umr.str_path.as_str(), path as *const c_char)
    ).unwrap();
    let mut sout: SyscallOut = Default::default();
    let res = unsafe {
        libc::statx(dirfd as c_int,
                    newpath.as_ptr(), flags as c_int, mask as c_uint,
                    statsbux as *mut statx)
    };
    generic_error_handle(&mut sout, res);
    sout
}
pub fn u_lookup_dcookie(sysin: SyscallIn, umr: &mut UserModeRuntime) -> SyscallOut {
    let cookie = sysin.args[0];
    let buf = sysin.args[1];
    let len = sysin.args[2];
    let mut sout: SyscallOut = Default::default();
    let res = unsafe {
        syscall(SYS_lookup_dcookie, cookie, buf, len)
    };
    generic_error_handle(&mut sout, res as i32);
    sout
}
pub fn u_getrandom(sysin: SyscallIn, umr: &mut UserModeRuntime) -> SyscallOut {
    let buf = sysin.args[0];
    let buflen = sysin.args[1];
    let flags = sysin.args[2];
    let mut sout: SyscallOut = Default::default();
    let res = unsafe {
        getrandom(buf as *mut c_void, buflen as size_t, flags as c_uint)
    };
    generic_error_handle_maxarch_int(&mut sout, res as i64, umr.is_64);
    sout
}
pub fn u_readlinkat(sysin: SyscallIn, umr: &mut UserModeRuntime) -> SyscallOut {
    let dirfd = sysin.args[0];
    let path = sysin.args[1];
    let buf = sysin.args[1];
    let bufs = sysin.args[2];
    let mut sout: SyscallOut = Default::default();
    let newpath = CString::new(
        fix_path(umr.str_path.as_str(), path as *const c_char)
    ).unwrap();
    let res = unsafe {
        readlinkat(dirfd as c_int, newpath.as_ptr(),
                   buf as *mut c_char, bufs as size_t)
    };
    generic_error_handle_maxarch_int(&mut sout, res as i64, umr.is_64);
    sout
}
pub fn u_readlink(sysin: SyscallIn, umr: &mut UserModeRuntime) -> SyscallOut {
    let path = sysin.args[0];
    let buf = sysin.args[1];
    let bufs = sysin.args[2];
    let mut sout: SyscallOut = Default::default();
    let newpath = CString::new(
        fix_path(umr.str_path.as_str(), path as *const c_char)
    ).unwrap();
    let res = unsafe {
        readlink(newpath.as_ptr(), buf as *mut c_char, bufs as size_t)
    };
    generic_error_handle_maxarch_int(&mut sout, res as i64, umr.is_64);
    sout
}
pub fn u_lseek(sysin: SyscallIn, umr: &mut UserModeRuntime) -> SyscallOut {
    let fd = sysin.args[0];
    let offset = sysin.args[1];
    let whence = sysin.args[2];
    // flags same for all archs
    let mut sout: SyscallOut = Default::default();
    let res = unsafe {
        lseek(fd as c_int, offset as off_t, whence as c_int)
    };
    generic_error_handle_maxarch_int(&mut sout, res as i64, true);
    sout
}
pub fn u_sysinfo<T: UsermodeCpu>(sysin: SyscallIn, cpu: &mut T) -> SyscallOut {
    let mut sinfo: sysinfo = unsafe { mem::zeroed() };
    let addr = sysin.args[0];
    let res = unsafe {
        sysinfo(&mut sinfo)
    };
    let mut sout = SyscallOut::default();
    generic_error_handle(&mut sout, res);
    if sout.is_error {
        return sout;
    }
    cpu.write_sysinfo_t(addr, sinfo);
    sout

}
pub fn u_pipe2(sysin: SyscallIn, umr: &mut UserModeRuntime) -> SyscallOut {
    let mut pipe_fds = [-1; 2];
    let addr = sysin.args[0];
    let flags = sysin.args[1];
    let endian = if umr.is_little_endian { MemEndian::Little } else { MemEndian::Big };

    let res = unsafe {
        pipe2(&mut pipe_fds[0], flags as c_int)
    };
    let mut sout = SyscallOut::default();
    generic_error_handle(&mut sout, res);
    if sout.is_error {
        return sout;
    }
    umr.mem_access.write_phys_32(addr, pipe_fds[0] as u32, endian);
    umr.mem_access.write_phys_32(addr + 4, pipe_fds[1] as u32, endian);

    return sout;

}
pub fn u_kill(sysin: SyscallIn, umr: &mut UserModeRuntime) -> SyscallOut {
    let pid = sysin.args[0]; // todo: signal significane
    let sig = sysin.args[1];
    let res = unsafe {
        kill(pid as pid_t, sig as c_int)
    };
    let mut sout: SyscallOut = Default::default();
    generic_error_handle(&mut sout, res);
    sout
}
pub fn u_mprotect(sysin: SyscallIn, umr: &mut UserModeRuntime) -> SyscallOut {
    // todo: promote signed len's from i32 to i64 if guest is 32-bit
    let addr = sysin.args[0];
    let len = sysin.args[1];
    let prot = sysin.args[2];
    let res = unsafe {
        mprotect(addr as *mut c_void, len as size_t, prot as c_int)
    };
    let mut sout: SyscallOut = Default::default();
    generic_error_handle(&mut sout, res);
    sout
}
pub fn u_ioctl(sysin: SyscallIn, umr: &mut UserModeRuntime) -> SyscallOut {
    let fd = sysin.args[0];
    let endian = if umr.is_little_endian { MemEndian::Little } else { MemEndian::Big };

    let host_ioctl: u64 = sysin.args[1];
    let mut sout: SyscallOut = Default::default();

    match host_ioctl {
        TCGETS => {
            // todo: fix if different endian/host on guest/host
            let addr = sysin.args[2];
            let mut str: *mut termios = addr as *mut termios;
            let ret = unsafe { ioctl(fd as c_int, TCGETS, str) };
            generic_error_handle(&mut sout, ret);
            if ret < 0 {
                return sout;
            }
        }
        TIOCGWINSZ => {
            let mut ws: winsize = unsafe { mem::zeroed() };
            let ret = unsafe { ioctl(fd as c_int, TIOCGWINSZ, &mut ws) };
            generic_error_handle(&mut sout, ret);
            if ret < 0 {
                return sout;
            }
            let addr = sysin.args[2];
            umr.mem_access.write_phys_16(addr, ws.ws_row, endian);
            umr.mem_access.write_phys_16(addr + 2, ws.ws_col, endian);
            umr.mem_access.write_phys_16(addr + 4, ws.ws_xpixel, endian);
            umr.mem_access.write_phys_16(addr + 6, ws.ws_ypixel, endian);

        },
        TIOCGPGRP => {
            let mut pt: pid_t = unsafe { mem::zeroed() };
            let ret = unsafe { ioctl(fd as c_int, TIOCGPGRP, &mut pt) };
            generic_error_handle(&mut sout, ret);
            if ret < 0 {
                return sout;
            }
            let addr = sysin.args[2];
            umr.mem_access.write_phys_32(addr, pt as u32, endian);

        }
        _ => panic!()
    }
    sout
}
pub fn u_mmap2(sysin: SyscallIn, umr: &mut UserModeRuntime) -> SyscallOut {
    let mut modsysin = sysin;
    modsysin.args[5] *= 4096;
    u_mmap(modsysin, umr)
}
pub fn u_munmap(sysin: SyscallIn, umr: &mut UserModeRuntime) -> SyscallOut {
    let addr = sysin.args[0];
    let len = sysin.args[1];
    // for now, don't bother to dellocate from mem struture
    let mut sout: SyscallOut = Default::default();
    let retval = unsafe {
        libc::munmap(addr as *mut c_void, len as size_t)
    };
    if retval == (-1 as c_int) {
        let err = base::Error::last();
        sout.ret1 = -err.errno() as i64 as u64;
        sout.is_error = true;
        return sout;
    }
    sout.ret1 = retval as u64;
    return sout;
}
pub fn u_mmap(sysin: SyscallIn, umr: &mut UserModeRuntime) -> SyscallOut {
    let mut ms = umr.memstate.lock();
    let pmask = umr.pagesize_mask;
    let addr = sysin.args[0];
    let mut sout: SyscallOut = Default::default();
    let len = sysin.args[1];
    if ((addr != 0) && (addr & pmask != 0)) || (len & pmask != 0) {
       // sout.ret1 = -EINVAL as i64 as u64;
       // sout.is_error = true;
       // return sout;
    }
    let fd = if umr.is_64 {
        sysin.args[4] as c_int
    } else {
        sysin.args[4] as i32 as i64 as c_int // can be -1
    };
    let offset = sysin.args[5] as off_t;
    let guest_prot = sysin.args[2];
    let mut guest_flags = sysin.args[3] as c_int;
    let mut guest_wants_fixed: bool = false;
    if (guest_flags & (MAP_FIXED as i32)) != 0 {
        guest_wants_fixed = true;
    }
    let mut finalmmapaddr: u64 = addr;
    let var = ms.anon_idx;
    ms.anon_idx += 1;
    if !guest_wants_fixed {
        // custom addr
        let res = if umr.heap_grow_down {
            ms.mmap_region.as_mut().unwrap().reverse_allocate(len, Alloc::Anon(var), String::new())
        } else {
            ms.mmap_region.as_mut().unwrap().allocate(len, Alloc::Anon(var), String::new())
        };
        if let Ok(addr) = res {
            finalmmapaddr = addr;
        } else {
            sout.ret1 = -ENOMEM as i64 as u64;
            sout.is_error = true;
            return sout;
        }
    } else {
        let size = round_up(len, umr.guest_pagesize); // todo: necessary?
        let origadr = AddressRange::from_start_and_size(addr, size).unwrap();
        let res = ms.mmap_region.as_mut().unwrap()
            .allocate_at(
                origadr,
                Alloc::Anon(var), String::new());
        if let Err(er) = res {
            if let resources::Error::ExistingAlloc(d) = er {
                let mut ralloc = d;
                loop {
                    let range = ms.mmap_region.as_mut().unwrap().release(ralloc).unwrap();
                    let newalloc = range.wide_net(origadr);
                    let resl = ms.mmap_region.as_mut().unwrap()
                        .allocate_at(
                            newalloc,
                            Alloc::Anon(var), String::new()).unwrap();
                    break;
                }
            }
        }
        // todo: since this is fixed, do we really care if it fails?
        // purpose of mem allocator is make sure non-fixed allocations don't overlap with fixed.
        // We really need to make sure any area covered by the mmap fixeed is reserved
    }
    guest_flags |= MAP_FIXED;
    let retval = unsafe {
        libc::mmap(finalmmapaddr as *mut libc::c_void,
                   len as usize, guest_prot as c_int
                   , guest_flags as c_int, fd, offset)
    };
    if retval == libc::MAP_FAILED {
        ms.mmap_region.as_mut().unwrap().release_containing(finalmmapaddr).unwrap();
        let err = base::Error::last();
        sout.ret1 = -err.errno() as i64 as u64;
        sout.is_error = true;
        return sout;
    }
    sout.ret1 = retval as u64;
    return sout;

}
pub fn u_gettid(sysin: SyscallIn, ume: &mut UserModeRuntime) -> SyscallOut {
    let tid = unsafe { libc::gettid() };
    debug!("tid system call: pid is {:x}", tid);
    SyscallOut {
        ret1: tid as u64,
        .. Default::default()
    }
}
pub fn u_exit_group(sysin: SyscallIn, ume: &mut UserModeRuntime) -> ! {
    let status = sysin.args[0];
    unsafe {
        syscall(SYS_exit_group, status)
    };
    unreachable!();
}
pub fn u_uname(sysin: SyscallIn, ume: &mut UserModeRuntime) -> SyscallOut {
    // todo: return arch specific string
    let addr = sysin.args[0];
    let retval = unsafe {
        uname(addr as *mut utsname)
    };
    let mut sysout = SyscallOut::default();
    generic_error_handle(&mut sysout, retval);
    sysout
}
pub fn u_set_tid_address(sysin: SyscallIn, ume: &mut UserModeRuntime) -> SyscallOut {
    let ptr = sysin.args[0];
    let endian = if ume.is_little_endian { MemEndian::Little } else { MemEndian::Big };
    let mut getval: c_int = ume.mem_access.read_phys_32(ptr, endian) as c_int;
    let retval = unsafe {
        syscall(SYS_set_tid_address, &mut getval)
    };
    let mut sysout = SyscallOut::default();
    sysout.ret1 = retval as u64;
    sysout
}
pub fn u_fcntl64(sysin: SyscallIn, ume: &mut UserModeRuntime) -> SyscallOut {
    let cmd = sysin.args[1] as c_int;

    if cmd == F_GETFL || cmd == F_SETFL || cmd == F_GETFD || cmd == F_SETFD {
        u_fcntl(sysin, ume)
    } else {
        unimplemented!();
    }
}
pub fn u_fcntl(sysin: SyscallIn, ume: &mut UserModeRuntime) -> SyscallOut {
    let fd = sysin.args[0] as c_int;
    let cmd = sysin.args[1] as c_int;
    let arg = sysin.args[2];
    if cmd == F_GETFL || cmd == F_SETFL || cmd == F_GETFD || cmd == F_SETFD {
        let retval = unsafe {
            fcntl(fd as c_int, cmd as c_int, arg as c_int)
        };
        let mut sysout = SyscallOut::default();
        generic_error_handle(&mut sysout, retval);
        sysout
    } else {
        unimplemented!();
    }

}
pub fn u_close(sysin: SyscallIn, ume: &mut UserModeRuntime) -> SyscallOut {
    let fd = sysin.args[0];
    let retval = unsafe {
        close(fd as c_int)
    };
    let mut sysout = SyscallOut::default();
    generic_error_handle(&mut sysout, retval);
    sysout
}
pub fn u_read(sysin: SyscallIn, ume: &mut UserModeRuntime) -> SyscallOut {
    let fd = sysin.args[0];
    let addr = sysin.args[1];
    let cnt = sysin.args[2];
    let retval = unsafe {
        read(fd as c_int, addr as *mut c_void, cnt as size_t)
    };
    let mut sysout = SyscallOut::default();
    if retval < 0 {
        sysout.is_error = true;
        let err = base::Error::last();
        sysout.ret1 = -err.errno() as i64 as u64;
    } else {
        sysout.ret1 = if ume.is_64 {
            retval as i64 as u64
        } else {
            retval as i32 as i64 as u64
        };
    }
    sysout
}
pub fn u_write(sysin: SyscallIn, ume: &mut UserModeRuntime) -> SyscallOut {
    let fd = sysin.args[0];
    let addr = sysin.args[1];
    let cnt = sysin.args[2];
    let retval = unsafe {
        write(fd as c_int, addr as *mut c_void, cnt as size_t)
    };
    let mut sysout = SyscallOut::default();
    generic_error_handle_maxarch_int(&mut sysout, retval as i64, ume.is_64);
    sysout
}
pub fn u_getuid(sysin: SyscallIn, ume: &mut UserModeRuntime) -> SyscallOut {
    let mut sout: SyscallOut = Default::default();
    let retval = unsafe {
        getuid()
    };
    sout.ret1 = retval as u64;
    sout
}
pub fn u_getpid(sysin: SyscallIn, ume: &mut UserModeRuntime) -> SyscallOut {
    let mut sout: SyscallOut = Default::default();
    let retval = unsafe {
        getpid()
    };
    sout.ret1 = retval as u64;
    sout
}
pub fn u_getppid(sysin: SyscallIn, ume: &mut UserModeRuntime) -> SyscallOut {
    let mut sout: SyscallOut = Default::default();
    let retval = unsafe {
        getppid()
    };
    sout.ret1 = retval as u64;
    sout
}
pub fn u_getpgid(sysin: SyscallIn, ume: &mut UserModeRuntime) -> SyscallOut {
    let mut sout: SyscallOut = Default::default();
    let pid = sysin.args[0];

    let retval = unsafe {
        getpgid(pid as pid_t)
    };
    generic_error_handle(&mut sout, retval);
    sout
}
pub fn u_getsid(sysin: SyscallIn, ume: &mut UserModeRuntime) -> SyscallOut {
    let mut sout: SyscallOut = Default::default();
    let pid = sysin.args[0];

    let retval = unsafe {
        getsid(pid as pid_t)
    };
    generic_error_handle(&mut sout, retval);
    sout
}
pub fn u_getgid(sysin: SyscallIn, ume: &mut UserModeRuntime) -> SyscallOut {
    let mut sout: SyscallOut = Default::default();
    let retval = unsafe {
        getgid()
    };
    sout.ret1 = retval as u64;
    sout

}
pub fn u_setgid(sysin: SyscallIn, ume: &mut UserModeRuntime) -> SyscallOut {
    let mut sout: SyscallOut = Default::default();
    let gid = sysin.args[0];
    let retval = unsafe {
        setgid(gid as gid_t)
    };
    sout.ret1 = retval as u64;
    sout

}
pub fn u_setuid(sysin: SyscallIn, ume: &mut UserModeRuntime) -> SyscallOut {
    let mut sout: SyscallOut = Default::default();
    let uid = sysin.args[0];
    let retval = unsafe {
        setuid(uid as gid_t)
    };
    sout.ret1 = retval as u64;
    sout

}
pub fn u_geteuid(sysin: SyscallIn, ume: &mut UserModeRuntime) -> SyscallOut {
    let mut sout: SyscallOut = Default::default();
    let retval = unsafe {
        geteuid()
    };
    sout.ret1 = retval as u64;
    sout

}
pub fn u_socket(sysin: SyscallIn, ume: &mut UserModeRuntime) -> SyscallOut {
    let mut sout: SyscallOut = Default::default();
    let domain = sysin.args[0];
    let typeval = sysin.args[1];
    let protocol = sysin.args[2];
    let retval = unsafe {
        socket(domain as c_int, typeval as c_int, protocol as c_int)
    };
    generic_error_handle(&mut sout, retval);
    sout

}
pub fn u_bind(sysin: SyscallIn, ume: &mut UserModeRuntime) -> SyscallOut {
    let endian = if ume.is_little_endian { MemEndian::Little } else { MemEndian::Big };
    if host_guest_endian_mismatch(endian) {
        panic!();
    }
    let sockfd = sysin.args[0];
    let addr = sysin.args[1];
    let addrlen = sysin.args[2];
    let mut sout: SyscallOut = Default::default();
    let retval = unsafe {
        bind(sockfd as c_int, addr as *const sockaddr, addrlen as socklen_t)
    };
    generic_error_handle(&mut sout, retval);
    sout
}
pub fn u_listen(sysin: SyscallIn, ume: &mut UserModeRuntime) -> SyscallOut {
    let mut sout: SyscallOut = Default::default();
    let sockfd = sysin.args[0];
    let backlog = sysin.args[1];
    let retval = unsafe {
        listen(sockfd as c_int, backlog as c_int)
    };
    generic_error_handle(&mut sout, retval);
    sout
}
pub fn u_connect(sysin: SyscallIn, ume: &mut UserModeRuntime) -> SyscallOut {
    let endian = if ume.is_little_endian { MemEndian::Little } else { MemEndian::Big };
    if host_guest_endian_mismatch(endian) {
        unimplemented!();
    }
    let mut sout: SyscallOut = Default::default();
    let sockfd = sysin.args[0];
    let sockaddr = sysin.args[1];
    let addrlen = sysin.args[2];
    let retval = unsafe {
        connect(sockfd as c_int, sockaddr as *const sockaddr, addrlen as socklen_t)
    };
    generic_error_handle(&mut sout, retval);
    sout
}
pub fn u_sendto(sysin: SyscallIn, ume: &mut UserModeRuntime) -> SyscallOut {
    let endian = if ume.is_little_endian { MemEndian::Little } else { MemEndian::Big };
    if host_guest_endian_mismatch(endian) {
        unimplemented!();
    }
    let socket = sysin.args[0];
    let message = sysin.args[1];
    let length = sysin.args[2];
    let flags = sysin.args[3];
    let dest_addr = sysin.args[4];
    let dest_len = sysin.args[5];

    let mut sout: SyscallOut = Default::default();
    let retval = unsafe {
        sendto(socket as c_int, message  as *const c_void, length as size_t,
               flags as c_int, dest_addr as *const sockaddr,
               dest_len as socklen_t)
    };
    generic_error_handle_maxarch_int(&mut sout, retval as i64, ume.is_64);
    sout
}
pub fn u_recvfrom(sysin: SyscallIn, ume: &mut UserModeRuntime) -> SyscallOut {
    let endian = if ume.is_little_endian { MemEndian::Little } else { MemEndian::Big };
    if host_guest_endian_mismatch(endian) {
        panic!();
    }
    let socket = sysin.args[0];
    let buf = sysin.args[1];
    let length = sysin.args[2];
    let flags = sysin.args[3];
    let addr = sysin.args[4];
    let addr_len = sysin.args[5];

    let mut sout: SyscallOut = Default::default();
    let retval = unsafe {
        recvfrom(socket as c_int, buf as *mut c_void, length as size_t,
               flags as c_int, addr as *mut sockaddr,
               addr_len as *mut socklen_t)
    };
    generic_error_handle_maxarch_int(&mut sout, retval as i64, ume.is_64);
    sout
}
pub fn u_socketpair(sysin: SyscallIn, ume: &mut UserModeRuntime) -> SyscallOut {
    let mut sout: SyscallOut = Default::default();
    let domain = sysin.args[0];
    let typeval = sysin.args[1];
    let protocol = sysin.args[2];
    let endian = if ume.is_little_endian { MemEndian::Little } else { MemEndian::Big };

    let arrayaddr = if ume.is_64 {
        sysin.args[3]
    } else {
        sysin.args[3] as u32 as u64
    };
    let mut resarr: [c_int; 2] = [0; 2];
    let retval = unsafe {
        socketpair(domain as c_int, typeval as c_int, protocol as c_int, &mut resarr[0])
    };
    generic_error_handle(&mut sout, retval);
    if retval < 0 {
        return sout;
    }
    ume.mem_access.write_phys_32(arrayaddr, resarr[0] as u32, endian);
    ume.mem_access.write_phys_32(arrayaddr + 4, resarr[1] as u32, endian);
    sout

}
pub fn u_clock_gettime(sysin: SyscallIn, ume: &mut UserModeRuntime) -> SyscallOut {
    let clk_id = sysin.args[0];
    let tpaddr = sysin.args[1];
    let mut sout: SyscallOut = Default::default();
    let endian = if ume.is_little_endian { MemEndian::Little } else { MemEndian::Big };

    let mut timespec: timespec = unsafe { mem::zeroed() };
    let ret = unsafe {
        clock_gettime(clk_id as clockid_t, &mut timespec)
    };
    generic_error_handle_maxarch_int(&mut sout, ret as i64, ume.is_64);
    if ret < 0 {
        return sout;
    }
    if ume.is_64 {
        ume.mem_access.write_phys_64(tpaddr, timespec.tv_sec as u64, endian);
        ume.mem_access.write_phys_64(tpaddr + 8, timespec.tv_nsec as u64, endian);
    } else {
        ume.mem_access.write_phys_32(tpaddr, timespec.tv_sec as u32, endian);
        ume.mem_access.write_phys_32(tpaddr + 4, timespec.tv_nsec as u32, endian);
    };
    return sout;
}
pub fn u_ppoll(sysin: SyscallIn, ume: &mut UserModeRuntime) -> SyscallOut {
    let fds = sysin.args[0];
    let nfds = sysin.args[1];
    let timeout = sysin.args[2];
    let mut sout: SyscallOut = Default::default();
    let endian = if ume.is_little_endian { MemEndian::Little } else { MemEndian::Big };
    let mut pfdvec: Vec<libc::pollfd> = Vec::new();
    let mut curraddr = fds;
    for _ in 0..nfds {
        let f = ume.mem_access.read_phys_32(curraddr, endian);
        let events = ume.mem_access.read_phys_16(curraddr + 4, endian);
        let revents = ume.mem_access.read_phys_16(curraddr + 6, endian);
        pfdvec.push(pollfd {
            fd: f as c_int,
            events: events as c_short,
            revents: revents as c_short
        });
        curraddr += 8;
    }
    let mut timeo : timespec = unsafe { mem::zeroed() };
    let mut set_time: bool = false;
    if timeout != 0 {
        set_time = true;
        if ume.is_64 {
            let fsec = ume.mem_access.read_phys_64(timeout, endian);
            let nsec = ume.mem_access.read_phys_64(timeout + 4, endian);
            timeo = timespec {
                tv_sec: fsec as time_t,
                tv_nsec: nsec as c_long
            };
        } else {
            let fsec = ume.mem_access.read_phys_32(timeout, endian);
            let nsec = ume.mem_access.read_phys_32(timeout + 2, endian);
            timeo = timespec {
                tv_sec: fsec as time_t,
                tv_nsec: nsec as c_long
            };
        }
    }
    let tptr: *const timespec = if set_time {
        &mut timeo
     } else {
        ptr::null_mut()
    };
    let ret = unsafe {
        ppoll(pfdvec.as_ptr() as *mut _, nfds, tptr, ptr::null_mut())
    };
    generic_error_handle(&mut sout, ret);
    return sout;
}
// ARM64_SYS_PRLIMIT64
pub fn u_prlimit64(sysin: SyscallIn, ume: &mut UserModeRuntime) -> SyscallOut {
    // todo: if we support anything other than 64 bit or little endian guest/host, change this
    let pid = sysin.args[0];
    let res = sysin.args[1];
    //let mut new: rlimit = unsafe {mem::zeroed()};
    //let mut old: rlimit = unsafe {mem::zeroed()};
    let newaddr = sysin.args[2];
    let oldaddr = sysin.args[3];
    let new_limit: *const rlimit64 = newaddr as *const rlimit64;
    let old_limit: *mut rlimit64 = oldaddr as *mut rlimit64;
    let mut sout: SyscallOut = Default::default();
    let res = unsafe {
        prlimit64(pid as pid_t, res as __rlimit_resource_t, new_limit, old_limit)
    };
    generic_error_handle(&mut sout, res);
    sout

}
pub fn u_getrlimit(sysin: SyscallIn, ume: &mut UserModeRuntime) -> SyscallOut {
    let res = sysin.args[0];
    let rlim = sysin.args[1];
    let mut rrl: rlimit = unsafe {mem::zeroed()};
    let mut sout: SyscallOut = Default::default();
    let endian = if ume.is_little_endian { MemEndian::Little } else { MemEndian::Big };
    let res = unsafe {
        getrlimit(res as __rlimit_resource_t, &mut rrl)
    };
    generic_error_handle(&mut sout, res);
    if sout.is_error {
        return sout;
    }
    if ume.is_64 {
        ume.mem_access.write_phys_64(rlim, rrl.rlim_cur, endian);
        ume.mem_access.write_phys_64(rlim + 8, rrl.rlim_max, endian);
    } else {
        ume.mem_access.write_phys_32(rlim, rrl.rlim_cur as u32, endian);
        ume.mem_access.write_phys_32(rlim + 4, rrl.rlim_max as u32, endian);
    }
    return sout;
}
pub fn u_utimensat(sysin: SyscallIn, ume: &mut UserModeRuntime) -> SyscallOut {
    let dirfd = sysin.args[0];
    let path = sysin.args[1];
    let times = sysin.args[2];
    let flags = sysin.args[3];
    let mut sout: SyscallOut = Default::default();

    let endian = if ume.is_little_endian { MemEndian::Little } else { MemEndian::Big };
    let newpath = if path != 0 {
        CString::new(
            fix_path(ume.str_path.as_str(), path as *const c_char)
        ).unwrap()
    } else {
        CString::new("").unwrap()
    };
    let tpaddr= times;
    let (s1, n1,s2,n2) = if tpaddr == 0 {
        (0,0,0,0)
    } else if ume.is_64 {
            let s1 = ume.mem_access.read_phys_64(tpaddr, endian);
            let n1 = ume.mem_access.read_phys_64(tpaddr + 8,  endian);
            let s2 = ume.mem_access.read_phys_64(tpaddr + 16, endian);
            let n2 = ume.mem_access.read_phys_64(tpaddr + 24, endian);
            (s1, n1, s2, n2)
    } else {
            let s1 = ume.mem_access.read_phys_32(tpaddr, endian) as i32 as i64 as u64;
            let n1 = ume.mem_access.read_phys_32(tpaddr + 4,  endian) as i32 as i64 as u64;
            let s2 = ume.mem_access.read_phys_32(tpaddr + 8, endian) as i32 as i64 as u64;
            let n2 = ume.mem_access.read_phys_32(tpaddr + 12,  endian) as i32 as i64 as u64;
            (s1, n1, s2, n2)
    };
    let mut ts: [timespec ; 2] = unsafe {mem::zeroed()};
    ts[0] = timespec {
        tv_sec: s1 as time_t,
        tv_nsec: n1 as c_long
    };
    ts[1] = timespec {
        tv_sec: s2 as time_t,
        tv_nsec: n2 as c_long
    };
    let tptr: *const timespec = if times != 0 {
        &mut ts[0]
    } else {
        ptr::null_mut()
    };
    let pptr: *const c_char = if path != 0 {
        newpath.as_ptr()
    } else {
        ptr::null_mut()
    };
    let ret = unsafe {
        utimensat(dirfd as c_int, pptr, tptr, flags as c_int)
    };
    generic_error_handle(&mut sout, ret);
    return sout;
}
pub fn u_getitimer(sysin: SyscallIn, ume: &mut UserModeRuntime) -> SyscallOut {
    let endian = if ume.is_little_endian { MemEndian::Little } else { MemEndian::Big };
    if host_guest_endian_mismatch(endian) {
        unimplemented!();
    }
    let mut sout: SyscallOut = Default::default();

    let which = sysin.args[0];
    let currval = sysin.args[1];
    let ret = unsafe {
        syscall(SYS_getitimer, which, currval)
    };
    generic_error_handle(&mut sout, ret as c_int); // we know ret is supposed to be int
    return sout;

}
pub fn u_getdents64(sysin: SyscallIn, ume: &mut UserModeRuntime) -> SyscallOut {
    let endian = if ume.is_little_endian { MemEndian::Little } else { MemEndian::Big };
    if host_guest_endian_mismatch(endian) {
        unimplemented!();
    }
    let fd = sysin.args[0];
    let dirp = sysin.args[1];
    let count = sysin.args[2];
    let mut sout: SyscallOut = Default::default();
    let ret = unsafe {
        syscall(SYS_getdents64, fd, dirp, count)
    };
    generic_error_handle(&mut sout, ret as c_int); // we know ret is supposed to be int
    return sout;
}
pub fn u_setitimer(sysin: SyscallIn, ume: &mut UserModeRuntime) -> SyscallOut {
    let endian = if ume.is_little_endian { MemEndian::Little } else { MemEndian::Big };
    if host_guest_endian_mismatch(endian) {
        unimplemented!();
    }
    let which = sysin.args[0];
    let newval = sysin.args[1];
    let oldval = sysin.args[2];
    let mut olds: itimerval = unsafe { mem::zeroed() };
    let mut sout: SyscallOut = Default::default();

    let ret = unsafe {
        syscall(SYS_setitimer, which, newval, oldval)
    };
    generic_error_handle(&mut sout, ret as c_int); // we know ret is supposed to be int
    return sout;

}
pub fn u_ftruncate(sysin: SyscallIn, ume: &mut UserModeRuntime) -> SyscallOut {
    let fildes = sysin.args[0];
    let length = sysin.args[1];
    let mut sout: SyscallOut = Default::default();

    let ret = unsafe {
        ftruncate(fildes as c_int, length as off_t)
    };
    generic_error_handle(&mut sout, ret as c_int); // we know ret is supposed to be int
    return sout;
}
pub fn u_truncate(sysin: SyscallIn, ume: &mut UserModeRuntime) -> SyscallOut {
    let path = sysin.args[0] as *const c_char;
    let length = sysin.args[1];
    let mut sout: SyscallOut = Default::default();
    let newpath = CString::new(fix_path(ume.str_path.as_str(), path)).unwrap();
    let ret = unsafe {
        truncate(newpath.as_ptr(), length as off_t)
    };
    generic_error_handle(&mut sout, ret as c_int); // we know ret is supposed to be int
    return sout;
}
pub fn u_clock_settime(sysin: SyscallIn, ume: &mut UserModeRuntime) -> SyscallOut {
    let clk_id = sysin.args[0];
    let tpaddr = sysin.args[1];
    let mut sout: SyscallOut = Default::default();
    let endian = if ume.is_little_endian { MemEndian::Little } else { MemEndian::Big };
    let (tv_sec, tv_nsec) = if ume.is_64 {
        let s = ume.mem_access.read_phys_64(tpaddr, endian);
        let n = ume.mem_access.read_phys_64(tpaddr + 8,  endian);
        (s, n)
    } else {
        let s = ume.mem_access.read_phys_32(tpaddr, endian) as i32 as i64 as u64;
        let n = ume.mem_access.read_phys_32(tpaddr + 4,  endian) as i32 as i64 as u64 ;
        (s, n)
    };
    let mut timespec: timespec = timespec {
        tv_sec: tv_sec as time_t,
        tv_nsec: tv_nsec as time_t
    };
    let ret = unsafe {
        clock_settime(clk_id as clockid_t, &mut timespec)
    };
    generic_error_handle(&mut sout, ret);
    return sout;
}
pub fn u_sendfile(sysin: SyscallIn, ume: &mut UserModeRuntime) -> SyscallOut {
    let outfd = sysin.args[0];
    let infd = sysin.args[1];
    let offset = sysin.args[2];
    let count = sysin.args[3];
    let ret = unsafe {
        sendfile(outfd as c_int, infd as c_int, offset as *mut off_t, count as size_t)
    };
    let mut sout: SyscallOut = Default::default();

    generic_error_handle_maxarch_int(&mut sout, ret as i64, ume.is_64);
    return sout;
}
pub fn u_dup3(sysin: SyscallIn, ume: &mut UserModeRuntime) -> SyscallOut {
    let oldfd = sysin.args[0];
    let newfd = sysin.args[1];
    let flags = sysin.args[2];
    let mut sout: SyscallOut = Default::default();
    let ret = unsafe {
        dup3(oldfd as c_int, newfd as c_int, flags as c_int)
    };
    // todo: constants for o_cloexec, different for some libcs
    generic_error_handle(&mut sout, ret);
    return sout;
}
pub fn u_readv(sysin: SyscallIn, ume: &mut UserModeRuntime) -> SyscallOut {
    let fd = sysin.args[0];
    let initaladdr = sysin.args[1];
    let cnt = sysin.args[2];
    let mut sout: SyscallOut = Default::default();
    let endian = if ume.is_little_endian { MemEndian::Little } else { MemEndian::Big };
    let mut iovecarr: Vec<iovec> = Vec::new();
    let mut curraddr = initaladdr;
    for _ in 0..cnt {
        let (base, len) = if ume.is_64 {
            let b = ume.mem_access.read_phys_64(curraddr, endian);
            let l = ume.mem_access.read_phys_64(curraddr + 8, endian);
            curraddr += 16;
            (b, l)
        } else {
            let b = ume.mem_access.read_phys_32(curraddr, endian) as u64;
            let l = ume.mem_access.read_phys_32(curraddr + 4, endian) as u64;
            curraddr += 8;
            (b, l)
        };
        iovecarr.push(iovec {
            iov_base: base as *mut c_void,
            iov_len: len as size_t
        });
    }
    let ret = unsafe {
        readv(fd as c_int, iovecarr.as_ptr() as *mut _, iovecarr.len() as c_int)
    };
    generic_error_handle_maxarch_int(&mut sout, ret as i64, ume.is_64);
    return sout;
}
pub fn u_writev(sysin: SyscallIn, ume: &mut UserModeRuntime) -> SyscallOut {
    let fd = sysin.args[0];
    let initaladdr = sysin.args[1];
    let cnt = sysin.args[2];
    let mut sout: SyscallOut = Default::default();
    let endian = if ume.is_little_endian { MemEndian::Little } else { MemEndian::Big };
    let mut iovecarr: Vec<iovec> = Vec::new();
    let mut curraddr = initaladdr;
    for _ in 0..cnt {
        let (base, len) = if ume.is_64 {
            let b = ume.mem_access.read_phys_64(curraddr, endian);
            let l = ume.mem_access.read_phys_64(curraddr + 8, endian);
            curraddr += 16;
            (b, l)
        } else {
            let b = ume.mem_access.read_phys_32(curraddr, endian) as u64;
            let l = ume.mem_access.read_phys_32(curraddr + 4, endian) as u64;
            curraddr += 8;
            (b, l)
        };
        iovecarr.push(iovec {
            iov_base: base as *mut c_void,
            iov_len: len as size_t
        });
    }
    let ret = unsafe {
        writev(fd as c_int, iovecarr.as_ptr() as *mut _, iovecarr.len() as c_int)
    };
    generic_error_handle_maxarch_int(&mut sout, ret as i64, ume.is_64);
    return sout;
}
pub fn u_brk(sysin: SyscallIn, ume: &mut UserModeRuntime) -> SyscallOut {
    let new_val = sysin.args[0];
    let mut ms = ume.memstate.lock();
    let new_value_page = round_up(new_val, ume.guest_pagesize);
    debug!("brk system call: current start is {:x}, current end is {:x}, value passed is {:x}",
            ms.brk, ms.brk_max, new_val);
    let mut sout: SyscallOut = Default::default();
    if new_val == 0 {
        // return current br;
        sout.ret1 = ms.brk;
    } else if new_val <= ms.brk_max {
        // todo - below orig brk? bugcheck
        ms.brk = new_val;
        sout.ret1 = new_val;
    } else {
        let size = (new_value_page - ms.brk_max);
        let maddr = unsafe {
            libc::mmap(ms.brk_max as *mut libc::c_void,
                       size as size_t,
                       PROT_READ | PROT_WRITE | PROT_EXEC, MAP_FIXED | MAP_ANONYMOUS | MAP_PRIVATE, -1, 0)
        };
        if maddr == MAP_FAILED {
            sout.is_error = true;
            sout.ret1 = -ENOMEM as i64 as u64;
            return sout;
        }
        let mut memmap = MemoryMapping {
            addr: maddr as *mut u8,
            size: size as usize,
        };
        ms.mem_maps.push(memmap);
        ms.brk_max = new_value_page;
        ms.brk = new_val;
        sout.ret1 = ms.brk;
    }
    return sout;
}
pub fn dispatch<T: UsermodeCpu>(cpu: &mut T, sysin: SyscallIn) -> SyscallOut {

    match sysin.syscall {
        SyscallType::Brk => u_brk(sysin, cpu.get_ume()),
        SyscallType::Writev => u_writev(sysin, cpu.get_ume()),
        SyscallType::ExitGroup => u_exit_group(sysin, cpu.get_ume()),
        SyscallType::Uname => u_uname(sysin, cpu.get_ume()),
        SyscallType::Faccessat => u_faccess_at(sysin, cpu.get_ume()),
        SyscallType::Open => u_open(sysin, cpu.get_ume()),
        SyscallType::Openat => u_openat(sysin, cpu.get_ume()),
        SyscallType::Fstatat => u_fstat_at(sysin, cpu),
        SyscallType::Read => u_read(sysin, cpu.get_ume()),
        SyscallType::Mmap => u_mmap(sysin, cpu.get_ume()),
        SyscallType::Close => u_close(sysin, cpu.get_ume()),
        SyscallType::Mprotect => u_mprotect(sysin, cpu.get_ume()),
        SyscallType::Write => u_write(sysin, cpu.get_ume()),
        SyscallType::SetTidAddr => u_set_tid_address(sysin, cpu.get_ume()),
        SyscallType::Fcntl => u_fcntl(sysin, cpu.get_ume()),
        SyscallType::Readv => u_readv(sysin, cpu.get_ume()),
        SyscallType::Lseek => u_lseek(sysin, cpu.get_ume()),
        SyscallType::Sigprocmask | SyscallType::RtSigprocmask => {
            // Technically, we don't have to actually block all signals
            // Just keep track of which ones the guest program doesn't want
            SyscallOut::default()
        }
        SyscallType::Sigaction  => {
            // nop for now
            /*SINFO.with(|z| {
                let mut k = z.borrow_mut();
                u_sigaction(cpu, sysin, &mut k)
            })
             */
            SyscallOut::default()
        }
        SyscallType::ClockSetTime => {
            u_clock_settime(sysin, cpu.get_ume())
        }
        SyscallType::ClockGetTime => {
            u_clock_gettime(sysin, cpu.get_ume())
        }
        SyscallType::Geteuid => u_geteuid(sysin, cpu.get_ume()),
        SyscallType::Getuid => u_getuid(sysin, cpu.get_ume()),
        SyscallType::Ioctl => u_ioctl(sysin, cpu.get_ume()),
        SyscallType::Socketpair => u_socketpair(sysin, cpu.get_ume()),
        SyscallType::Ppoll => u_ppoll(sysin, cpu.get_ume()),
        SyscallType::Socket => u_socket(sysin,cpu.get_ume()),
        SyscallType::Clone => u_clone(sysin, cpu),
        SyscallType::Pipe2 => u_pipe2(sysin, cpu.get_ume()),
        SyscallType::Sysinfo => u_sysinfo(sysin, cpu),
        SyscallType::Fstat => u_fstat(sysin, cpu),
        SyscallType::Fadvise64 => u_fadvise64(sysin, cpu.get_ume()),
        SyscallType::Fchown => u_fchown(sysin, cpu.get_ume()),
        SyscallType::Fchmod => u_fchmod(sysin, cpu.get_ume()),
        SyscallType::Utimensat => u_utimensat(sysin, cpu.get_ume()),
        SyscallType::LookupDcookie => u_lookup_dcookie(sysin, cpu.get_ume()),
        SyscallType::Dup3 => u_dup3(sysin, cpu.get_ume()),
        SyscallType::Getgid => u_getgid(sysin, cpu.get_ume()),
        SyscallType::Setgid => u_setgid(sysin, cpu.get_ume()),
        SyscallType::Setuid => u_setuid(sysin, cpu.get_ume()),
        SyscallType::Sendfile => u_sendfile(sysin, cpu.get_ume()),
        SyscallType::Bind => u_bind(sysin, cpu.get_ume()),
        SyscallType::Sendto => u_sendto(sysin, cpu.get_ume()),
        SyscallType::Recvfrom => u_recvfrom(sysin, cpu.get_ume()),
        SyscallType::Getitimer => u_getitimer(sysin, cpu.get_ume()),
        SyscallType::Setitimer => u_setitimer(sysin, cpu.get_ume()),
        SyscallType::Connect => u_connect(sysin, cpu.get_ume()),
        SyscallType::Listen => u_listen(sysin, cpu.get_ume()),
        SyscallType::Ftruncate => u_ftruncate(sysin, cpu.get_ume()),
        SyscallType::Getpid => u_getpid(sysin, cpu.get_ume()),
        SyscallType::Getppid => u_getppid(sysin, cpu.get_ume()),
        SyscallType::Getpgid => u_getpgid(sysin, cpu.get_ume()),
        SyscallType::Getsid => u_getsid(sysin, cpu.get_ume()),
        SyscallType::Kill => u_kill(sysin, cpu.get_ume()),
        SyscallType::Getdents64 => u_getdents64(sysin, cpu.get_ume()),
        SyscallType::ArmSetTls => {
            cpu.get_ume().tls_base = sysin.args[0];
            SyscallOut::default()
        }
        SyscallType::Mmap2 => u_mmap2(sysin, cpu.get_ume()),
        SyscallType::Truncate => u_truncate(sysin, cpu.get_ume()),
        SyscallType::Access => u_access(sysin, cpu.get_ume()),
        SyscallType::Statx => u_statx(sysin, cpu.get_ume()),
        SyscallType::Munmap => u_munmap(sysin, cpu.get_ume()),
        SyscallType::Fcntl64 => u_fcntl64(sysin, cpu.get_ume()),
        SyscallType::SetRobustList => {
            SyscallOut::default()
        }
        SyscallType::Getaffinity => {
            u_getaffinity(sysin, cpu.get_ume())
        }
        SyscallType::Rseq => {
            let mut s = SyscallOut::default();
            s.ret1 = -EINVAL as i32 as i64 as u64;
            s.is_error = true;
            s
        }
        SyscallType::Getrlimit => u_getrlimit(sysin, cpu.get_ume()),
        SyscallType::Readlink => u_readlink(sysin, cpu.get_ume()),
        SyscallType::Getrandom => u_getrandom(sysin, cpu.get_ume()),
        SyscallType::Prlimit64 => u_prlimit64(sysin, cpu.get_ume()),
        SyscallType::Readlinkat => u_readlinkat(sysin, cpu.get_ume()),
        SyscallType::Gettid => u_gettid(sysin, cpu.get_ume()),
        SyscallType::Futex => {
            u_futex(sysin, cpu.get_ume())
        }
        _ => {
            panic!("unimpl syscall");
        },
    }
}
pub trait UsermodeCpu {
    fn push_stack_natural(&mut self, val: u64);
    fn pop_stack_natural(&mut self) -> u64;
    fn get_stack_reg(&mut self) -> u64;
    fn get_ume(&mut self) -> &mut UserModeRuntime;
    fn write_stat_t(&mut self, addr: u64, stat_t: GenericStat);
    fn write_sysinfo_t(&mut self, addr: u64, si: sysinfo);
    fn get_sigaction(&mut self, addr: u64) -> GenericSigactionArg;
    fn get_mask(&mut self, addr: u64) -> Sigmask;
    fn set_old_sigaction(&mut self, addr: u64, se: SigEntry);
    fn set_altstack(&mut self, addr: u64, si: &SigInfo);
    fn get_altstack(&mut self, addr: u64) -> GenericStackt;
    fn rt_frame_setup(&mut self, sig: i32, si: &mut SigInfo);
}