use crate::VfioDevice;
use anyhow::{bail, Result};
use deku::{DekuContainerRead, DekuContainerWrite, DekuRead, DekuWrite};
use std::os::fd::AsRawFd;

#[derive(Debug, DekuRead, DekuWrite, PartialEq, Eq)]
pub struct VfioRegionInfo {
    argsz: u32,
    flags: VfioRegionInfoFlags,
    index: u32,
    cap_offset: u32,
    size: u64,
    offset: u64,
}

impl VfioRegionInfo {
    const SERIALIZED_BYTE_SIZE: usize = 32;

    fn default() -> Self {
        Self {
            argsz: Self::SERIALIZED_BYTE_SIZE as u32,
            flags: VfioRegionInfoFlags::default(),
            index: 0,
            cap_offset: 0,
            size: 0,
            offset: 0,
        }
    }

    pub fn new(device: &VfioDevice, index: u8) -> Result<Self> {
        let device_fd = device.as_raw_fd();
        let mut default_status = Self::default();
        default_status.index = index as u32;
        let mut bytes = default_status.to_bytes()?;
        let ret = unsafe { libc::ioctl(device_fd, crate::VFIO_DEVICE_GET_REGION_INFO, bytes.as_mut_ptr()) };
        if ret < 0 {
            bail! { std::io::Error::last_os_error() };
        }
        let ((_, remaining), region_info) = Self::from_bytes((&bytes, 0))?;
        debug_assert!(remaining == 0);
        Ok(region_info)
    }

    pub fn get_flag(&self, flag: VfioRegionInfoFlag) -> bool {
        match flag {
            VfioRegionInfoFlag::Read  => self.flags.read,
            VfioRegionInfoFlag::Write => self.flags.write,
            VfioRegionInfoFlag::Mmap  => self.flags.mmap,
            VfioRegionInfoFlag::Caps  => self.flags.caps,
        }
    }

    pub fn get_size(&self) -> usize {
        self.size as usize
    }

    pub fn get_offset(&self) -> u64 {
        self.offset
    }
}

pub enum VfioRegionInfoFlag {
    Read,
    Write,
    Mmap,
    Caps,
}

// NOTE: This is only valid for little endian architectures
// TODO: maybe detect the arch? config option? does deku support this already?
#[derive(Debug, DekuRead, DekuWrite, PartialEq, Eq, Default)]
struct VfioRegionInfoFlags {
    #[deku(bits = 4)]
    _reserved_31_28: u8,

    #[deku(bits = 1)]
    caps: bool,

    #[deku(bits = 1)]
    mmap: bool,

    #[deku(bits = 1)]
    write: bool,

    #[deku(bits = 1)]
    read: bool,

    #[deku(bits = 24)]
    _reserved_23_00: u32,
}
