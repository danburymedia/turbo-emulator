use std::ffi::CString;
use std::sync::Arc;
use base::platform::MemoryMapping;
use base::{debug, info, MappedRegion, Protection};
use goblin::elf::Elf;
use sync::Mutex;
use crate::armv8::common::ARM64_PAGE_SIZE;
use crate::armv8::interpreter::main::Arm64Cpu;
use crate::common::memory::{flat_mem, MemEndian};
use crate::elf::{AuxType, Auxv, MachineType, MemState, UserModeInit, UserModeRuntime};
use crate::linux_usermode::defs::SigConstants;

pub fn init_arm64_runtime(ef: &Elf) -> UserModeRuntime {
    let is64 = ef.is_64;
    let (stackbase, mmap_end) = (0x8000000000 as u64, 0x40000000 as u64);
    let max_stack_size: u64 = 1024 * 1024 * 8;
    let memstate = MemState {
        stack_size: max_stack_size,
        brk: 0,
        orig_brk: 0,
        brk_max: 0,
        mem_maps: vec![],
        mmap_region: None,
        stack_base: stackbase,
        next_thread_stack_base: stackbase - max_stack_size,
        anon_idx: 0
    };
    let ival = UserModeInit {
        real_entry_point: 0,
        mmap_barrier: mmap_end,
        objects: vec![],
        obj_idx: None,
        intrp_idx: None,
        args: vec![],
        envp: vec![],
    };
    UserModeRuntime {
        initvars: Arc::new(Mutex::new(ival)),
        mem_access: flat_mem::new_usermode(),
        guest_pagesize: 4096,
        host_pagesize: base::pagesize() as u64,
        pagesize_mask: 4096 - 1,
        is_debug: false,
        machine_type: MachineType::Arm64,
        is_little_endian: true,
        heap_grow_down: false,
        sig_tramp: 0,
        memstate: Arc::new(Mutex::new(memstate)),
        is_64: is64,
        sigcnst: Arc::new(Mutex::new(SigConstants::default())),
        search_path: Default::default(),
        str_path: "".to_string(),
        tls_base: 0
    }
}
fn push_stack_val(ai: &mut Arm64Cpu, val: u64) {
    let ms = ai.user_struct.memstate.lock();
    if (ms.stack_base - ms.stack_size) > ai.stack_reg {
        panic!("ran out stack")
    }
    ai.stack_reg -= 8;
    ai.memory_access.write_phys_64(ai.stack_reg, val, MemEndian::Little);

}
fn push_stack(ri: &mut Arm64Cpu, val: &[u8]) {
    let ms = ri.user_struct.memstate.lock();
    ri.stack_reg -= val.len() as u64;
    if (ms.stack_base - ms.stack_size) > ri.stack_reg {
        panic!("ran out stack")
    }
    let mut stack_ptr_up = ri.stack_reg as *mut u8;
    for i in val {
        unsafe {
            *stack_ptr_up = *i;
            stack_ptr_up = stack_ptr_up.add(1);
        }

    }
}
fn map_stack(ri: &mut Arm64Cpu) {
    let mut ms = ri.user_struct.memstate.lock();
    let mapreg = MemoryMapping::new_protection_fixed(
        (ms.stack_base - ms.stack_size) as *mut u8
        ,  ms.stack_size as usize
        , Protection::read_write_execute(),
        false).unwrap();
    ri.stack_reg = ms.stack_base;
    ms.mem_maps.push(mapreg);

}
pub fn init_stack(ri: &mut Arm64Cpu, ef: &Elf) {
    ri.stack_reg -= 16;
    // let ms = &mut ume.memstate;
    let random_ptr = ri.get_stack_reg();
    let mut auxv: Vec<Auxv> = Vec::new();
    // todo: phdr
    let iv = ri.user_struct.initvars.lock();
    let objidx = iv.obj_idx.unwrap();
    let elfbase = iv.objects[objidx].mem.as_ptr() as u64;
    let logbase = iv.objects[objidx].base as u64;
    auxv.push(Auxv { typ: AuxType::Phdr, value: elfbase + ef.header.e_phoff});
    if iv.intrp_idx.is_some() {
        // we know we always load interp at 0x40000000. todo if we change that, we need to reflect it
        auxv.push(Auxv { typ: AuxType::Base, value: 0x40000000});
    }
    // auxv.push(Auxv { typ: AuxType::Base, value: 0x40000000});
    //auxv.push(Auxv { typ: AuxType::Phdr, value: logbase + ef.header.e_phoff}); //0x40000000
    auxv.push(Auxv { typ: AuxType::Entry, value: iv.objects[objidx].entry_point});
    auxv.push(Auxv { typ: AuxType::PhNum, value: ef.header.e_phnum as u64 });
    auxv.push(Auxv { typ: AuxType::PhEnt, value: ef.header.e_phentsize as u64 });
    auxv.push(Auxv { typ: AuxType::PageSz, value: ARM64_PAGE_SIZE as u64 });
    auxv.push(Auxv { typ: AuxType::Secure, value: 0 as u64 });
    auxv.push(Auxv { typ: AuxType::Flags, value: 0 as u64 });
    auxv.push(Auxv { typ: AuxType::Random, value: random_ptr });
    auxv.push(Auxv { typ: AuxType::Null, value: 0 as u64 });
    auxv.push(Auxv { typ: AuxType::ExecFn, value: 0 as u64 });

    let subval = 8;
    let envpclone = iv.envp.clone();
    let argclone = iv.args.clone();
    drop(iv);
    let mut envPtrs: Vec<u64> = Vec::new();
    for i in &envpclone {
        let pval = CString::new(i.clone().as_bytes()).unwrap().into_bytes_with_nul();
        debug!("going to write env val value {} to addr 0x{:x}", i, ri.get_stack_reg() - (pval.len() as u64));
        push_stack(ri, &pval);
        envPtrs.push(ri.get_stack_reg())
    }
    envPtrs.push(0);
    let mut argPtrs: Vec<u64> = Vec::new();
    for i in &argclone {
        let pval = CString::new(i.clone().as_bytes()).unwrap().into_bytes_with_nul();
        debug!("going to write arg val value {} to addr 0x{:x}", i, ri.get_stack_reg() - (pval.len() as u64));
        push_stack(ri,  &pval);
        argPtrs.push(ri.get_stack_reg())
    }
    argPtrs.push(0);
    ri.stack_reg &= !15;
    for i in auxv.into_iter().rev() {
        debug!("going to write aux value {} to addr 0x{:x}", i.value, ri.get_stack_reg() - subval);
        push_stack_val(ri, i.value as u64);
        debug!("going to write aux key {:?} to addr 0x{:x}", i.typ, ri.get_stack_reg() - subval);
        push_stack_val(ri,  i.typ as u64);
    }
    for i in envPtrs.into_iter().rev() {
        debug!("going to write envp ptr 0x{:x} to addr 0x{:x}", i, ri.get_stack_reg() - subval);
        push_stack_val(ri,  i as u64);
    }
    for i in argPtrs.into_iter().rev() {
        // the last valuee should be higher on stack
        debug!("going to write arg ptr 0x{:x} to addr 0x{:x}", i, ri.get_stack_reg() - subval);
        push_stack_val(ri,  i as u64);
    }
    let argc = ri.user_struct.initvars.lock().args.len() as u64;
    push_stack_val(ri, argc);

}
pub fn init_arm64_ume(ume: UserModeRuntime, ef: &Elf) {
    let iv = ume.initvars.lock();

    let mut maxaddr = iv.objects[iv.obj_idx.unwrap()].mem_range.end;
    if iv.intrp_idx.is_some() {
        let intrpend = iv.objects[iv.intrp_idx.unwrap()].mem_range.end;
        if intrpend > maxaddr {
            maxaddr = intrpend;
        }
    }
    drop(iv);
    let mut arm64cpu = Arm64Cpu::init_usermode(ume);
    map_stack(&mut arm64cpu);
    init_stack(&mut arm64cpu, ef);
    arm64cpu.pc = arm64cpu.user_struct.initvars.lock().real_entry_point;
    //rm64cpu.set_reg(1, 1, false);
    //arm64cpu.set_reg(2, 2, false);

    arm64cpu.run();
    // anything below run() should not happen.
    unreachable!("arm64 processor error")
}
