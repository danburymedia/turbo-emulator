use std::fs::File;
use std::{fmt, mem, process, result};
use std::borrow::Borrow;
use std::cmp::{max, min};
use std::collections::HashMap;
use std::fmt::Display;
use std::io::Read;
use std::ops::Range;
use anyhow::*;
use std::os::unix::io::AsRawFd;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use base::platform::MemoryMapping;
use base::{debug, info, MappedRegion, MemoryMappingArena, Protection, warn};
use goblin::elf::*;
use goblin::elf::dynamic::{DT_INIT, DT_NEEDED, DT_RPATH, DT_RUNPATH, DT_STRTAB};
use goblin::elf::program_header::PT_LOAD;
use multimap::MultiMap;
use thiserror::Error as ThisError;
use thiserror::*;
use crate::common::genfunc::round_up;
use base::pagesize;
use resources::address_allocator::AddressAllocator;
use resources::AddressRange;
use sync::Mutex;
use crate::armv8::ume::load::init_arm64_runtime;

use crate::common::memory::*;
use crate::linux_usermode::defs::SigConstants;
use crate::riscv::ume::load::{init_riscv_runtime};
#[derive(ThisError, Debug)]
pub enum Error {
    #[error("Failed to open ELF file")]
    ElfFileError,
    #[error("I/O error when accessing {0}: {1}")]
    Io(PathBuf, std::io::Error),
    #[error("Parsing error when accessing {0}: {1}")]
    ParseError(PathBuf, goblin::error::Error),
    #[error("An invalid or unsupported path was encountered: {0}")]
    InvalidPath(PathBuf),
    #[error("ELF object has no load segments")]
    NoLoadSegments,
    #[error("ELF object could not be found")]
    NotFound(String),
    #[error("Executable wants an interpreter, but no sysroot path was given")]
    NoInterp,
}
#[derive(Copy, Clone)]
pub enum MachineType {
    Riscv,
    Arm64,
    None
}
// doesnt need to be sent across threads
pub struct UserModeInit {
    pub real_entry_point: u64,
    pub mmap_barrier: u64,
    pub objects: Vec<Object>,
    pub obj_idx: Option<usize>,
    pub intrp_idx: Option<usize>,
    pub args: Vec<String>,
    pub envp: Vec<String>,
}
// this does
impl Default for UserModeInit {
    fn default() -> Self {
        UserModeInit {
            real_entry_point: 0,
            mmap_barrier: 0,
            objects: vec![],
            obj_idx: None,
            intrp_idx: None,
            args: vec![],
            envp: vec![],
        }
    }
}
#[derive(Clone)]
pub struct UserModeRuntime {
    pub initvars: Arc<Mutex<UserModeInit>>,
    pub mem_access: flat_mem,
    /// Guaranted to be equal to host pagezie or a bigger multiple. If not then won't execute
    pub guest_pagesize: u64,
    pub host_pagesize: u64,
    pub pagesize_mask: u64, // = (guest pagesize - 1)
    pub is_debug: bool,
    pub is_little_endian: bool,
    pub heap_grow_down: bool,
    pub machine_type: MachineType,
    pub sig_tramp: u64,
    pub memstate: Arc<Mutex<MemState>>,
    pub is_64: bool,
    pub sigcnst: Arc<Mutex<SigConstants>>,
    pub search_path: PathBuf,
    pub str_path: String,
    pub tls_base: u64,
}
#[derive(Default)]
pub struct MemState {
    pub stack_size: u64,
    pub brk: u64,
    pub orig_brk: u64,
    pub brk_max: u64, // max value brk has seen
    pub mem_maps: Vec<MemoryMapping>,
    pub mmap_region: Option<AddressAllocator>,
    pub stack_base: u64,
    pub next_thread_stack_base: u64,
    pub anon_idx: usize, // todo: better way
    // stack_min: u64,
    // stack_base: u64,
    // max_stack_size: u64,
    // next_thread_stack_base: u64,
    // brk_point: u64,
    // mmap_end: u64,
}

