use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::{fmt, result};
use std::sync::{Arc, Mutex};
use base::Event;
use serde::{Serialize, Deserialize};
use remain::sorted;
use crate::pciaddr::PciAddress;
use thiserror::Error;

/// Used in `Vm::register_ioevent` to indicate a size and optionally value to match.
#[derive(PartialEq, Eq, Serialize, Deserialize, Debug)]
pub enum Datamatch {
    AnyLength,
    U8(Option<u8>),
    U16(Option<u16>),
    U32(Option<u32>),
    U64(Option<u64>),
}
/// Information about how a device was accessed.
#[derive(Copy, Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct BusAccessInfo {
    /// Offset from base address that the device was accessed at.
    pub offset: u64,
    /// Absolute address of the device's access in its address space.
    pub address: u64,
    /// ID of the entity requesting a device access, usually the VCPU id.
    pub id: usize,
    /// Access type describes offset in more detail. Example, VGA has two offsets, this tells the difference
    pub access_type: u16,
}

// Implement `Display` for `MinMax`.
impl std::fmt::Display for BusAccessInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// Result of a write to a device's PCI configuration space.
/// This value represents the state change(s) that occurred due to the write.
#[derive(Debug, Default, PartialEq)]
pub struct ConfigWriteResult {
    /// The BusRange in the vector will be removed from mmio_bus
    pub mmio_remove: Vec<BusRange>,

    /// The BusRange in the vector will be added into mmio_bus
    pub mmio_add: Vec<BusRange>,

    /// The BusRange in the vector will be removed from io_bus
    pub io_remove: Vec<BusRange>,

    /// The BusRange in the vector will be added into io_bus
    pub io_add: Vec<BusRange>,
    pub ioevent_remove: Vec<(Event, u64, Datamatch)>,
    pub ioevent_add: Vec<(Event, u64, Datamatch)>,
    pub removed_pci_devices: Vec<PciAddress>,
}
#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum BusType {
    Mmio,
    Io,
}

/// Trait for devices that respond to reads or writes in an arbitrary address space.
///
/// The device does not care where it exists in address space as each method is only given an offset
/// into its allocated portion of address space.
#[allow(unused_variables)]
pub trait BusDevice: Send {
    /// Returns a label suitable for debug output.
    fn debug_label(&self) -> String;
    /// Reads at `offset` from this device
    fn read(&mut self, offset: BusAccessInfo, data: &mut [u8]) {}
    /// Writes at `offset` into this device
    fn write(&mut self, offset: BusAccessInfo, data: &[u8]) {}
    /// Sets a register in the configuration space. Only used by PCI.
    /// * `reg_idx` - The index of the config register to modify.
    /// * `offset` - Offset in to the register.
    fn config_register_write(
        &mut self,
        reg_idx: usize,
        offset: u64,
        data: &[u8],
    ) -> ConfigWriteResult {
        ConfigWriteResult {
            ..Default::default()
        }
    }
    /// Gets a register from the configuration space. Only used by PCI.
    /// * `reg_idx` - The index of the config register to read.
    fn config_register_read(&self, reg_idx: usize) -> u32 {
        0
    }
    /// Sets a register in the virtual config space. Only used by PCI.
    /// * `reg_idx` - The index of the config register to modify.
    /// * `value` - The value to be written.
    fn virtual_config_register_write(&mut self, reg_idx: usize, value: u32) {}
    /// Gets a register from the virtual config space. Only used by PCI.
    /// * `reg_idx` - The index of the config register to read.
    fn virtual_config_register_read(&self, reg_idx: usize) -> u32 {
        0
    }
    /// Invoked when the device is sandboxed.
    fn on_sandboxed(&mut self) {}

    /// Gets a list of all ranges registered by this BusDevice.
    fn get_ranges(&self) -> Vec<(BusRange, BusType)> {
        Vec::new()
    }

    /// Invoked when the device is destroyed
    fn destroy_device(&mut self) {}

