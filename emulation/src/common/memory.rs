use std::{ptr, result};
use std::sync::atomic::{AtomicPtr, Ordering};
use vm_memory::*;
//use lawson_log::*;
//use lawson_log::logs::debug!;
use base::debug;
use crate::common::{host_guest_endian_mismatch, IS_LITTLE_ENDIAN};
use setjmp::{jmp_buf, longjmp, setjmp};

#[derive(Copy, Clone, PartialEq)]
pub enum MemEndian {
    Little,
    Big
}
#[derive(Clone)]
pub struct flat_mem {
    pub guest_mem: GuestMemory,
    pub is_usermode: bool,
  //  should_panic: bool,
}
#[derive(Debug)]
pub enum MemError {
    FlatErr(vm_memory::guest_memory::Error)
}
pub fn raw2array(addr: u64, buf: &mut [u8]) {
    let mut ptr: *const u8 = addr as *const u8;
    unsafe  {
        for i in 0..buf.len() {
            buf[i] = *ptr.offset(i as isize);
        }
    }
}
pub fn array2raw(addr: u64, buf: &[u8]) {
    let mut ptr: *mut u8 = addr as *mut u8;
    unsafe  {
        for i in 0..buf.len() {
            *ptr.offset(i as isize) = buf[i];
        }
    }
}
impl flat_mem {
    pub fn new_usermode() -> flat_mem {
        let gm = GuestMemory::new(&[]).unwrap();
        flat_mem {
            is_usermode: true,
            guest_mem: gm
        }
    }
    pub fn new_system(gm: GuestMemory) -> flat_mem {
        flat_mem {
            is_usermode: false,
            guest_mem: gm
        }
    }
    pub fn read_phys_n(&mut self, addr: u64, len: usize) -> result::Result<Vec<u8>, MemError> {
        let mut retthis: Vec<u8> = vec![0; len];
        if self.is_usermode {
            raw2array(addr, &mut retthis);
            return Ok(retthis);
        }
        let s = self.guest_mem.read_at_addr(&mut retthis, GuestAddress(addr))
            .map_err(MemError::FlatErr)?;
        if s != len {
            debug!("mismatch in read_phys_n()");
        }
        return Ok(retthis);
    }
    pub fn read_phys_8(&mut self, addr: u64) ->  result::Result<u8, MemError> {
        if self.is_usermode {
            let mut ptr: *mut u8 = addr as *mut u8;
            let val = unsafe { *ptr };
            return Ok(val);
        }
        let mut buf: [u8; 1] = Default::default();
        if self.is_usermode {
            let mut ptr: *const u8 = addr as *const u8;
            unsafe  {
                buf[0] = *ptr;
            }
            return Ok(buf[0]);
        }
        let s = self.guest_mem.read_at_addr(&mut buf, GuestAddress(addr))
            .map_err(MemError::FlatErr)?;
        if s != 1 {
            debug!("mismatch in read");
        }
        return Ok(buf[0]); // we should panic before we get here in case invalid
    }
    pub fn read_phys_16(&mut self, addr: u64, endian: MemEndian) ->  result::Result<u16, MemError> {
        if self.is_usermode {
            let mut ptr: *mut u16 = addr as *mut u16;
            let val = unsafe { ptr.read_unaligned() };
            let mut retval = if host_guest_endian_mismatch(endian) {
                val.swap_bytes()
            } else {
                val
            };
            return Ok(retval);
        }
        let mut buf: [u8; 2] = Default::default();
        if self.is_usermode {
            raw2array(addr, &mut buf);
        } else {
            let s = self.guest_mem.read_at_addr(&mut buf, GuestAddress(addr))
                .map_err(MemError::FlatErr)?;
            if s != 2 {
                //   panic!("mismatch in read")
                // println
                debug!("mismatch in read");
            }
        }

        if endian == MemEndian::Big {
            return Ok(u16::from_be_bytes(buf));
        } else {
            return Ok(u16::from_le_bytes(buf));
        };

    }
    pub fn read_phys_32(&mut self, addr: u64, endian: MemEndian) ->  result::Result<u32, MemError> {
        if self.is_usermode {
            let mut ptr: *mut u32 = addr as *mut u32;
            let val = unsafe { ptr::read_unaligned(ptr) };
            let mut retval = if host_guest_endian_mismatch(endian) {
                val.swap_bytes()
            } else {
                val
            };
            return Ok(retval);
        }
        let mut buf: [u8; 4] = Default::default();
        if self.is_usermode {
            raw2array(addr, &mut buf);
        } else {
            let s = self.guest_mem.read_at_addr(&mut buf, GuestAddress(addr))
                .map_err(MemError::FlatErr)?;
            if s != 4 {
                debug!("mismatch in read");
            }
        }
        if endian == MemEndian::Big {
            return Ok(u32::from_be_bytes(buf));
        } else {
            return Ok(u32::from_le_bytes(buf));
        };
    }
    pub fn read_phys_64(&mut self, addr: u64, endian: MemEndian) ->  result::Result<u64, MemError> {
        if self.is_usermode {
            let mut ptr: *mut u64 = addr as *mut u64;
            let val = unsafe { ptr.read_unaligned() };
            let mut retval = if host_guest_endian_mismatch(endian) {
                val.swap_bytes()
            } else {
                val
            };
            return Ok(retval);
        }
        let mut buf: [u8; 8] = Default::default();
        if self.is_usermode {
            raw2array(addr, &mut buf);
        } else {
            let s = self.guest_mem.read_at_addr(&mut buf, GuestAddress(addr))
                .map_err(MemError::FlatErr)?;
            if s != 8 {
                debug!("mismatch in read_phys_64()");
            }
        }
        if endian == MemEndian::Big {
            return Ok(u64::from_be_bytes(buf));
        } else {
            return Ok(u64::from_le_bytes(buf));
        };
    }
    pub fn write_phys_n(&mut self, addr: u64, dat: Vec<u8>) ->  result::Result<(), MemError> {
        if self.is_usermode {
            array2raw(addr, &dat);
        } else {
            self.guest_mem.write_all_at_addr(&dat, GuestAddress(addr))
                .map_err(MemError::FlatErr)?;

        }
        return Ok(());
    }
    pub fn write_phys_8(&mut self, addr: u64, val: u8) ->  result::Result<(), MemError>  {
        if self.is_usermode {
            let mut ptr: *mut u8 = addr as *mut u8;
            unsafe {
                *ptr = val;
            }
            return Ok(());
        }
        let mut buf: [u8; 1] = [val];
        if self.is_usermode {
            array2raw(addr, &buf)
        } else {
            let s = self.guest_mem.write_at_addr(& buf, GuestAddress(addr))
                .map_err(MemError::FlatErr)?;
            if s != 1 {
                debug!("mismatch in write");
            }
        }
        Ok(())
    }
    pub fn write_phys_16(&mut self, addr: u64, val: u16, endian: MemEndian) ->  result::Result<(), MemError> {
        if self.is_usermode {
            let mut ptr: *mut u16 = addr as *mut u16;
            let mut writeval = if host_guest_endian_mismatch(endian) {
                val.swap_bytes()
            } else {
                val
            };
            unsafe {
                *ptr = writeval;
            }
            return Ok(());
        }
        let mut buf: [u8; 2] = if endian == MemEndian::Big {
            val.to_be_bytes()
        } else {
            val.to_le_bytes()
        };
        if self.is_usermode {
            array2raw(addr, &buf)
        } else {
            let s = self.guest_mem.write_at_addr(& buf, GuestAddress(addr))
                .map_err(MemError::FlatErr)?;
            if s != 2 {
                debug!("mismatch in write");
            }
        }
        Ok(())

    }

