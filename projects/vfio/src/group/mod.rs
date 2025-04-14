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

    pub fn get_id_from_address(address: &PciAddress) -> Result<u32> {
        let symlink_path = format!("/sys/bus/pci/devices/{}/iommu_group", address);
        let resolved_path = std::fs::read_link(symlink_path)?;
        // TODO: I dont like the unwraps here, but under Linux this should always be a file
        //       and it should always be a u32 value (in string form)
        let id = resolved_path.file_name().unwrap().to_str().unwrap();
        Ok(id.parse::<u32>()?)
    }

    pub fn add_device(&mut self, address: &PciAddress) -> Result<()> {
        let device = VfioDevice::new(self, address)?;
        self.devices.push(device);
        Ok(())
    }

    pub fn get_device(&mut self, address: &PciAddress) -> Result<&mut VfioDevice> {
        for dev in &mut self.devices {
            if dev.get_address() == address {
                return Ok(dev);
            }
        }
        bail! {format!{"No pci device with address {} found in VfioGroup. Did you forget to group.add_device()?", address}}
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

    pub fn get_status(&self) -> Result<VfioGroupStatus> {
        VfioGroupStatus::new(self)
    }
}

impl AsRawFd for VfioGroup {
    fn as_raw_fd(&self) -> RawFd {
        self.handle.as_raw_fd()
    }
}