impl Default for UserModeRuntime {
    fn default() -> Self {
        UserModeRuntime {
            initvars: Arc::new(Mutex::new(UserModeInit::default())),
            mem_access: flat_mem::new_usermode(),
            guest_pagesize: 0,
            host_pagesize: 0,
            pagesize_mask: 0,
            is_debug: false,
            machine_type: MachineType::None,
            is_little_endian: false,
            heap_grow_down: false,
            sig_tramp: 0,
            memstate: Arc::new(Default::default()),
            is_64: false,
            sigcnst: Arc::new(Default::default()),
            search_path: Default::default(),
            str_path: "".to_string(),
            tls_base: 0,
        }
    }
}
/// A memory segment.
#[derive(Debug)]
pub struct Segment {
    pub vaddr_range: Range<u64>,
    pub padding: u64,
    pub flags: u32,
}

pub type initResult<T> = result::Result<T, Error>;

pub fn init_user_mode_emulation(execpath: String, args: Vec<String>, search_path: String) -> initResult<()> {
    // todo dont forget to check pagesize validiy (and file exists)
    let pbuf = PathBuf::from(execpath.clone());
    let mut fle = File::open(pbuf.clone()).map_err(|_| Error::ElfFileError)?;
    let mut data = Vec::new();
    fle.read_to_end(&mut data).map_err(|_| Error::ElfFileError)?;
    let ef = goblin::elf::Elf::parse(&data).map_err(|e| Error::ParseError(PathBuf::from(pbuf.clone()), e))?;
    let mut args_str: Vec<String> = vec![execpath.clone()];
    for i in &args {
        args_str.push(i.clone());
    }
    let machine_type = ef.header.e_machine;
    let mut umr = if machine_type == goblin::elf::header::EM_RISCV {
        init_riscv_runtime(&ef)
    } else if machine_type == goblin::elf::header::EM_AARCH64 {
        init_arm64_runtime(&ef)
    } else {
        panic!();
    };
    {
        let mut initm = umr.initvars.lock();
        initm.args = args_str;
        initm.envp = std::env::vars()
            .map(|(k, v)| format!("{}={}", k, v))
            .collect();
        umr.str_path = search_path.clone();
        umr.search_path = PathBuf::from(search_path);
    }
    // todo call arch specific filler
    let mut p_load_vaddr = 0;
    for zi in &ef.program_headers {
        if zi.p_type == PT_LOAD {
            p_load_vaddr = zi.p_vaddr;
            break;
        }
    }
    // let mut usebase = ef.program_headers[0].p_vaddr & !umr.pagesize_mask;
    let mut usebase = p_load_vaddr & !umr.pagesize_mask;
    if usebase == 0 {
        usebase = 0x10000; // todo: arch agnostic?
    }
    let exec_index = umr.load_object( pbuf.clone(),
                                      Some(usebase),
                                      false).unwrap();
    {
        let mut meminit = umr.memstate.lock();
        let mut ivi = umr.initvars.lock();
        ivi.obj_idx = Some(exec_index);
        meminit.brk = round_up(ivi.objects[exec_index].real_end, umr.guest_pagesize);
        meminit.brk_max = meminit.brk;
        meminit.orig_brk = meminit.brk;
    }
    let mmapdown = umr.heap_grow_down;

    let intrpidx: Option<usize> = if ef.interpreter.is_some() {
        let v = ef.interpreter.unwrap();
        let path = umr.object_path(v).unwrap();
        let ibase = umr.initvars.lock().mmap_barrier;
        let retval = umr.load_object(path, Some(ibase), mmapdown).unwrap();
        let mut iv = umr.initvars.lock();
        let psize = iv.objects[retval].mem.size() as u64;
        if mmapdown {
            iv.mmap_barrier -= psize;
        } else {
            iv.mmap_barrier += psize;
        }
        Some(retval)
    } else {
        None
    };
    let mut iv = umr.initvars.lock();

    if mmapdown {
        let endpoint = iv.mmap_barrier - (1024 * 1024 * 1024);
        let mut addr = AddressAllocator::new(AddressRange::from_start_and_end(endpoint, iv.mmap_barrier),
                                         Some(umr.guest_pagesize),
                                         None).unwrap();
        // addr.dont_care = true;
        umr.memstate.lock().mmap_region = Some(addr);

    } else {
        let endpoint = iv.mmap_barrier + (1024 * 1024 * 1024);
        let mut addr = AddressAllocator::new(AddressRange::from_start_and_end( iv.mmap_barrier, endpoint),
                                         Some(umr.guest_pagesize),
                                         None).unwrap();
        // addr.dont_care = true;
        umr.memstate.lock().mmap_region = Some(addr);

    }; // 1 gb for testing for now
    iv.intrp_idx = intrpidx;
    iv.real_entry_point = if let Some(z) = iv.intrp_idx {
        iv.objects[z].entry_point
    } else {
        iv.objects[iv.obj_idx.unwrap()].entry_point
    };
    // goblin::elf::header::EM_RISCV
    mem::drop(iv);
    match umr.machine_type {
        MachineType::Riscv => {
            crate::riscv::ume::load::init_riscv_ume(umr, &ef);
        },
        MachineType::Arm64 => {
            crate::armv8::ume::load::init_arm64_ume(umr, &ef);
        }
        _ => {
            panic!("unsupported machine type");
        }

    }
    process::exit(0);
}
/// Computes the minimal range that contains two ranges.
fn convex_hull<T: std::cmp::Ord>(a: Range<T>, b: Range<T>) -> Range<T> {
    (min(a.start, b.start))..(max(a.end, b.end))
}