    pub fn write_phys_32(&mut self, addr: u64, val: u32, endian: MemEndian) ->  result::Result<(), MemError> {
        if self.is_usermode {
            let mut ptr: *mut u32 = addr as *mut u32;
            let mut writeval = if host_guest_endian_mismatch(endian) {
                val.swap_bytes()
            } else {
                val
            };
            unsafe {
                *ptr = writeval;
            }
            return Ok(());
        }
        let mut buf: [u8; 4] = if endian == MemEndian::Big {
            val.to_be_bytes()
        } else {
            val.to_le_bytes()
        };
        if self.is_usermode {
            array2raw(addr, &buf)
        } else {
            let s = self.guest_mem.write_at_addr(& buf, GuestAddress(addr))
                .map_err(MemError::FlatErr)?;
            if s != 4 {
                debug!("mismatch in write");
            }
        }
        Ok(())
    }
    pub fn write_phys_64(&mut self, addr: u64, val: u64,
                         endian: MemEndian) ->  result::Result<(), MemError>  {
        if self.is_usermode {
            let mut ptr: *mut u64 = addr as *mut u64;
            let mut writeval = if host_guest_endian_mismatch(endian) {
                val.swap_bytes()
            } else {
                val
            };
            unsafe {
                ptr.write_unaligned(writeval);
            }
            return Ok(());
        }
        let mut buf: [u8; 8] = if endian == MemEndian::Big {
            val.to_be_bytes()
        } else {
            val.to_le_bytes()
        };
        if self.is_usermode {
            array2raw(addr, &buf)
        } else {
            let s = self.guest_mem.write_at_addr(& buf, GuestAddress(addr))
                .map_err(MemError::FlatErr)?;
            if s != 8 {
                debug!("mismatch in write");
            }
        }
        Ok(())

    }
    pub fn write_phys_8_atomic(&mut self, addr: u64, val: u8, order: Ordering) {
        let mut ptr: *mut u8 = if self.is_usermode {
            addr as *mut u8
        } else {
            self.guest_mem.get_host_address(GuestAddress(addr)).unwrap() as *mut u8
        };
        let atom_ptr = AtomicPtr::new(ptr);

        let mut writeval = val;
        atom_ptr.store(&mut writeval, order);
    }
    pub fn write_phys_16_atomic(&mut self, addr: u64, val: u16, endian: MemEndian, order: Ordering) {
        let mut ptr: *mut u16 = if self.is_usermode {
            addr as *mut u16
        } else {
            self.guest_mem.get_host_address(GuestAddress(addr)).unwrap() as *mut u16
        };
        let atom_ptr = AtomicPtr::new(ptr);
        let mut writeval = if (IS_LITTLE_ENDIAN && endian == MemEndian::Big) || (!IS_LITTLE_ENDIAN && endian == MemEndian::Little) {
            val.swap_bytes()
        } else {
            val
        };
        atom_ptr.store(&mut writeval, order);
    }
    pub fn write_phys_32_atomic(&mut self, addr: u64, val: u32, endian: MemEndian, order: Ordering) {
        let mut ptr: *mut u32 = if self.is_usermode {
            addr as *mut u32
        } else {
            self.guest_mem.get_host_address(GuestAddress(addr)).unwrap() as *mut u32
        };
        let atom_ptr = AtomicPtr::new(ptr);
        let mut writeval = if (IS_LITTLE_ENDIAN && endian == MemEndian::Big) || (!IS_LITTLE_ENDIAN && endian == MemEndian::Little) {
            val.swap_bytes()
        } else {
            val
        };
        atom_ptr.store(&mut writeval, order);
    }
    pub fn write_phys_64_atomic(&mut self, addr: u64, val: u64, endian: MemEndian, order: Ordering) {
        let mut ptr: *mut u64 = if self.is_usermode {
            addr as *mut u64
        } else {
            self.guest_mem.get_host_address(GuestAddress(addr)).unwrap() as *mut u64
        };
        let atom_ptr = AtomicPtr::new(ptr);
        let mut writeval = if (IS_LITTLE_ENDIAN && endian == MemEndian::Big) || (!IS_LITTLE_ENDIAN && endian == MemEndian::Little) {
            val.swap_bytes()
        } else {
            val
        };
        atom_ptr.store(&mut writeval, order);
    }
    pub fn read_phys_8_atomic(&mut self, addr: u64, order: Ordering, ) -> u8 {
        let mut ptr: *mut u8 = if self.is_usermode {
            addr as *mut u8
        } else {
            self.guest_mem.get_host_address(GuestAddress(addr)).unwrap() as *mut u8
        };
        let atom_ptr = AtomicPtr::new(ptr);
        unsafe  {
            *atom_ptr.load(order)
        }
    }
    pub fn read_phys_16_atomic(&mut self, addr: u64, endian: MemEndian, order: Ordering) -> u16 {
        let mut ptr: *mut u16 = if self.is_usermode {
            addr as *mut u16
        } else {
            self.guest_mem.get_host_address(GuestAddress(addr)).unwrap() as *mut u16
        };
        let atom_ptr = AtomicPtr::new(ptr);
        let v = unsafe  {
            *atom_ptr.load(order)
        };
        if (IS_LITTLE_ENDIAN && endian == MemEndian::Big) || (!IS_LITTLE_ENDIAN && endian == MemEndian::Little) {
            v.swap_bytes()
        } else {
            v
        }
    }
    pub fn read_phys_32_atomic(&mut self, addr: u64, endian: MemEndian, order: Ordering) -> u32 {
        let mut ptr: *mut u32 = if self.is_usermode {
            addr as *mut u32
        } else {
            self.guest_mem.get_host_address(GuestAddress(addr)).unwrap() as *mut u32
        };
        let atom_ptr = AtomicPtr::new(ptr);
        let v = unsafe  {
            *atom_ptr.load(order)
        };
        if (IS_LITTLE_ENDIAN && endian == MemEndian::Big) || (!IS_LITTLE_ENDIAN && endian == MemEndian::Little) {
            v.swap_bytes()
        } else {
            v
        }
    }
    pub fn read_phys_64_atomic(&mut self, addr: u64, endian: MemEndian, order: Ordering) -> u64 {
        let mut ptr: *mut u64 = if self.is_usermode {
            addr as *mut u64
        } else {
            self.guest_mem.get_host_address(GuestAddress(addr)).unwrap() as *mut u64
        };
        let atom_ptr = AtomicPtr::new(ptr);
        let v = unsafe  {
            *atom_ptr.load(order)
        };
        if (IS_LITTLE_ENDIAN && endian == MemEndian::Big) || (!IS_LITTLE_ENDIAN && endian == MemEndian::Little) {
            v.swap_bytes()
        } else {
            v
        }
    }
    pub fn swap_atomic_imm_32(&mut self, addr: u64, imm: u32, endian: MemEndian, order: Ordering) -> u32 {
        let mut realimm = if (IS_LITTLE_ENDIAN && endian == MemEndian::Big) || (!IS_LITTLE_ENDIAN && endian == MemEndian::Little) {
            imm.swap_bytes()
        } else {
            imm
        };
        let mut ptr: *mut u32 = if self.is_usermode {
            addr as *mut u32
        } else {
            self.guest_mem.get_host_address(GuestAddress(addr)).unwrap() as *mut u32
        };
        let mut ptr2: *mut u32 = &mut realimm;
        // let mut atom_ptr = AtomicPtr::new(ptr);
        let mut rawptr = ptr as u64;
        let rawptrlocat = (&mut rawptr) as *const u64 as u64;
        let mut atom_ptr: &mut AtomicPtr<u32> = unsafe {

            // &mut *(ptr as *mut *mut u32 as *mut AtomicPtr<u32>)
            // std::mem::transmute((&rawptr) as *mut u32)
            &mut *(rawptrlocat as *mut AtomicPtr<u32>)
        };
        let origval = unsafe { *atom_ptr.swap(ptr2, order) };
        let mut retval = origval;
        if (IS_LITTLE_ENDIAN && endian == MemEndian::Big) || (!IS_LITTLE_ENDIAN && endian == MemEndian::Little) {
            retval.swap_bytes()
        } else {
            retval
        }

    }
}