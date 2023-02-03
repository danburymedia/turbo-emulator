use std::collections::HashMap;
use std::ffi::CString;
use std::sync::Arc;
use base::platform::MemoryMapping;
use base::{debug, info, MappedRegion, Protection};
use goblin::elf::Elf;
use sync::Mutex;
use crate::common::genfunc::{round_down, round_up};
use crate::common::memory::{flat_mem, MemEndian};
use crate::elf::{AuxType, Auxv, MachineType, MemState, UserModeInit, UserModeRuntime};
use crate::linux_usermode::defs::SigConstants;
use crate::riscv::common::{RISCV_PAGE_SIZE, RISCV_STACKPOINTER_REG, Xlen};
use crate::riscv::common::Xlen::{X64, X32};
use crate::riscv::interpreter::main::RiscvInt;
use crate::riscv::ume::signals::riscv64_init_sigconstant;

pub fn init_riscv_runtime(ef: &Elf) -> UserModeRuntime {
    let is64 = ef.is_64;
    let (stackbase, mmap_end) = if is64 {
        (0x8000000000 as u64, 0x40000000 as u64)
    } else {
        (0x7FFFFFFF as u64, 0x40000000 as u64)
    };
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
        machine_type: MachineType::Riscv,
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
fn push_stack_val(ri: &mut RiscvInt, val: u64) {
    let ms = ri.user_struct.memstate.lock();
    if (ms.stack_base - ms.stack_size) > ri.regs[RISCV_STACKPOINTER_REG] {
        panic!("ran out stack")
    }
    if ri.xlen == X64 {
        ri.regs[RISCV_STACKPOINTER_REG] -= 8;
        ri.memsource.guest_mem.write_phys_64(ri.regs[RISCV_STACKPOINTER_REG],val, MemEndian::Little);

    } else if  ri.xlen == Xlen::X32 {
        ri.regs[RISCV_STACKPOINTER_REG] -= 4;
        ri.memsource.guest_mem.write_phys_32(ri.regs[RISCV_STACKPOINTER_REG],val as u32, MemEndian::Little);

    } else {
        panic!();
    }
}
fn push_stack(ri: &mut RiscvInt, val: &[u8]) {
    let ms = ri.user_struct.memstate.lock();
    ri.regs[RISCV_STACKPOINTER_REG] -= val.len() as u64;
    if (ms.stack_base - ms.stack_size) > ri.regs[RISCV_STACKPOINTER_REG] {
        panic!("ran out stack")
    }
    let mut stack_ptr_up = ri.regs[RISCV_STACKPOINTER_REG] as *mut u8;
    for i in val {
        unsafe {
            *stack_ptr_up = *i;
            stack_ptr_up = stack_ptr_up.add(1);
        }

    }
}
fn map_stack(ri: &mut RiscvInt) {
    let mut ms = ri.user_struct.memstate.lock();
    let mapreg = MemoryMapping::new_protection_fixed(
        (ms.stack_base - ms.stack_size) as *mut u8
        ,  ms.stack_size as usize
        , Protection::read_write_execute(),
        false).unwrap();
    ri.regs[RISCV_STACKPOINTER_REG] = ms.stack_base;
    ms.mem_maps.push(mapreg);

}
pub fn init_stack(ri: &mut RiscvInt, ef: &Elf) {
    ri.regs[RISCV_STACKPOINTER_REG] -= 16;
   // let ms = &mut ume.memstate;
    let random_ptr = ri.get_stack_reg();
    let mut auxv: Vec<Auxv> = Vec::new();
    // todo: phdr
    let iv = ri.user_struct.initvars.lock();
    let objidx = iv.obj_idx.unwrap();
    let elfbase = iv.objects[objidx].mem.as_ptr() as u64;
    let logbase = iv.objects[objidx].base as u64;
    auxv.push(Auxv { typ: AuxType::Phdr, value: elfbase + ef.header.e_phoff});
    // auxv.push(Auxv { typ: AuxType::Base, value: 0x40000000});
    // auxv.push(Auxv { typ: AuxType::Phdr, value: logbase + 0x10000 + ef.header.e_phoff}); 0x40000000
    auxv.push(Auxv { typ: AuxType::Entry, value: iv.objects[objidx].entry_point});
    auxv.push(Auxv { typ: AuxType::PhNum, value: ef.header.e_phnum as u64 });
    auxv.push(Auxv { typ: AuxType::PhEnt, value: ef.header.e_phentsize as u64 });
    auxv.push(Auxv { typ: AuxType::PageSz, value: RISCV_PAGE_SIZE as u64 });
    auxv.push(Auxv { typ: AuxType::Secure, value: 0 as u64 });
    auxv.push(Auxv { typ: AuxType::Flags, value: 0 as u64 });
    auxv.push(Auxv { typ: AuxType::Random, value: random_ptr });
    auxv.push(Auxv { typ: AuxType::Null, value: 0 as u64 });
    let subval = if ri.xlen == Xlen::X64 { 8 } else { 4 };
    let envpclone = iv.envp.clone();
    let argclone = iv.args.clone();
    drop(iv);
    let mut envPtrs: Vec<u64> = Vec::new();
    for i in &envpclone {
        let pval = CString::new(i.clone().as_bytes()).unwrap().into_bytes_with_nul();
        info!("going to write env val value {} to addr 0x{:x}", i, ri.get_stack_reg() - (pval.len() as u64));
        push_stack(ri, &pval);
        envPtrs.push(ri.get_stack_reg())
    }
    envPtrs.push(0);
    let mut argPtrs: Vec<u64> = Vec::new();
    for i in &argclone {
        let pval = CString::new(i.clone().as_bytes()).unwrap().into_bytes_with_nul();
        info!("going to write arg val value {} to addr 0x{:x}", i, ri.get_stack_reg() - (pval.len() as u64));
        push_stack(ri,  &pval);
        argPtrs.push(ri.get_stack_reg())
    }
    argPtrs.push(0);
    ri.regs[RISCV_STACKPOINTER_REG] &= !15;
    for i in auxv.into_iter().rev() {
        info!("going to write aux value {} to addr 0x{:x}", i.value, ri.get_stack_reg() - subval);
        push_stack_val(ri, i.value as u64);
        info!("going to write aux key {:?} to addr 0x{:x}", i.typ, ri.get_stack_reg() - subval);
        push_stack_val(ri,  i.typ as u64);
    }
    for i in envPtrs.into_iter().rev() {
        info!("going to write envp ptr 0x{:x} to addr 0x{:x}", i, ri.get_stack_reg() - subval);
        push_stack_val(ri,  i as u64);
    }
    for i in argPtrs.into_iter().rev() {
        // the last valuee should be higher on stack
        info!("going to write arg ptr 0x{:x} to addr 0x{:x}", i, ri.get_stack_reg() - subval);
        push_stack_val(ri,  i as u64);
    }
    let argc = ri.user_struct.initvars.lock().args.len() as u64;
    push_stack_val(ri, argc);

}
pub fn init_riscv_ume(ume: UserModeRuntime, ef: &Elf) {
    let iv = ume.initvars.lock();

    let mut maxaddr = iv.objects[iv.obj_idx.unwrap()].mem_range.end;
    if iv.intrp_idx.is_some() {
        let intrpend = iv.objects[iv.intrp_idx.unwrap()].mem_range.end;
        if intrpend > maxaddr {
            maxaddr = intrpend;
        }
    }
    let is64bit: bool = if ume.is_64 {
        true
    } else {
        false
    };
    drop(iv);
    let mut riscvcpu = RiscvInt::init_usermode(if is64bit {Xlen::X64} else {Xlen::X32}, ume);
    map_stack(&mut riscvcpu);
    init_stack(&mut riscvcpu, ef);
    riscvcpu.pc = riscvcpu.user_struct.initvars.lock().real_entry_point;
    riscvcpu.cache_enabled = true;
    riscvcpu.run();
    // anything below run() should not happen.
    unreachable!("riscv processor error")

}
