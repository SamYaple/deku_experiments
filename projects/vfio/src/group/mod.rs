mod status;
pub use status::{VfioGroupStatus, VfioGroupStatusFlag};

use crate::{PciAddress, VfioDevice, VfioContainer};
use anyhow::{bail, Result};
use std::fs::{OpenOptions, File};
use std::os::fd::{AsRawFd, RawFd};

#[derive(Debug)]
pub struct VfioGroup {
    handle: File,
    id: u32,
    devices: Vec<VfioDevice>,
}

impl VfioGroup {
    pub fn new(container: &VfioContainer, id: u32) -> Result<Self> {
        let handle = OpenOptions::new()
            .read(true)
            .write(true)
            .open(format! {"/dev/vfio/{id}"})?;
        let group = Self {
            handle,
            id,
            devices: Vec::new(),
        };
        group.init(container)?;
        Ok(group)
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }

    pub fn add_device(&mut self, address: PciAddress) -> Result<&VfioDevice> {
        let device = VfioDevice::new(self, address)?;
        self.devices.push(device);
        Ok(self.devices.last().unwrap())
    }

    fn init(&self, container: &VfioContainer) -> Result<()> {
        // Check that the vfio group is viable and we can associate it to a
        // vfio container
        let status = VfioGroupStatus::new(self)?;
        if !status.get_flag(VfioGroupStatusFlag::Viable) {
            bail! {"VFIO group not viable"};
        }

        // Associate the VFIO group with the container
        let group_fd = self.as_raw_fd();
        let container_fd = container.as_raw_fd();
        let ret = unsafe { libc::ioctl(group_fd, crate::VFIO_GROUP_SET_CONTAINER, &container_fd) };
        if ret < 0 {
            bail! {std::io::Error::last_os_error()};
        }
        let status = VfioGroupStatus::new(self)?;
        if !status.get_flag(VfioGroupStatusFlag::ContainerSet) {
            bail! {"Failed to set vfio container for vfio group"};
        }

        // Set iommu type on container
        let ret = unsafe { libc::ioctl(container_fd, crate::VFIO_SET_IOMMU_IOCTL, crate::VFIO_IOMMU_TYPE1V2) };
        if ret < 0 {
            bail! {std::io::Error::last_os_error()};
        }

        Ok(())
    }
}

impl AsRawFd for VfioGroup {
    fn as_raw_fd(&self) -> RawFd {
        self.handle.as_raw_fd()
    }
}
