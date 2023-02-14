use std::cmp::Ordering;
use std::collections::HashMap;
use vm_memory::{GuestAddress, GuestMemory};
use crate::common::memory::{flat_mem, MemEndian};
use crate::riscv::common::{Exception, Priv, Trap, Xlen};
use crate::riscv::common::Priv::{Machine, Supervisor, UserApp};
use base::{debug, info, warn};
use crate::riscv::interpreter::main::RiscvInt;

pub const RISCV_PAGE_SIZE: u64 = 4096; // smallest possible, just to be safe. In riscv, it is the only possible page size
pub const RISCV_PAGE_OFFSET: u64 = RISCV_PAGE_SIZE - 1;
pub const RISCV_PAGE_SHIFT: u64 = 12;
#[derive(Debug,Copy, Clone,Eq, PartialEq, Default)]
struct Pte {
    n: u8, //snvapot field
    pbmt: u8, //svpbmt (todo: gen page fault if these are unsupported)
    ppns: [u64; 5],
    ppn: u64,
    rsw: u8,  // "reserved for use by supervisor software" according to spec
    d: u8,    // dirty
    a: u8,    // accessed
    g: u8,    // global
    u: u8,    // user mode access
    x: u8,    // execute
    w: u8,    // write
    r: u8,    // read
    v: u8,    //  valid
}
#[derive(Debug,Copy, Clone,Eq, PartialEq)]
pub enum PageMode {
    None, // analogus to x86 "real mode"
    Sv32,
    Sv39,
    Sv48,
    Sv57
}
#[derive(Debug,Copy, Clone,Eq, PartialEq)]
pub enum MemAccessType {
    Read,
    Write,
    Execute
}
#[derive(Debug,Copy, Clone,Eq, PartialEq)]
pub struct MemAccessCircumstances {
    pub access_type: MemAccessType,
    pub mxr: bool, // when 0, only read when said we can read. When 1, we can read exec and read
    pub sum: bool, // when 0, s mode cant access u mode, when 1, yes
    pub prv: Priv

}
pub struct RiscVMem {
    pub guest_mem: flat_mem,
    reglen: Xlen,
    mstatus: u64,
    pmode: PageMode,
    pbmt_supported: bool,
    ppn: u64,
    usermode: bool, // in usermode, paging doesnt matter
    tlb: HashMap<u64, u64>
}
// reads will be return in native form, writes are expected in native form
impl RiscVMem {
    pub fn new_usermode(xlen: Xlen) -> RiscVMem {
        RiscVMem {
            guest_mem: flat_mem::new_usermode(),
            reglen: xlen,
            pmode: PageMode::None,
            pbmt_supported: false,
            ppn: 0,
            mstatus: 0,
            usermode: true,
            tlb: Default::default()
        }
    }