    /// Returns the secondary bus number if this bus device is pci bridge
    fn is_bridge(&self) -> Option<u8> {
        None
    }
}
pub trait BusDeviceSync: BusDevice + Sync {
    fn read(&self, offset: BusAccessInfo, data: &mut [u8]);
    fn write(&self, offset: BusAccessInfo, data: &[u8]);
}

#[sorted]
#[derive(Error, Debug)]
pub enum Error {
    #[error("Bus Range not found")]
    Empty,
    /// The insertion failed because the new device overlapped with an old device.
    #[error("new device {base},{len} overlaps with an old device {other_base},{other_len}")]
    Overlap {
        base: u64,
        len: u64,
        other_base: u64,
        other_len: u64,
    },
}
pub type Result<T> = result::Result<T, Error>;
/// Holds a base and length representing the address space occupied by a `BusDevice`.
///
/// * base - The address at which the range start.
/// * len - The length of the range in bytes.
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct BusRange {
    pub base: u64,
    pub len: u64,
}

impl BusRange {
    /// Returns true if `addr` is within the range.
    pub fn contains(&self, addr: u64) -> bool {
        self.base <= addr && addr < self.base + self.len
    }

    /// Returns true if there is overlap with the given range.
    pub fn overlaps(&self, base: u64, len: u64) -> bool {
        self.base < (base + len) && base < self.base + self.len
    }
}

impl Eq for BusRange {}

impl PartialEq for BusRange {
    fn eq(&self, other: &BusRange) -> bool {
        self.base == other.base
    }
}

impl Ord for BusRange {
    fn cmp(&self, other: &BusRange) -> Ordering {
        self.base.cmp(&other.base)
    }
}

impl PartialOrd for BusRange {
    fn partial_cmp(&self, other: &BusRange) -> Option<Ordering> {
        self.base.partial_cmp(&other.base)
    }
}

#[derive(Clone)]
enum BusDeviceEntryMethod {
    OuterSync(Arc<Mutex<dyn BusDevice>>),
    InnerSync(Arc<dyn BusDeviceSync>),
}
#[derive(Clone)]
struct BusDeviceDesc {
    pub BusDeviceEntry: BusDeviceEntryMethod,
    pub access_type: u16

}

/// A device container for routing reads and writes over some address space.
///
/// This doesn't have any restrictions on what kind of device or address space this applies to. The
/// only restriction is that no two devices can overlap in this address space.
#[derive(Clone)]
pub struct Bus {
    devices: Arc<Mutex<BTreeMap<BusRange, BusDeviceDesc>>>,
    access_id: usize,
}

impl Bus {
    /// Constructs an a bus with an empty address space.
    pub fn new() -> Bus {
        Bus {
            devices: Arc::new(Mutex::new(BTreeMap::new())),
            access_id: 0,
        }
    }

    /// Sets the id that will be used for BusAccessInfo.
    pub fn set_access_id(&mut self, id: usize) {
        self.access_id = id;
    }

    fn first_before(&self, addr: u64) -> Option<(BusRange, BusDeviceDesc)> {
        let devices = self.devices.lock().unwrap();
        let (range, dev) = devices
            .range(..=BusRange { base: addr, len: 1 })
            .rev()
            .next()?;
        Some((*range, dev.clone()))
    }