impl UserModeRuntime {
    pub fn object_path(&self, name: &str) -> Result<PathBuf> {
        // this function is just for interpreter,
        if name.is_empty() {
            return Err(anyhow::Error::from(Error::NoInterp));
        }
        let val = self.search_path.join(&name[1..]).canonicalize().unwrap();
        if val.exists() {
            Ok(val)
        } else {
            Err(anyhow::Error::from(Error::NotFound(name.into())))
        }

    }
    // inspired from https://fasterthanli.me/
    pub fn load_object<P: AsRef<Path>>(&mut self, path: P, use_base: Option<u64>, base_subtract: bool) -> anyhow::Result<usize> {
        let mut iv = self.initvars.lock();
        let path = path
            .as_ref()
            .canonicalize()
            .map_err(|e| Error::Io(path.as_ref().to_path_buf(), e))?;
        let mut fs_file = std::fs::File::open(&path).map_err(|e| Error::Io(path.clone(), e))?;
        let mut input = Vec::new();
        fs_file
            .read_to_end(&mut input)
            .map_err(|e| Error::Io(path.clone(), e))?;
        debug!("Loading {:?}", path);
        let ef = goblin::elf::Elf::parse(&input).map_err(|e| Error::ParseError(path.clone(), e))?;
        let load_segments = || {
            ef.program_headers
                .iter()
                .filter(|ph| ph.p_type == PT_LOAD)
        };
        let mem_range = load_segments()
            .map(|ph| ph.vm_range())
            .fold(None, |acc, range| match acc {
                None => Some(range),
                Some(acc) => Some(convex_hull(acc, range)),
            })
            .ok_or(Error::NoLoadSegments)?;
       // let mem_size: usize = (mem_range.end - mem_range.start) + pagesize() ;
        let mem_size: usize = round_up((mem_range.end - mem_range.start) as u64, self.guest_pagesize) as usize;
        let mut memareana = if let Some(ss) = use_base {
            let use_ss = if base_subtract {
                ss - (mem_size as u64)
            } else {
                ss
            };
            base::platform::MemoryMappingArena::new_protection_fixed_usermode(mem_size, use_ss as *mut u8).unwrap()
        } else {
            base::platform::MemoryMappingArena::new_protection_usermode(mem_size).unwrap()
        };
        let base = (memareana.as_ptr() as usize) - mem_range.start; // yes, it will go below. So when we add offset to "base" we get the beginning of the actual map
        debug!("Mapping memory segments");
        let segments = load_segments()
            // First, filter out zero-sized segments:
            .filter(|ph| ph.p_memsz > 0)
            // Then, map the remaining ones!
            .map(|ph| -> anyhow::Result<_> {
                debug!("\t- Mapping {:#?}", ph);
                let alignmask = ph.p_align - 1;
                let vaddr = ph.p_vaddr & !(alignmask);
                // let vaddr = ph.p_vaddr & !(self.pagesize_mask); // todo: pick the biggest of the smallest pagesizes of native and emulatee architeor
                let padding = ph.p_vaddr - vaddr;
                let offset = ph.p_offset - padding;
                let memsz = ph.p_memsz + padding;
                let filesz = ph.p_filesz + padding;

                debug!(
                    "\t  └──> to file {:x?} | mem {:x?} | base {:x?} | filesz {:x}",
                    offset..(offset + memsz),
                    vaddr..(vaddr + memsz),
                    (vaddr + (base as u64))..((base as u64) + vaddr + memsz),
                    filesz
                );
                let realsize = round_up(filesz, pagesize() as u64); // we use native for this one
                memareana.add_fd_offset((vaddr as usize) - mem_range.start, realsize as usize, &fs_file, offset).unwrap();
                // But if there's some bytes left over...
                if ph.p_memsz > ph.p_filesz {
                    // ...then we zero them!
                    // NOTE: This works becuase we already reserved the *convex hull*
                    // of all segments in memory in our initial `MemoryMap::new` call,
                    // so that memory is there.
                    let mut zero_start: *mut u8 = (base + ph.vm_range().start + (ph.p_filesz as usize)) as *mut u8;
                    let zero_len = ph.p_memsz - ph.p_filesz;

                    unsafe {
                        // This will probably get optimized to something good later.
                        for i in 0..zero_len {
                            *zero_start = 0;
                            zero_start = zero_start.add(1);
                        }
                    }
                }
                if realsize > filesz {
                    // correct what we did
                    let bptr = memareana.as_ptr() as usize;
                    let mut zero_start: *mut u8 = (bptr + (vaddr as usize) - mem_range.start + (filesz as usize)) as *mut u8;
                    let zero_len = realsize - filesz;

                    unsafe {
                        // This will probably get optimized to something good later.
                        for i in 0..zero_len {
                            *zero_start = 0;
                            zero_start = zero_start.add(1);
                        }
                    }
                }
                Ok(Segment {
                    vaddr_range: vaddr..(ph.p_vaddr + ph.p_memsz),
                    padding,
                    flags: ph.p_flags,
                })
            })
            .collect::<Result<Vec<_>>>()?;
        let ep = ((base as u64) + ef.entry.clone()) as u64;
        let realend = ((memareana.as_ptr() as u64) + (memareana.size() as u64));
        let obj = Object {
            // ef,
            entry_point: ep,
            path: path.clone(),
            base: base as usize,
            mem_range,
            real_end: realend,
            segments,
            mem: memareana,
            fs_file
        };
        let idx = iv.objects.len();
        iv.objects.push(obj);

        Ok(idx)
    }
}
pub struct Object {
    // The ELF file associated with this object.
    //Skipped in debug output because it can get *really* verbose.
    //pub ef: Elf<'a>,
    /// Actual entry point (meaning compesnaed for base)
    pub entry_point: u64,

