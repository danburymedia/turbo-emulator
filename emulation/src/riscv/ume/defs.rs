use std::os::linux::raw::*;
use std::os::raw::{c_int, c_long};
use std::os::unix::raw::{gid_t, uid_t};
use base::sys::Signal::Sys;
use libc::sysinfo;
use crate::common::memory::{flat_mem, MemEndian};
use crate::linux_usermode::defs::{GenericStat, read32_advance_ptr, read64_advance_ptr, write16_advance_ptr, write32_advance_ptr, write64_advance_ptr};
use crate::linux_usermode::main::SyscallType;
use crate::riscv::common::Xlen;
use crate::riscv::interpreter::main::RiscvInt;

pub const RISCV_SYS_IO_SETUP: u16 = 0;
pub const RISCV_SYS_IO_DESTROY: u16 = 1;
pub const RISCV_SYS_IO_SUBMIT: u16 = 2;
pub const RISCV_SYS_IO_CANCEL: u16 = 3;
pub const RISCV_SYS_IO_GETEVENTS: u16 = 4;
pub const RISCV_SYS_SETXATTR: u16 = 5;
pub const RISCV_SYS_LSETXATTR: u16 = 6;
pub const RISCV_SYS_FSETXATTR: u16 = 7;
pub const RISCV_SYS_GETXATTR: u16 = 8;
pub const RISCV_SYS_LGETXATTR: u16 = 9;
pub const RISCV_SYS_FGETXATTR: u16 = 10;
pub const RISCV_SYS_LISTXATTR: u16 = 11;
pub const RISCV_SYS_LLISTXATTR: u16 = 12;
pub const RISCV_SYS_FLISTXATTR: u16 = 13;
pub const RISCV_SYS_REMOVEXATTR: u16 = 14;
pub const RISCV_SYS_LREMOVEXATTR: u16 = 15;
pub const RISCV_SYS_FREMOVEXATTR: u16 = 16;
pub const RISCV_SYS_GETCWD: u16 = 17;
pub const RISCV_SYS_LOOKUP_DCOOKIE: u16 = 18;
pub const RISCV_SYS_EVENTFD2: u16 = 19;
pub const RISCV_SYS_EPOLL_CREATE1: u16 = 20;
pub const RISCV_SYS_EPOLL_CTL: u16 = 21;
pub const RISCV_SYS_EPOLL_PWAIT: u16 = 22;
pub const RISCV_SYS_DUP: u16 = 23;
pub const RISCV_SYS_DUP3: u16 = 24;
pub const RISCV_SYS_FCNTL: u16 = 25;
pub const RISCV_SYS_INOTIFY_INIT1: u16 = 26;
pub const RISCV_SYS_INOTIFY_ADD_WATCH: u16 = 27;
pub const RISCV_SYS_INOTIFY_RM_WATCH: u16 = 28;
pub const RISCV_SYS_IOCTL: u16 = 29;
pub const RISCV_SYS_IOPRIO_SET: u16 = 30;
pub const RISCV_SYS_IOPRIO_GET: u16 = 31;
pub const RISCV_SYS_FLOCK: u16 = 32;
pub const RISCV_SYS_MKNODAT: u16 = 33;
pub const RISCV_SYS_MKDIRAT: u16 = 34;
pub const RISCV_SYS_UNLINKAT: u16 = 35;
pub const RISCV_SYS_SYMLINKAT: u16 = 36;
pub const RISCV_SYS_LINKAT: u16 = 37;
pub const RISCV_SYS_UMOUNT2: u16 = 39;
pub const RISCV_SYS_MOUNT: u16 = 40;
pub const RISCV_SYS_PIVOT_ROOT: u16 = 41;
pub const RISCV_SYS_NFSSERVCTL: u16 = 42;
pub const RISCV_SYS_STATFS: u16 = 43;
pub const RISCV_SYS_FSTATFS: u16 = 44;
pub const RISCV_SYS_TRUNCATE: u16 = 45;
pub const RISCV_SYS_FTRUNCATE: u16 = 46;
pub const RISCV_SYS_FALLOCATE: u16 = 47;
pub const RISCV_SYS_FACCESSAT: u16 = 48;
pub const RISCV_SYS_CHDIR: u16 = 49;
pub const RISCV_SYS_FCHDIR: u16 = 50;
pub const RISCV_SYS_CHROOT: u16 = 51;
pub const RISCV_SYS_FCHMOD: u16 = 52;
pub const RISCV_SYS_FCHMODAT: u16 = 53;
pub const RISCV_SYS_FCHOWNAT: u16 = 54;
pub const RISCV_SYS_FCHOWN: u16 = 55;
pub const RISCV_SYS_OPENAT: u16 = 56;
pub const RISCV_SYS_CLOSE: u16 = 57;
pub const RISCV_SYS_VHANGUP: u16 = 58;
pub const RISCV_SYS_PIPE2: u16 = 59;
pub const RISCV_SYS_QUOTACTL: u16 = 60;
pub const RISCV_SYS_GETDENTS64: u16 = 61;
pub const RISCV_SYS_LSEEK: u16 = 62;
pub const RISCV_SYS_READ: u16 = 63;
pub const RISCV_SYS_WRITE: u16 = 64;
pub const RISCV_SYS_READV: u16 = 65;
pub const RISCV_SYS_WRITEV: u16 = 66;
pub const RISCV_SYS_PREAD64: u16 = 67;
pub const RISCV_SYS_PWRITE64: u16 = 68;
pub const RISCV_SYS_PREADV: u16 = 69;
pub const RISCV_SYS_PWRITEV: u16 = 70;
pub const RISCV_SYS_SENDFILE: u16 = 71;
pub const RISCV_SYS_PSELECT6: u16 = 72;
pub const RISCV_SYS_PPOLL: u16 = 73;
pub const RISCV_SYS_SIGNALFD4: u16 = 74;
pub const RISCV_SYS_VMSPLICE: u16 = 75;
pub const RISCV_SYS_SPLICE: u16 = 76;
pub const RISCV_SYS_TEE: u16 = 77;
pub const RISCV_SYS_READLINKAT: u16 = 78;
pub const RISCV_SYS_FSTATAT: u16 = 79;
pub const RISCV_SYS_FSTAT: u16 = 80;
pub const RISCV_SYS_SYNC: u16 = 81;
pub const RISCV_SYS_FSYNC: u16 = 82;
pub const RISCV_SYS_FDATASYNC: u16 = 83;
pub const RISCV_SYS_SYNC_FILE_RANGE: u16 = 84;
pub const RISCV_SYS_TIMERFD_CREATE: u16 = 85;
pub const RISCV_SYS_TIMERFD_SETTIME: u16 = 86;
pub const RISCV_SYS_TIMERFD_GETTIME: u16 = 87;
pub const RISCV_SYS_UTIMENSAT: u16 = 88;
pub const RISCV_SYS_ACCT: u16 = 89;
pub const RISCV_SYS_CAPGET: u16 = 90;
pub const RISCV_SYS_CAPSET: u16 = 91;
pub const RISCV_SYS_PERSONALITY: u16 = 92;
pub const RISCV_SYS_EXIT: u16 = 93;
pub const RISCV_SYS_EXIT_GROUP: u16 = 94;
pub const RISCV_SYS_WAITID: u16 = 95;
pub const RISCV_SYS_SET_TID_ADDRESS: u16 = 96;
pub const RISCV_SYS_UNSHARE: u16 = 97;
pub const RISCV_SYS_FUTEX: u16 = 98;
pub const RISCV_SYS_SET_ROBUST_LIST: u16 = 99;
pub const RISCV_SYS_GET_ROBUST_LIST: u16 = 100;
pub const RISCV_SYS_NANOSLEEP: u16 = 101;
pub const RISCV_SYS_GETITIMER: u16 = 102;
pub const RISCV_SYS_SETITIMER: u16 = 103;
pub const RISCV_SYS_KEXEC_LOAD: u16 = 104;
pub const RISCV_SYS_INIT_MODULE: u16 = 105;
pub const RISCV_SYS_DELETE_MODULE: u16 = 106;
pub const RISCV_SYS_TIMER_CREATE: u16 = 107;
pub const RISCV_SYS_TIMER_GETTIME: u16 = 108;
pub const RISCV_SYS_TIMER_GETOVERRUN: u16 = 109;
pub const RISCV_SYS_TIMER_SETTIME: u16 = 110;
pub const RISCV_SYS_TIMER_DELETE: u16 = 111;
pub const RISCV_SYS_CLOCK_SETTIME: u16 = 112;
pub const RISCV_SYS_CLOCK_GETTIME: u16 = 113;
pub const RISCV_SYS_CLOCK_GETRES: u16 = 114;
pub const RISCV_SYS_CLOCK_NANOSLEEP: u16 = 115;
pub const RISCV_SYS_SYSLOG: u16 = 116;
pub const RISCV_SYS_PTRACE: u16 = 117;
pub const RISCV_SYS_SCHED_SETPARAM: u16 = 118;
pub const RISCV_SYS_SCHED_SETSCHEDULER: u16 = 119;
pub const RISCV_SYS_SCHED_GETSCHEDULER: u16 = 120;
pub const RISCV_SYS_SCHED_GETPARAM: u16 = 121;
pub const RISCV_SYS_SCHED_SETAFFINITY: u16 = 122;
pub const RISCV_SYS_SCHED_GETAFFINITY: u16 = 123;
pub const RISCV_SYS_SCHED_YIELD: u16 = 124;
pub const RISCV_SYS_SCHED_GET_PRIORITY_MAX: u16 = 125;
pub const RISCV_SYS_SCHED_GET_PRIORITY_MIN: u16 = 126;
pub const RISCV_SYS_SCHED_RR_GET_INTERVAL: u16 = 127;
pub const RISCV_SYS_RESTART_SYSCALL: u16 = 128;
pub const RISCV_SYS_KILL: u16 = 129;
pub const RISCV_SYS_TKILL: u16 = 130;
pub const RISCV_SYS_TGKILL: u16 = 131;
pub const RISCV_SYS_SIGALTSTACK: u16 = 132;
pub const RISCV_SYS_RT_SIGSUSPEND: u16 = 133;
pub const RISCV_SYS_RT_SIGACTION: u16 = 134;
pub const RISCV_SYS_RT_SIGPROCMASK: u16 = 135;
pub const RISCV_SYS_RT_SIGPENDING: u16 = 136;
pub const RISCV_SYS_RT_SIGTIMEDWAIT: u16 = 137;
pub const RISCV_SYS_RT_SIGQUEUEINFO: u16 = 138;
pub const RISCV_SYS_RT_SIGRETURN: u16 = 139;
pub const RISCV_SYS_SETPRIORITY: u16 = 140;
pub const RISCV_SYS_GETPRIORITY: u16 = 141;
pub const RISCV_SYS_REBOOT: u16 = 142;
pub const RISCV_SYS_SETREGID: u16 = 143;
pub const RISCV_SYS_SETGID: u16 = 144;
pub const RISCV_SYS_SETREUID: u16 = 145;
pub const RISCV_SYS_SETUID: u16 = 146;
pub const RISCV_SYS_SETRESUID: u16 = 147;
pub const RISCV_SYS_GETRESUID: u16 = 148;
pub const RISCV_SYS_SETRESGID: u16 = 149;
pub const RISCV_SYS_GETRESGID: u16 = 150;
pub const RISCV_SYS_SETFSUID: u16 = 151;
pub const RISCV_SYS_SETFSGID: u16 = 152;
pub const RISCV_SYS_TIMES: u16 = 153;
pub const RISCV_SYS_SETPGID: u16 = 154;
pub const RISCV_SYS_GETPGID: u16 = 155;
pub const RISCV_SYS_GETSID: u16 = 156;
pub const RISCV_SYS_SETSID: u16 = 157;
pub const RISCV_SYS_GETGROUPS: u16 = 158;
pub const RISCV_SYS_SETGROUPS: u16 = 159;
pub const RISCV_SYS_UNAME: u16 = 160;
pub const RISCV_SYS_SETHOSTNAME: u16 = 161;
pub const RISCV_SYS_SETDOMAINNAME: u16 = 162;
pub const RISCV_SYS_GETRLIMIT: u16 = 163;
pub const RISCV_SYS_SETRLIMIT: u16 = 164;
pub const RISCV_SYS_GETRUSAGE: u16 = 165;
pub const RISCV_SYS_UMASK: u16 = 166;
pub const RISCV_SYS_PRCTL: u16 = 167;
pub const RISCV_SYS_GETCPU: u16 = 168;
pub const RISCV_SYS_GETTIMEOFDAY: u16 = 169;
pub const RISCV_SYS_SETTIMEOFDAY: u16 = 170;
pub const RISCV_SYS_ADJTIMEX: u16 = 171;
pub const RISCV_SYS_GETPID: u16 = 172;
pub const RISCV_SYS_GETPPID: u16 = 173;
pub const RISCV_SYS_GETUID: u16 = 174;
pub const RISCV_SYS_GETEUID: u16 = 175;
pub const RISCV_SYS_GETGID: u16 = 176;
pub const RISCV_SYS_GETEGID: u16 = 177;
pub const RISCV_SYS_GETTID: u16 = 178;
pub const RISCV_SYS_SYSINFO: u16 = 179;
pub const RISCV_SYS_MQ_OPEN: u16 = 180;
pub const RISCV_SYS_MQ_UNLINK: u16 = 181;
pub const RISCV_SYS_MQ_TIMEDSEND: u16 = 182;
pub const RISCV_SYS_MQ_TIMEDRECEIVE: u16 = 183;
pub const RISCV_SYS_MQ_NOTIFY: u16 = 184;
pub const RISCV_SYS_MQ_GETSETATTR: u16 = 185;
pub const RISCV_SYS_MSGGET: u16 = 186;
pub const RISCV_SYS_MSGCTL: u16 = 187;
pub const RISCV_SYS_MSGRCV: u16 = 188;
pub const RISCV_SYS_MSGSND: u16 = 189;
pub const RISCV_SYS_SEMGET: u16 = 190;
pub const RISCV_SYS_SEMCTL: u16 = 191;
pub const RISCV_SYS_SEMTIMEDOP: u16 = 192;
pub const RISCV_SYS_SEMOP: u16 = 193;
pub const RISCV_SYS_SHMGET: u16 = 194;
pub const RISCV_SYS_SHMCTL: u16 = 195;
pub const RISCV_SYS_SHMAT: u16 = 196;
pub const RISCV_SYS_SHMDT: u16 = 197;
pub const RISCV_SYS_SOCKET: u16 = 198;
pub const RISCV_SYS_SOCKETPAIR: u16 = 199;
pub const RISCV_SYS_BIND: u16 = 200;
pub const RISCV_SYS_LISTEN: u16 = 201;
pub const RISCV_SYS_ACCEPT: u16 = 202;
pub const RISCV_SYS_CONNECT: u16 = 203;
pub const RISCV_SYS_GETSOCKNAME: u16 = 204;
pub const RISCV_SYS_GETPEERNAME: u16 = 205;
pub const RISCV_SYS_SENDTO: u16 = 206;
pub const RISCV_SYS_RECVFROM: u16 = 207;
pub const RISCV_SYS_SETSOCKOPT: u16 = 208;
pub const RISCV_SYS_GETSOCKOPT: u16 = 209;
pub const RISCV_SYS_SHUTDOWN: u16 = 210;
pub const RISCV_SYS_SENDMSG: u16 = 211;
pub const RISCV_SYS_RECVMSG: u16 = 212;
pub const RISCV_SYS_READAHEAD: u16 = 213;
pub const RISCV_SYS_BRK: u16 = 214;
pub const RISCV_SYS_MUNMAP: u16 = 215;
pub const RISCV_SYS_MREMAP: u16 = 216;
pub const RISCV_SYS_ADD_KEY: u16 = 217;
pub const RISCV_SYS_REQUEST_KEY: u16 = 218;
pub const RISCV_SYS_KEYCTL: u16 = 219;
pub const RISCV_SYS_CLONE: u16 = 220;
pub const RISCV_SYS_EXECVE: u16 = 221;
pub const RISCV_SYS_MMAP: u16 = 222;
pub const RISCV_SYS_FADVISE64: u16 = 223;
pub const RISCV_SYS_SWAPON: u16 = 224;
pub const RISCV_SYS_SWAPOFF: u16 = 225;
pub const RISCV_SYS_MPROTECT: u16 = 226;
pub const RISCV_SYS_MSYNC: u16 = 227;
pub const RISCV_SYS_MLOCK: u16 = 228;
pub const RISCV_SYS_MUNLOCK: u16 = 229;
pub const RISCV_SYS_MLOCKALL: u16 = 230;
pub const RISCV_SYS_MUNLOCKALL: u16 = 231;
pub const RISCV_SYS_MINCORE: u16 = 232;
pub const RISCV_SYS_MADVISE: u16 = 233;
pub const RISCV_SYS_REMAP_FILE_PAGES: u16 = 234;
pub const RISCV_SYS_MBIND: u16 = 235;
pub const RISCV_SYS_GET_MEMPOLICY: u16 = 236;
pub const RISCV_SYS_SET_MEMPOLICY: u16 = 237;
pub const RISCV_SYS_MIGRATE_PAGES: u16 = 238;
pub const RISCV_SYS_MOVE_PAGES: u16 = 239;
pub const RISCV_SYS_RT_TGSIGQUEUEINFO: u16 = 240;
pub const RISCV_SYS_PERF_EVENT_OPEN: u16 = 241;
pub const RISCV_SYS_ACCEPT4: u16 = 242;
pub const RISCV_SYS_RECVMMSG: u16 = 243;
pub const RISCV_SYS_ARCH_SPECIFIC_SYSCALL: u16 = 244;
pub const RISCV_SYS_WAIT4: u16 = 260;
pub const RISCV_SYS_PRLIMIT64: u16 = 261;
pub const RISCV_SYS_FANOTIFY_INIT: u16 = 262;
pub const RISCV_SYS_FANOTIFY_MARK: u16 = 263;
pub const RISCV_SYS_NAME_TO_HANDLE_AT: u16 = 264;
pub const RISCV_SYS_OPEN_BY_HANDLE_AT: u16 = 265;
pub const RISCV_SYS_CLOCK_ADJTIME: u16 = 266;
pub const RISCV_SYS_SYNCFS: u16 = 267;
pub const RISCV_SYS_SETNS: u16 = 268;
pub const RISCV_SYS_SENDMMSG: u16 = 269;
pub const RISCV_SYS_PROCESS_VM_READV: u16 = 270;
pub const RISCV_SYS_PROCESS_VM_WRITEV: u16 = 271;
pub const RISCV_SYS_KCMP: u16 = 272;
pub const RISCV_SYS_FINIT_MODULE: u16 = 273;
pub const RISCV_SYS_SCHED_SETATTR: u16 = 274;
pub const RISCV_SYS_SCHED_GETATTR: u16 = 275;
pub const RISCV_SYS_RENAMEAT2: u16 = 276;
pub const RISCV_SYS_SECCOMP: u16 = 277;
pub const RISCV_SYS_GETRANDOM: u16 = 278;
pub const RISCV_SYS_MEMFD_CREATE: u16 = 279;
pub const RISCV_SYS_BPF: u16 = 280;
pub const RISCV_SYS_EXECVEAT: u16 = 281;
pub const RISCV_SYS_USERFAULTFD: u16 = 282;
pub const RISCV_SYS_MEMBARRIER: u16 = 283;
pub const RISCV_SYS_MLOCK2: u16 = 284;
pub const RISCV_SYS_COPY_FILE_RANGE: u16 = 285;
pub const RISCV_SYS_PREADV2: u16 = 286;
pub const RISCV_SYS_PWRITEV2: u16 = 287;
pub const RISCV_SYS_PKEY_MPROTECT: u16 = 288;
pub const RISCV_SYS_PKEY_ALLOC: u16 = 289;
pub const RISCV_SYS_PKEY_FREE: u16 = 290;
pub const RISCV_SYS_STATX: u16 = 291;
pub const RISCV_SYS_IO_PGETEVENTS: u16 = 292;
pub const RISCV_SYS_RSEQ: u16 = 293;
pub const RISCV_SYS_KEXEC_FILE_LOAD: u16 = 294;
pub const RISCV_SYS_PIDFD_SEND_SIGNAL: u16 = 424;
pub const RISCV_SYS_IO_URING_SETUP: u16 = 425;
pub const RISCV_SYS_IO_URING_ENTER: u16 = 426;
pub const RISCV_SYS_IO_URING_REGISTER: u16 = 427;
pub const RISCV_SYS_OPEN_TREE: u16 = 428;
pub const RISCV_SYS_MOVE_MOUNT: u16 = 429;
pub const RISCV_SYS_FSOPEN: u16 = 430;
pub const RISCV_SYS_FSCONFIG: u16 = 431;
pub const RISCV_SYS_FSMOUNT: u16 = 432;
pub const RISCV_SYS_FSPICK: u16 = 433;
pub const RISCV_SYS_PIDFD_OPEN: u16 = 434;
pub const RISCV_SYS_CLONE3: u16 = 435;
pub const RISCV_SYS_CLOSE_RANGE: u16 = 436;
pub const RISCV_SYS_OPENAT2: u16 = 437;
pub const RISCV_SYS_PIDFD_GETFD: u16 = 438;
pub const RISCV_SYS_FACCESSAT2: u16 = 439;
pub const RISCV_SYS_PROCESS_MADVISE: u16 = 440;
pub const RISCV_SYS_EPOLL_PWAIT2: u16 = 441;
pub const RISCV_SYS_MOUNT_SETATTR: u16 = 442;
pub const RISCV_SYS_QUOTACTL_FD: u16 = 443;
pub const RISCV_SYS_LANDLOCK_CREATE_RULESET: u16 = 444;
pub const RISCV_SYS_LANDLOCK_ADD_RULE: u16 = 445;
pub const RISCV_SYS_LANDLOCK_RESTRICT_SELF: u16 = 446;
pub const RISCV_SYS_MEMFD_SECRET: u16 = 447;
pub const RISCV_SYS_PROCESS_MRELEASE: u16 = 448;
pub const RISCV_SYS_FUTEX_WAITV: u16 = 449;
pub const RISCV_SYS_SET_MEMPOLICY_HOME_NODE: u16 = 450;
#[derive(Copy, Clone)]
pub struct RiscvStat {
    pub st_dev: u64,
    pub st_ino: u64,
    pub st_mode: u32,
    pub st_nlink: u32,
    pub st_uid: u32,
    pub st_gid: u32,
    pub st_rdev: u64,
    pub __pad1: u64,
    pub st_size: i64,
    pub st_blksize: i32,
    pub __pad2: i32,
    pub st_blocks: i64,
    pub st_atime: i64,
    pub st_atime_nsec: i64,
    pub st_mtime: i64,
    pub st_mtime_nsec: i64,
    pub st_ctime: i64,
    pub st_ctime_nsec: i64,
    __unused: [i32; 2usize],
}
pub fn read_riscv_stat(addr: u64, end: MemEndian) -> RiscvStat {
    let mut realaddr = addr;
    let st_dev = read64_advance_ptr(&mut realaddr, end);
    let st_ino = read64_advance_ptr(&mut realaddr, end);
    let st_mode = read32_advance_ptr(&mut realaddr, end);
    let st_nlink = read32_advance_ptr(&mut realaddr, end);
    let st_uid = read32_advance_ptr(&mut realaddr, end);
    let st_gid = read32_advance_ptr(&mut realaddr, end);
    let st_rdev = read64_advance_ptr(&mut realaddr, end);
    let pad1 = read64_advance_ptr(&mut realaddr, end);
    let st_size = read64_advance_ptr(&mut realaddr, end) as i64;
    let st_blksize = read32_advance_ptr(&mut realaddr, end) as i32;
    let pad2 = read32_advance_ptr(&mut realaddr, end) as i32;
    let st_blocks = read32_advance_ptr(&mut realaddr, end) as i64;
    let st_atime = read64_advance_ptr(&mut realaddr, end) as i64;
    let st_atime_nsec = read64_advance_ptr(&mut realaddr, end) as i64;
    let st_mtime = read64_advance_ptr(&mut realaddr, end) as i64;
    let st_mtime_nsec = read64_advance_ptr(&mut realaddr, end) as i64;
    let st_ctime = read64_advance_ptr(&mut realaddr, end) as i64;
    let st_ctime_nsec = read64_advance_ptr(&mut realaddr, end) as i64;
    RiscvStat {
        st_dev,
        st_ino,
        st_mode,
        st_nlink,
        st_uid,
        st_gid,
        st_rdev,
        __pad1: pad1,
        st_size,
        st_blksize,
        __pad2: pad2,
        st_blocks,
        st_atime,
        st_atime_nsec,
        st_mtime,
        st_mtime_nsec,
        st_ctime,
        st_ctime_nsec,
        __unused: [0,0]
    }

}
pub fn write_riscv_stat(addr: u64, end: MemEndian, stat: GenericStat) {
    let mut realaddr = addr;
    write64_advance_ptr(&mut realaddr, stat.st_dev, end);
    write64_advance_ptr(&mut realaddr, stat.st_ino, end);
    write32_advance_ptr(&mut realaddr, stat.st_mode as u32, end);
    write32_advance_ptr(&mut realaddr, stat.st_nlink as u32, end);
    write32_advance_ptr(&mut realaddr, stat.st_uid as u32, end);
    write32_advance_ptr(&mut realaddr, stat.st_gid as u32, end);
    write64_advance_ptr(&mut realaddr, stat.st_rdev, end);
    write64_advance_ptr(&mut realaddr, 0, end); // pad1
    write64_advance_ptr(&mut realaddr, stat.st_size as u64, end);
    write32_advance_ptr(&mut realaddr, stat.st_blksize as u32, end);
    write32_advance_ptr(&mut realaddr, 0, end); // pad2
    write64_advance_ptr(&mut realaddr, stat.st_blocks as u64, end);
    write64_advance_ptr(&mut realaddr, stat.st_atime as u64, end);
    write64_advance_ptr(&mut realaddr, stat.st_atime_nsec as u64, end);
    write64_advance_ptr(&mut realaddr, stat.st_mtime as u64, end);
    write64_advance_ptr(&mut realaddr, stat.st_mtime_nsec as u64, end);
    write64_advance_ptr(&mut realaddr, stat.st_ctime as u64, end);
    write64_advance_ptr(&mut realaddr, stat.st_ctime_nsec as u64, end);
    write32_advance_ptr(&mut realaddr, 0, end); // unused
    write32_advance_ptr(&mut realaddr, 0, end); // unused

}
pub fn write_riscv_sysinfo(addr: u64, end: MemEndian, si: sysinfo) {
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
pub fn riscv_translate_syscall(val: u16) -> Option<SyscallType> {
    match val {
        RISCV_SYS_BRK => Some(SyscallType::Brk),
        RISCV_SYS_WRITEV => Some(SyscallType::Writev),
        RISCV_SYS_EXIT_GROUP => Some(SyscallType::ExitGroup),
        RISCV_SYS_UNAME => Some(SyscallType::Uname),
        RISCV_SYS_FACCESSAT => Some(SyscallType::Faccessat),
        RISCV_SYS_OPENAT => Some(SyscallType::Openat),
        RISCV_SYS_FSTATAT => Some(SyscallType::Fstatat),
        RISCV_SYS_READ => Some(SyscallType::Read),
        RISCV_SYS_MMAP => Some(SyscallType::Mmap),
        RISCV_SYS_CLOSE => Some(SyscallType::Close),
        RISCV_SYS_MPROTECT => Some(SyscallType::Mprotect),
        RISCV_SYS_WRITE => Some(SyscallType::Write),
        RISCV_SYS_SET_TID_ADDRESS => Some(SyscallType::SetTidAddr),
        RISCV_SYS_FCNTL => Some(SyscallType::Fcntl),
        RISCV_SYS_RT_SIGACTION => Some(SyscallType::Sigaction),
        RISCV_SYS_READV => Some(SyscallType::Readv),
        RISCV_SYS_LSEEK => Some(SyscallType::Lseek),
        RISCV_SYS_CLOCK_SETTIME => Some(SyscallType::ClockSetTime),
        RISCV_SYS_CLOCK_GETTIME => Some(SyscallType::ClockGetTime),
        RISCV_SYS_GETEUID => Some(SyscallType::Geteuid),
        RISCV_SYS_GETUID => Some(SyscallType::Getuid),
        RISCV_SYS_IOCTL => Some(SyscallType::Ioctl),
        RISCV_SYS_SOCKETPAIR => Some(SyscallType::Socketpair),
        RISCV_SYS_PPOLL => Some(SyscallType::Ppoll),
        RISCV_SYS_SOCKET => Some(SyscallType::Socket),
        RISCV_SYS_RT_SIGPROCMASK => Some(SyscallType::Sigprocmask),
        RISCV_SYS_CLONE => Some(SyscallType::Clone),
        RISCV_SYS_PIPE2 => Some(SyscallType::Pipe2),
        RISCV_SYS_SYSINFO => Some(SyscallType::Sysinfo),
        RISCV_SYS_FSTAT => Some(SyscallType::Fstat),
        RISCV_SYS_FADVISE64 => Some(SyscallType::Fadvise64),
        RISCV_SYS_FCHOWN => Some(SyscallType::Fchown),
        RISCV_SYS_FCHMOD => Some(SyscallType::Fchmod),
        RISCV_SYS_UTIMENSAT => Some(SyscallType::Utimensat),
        RISCV_SYS_LOOKUP_DCOOKIE => Some(SyscallType::LookupDcookie),
        RISCV_SYS_DUP3 => Some(SyscallType::Dup3),
        RISCV_SYS_GETGID => Some(SyscallType::Getgid),
        RISCV_SYS_SETUID => Some(SyscallType::Setuid),
        RISCV_SYS_SETGID => Some(SyscallType::Setgid),
        RISCV_SYS_SENDFILE => Some(SyscallType::Sendfile),
        RISCV_SYS_BIND => Some(SyscallType::Bind),
        RISCV_SYS_SENDTO => Some(SyscallType::Sendto),
        RISCV_SYS_RECVFROM => Some(SyscallType::Recvfrom),
        RISCV_SYS_SETITIMER => Some(SyscallType::Setitimer),
        RISCV_SYS_GETITIMER => Some(SyscallType::Getitimer),
        RISCV_SYS_CONNECT => Some(SyscallType::Connect),
        RISCV_SYS_LISTEN => Some(SyscallType::Listen),
        RISCV_SYS_FTRUNCATE => Some(SyscallType::Ftruncate),
        RISCV_SYS_GETPID => Some(SyscallType::Getpid),
        RISCV_SYS_GETPPID => Some(SyscallType::Getppid),
        RISCV_SYS_GETPGID => Some(SyscallType::Getpgid),
        RISCV_SYS_GETSID => Some(SyscallType::Getsid),
        RISCV_SYS_KILL => Some(SyscallType::Kill),
        RISCV_SYS_GETDENTS64 => Some(SyscallType::Getdents64),
        RISCV_SYS_SET_ROBUST_LIST => Some(SyscallType::SetRobustList),
        RISCV_SYS_PRLIMIT64 => Some(SyscallType::Prlimit64),
        RISCV_SYS_SCHED_GETAFFINITY => Some(SyscallType::Getaffinity),
        _ => None
    }

}
pub struct RiscvCpuState {
    pub regs: [u64; 32], // registeres can be smaller than this, but we do biggest for somplicity,
    pub fregs: [u64; 32],
    pub xlen: Xlen,
    pub csr: [u64; 4096],
}
pub unsafe extern "C" fn start_riscv_emu(sig: c_int ) {
   // let rcpu = RiscvInt::init_usermode()
}