    fn get_device(&self, addr: u64) -> Option<(u64, u64, BusDeviceDesc)> {
        if let Some((range, dev)) = self.first_before(addr) {
            let offset = addr - range.base;
            if offset < range.len {
                return Some((offset, addr, dev));
            }
        }
        None
    }
    pub fn insert(&self, device: Arc<Mutex<dyn BusDevice>>, base: u64, len: u64) -> Result<()> {
        self.insert_with_access_type(device,base,len,0)

    }
    /// Puts the given device at the given address space.
    pub fn insert_with_access_type(&self, device: Arc<Mutex<dyn BusDevice>>, base: u64, len: u64, access_type: u16) -> Result<()> {
        if len == 0 {
            return Err(Error::Overlap {
                base,
                len,
                other_base: 0,
                other_len: 0,
            });
        }

        // Reject all cases where the new device's range overlaps with an existing device.
        let mut devices = self.devices.lock().unwrap();
        devices.iter().try_for_each(|(range, _dev)| {
            if range.overlaps(base, len) {
                Err(Error::Overlap {
                    base,
                    len,
                    other_base: range.base,
                    other_len: range.len,
                })
            } else {
                Ok(())
            }
        })?;
        if devices
            .insert(BusRange { base, len }, BusDeviceDesc{
                BusDeviceEntry: BusDeviceEntryMethod::OuterSync(device),
                access_type: access_type
            })
            .is_some()
        {
            return Err(Error::Overlap {
                base,
                len,
                other_base: base,
                other_len: len,
            });
        }

        Ok(())
    }
    pub fn insert_sync(&self, device: Arc<dyn BusDeviceSync>, base: u64, len: u64) -> Result<()> {
        self.insert_sync_with_access_type(device,base,len,0)
    }
    /// Puts the given device that implements BusDeviceSync at the given address space. Devices
    /// that implement BusDeviceSync manage thread safety internally, and thus can be written to
    /// by multiple threads simultaneously.
    pub fn insert_sync_with_access_type(&self, device: Arc<dyn BusDeviceSync>, base: u64, len: u64, access_type: u16) -> Result<()> {
        if len == 0 {
            return Err(Error::Overlap {
                base,
                len,
                other_base: 0,
                other_len: 0,
            });
        }

        // Reject all cases where the new device's range overlaps with an existing device.
        let mut devices = self.devices.lock().unwrap();
        devices.iter().try_for_each(|(range, _dev)| {
            if range.overlaps(base, len) {
                Err(Error::Overlap {
                    base,
                    len,
                    other_base: range.base,
                    other_len: range.len,
                })
            } else {
                Ok(())
            }
        })?;

        if devices
            .insert(BusRange { base, len }, BusDeviceDesc {
                BusDeviceEntry: BusDeviceEntryMethod::InnerSync(device),
                access_type: access_type
            })
            .is_some()
        {
            return Err(Error::Overlap {
                base,
                len,
                other_base: base,
                other_len: len,
            });
        }

        Ok(())
    }

    /// Remove the given device at the given address space.
    pub fn remove(&self, base: u64, len: u64) -> Result<()> {
        if len == 0 {
            return Err(Error::Overlap {
                base,
                len,
                other_base: 0,
                other_len: 0,
            });
        }

        let mut devices = self.devices.lock().unwrap();
        if devices
            .iter()
            .any(|(range, _dev)| range.base == base && range.len == len)
        {
            let ret = devices.remove(&BusRange { base, len });
            if ret.is_some() {
                Ok(())
            } else {
                Err(Error::Empty)
            }
        } else {
            Err(Error::Empty)
        }
    }

    /// Reads data from the device that owns the range containing `addr` and puts it into `data`.
    ///
    /// Returns true on success, otherwise `data` is untouched.
    pub fn read(&self, addr: u64, data: &mut [u8]) -> bool {
        if let Some((offset, address, dev)) = self.get_device(addr) {
            //debug!("read to address {:02X}", addr);
            let io = BusAccessInfo {
                address,
                offset,
                id: self.access_id,
                access_type: dev.access_type
            };
            match dev.BusDeviceEntry {
                BusDeviceEntryMethod::OuterSync(dev) => {
                    let mut dev= dev.lock().unwrap();
                    // debug!("read from address {:02X}, device is {}", addr, dev.debug_label());
                    dev.read(io, data)
                },
                BusDeviceEntryMethod::InnerSync(dev) => dev.read(io, data),
            }
            true
        } else {
            /// debug debug remove
            println!("unhandled read: {:02X}", addr);
            false
        }
    }