    pub fn new_system(xlen: Xlen, vm_memory: GuestMemory) -> RiscVMem {
        RiscVMem {
            guest_mem: flat_mem::new_system(vm_memory),
            reglen: xlen,
            pmode: PageMode::None,
            pbmt_supported: false,
            ppn: 0,
            mstatus: 0,
            usermode: false,
            tlb: Default::default()
        }
    }
    pub fn clear_cache(&mut self) {
        // sfence.vma
        self.tlb.clear();
    }
    fn trunc(&self, addr: u64) -> u64 {
        match self.reglen {
            Xlen::X32 => addr & 0xffffffff,
            Xlen::X64 => addr
        }
    }
    pub fn satp_flush(&mut self, value: u64) {
        // write to satp
        self.pmode = match self.reglen {
            Xlen::X32 => match value & 0x80000000 {
                // this can only be 0 or 1
                0 => PageMode::None,
                _ => PageMode::Sv32
            },
            Xlen::X64 => match value >> 60 {
                0 => PageMode::None,
                8 => PageMode::Sv39,
                9 => PageMode::Sv48,
                10 => PageMode::Sv57,
                _ => {
                    info!("Unknown addressing_mode {:x}", value >> 60);
                    panic!();
                }
            }
        };
        self.ppn = match self.reglen {
            Xlen::X32 => value & 0x3fffff,
            Xlen::X64 => value & 0xfffffffffff
        };
        self.clear_cache()
    }
    fn check_over_page_table(&mut self, addr: u64, len: u64) -> bool {
        if len ==0 {
            panic!();
        }
        // could overflow, but really shouldnt have len above 8
        if ((addr + (len - 1)) & !RISCV_PAGE_OFFSET) ^ (addr & !RISCV_PAGE_OFFSET) != 0 {
            // explaination: if the upper bits change, xor will catch
            true
        } else {
            false
        }
    }
    pub fn write_n_bytes(&mut self, addr: u64, access: MemAccessCircumstances, dat: Vec<u8>) -> Result<(), u64> {
        if self.check_over_page_table(addr, dat.len() as u64) {
            for i in 0..(dat.len()) {
                match self.write8(addr + (i as u64), access, dat[i]) {
                    Ok(()) => {},
                    Err(z) => {
                        return Err(z);
                        //return Err(addr + (i as u64));
                    }
                }

            }
            return Ok(());

        } else {
            let realaddr = match self.virt2phys(addr, access) {
                Ok(ra) => ra,
                Err(_) => return Err(addr),
            };
            self.guest_mem.write_phys_n(realaddr, dat);
            return Ok(());

        }


    }

    pub fn read_n_bytes(&mut self, addr: u64, len: usize, access: MemAccessCircumstances) -> Result<Vec<u8>, u64> {
        // todo: optimize for system mode. We can't be creating a vect every memaccess
        if self.check_over_page_table(addr, len as u64) {
            let mut retval: Vec<u8> = vec![0; len];
            for i in 0..len {
                match self.read8(addr + (i as u64), access) {
                    Ok(app) => {
                        retval[i] = app;
                    },
                    Err(z) => {
                        return Err(addr + (i as u64));
                    }
                };

            }
            return Ok(retval);

        } else {
            let realaddr = match self.virt2phys(addr, access) {
                Ok(ra) => ra,
                Err(_) => return Err(addr),
            };
            let vec = self.guest_mem.read_phys_n(realaddr, len);
            return Ok(vec);

        }


    }

