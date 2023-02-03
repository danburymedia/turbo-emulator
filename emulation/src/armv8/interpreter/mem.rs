use std::sync::atomic::Ordering;
use crate::armv8::interpreter::main::{Arm64Cpu};
use crate::common::memory::MemEndian;
//pub ThirtyFir
#[derive(Copy, Clone)]
pub enum MemAccessType {
    LoadStore,
    Fetch
}
#[derive(Copy, Clone)]
pub struct MemAccessStr {
    pub mtype: MemAccessType,
    pub is_atomic: bool,
}
pub struct MemData {
    pub exc_addr: u64,
    pub exc_size: u64
}
impl Default for MemData {
    fn default() -> Self {
        MemData {
            exc_addr: 0,
            exc_size: 0
        }
    }
}
impl MemAccessStr {
    pub fn std_loadstore() -> MemAccessStr {
        MemAccessStr {
            mtype: MemAccessType::LoadStore,
            is_atomic: false
        }
    }
    pub fn atomic_load() -> MemAccessStr {
        MemAccessStr {
            mtype: MemAccessType::LoadStore,
            is_atomic: true
        }
    }
}
impl Arm64Cpu {
    pub fn read8(&mut self, addr: u64, mem_type: MemAccessStr) -> Option<u8> {
        let val = if mem_type.is_atomic {
            self.memory_access.read_phys_8_atomic(addr as u64,
                                                   Ordering::SeqCst)
        } else {
            self.memory_access.read_phys_8(addr as u64)
        };
        Some(val)

    }
    pub fn read16(&mut self, addr: u64, mem_type: MemAccessStr) -> Option<u16> {
        let val = if mem_type.is_atomic {
            self.memory_access.read_phys_16_atomic(addr as u64,
                                                   MemEndian::Little,
                                                   Ordering::SeqCst)
        } else {
            self.memory_access.read_phys_16(addr as u64, MemEndian::Little)
        };
        Some(val)
    }
    pub fn read32(&mut self, addr: u64, mem_type: MemAccessStr) -> Option<u32> {
        let val = if mem_type.is_atomic {
            self.memory_access.read_phys_32_atomic(addr as u64,
                                                   MemEndian::Little,
                                                   Ordering::SeqCst)
        } else {
            self.memory_access.read_phys_32(addr as u64, MemEndian::Little)
        };
        Some(val)
    }
    pub fn read64(&mut self, addr: u64, mem_type: MemAccessStr) -> Option<u64> {
        let val = if mem_type.is_atomic {
            self.memory_access.read_phys_64_atomic(addr as u64,
                                                   MemEndian::Little,
                                                   Ordering::SeqCst)
        } else {
            self.memory_access.read_phys_64(addr as u64, MemEndian::Little)
        };
        Some(val)
    }

    pub fn write8(&mut self, addr: u64, val: u8, mem_type: MemAccessStr) -> bool {
        if mem_type.is_atomic {
            self.memory_access.write_phys_8_atomic(addr as u64,
                                                    val,
                                                    Ordering::SeqCst);
        } else {
            self.memory_access.write_phys_8(addr as u64, val);
        }
        false
    }
    pub fn write16(&mut self, addr: u64, val: u16, mem_type: MemAccessStr) -> bool {
        if mem_type.is_atomic {
            self.memory_access.write_phys_16_atomic(addr as u64,
                                                    val, MemEndian::Little,
                                                    Ordering::SeqCst);
        } else {
            self.memory_access.write_phys_16(addr as u64, val, MemEndian::Little);
        }
        false
    }
    pub fn write32(&mut self, addr: u64, val: u32, mem_type: MemAccessStr) -> bool {

        if mem_type.is_atomic {
            self.memory_access.write_phys_32_atomic(addr as u64,
                                                    val, MemEndian::Little,
                                                    Ordering::SeqCst);
        } else {
            self.memory_access.write_phys_32(addr as u64, val, MemEndian::Little);
        }
        false
    }
    pub fn write64(&mut self, addr: u64, val: u64, mem_type: MemAccessStr) -> bool {

        if mem_type.is_atomic {
            self.memory_access.write_phys_64_atomic(addr as u64, val,
                                                    MemEndian::Little,
                                                    Ordering::SeqCst);
        } else {
            self.memory_access.write_phys_64(addr as u64, val, MemEndian::Little);
        }
        false
    }
    pub fn set_exclusive_monitors(&mut self, addr: u64, size: u64) {
        self.mdata.exc_addr = addr;
        self.mdata.exc_size = size;
    }
    pub fn clear_exclusive_monitor(&mut self) {
        self.mdata.exc_addr = 0;
        self.mdata.exc_size = 0;
    }
    pub fn check_exclusive_monitors(&mut self, addr: u64, size: u64) -> bool {
        self.mdata.exc_addr == addr && self.mdata.exc_size == size
    }
}