    /// Writes `data` to the device that owns the range containing `addr`.
    ///
    /// Returns true on success, otherwise `data` is untouched.
    pub fn write(&self, addr: u64, data: &[u8]) -> bool {
        if let Some((offset, address, dev)) = self.get_device(addr) {
            //debug!("write to address {:02X}", addr);

            let io = BusAccessInfo {
                address,
                offset,
                id: self.access_id,
                access_type: dev.access_type
            };
            match dev.BusDeviceEntry {
                BusDeviceEntryMethod::OuterSync(dev) => {
                    let mut dev= dev.lock().unwrap();
                    //   debug!("write to address {:02X}, device is {}", addr, dev.debug_label());
                    dev.write(io, data)
                }
                BusDeviceEntryMethod::InnerSync(dev) => dev.write(io, data),
            }
            true
        } else {
            /// debug debug remove
            println!("unhandled write: {:02X}", addr);
            false
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    struct DummyDevice;
    impl BusDevice for DummyDevice {
        fn debug_label(&self) -> String {
            "dummy device".to_owned()
        }
    }

    struct ConstantDevice {
        uses_full_addr: bool,
    }

    impl BusDevice for ConstantDevice {
        fn debug_label(&self) -> String {
            "constant device".to_owned()
        }

        fn read(&mut self, info: BusAccessInfo, data: &mut [u8]) {
            let addr = if self.uses_full_addr {
                info.address
            } else {
                info.offset
            };

            for (i, v) in data.iter_mut().enumerate() {
                if info.access_type == 1 {
                    *v = 0x76;
                } else {
                    *v = (addr as u8) + (i as u8);
                }

            }
        }

        fn write(&mut self, info: BusAccessInfo, data: &[u8]) {
            let addr = if self.uses_full_addr {
                info.address
            } else {
                info.offset
            };
            for (i, v) in data.iter().enumerate() {
                if info.access_type == 1 {
                    assert_eq!(*v, 0x76)
                } else {
                    assert_eq!(*v, (addr as u8) + (i as u8))
                }

            }
        }
    }

    #[test]
    fn bus_insert() {
        let bus = Bus::new();
        let dummy = Arc::new(Mutex::new(DummyDevice));
        assert!(bus.insert(dummy.clone(), 0x10, 0).is_err());
        assert!(bus.insert(dummy.clone(), 0x10, 0x10).is_ok());
        assert!(bus.insert(dummy.clone(), 0x0f, 0x10).is_err());
        assert!(bus.insert(dummy.clone(), 0x10, 0x10).is_err());
        assert!(bus.insert(dummy.clone(), 0x10, 0x15).is_err());
        assert!(bus.insert(dummy.clone(), 0x12, 0x15).is_err());
        assert!(bus.insert(dummy.clone(), 0x12, 0x01).is_err());
        assert!(bus.insert(dummy.clone(), 0x0, 0x20).is_err());
        assert!(bus.insert(dummy.clone(), 0x20, 0x05).is_ok());
        assert!(bus.insert(dummy.clone(), 0x25, 0x05).is_ok());
        assert!(bus.insert(dummy, 0x0, 0x10).is_ok());
    }

    #[test]
    fn bus_insert_full_addr() {
        let bus = Bus::new();
        let dummy = Arc::new(Mutex::new(DummyDevice));
        assert!(bus.insert(dummy.clone(), 0x10, 0).is_err());
        assert!(bus.insert(dummy.clone(), 0x10, 0x10).is_ok());
        assert!(bus.insert(dummy.clone(), 0x0f, 0x10).is_err());
        assert!(bus.insert(dummy.clone(), 0x10, 0x10).is_err());
        assert!(bus.insert(dummy.clone(), 0x10, 0x15).is_err());
        assert!(bus.insert(dummy.clone(), 0x12, 0x15).is_err());
        assert!(bus.insert(dummy.clone(), 0x12, 0x01).is_err());
        assert!(bus.insert(dummy.clone(), 0x0, 0x20).is_err());
        assert!(bus.insert(dummy.clone(), 0x20, 0x05).is_ok());
        assert!(bus.insert(dummy.clone(), 0x25, 0x05).is_ok());
        assert!(bus.insert(dummy, 0x0, 0x10).is_ok());
    }

    #[test]
    fn bus_read_write() {
        let bus = Bus::new();
        let dummy = Arc::new(Mutex::new(DummyDevice));
        assert!(bus.insert(dummy, 0x10, 0x10).is_ok());
        assert!(bus.read(0x10, &mut [0, 0, 0, 0]));
        assert!(bus.write(0x10, &[0, 0, 0, 0]));
        assert!(bus.read(0x11, &mut [0, 0, 0, 0]));
        assert!(bus.write(0x11, &[0, 0, 0, 0]));
        assert!(bus.read(0x16, &mut [0, 0, 0, 0]));
        assert!(bus.write(0x16, &[0, 0, 0, 0]));
        assert!(!bus.read(0x20, &mut [0, 0, 0, 0]));
        assert!(!bus.write(0x20, &[0, 0, 0, 0]));
        assert!(!bus.read(0x06, &mut [0, 0, 0, 0]));
        assert!(!bus.write(0x06, &[0, 0, 0, 0]));
    }

    #[test]
    fn bus_read_write_values() {
        let bus = Bus::new();
        let dummy = Arc::new(Mutex::new(ConstantDevice {
            uses_full_addr: false,
        }));
        assert!(bus.insert(dummy, 0x10, 0x10).is_ok());

        let mut values = [0, 1, 2, 3];
        assert!(bus.read(0x10, &mut values));
        assert_eq!(values, [0, 1, 2, 3]);
        assert!(bus.write(0x10, &values));
        assert!(bus.read(0x15, &mut values));
        assert_eq!(values, [5, 6, 7, 8]);
        assert!(bus.write(0x15, &values));
    }

    #[test]
    fn bus_read_write_values_with_access_type() {
        let bus = Bus::new();
        let dummy = Arc::new(Mutex::new(ConstantDevice {
            uses_full_addr: false,
        }));
        assert!(bus.insert_with_access_type(dummy, 0x10, 0x10, 1).is_ok());

        let mut values = [0, 1, 2, 3];
        assert!(bus.read(0x10, &mut values));
        assert_eq!(values, [0x76, 0x76, 0x76, 0x76]);
        assert!(bus.write(0x10, &values));
    }

    #[test]
    fn bus_read_write_full_addr_values() {
        let bus = Bus::new();
        let dummy = Arc::new(Mutex::new(ConstantDevice {
            uses_full_addr: true,
        }));
        assert!(bus.insert(dummy, 0x10, 0x10).is_ok());

        let mut values = [0u8; 4];
        assert!(bus.read(0x10, &mut values));
        assert_eq!(values, [0x10, 0x11, 0x12, 0x13]);
        assert!(bus.write(0x10, &values));
        assert!(bus.read(0x15, &mut values));
        assert_eq!(values, [0x15, 0x16, 0x17, 0x18]);
        assert!(bus.write(0x15, &values));
    }

    #[test]
    fn bus_range_contains() {
        let a = BusRange {
            base: 0x1000,
            len: 0x400,
        };
        assert!(a.contains(0x1000));
        assert!(a.contains(0x13ff));
        assert!(!a.contains(0xfff));
        assert!(!a.contains(0x1400));
        assert!(a.contains(0x1200));
    }

    #[test]
    fn bus_range_overlap() {
        let a = BusRange {
            base: 0x1000,
            len: 0x400,
        };
        assert!(a.overlaps(0x1000, 0x400));
        assert!(a.overlaps(0xf00, 0x400));
        assert!(a.overlaps(0x1000, 0x01));
        assert!(a.overlaps(0xfff, 0x02));
        assert!(a.overlaps(0x1100, 0x100));
        assert!(a.overlaps(0x13ff, 0x100));
        assert!(!a.overlaps(0x1400, 0x100));
        assert!(!a.overlaps(0xf00, 0x100));
    }
}