    pub fn read8(&mut self, addr: u64, access: MemAccessCircumstances) -> Result<u8, u64> {
        let realaddr = match self.virt2phys(addr, access) {
            Ok(ra) => ra,
            Err(_) => return Err(addr),
        };
        let val = self.guest_mem.read_phys_8(realaddr);
        return Ok(val);
    }
    pub fn swap32imm(&mut self, addr: u64, imm: u32, ord: core::sync::atomic::Ordering, access: MemAccessCircumstances) -> Result<u32, u64> {
        let realaddr = match self.virt2phys(addr, access) {
            Ok(ra) => ra,
            Err(_) => return Err(addr),
        };
        let val = self.guest_mem.swap_atomic_imm_32(realaddr, imm, MemEndian::Little, ord);
        return Ok(val);
    }
    pub fn read16(&mut self, addr: u64, access: MemAccessCircumstances) -> Result<u16, u64> {
        return match self.read_n_bytes(addr, 2, access) {
            Ok(vect) => {
                let mut retval: [u8; 2] = [0; 2];
                retval.copy_from_slice(&vect.as_slice()[0..2]);
                Ok(u16::from_le_bytes(retval))
            },
            Err(er) => Err(er)
        }

    }
    pub fn read32(&mut self, addr: u64, access: MemAccessCircumstances) -> Result<u32, u64> {
        return match self.read_n_bytes(addr, 4, access) {
            Ok(vect) => {
                let mut retval: [u8; 4] = [0; 4];
                retval.copy_from_slice(&vect.as_slice()[0..4]);
                Ok(u32::from_le_bytes(retval))
            },
            Err(er) => Err(er)
        }
    }
    pub fn read64(&mut self, addr: u64, access: MemAccessCircumstances) -> Result<u64, u64> {
        return match self.read_n_bytes(addr, 8, access) {
            Ok(vect) => {
                let mut retval: [u8; 8] = [0; 8];
                retval.copy_from_slice(&vect.as_slice()[0..8]);
                Ok(u64::from_le_bytes(retval))
            },
            Err(er) => Err(er)
        }
    }
    pub fn write8(&mut self, addr: u64, access: MemAccessCircumstances, val: u8) -> Result<(), u64> {
        let realaddr = match self.virt2phys(addr, access) {
            Ok(ra) => ra,
            Err(_) => return Err(addr),
        };
        self.guest_mem.write_phys_8(realaddr, val);
        return Ok(());
    }
    pub fn write64(&mut self, addr: u64, access: MemAccessCircumstances, val: u64) -> Result<(), u64> {
        return match self.write_n_bytes(addr, access, val.to_le_bytes().to_vec()) {
            Ok(_) => Ok(()),
            Err(z) => Err(z)
        }
    }
    pub fn write32(&mut self, addr: u64, access: MemAccessCircumstances, val: u32) -> Result<(), u64> {
        return match self.write_n_bytes(addr, access, val.to_le_bytes().to_vec()) {
            Ok(_) => Ok(()),
            Err(z) => Err(z)
        }
    }
    pub fn write16(&mut self, addr: u64, access: MemAccessCircumstances, val: u16) -> Result<(), u64> {
        return match self.write_n_bytes(addr, access, val.to_le_bytes().to_vec()) {
            Ok(_) => Ok(()),
            Err(z) => Err(z)
        }
    }
    pub fn virt2phys(&mut self, addr: u64, access: MemAccessCircumstances) -> Result<u64, ()> {
        // riscv is not clear
        if access.prv == Machine || self.usermode   {
            // mprv if set, means we are acting as if we are in machine mode || (((self.mstatus >> 17) & 1) != 0)
            return Ok(addr)
        }
        match self.pmode {
            PageMode::None => {
                Ok(addr)
            }
            _ => self.page_walk(addr, access)
        }

    }
    fn page_walk(&mut self, addr: u64, acctype: MemAccessCircumstances) -> Result<u64, ()> {
        let (mut ptesize, mut level) = match self.pmode {
            PageMode::None => panic!("how are we here?"),
            PageMode::Sv32 => (4, 2),
            PageMode::Sv39 => (8,3),
            PageMode::Sv48 => (8, 4),
            PageMode::Sv57 => (8, 5),
        };
        let mut i = level - 1;
        let mut ppn = self.ppn;
        let vpns_index: Vec<u64> = match level {
            2 => {
                [(addr >> 12) & 0x3ff, (addr >> 22) & 0x3ff].to_vec()
            },
            3 => {
                [(addr >> 12) & 0x1ff, (addr >> 21) & 0x1ff, (addr >> 30) & 0x1ff].to_vec()
            }
            4 => {
                [(addr >> 12) & 0x1ff, (addr >> 21) & 0x1ff,
                    (addr >> 30) & 0x1ff, (addr >> 39) & 0x1ff].to_vec()

            },
            5 => {
                [(addr >> 12) & 0x1ff, (addr >> 21) & 0x1ff,
                    (addr >> 30) & 0x1ff, (addr >> 39) & 0x1ff, (addr >> 48) & 0x1ff].to_vec()

            }
            _ => panic!()
        };
        let mut ptestr: Pte = Default::default();
        while i >= 0 {
            let a = ppn * RISCV_PAGE_SIZE + vpns_index[level as usize] * ptesize;
            let pte: u64 = match ptesize {
                4 => self.guest_mem.read_phys_32(self.trunc(a), MemEndian::Little) as u64,
                8 => self.guest_mem.read_phys_64(self.trunc(a), MemEndian::Little),
                _ => panic!()
            };
            ptestr = self.pte_parse(pte);
            if ptestr.n == 1 || ptestr.pbmt != 0 {
                warn!("riscv: page_walk() encountered unsupported extension");
                return Err(()); // that extension not supported
            }
            if ptestr.v == 0 || (ptestr.r == 0 && ptestr.w == 1) {
                return Err(());
            }

            if ptestr.r == 0 && ptestr.x == 0 {
                match i {
                    0 => {
                        return Err(());
                    },
                    _ => {
                        i = i - 1;
                        continue;
                    }

                }
            }
            break;
        }
        match acctype.access_type {
            MemAccessType::Read => {
                if ptestr.r == 0 {
                    if !(ptestr.x == 1 && acctype.mxr == false) { // or ptestr.x == 0 || acctype.mxr
                        return Err(());

                    }
                }
                if acctype.sum == false && acctype.prv == Supervisor && ptestr.u != 0 {
                    return Err(())

                }

            }
            MemAccessType::Write => {
                if ptestr.w == 0 {
                    return Err(());
                }
            }
            MemAccessType::Execute => {
                if ptestr.x == 0 {
                    return Err(());

                }
            }
        }
        let offset = addr & 0xfff;
        let phys = match self.pmode {
            PageMode::None => panic!(""),
            PageMode::Sv32 => {
                match level {
                    1 => {
                        if ptestr.ppns[0] != 0 {
                            return Err(());

                        }
                        (ptestr.ppns[1] << 22) | (vpns_index[0] << 12) | offset
                    },
                    0 => {
                        (ptestr.ppn << 12) | offset
                    },
                    _ => panic!()
                }
            }
            _ => {
                match level {
                    2 => {
                        if ptestr.ppns[1] != 0 || ptestr.ppns[0] != 0 {
                            return Err(());
                        }
                        (ptestr.ppns[2] << 30) | (vpns_index[1] << 21) | (vpns_index[0] << 12) | offset
                    }
                    1 => {
                        if ptestr.ppns[0] != 0 {
                            return Err(());
                        }
                        (ptestr.ppns[2] << 30) | (ptestr.ppns[1] << 21) | (vpns_index[0] << 12) | offset
                    }
                    0 => (ptestr.ppn << 12) | offset,
                    _ => panic!(),
                }
            }
        };


        Ok(phys)
    }
    fn pte_parse(&self, pte: u64) -> Pte  {
        let ppn: u64 = match self.pmode {
            PageMode::Sv32 => (pte >> 10) & 0x3fffff,
            _ => (pte >> 10) & 0xfff_ffffffff,
        };
        let mut ppns = match self.pmode {
            PageMode::None => panic!(),
            PageMode::Sv32 => {
                [
                    (pte >> 10) & 0x3ff,
                    (pte >> 20) & 0xfff,
                    0,
                    0,
                    0,
                ]
            }
            PageMode::Sv39 => {
                [
                    (pte >> 10) & 0x1ff,
                    (pte >> 19) & 0x1ff,
                    (pte >> 28) & 0x3ffffff,
                    0,
                    0,
                ]
            }
            PageMode::Sv48 => {
                [
                    (pte >> 10) & 0x1ff,
                    (pte >> 19) & 0x1ff,
                    (pte >> 28) & 0x1ff,
                    (pte >> 37) & 0x1ffff,
                    0,
                ]
            }
            PageMode::Sv57 => {
                [
                    (pte >> 10) & 0x1ff,
                    (pte >> 19) & 0x1ff,
                    (pte >> 28) & 0x1ff,
                    (pte >> 37) & 0x1ff,
                    (pte >> 46) & 0x1ff,
                ]
            }
        };
        Pte {
            n: ((pte >> 63) & 1) as u8,
            pbmt: ((pte >> 63) & 3) as u8,
            ppns,
            ppn,
            rsw: ((pte >> 8) & 0x3) as u8,
            d: ((pte >> 7) & 1) as u8,
            a: ((pte >> 6) & 1) as u8,
            g: ((pte >> 5) & 1) as u8,
            u: ((pte >> 4) & 1) as u8,
            x: ((pte >> 3) & 1) as u8,
            w: ((pte >> 2) & 1) as u8,
            r: ((pte >> 1) & 1) as u8,
            v: (pte & 1) as u8,
        }

    }
   // fn read_

}
pub fn get_read_access_type(is_exec: bool) -> MemAccessType {
    if is_exec {
        MemAccessType::Execute
    } else {
        MemAccessType::Read
    }
}
impl RiscvInt {
    fn get_effective_address(&self, address: u64) -> u64 {
        match self.xlen {
            Xlen::X32 => address & 0xffffffff,
            Xlen::X64 => address
        }
    }
    pub fn mem_fn_handler<T>(&mut self, res: Result<T, u64>, set_trap: bool, acctype: MemAccessType) -> Result<T, Trap> {
        match res {
            Ok(p) => {
                Ok(p)
            }
            Err(z) => {
                let trp = self.mem_trap(acctype, z);
                if set_trap {
                    self.set_trap(trp);
                }
                Err(trp)
            }
        }
    }

