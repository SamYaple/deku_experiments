use crate::VfioGroup;
use anyhow::{bail, Result};
use std::fs::{File, OpenOptions};
use std::os::fd::{AsRawFd, RawFd};

#[derive(Debug)]
pub struct VfioContainer {
    handle: File,
    groups: Vec<VfioGroup>,
}

impl AsRawFd for VfioContainer {
    fn as_raw_fd(&self) -> RawFd {
        self.handle.as_raw_fd()
    }
}

impl VfioContainer {
    pub fn new() -> Result<Self> {
        let container = Self::init()?;
        container.check()?;
        Ok(container)
    }

    fn init() -> Result<Self> {
        let handle = OpenOptions::new()
            .read(true)
            .write(true)
            .open("/dev/vfio/vfio")?;
        Ok(Self {
            handle,
            groups: Vec::new(),
        })
    }

    fn check(&self) -> Result<()> {
        let container_fd = self.as_raw_fd();

        // Check vfio api version (always 0?)
        let ret = unsafe { libc::ioctl(container_fd, crate::VFIO_GET_API_VERSION_IOCTL) };
        if ret < 0 {
            bail! {std::io::Error::last_os_error()};
        }
        let api_version = ret as u32;
        if api_version != crate::VFIO_API_VERSION_EXPECTED {
            bail! {"VFIO API version mismatch"};
        }

        // Check for IOMMU Type1v2
        let ret = unsafe {
            libc::ioctl(
                container_fd,
                crate::VFIO_CHECK_EXTENSION_IOCTL,
                crate::VFIO_IOMMU_TYPE1V2,
            )
        };
        if ret < 0 {
            bail! {std::io::Error::last_os_error()};
        }
        if ret == 0 {
            bail! {"VFIO TYPE1v2 IOMMU not supported"};
        }

        Ok(())
    }

    pub fn add_group(&mut self, group_id: u32) -> Result<&mut VfioGroup> {
        let group = VfioGroup::new(self, group_id)?;
        self.groups.push(group);
        Ok(self.groups.last_mut().unwrap())
    }
}
