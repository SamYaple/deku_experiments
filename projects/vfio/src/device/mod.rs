mod region_info;
pub use region_info::VfioRegionInfo;

mod device_info;
pub use device_info::VfioDeviceInfo;

use crate::{VfioGroup, PciAddress};
use anyhow::{bail, Result};
use std::ffi::CString;
use std::fs::File;
use std::os::fd::{AsRawFd, FromRawFd, RawFd};

#[derive(Debug)]
pub struct VfioDevice {
    handle: File,
    group_id: u32,
    address: PciAddress,
}

impl VfioDevice {
    pub fn new(group: &VfioGroup, address: &PciAddress) -> Result<Self> {
        let group_fd = group.as_raw_fd();
        let device_str = CString::new(format!{"{}", address})?;
        let ret = unsafe {
            libc::ioctl(
                group_fd,
                crate::VFIO_GROUP_GET_DEVICE_FD,
                device_str.as_ptr(),
            )
        };
        if ret < 0 {
            bail! {std::io::Error::last_os_error()};
        }
        let handle = unsafe { File::from_raw_fd(ret) };
        let group_id = group.get_id();
        Ok(Self { handle, group_id, address: address.clone() })
    }

    pub fn get_address(&self) -> &PciAddress {
        &self.address
    }

    pub fn get_group_id(&self) -> u32 {
        self.group_id
    }

    pub fn get_device_info(&self) -> Result<VfioDeviceInfo> {
        VfioDeviceInfo::new(self)
    }

    pub fn get_region_info(&self, index: u8) -> Result<VfioRegionInfo> {
        if index >= 9 {
            bail! {"Vfio region index is out of range"};
        }
        VfioRegionInfo::new(self, index)
    }
}

impl AsRawFd for VfioDevice {
    fn as_raw_fd(&self) -> RawFd {
        self.handle.as_raw_fd()
    }
}