    pub fn deal_with_cache(&mut self, addr: u64) {
        let hashaddr = addr >> RISCV_PAGE_SHIFT;
        let hashaddr1 = hashaddr + 1; // we can technically write to two pages
        // todo: make if statment to see if we actually are
        // todo: refactor write functions once we use virtual mem. We need to anyway
        unsafe {
            for i in (*self.ainstr.get()).ainstr.iter_mut() {
                let addr = i.begin >> RISCV_PAGE_SHIFT;
                if addr == hashaddr || addr == hashaddr1 {
                    // we wrote to that page, so remove from cache and stop exec.
                    // outer loop is noop if nothing else is set, we will restart from exec block
                    // we could also page fault,
                    // what matters is outer loop return guaranteed + cache invalid is done
                    self.stop_exec = true;
                    i.begin = 0;
                    i.end = 0;
                    i.instrs.clear();
                }
            }
        }
    }
    pub fn readn(&mut self, addr: u64, size: u64, is_exec: bool, set_trap: bool) -> Result<Vec<u8>, Trap> {
        let macc = self.gen_mem_cirum(get_read_access_type(is_exec));
        let x = self.memsource.read_n_bytes(self.get_effective_address(addr), size as usize, macc);
        self.mem_fn_handler(x, set_trap, macc.access_type)
    }
    pub fn write_n(&mut self, addr: u64, vals: Vec<u8>, set_trap: bool) -> Result<(), Trap> {

        let macc = self.gen_mem_cirum(MemAccessType::Write);
        let x = self.memsource.write_n_bytes(self.get_effective_address(addr),  macc, vals);
        self.mem_fn_handler(x,  set_trap, macc.access_type)
    }
    pub fn read64(&mut self, addr: u64, is_exec: bool, set_trap: bool) -> Result<u64, Trap> {
        // todo- check mmio, etc
        if self.usermode {
            return Ok(self.memsource.guest_mem.read_phys_64(addr, MemEndian::Little));
        }
        // we "can" do a usermode read/write from the internal read funcs, but we shouldnt reach there
        let macc = self.gen_mem_cirum(get_read_access_type(is_exec));
        let res = self.memsource.read64(self.get_effective_address(addr), macc);
        self.mem_fn_handler(res, set_trap, macc.access_type)
    }