    /// The path this ELF object was loaded from.
    pub path: PathBuf,

    /// The base address for mapping this ELF object to memory.
    pub base: usize,

    /// The memory range associated with this object.
    pub mem_range: Range<usize>,

    pub real_end: u64,
    /// The memory segments associated with this object.
    pub segments: Vec<Segment>,

    pub mem: MemoryMappingArena,
    pub fs_file: File,

}

#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(u64)]
#[allow(dead_code)]
pub enum AuxType {
    /// End of vector
    Null = 0,
    /// Entry should be ignored
    Ignore = 1,
    /// File descriptor of program
    ExecFd = 2,
    /// Program headers for program
    Phdr = 3,
    /// Size of program header entry
    PhEnt = 4,
    /// Number of program headers
    PhNum = 5,
    /// System page size
    PageSz = 6,
    /// Base address of interpreter
    Base = 7,
    /// Flags
    Flags = 8,
    /// Entry point of program
    Entry = 9,
    /// Program is not ELF
    NotElf = 10,
    /// Real uid
    Uid = 11,
    /// Effective uid
    EUid = 12,
    /// Real gid
    Gid = 13,
    /// Effective gid
    EGid = 14,
    /// String identifying CPU for optimizations
    Platform = 15,
    /// Arch-dependent hints at CPU capabilities
    HwCap = 16,
    /// Frequency at which times() increments
    ClkTck = 17,
    /// Secure mode boolean
    Secure = 23,
    /// String identifying real platform, may differ from Platform
    BasePlatform = 24,
    /// Address of 16 random bytes
    Random = 25,
    // Extension of HwCap
    HwCap2 = 26,
    /// Filename of program
    ExecFn = 31,

    SysInfo = 32,
    SysInfoEHdr = 33,
}
/// Represents an auxiliary vector.
pub struct Auxv {
    pub typ: AuxType,
    pub value: u64,
}
