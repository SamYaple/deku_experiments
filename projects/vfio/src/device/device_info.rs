use crate::VfioDevice;
use anyhow::{bail, Result};
use deku::{DekuContainerRead, DekuContainerWrite, DekuRead, DekuWrite};
use std::os::fd::AsRawFd;

#[derive(Debug, DekuRead, DekuWrite, PartialEq, Eq)]
pub struct VfioDeviceInfo {
    argsz: u32,
    flags: VfioDeviceInfoFlags,
    num_regions: u32,
    num_irqs: u32,
    cap_offset: u32,
    _padding: u32,
}

impl VfioDeviceInfo {
    const SERIALIZED_BYTE_SIZE: usize = 24;

    fn default() -> Self {
        Self {
            argsz: Self::SERIALIZED_BYTE_SIZE as u32,
            flags: VfioDeviceInfoFlags::default(),
            num_regions: 0,
            num_irqs: 0,
            cap_offset: 0,
            _padding: 0,
        }
    }

    pub fn new(device: &VfioDevice) -> Result<Self> {
        let device_fd = device.as_raw_fd();
        let default_status = Self::default();
        let mut bytes = default_status.to_bytes()?;
        let ret = unsafe { libc::ioctl(device_fd, crate::VFIO_DEVICE_GET_INFO, bytes.as_mut_ptr()) };
        if ret < 0 {
            bail! { std::io::Error::last_os_error() };
        }
        let ((_, remaining), device_info) = Self::from_bytes((&bytes, 0))?;
        debug_assert!(remaining == 0);
        Ok(device_info)
    }

    pub fn get_flag(&self, flag: VfioDeviceInfoFlag) -> bool {
        match flag {
            VfioDeviceInfoFlag::Reset => self.flags.reset,
            VfioDeviceInfoFlag::Pci => self.flags.pci,
            VfioDeviceInfoFlag::Platform => self.flags.platform,
            VfioDeviceInfoFlag::Amba => self.flags.amba,
            VfioDeviceInfoFlag::Ccw => self.flags.ccw,
            VfioDeviceInfoFlag::Ap => self.flags.ap,
            VfioDeviceInfoFlag::FslMc => self.flags.fsl_mc,
            VfioDeviceInfoFlag::Caps => self.flags.caps,
            VfioDeviceInfoFlag::Cdx => self.flags.cdx,
        }
    }

    pub fn get_flags(&self) -> Vec<VfioDeviceInfoFlag> {
        let mut flags = Vec::new();
        if self.flags.reset { flags.push(VfioDeviceInfoFlag::Reset); }
        if self.flags.pci { flags.push(VfioDeviceInfoFlag::Pci); }
        if self.flags.platform { flags.push(VfioDeviceInfoFlag::Platform); }
        if self.flags.amba { flags.push(VfioDeviceInfoFlag::Amba); }
        if self.flags.ccw { flags.push(VfioDeviceInfoFlag::Ccw); }
        if self.flags.ap { flags.push(VfioDeviceInfoFlag::Ap); }
        if self.flags.fsl_mc { flags.push(VfioDeviceInfoFlag::FslMc); }
        if self.flags.caps { flags.push(VfioDeviceInfoFlag::Caps); }
        if self.flags.cdx { flags.push(VfioDeviceInfoFlag::Cdx); }
        flags
    }

    pub fn get_num_regions(&self) -> usize {
        self.num_regions as usize
    }

    pub fn get_num_irqs(&self) -> usize {
        self.num_irqs as usize
    }
}

#[derive(Debug)]
pub enum VfioDeviceInfoFlag {
    Reset,
    Pci,
    Platform,
    Amba,
    Ccw,
    Ap,
    FslMc,
    Caps,
    Cdx,
}

// NOTE: This is only valid for little endian architectures
// TODO: maybe detect the arch? config option? does deku support this already?
#[derive(Debug, DekuRead, DekuWrite, PartialEq, Eq, Default)]
struct VfioDeviceInfoFlags {
    #[deku(bits = 1)]
    caps: bool,

    #[deku(bits = 1)]
    fsl_mc: bool,

    #[deku(bits = 1)]
    ap: bool,

    #[deku(bits = 1)]
    ccw: bool,

    #[deku(bits = 1)]
    amba: bool,

    #[deku(bits = 1)]
    platform: bool,

    #[deku(bits = 1)]
    pci: bool,

    #[deku(bits = 1)]
    reset: bool,

    #[deku(bits = 7)]
    _reserved_23_17: u8,

    #[deku(bits = 1)]
    cdx: bool,

    #[deku(bits = 16)]
    _reserved_15_00: u16,
}