    pub fn read32(&mut self, addr: u64, is_exec: bool, set_trap: bool) -> Result<u32, Trap> {
        if self.usermode {
            return Ok(self.memsource.guest_mem.read_phys_32(addr, MemEndian::Little));
        }
        let macc = self.gen_mem_cirum(get_read_access_type(is_exec));
        let res = self.memsource.read32(self.get_effective_address(addr), macc);
        self.mem_fn_handler(res, set_trap, macc.access_type)
    }

    pub fn read16(&mut self, addr: u64, is_exec: bool, set_trap: bool) -> Result<u16, Trap> {
        if self.usermode {
            return Ok(self.memsource.guest_mem.read_phys_16(addr, MemEndian::Little));
        }
        let macc = self.gen_mem_cirum(get_read_access_type(is_exec));
        let res = self.memsource.read16(self.get_effective_address(addr), macc);
        self.mem_fn_handler(res, set_trap, macc.access_type)
    }

    pub fn read8(&mut self, addr: u64, is_exec: bool, set_trap: bool) -> Result<u8, Trap> {
        if self.usermode {
            return Ok(self.memsource.guest_mem.read_phys_8(addr));
        }
        let macc = self.gen_mem_cirum(get_read_access_type(is_exec));
        let res = self.memsource.read8(self.get_effective_address(addr), macc);
        self.mem_fn_handler(res, set_trap, macc.access_type)

    }
    pub fn swap32imm(&mut self, addr: u64, imm: u32, ord: core::sync::atomic::Ordering, is_exec: bool, set_trap: bool) -> Result<u32, Trap> {
        let macc = self.gen_mem_cirum(get_read_access_type(is_exec));
        let res = self.memsource.swap32imm(
            self.get_effective_address(addr), imm, ord, macc);
        self.mem_fn_handler(res, set_trap, macc.access_type)

    }

    pub fn write64(&mut self, addr: u64, val: u64, set_trap: bool) -> Result<(), Trap> {
        if self.cache_enabled {
            self.deal_with_cache(addr);
        }
        if self.usermode {
            self.memsource.guest_mem.write_phys_64(addr, val, MemEndian::Little);
            return Ok(());
        }
        let macc = self.gen_mem_cirum(MemAccessType::Write);
        let res = self.memsource.write64(self.get_effective_address(addr),  macc, val);
        self.mem_fn_handler(res, set_trap, macc.access_type)

    }
    pub fn write32(&mut self, addr: u64, val: u32, set_trap: bool) -> Result<(), Trap> {
        if self.cache_enabled {
            self.deal_with_cache(addr);
        }
        if self.usermode {
            self.memsource.guest_mem.write_phys_32(addr, val, MemEndian::Little);
            return Ok(());
        }
        let macc = self.gen_mem_cirum(MemAccessType::Write);
        let res = self.memsource.write32(self.get_effective_address(addr),  macc, val);
        self.mem_fn_handler(res, set_trap, macc.access_type)
    }
    pub fn write16(&mut self, addr: u64, val: u16, set_trap: bool) -> Result<(), Trap> {
        if self.cache_enabled {
            self.deal_with_cache(addr);
        }
        if self.usermode {
            self.memsource.guest_mem.write_phys_16(addr, val, MemEndian::Little);
            return Ok(());
        }
        let macc = self.gen_mem_cirum(MemAccessType::Write);
        let res = self.memsource.write16(self.get_effective_address(addr),  macc, val);
        self.mem_fn_handler(res, set_trap, macc.access_type)

    }
    pub fn write8(&mut self, addr: u64, val: u8, set_trap: bool) -> Result<(), Trap> {
        if self.cache_enabled {
            self.deal_with_cache(addr);
        }
        if self.usermode {
            self.memsource.guest_mem.write_phys_8(addr, val);
            return Ok(());
        }
        let macc = self.gen_mem_cirum(MemAccessType::Write);
        let res = self.memsource.write8(self.get_effective_address(addr),  macc, val);
        self.mem_fn_handler(res, set_trap, macc.access_type)

    }
    pub fn gen_mem_cirum(&self, access_type: MemAccessType) -> MemAccessCircumstances {
        MemAccessCircumstances {
            access_type,
            mxr: true,
            sum: false,
            prv: Priv::Machine // todo: fix
        }
    }
    pub fn mem_trap(&self, acc_type: MemAccessType, addr: u64) -> Trap {
        Trap {
            ttype: if acc_type == MemAccessType::Read {
                Exception::LoadPageFault
            } else if acc_type == MemAccessType::Write {
                Exception::StorePageFault
            } else {
                Exception::InstructionPageFault
            },
            val: addr
        }
    }
}