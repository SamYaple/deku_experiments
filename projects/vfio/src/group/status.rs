use crate::VfioGroup;
use anyhow::{bail, Result};
use deku::{DekuContainerRead, DekuContainerWrite, DekuRead, DekuWrite};
use std::os::fd::AsRawFd;

#[derive(Debug, DekuRead, DekuWrite, PartialEq, Eq)]
pub struct VfioGroupStatus {
    argsz: u32,
    flags: VfioGroupStatusFlags,
}

impl VfioGroupStatus {
    const SERIALIZED_BYTE_SIZE: usize = 8;

    fn default() -> Self {
        Self {
            argsz: Self::SERIALIZED_BYTE_SIZE as u32,
            flags: VfioGroupStatusFlags::default(),
        }
    }

    pub fn new(group: &VfioGroup) -> Result<Self> {
        let group_fd = group.as_raw_fd();
        let default_status = Self::default();
        let mut bytes = default_status.to_bytes()?;
        let ret = unsafe { libc::ioctl(group_fd, crate::VFIO_GROUP_GET_STATUS, bytes.as_mut_ptr()) };
        if ret < 0 {
            bail! { std::io::Error::last_os_error() };
        }
        let ((_, remaining), status) = Self::from_bytes((&bytes, 0))?;
        debug_assert!(remaining == 0);
        Ok(status)
    }

    pub fn get_flag(&self, flag: VfioGroupStatusFlag) -> bool {
        match flag {
            VfioGroupStatusFlag::ContainerSet => self.flags.container_set,
            VfioGroupStatusFlag::Viable       => self.flags.viable,
        }
    }

    pub fn get_flags(&self) -> Vec<VfioGroupStatusFlag> {
        let mut flags = Vec::new();
        if self.flags.container_set { flags.push(VfioGroupStatusFlag::ContainerSet); }
        if self.flags.viable { flags.push(VfioGroupStatusFlag::Viable); }
        flags
    }
}


#[derive(Debug)]
pub enum VfioGroupStatusFlag {
    ContainerSet,
    Viable,
}

// NOTE: This is only valid for little endian architectures
// TODO: maybe detect the arch? config option? does deku support this already?
#[derive(Debug, DekuRead, DekuWrite, PartialEq, Eq, Default)]
struct VfioGroupStatusFlags {
    #[deku(bits = 6)]
    _reserved_31_26: u32,

    #[deku(bits = 1)]
    container_set: bool,

    #[deku(bits = 1)]
    viable: bool,

    #[deku(bits = 24)]
    _reserved_23_00: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_status_decode() {
        let input: &[u8] = &[0x10, 0x00, 0x00, 0x00, 0x03, 0x00, 0x00, 0x00];

        let ((_, remaining), status) =
            VfioGroupStatus::from_bytes((input, 0)).expect("Decoding should succeed");
        assert!(remaining == 0);
        assert_eq!(status.argsz, 0x10);
        assert!(status.get_flag(VfioGroupStatusFlag::Viable) == true);
        assert!(status.get_flag(VfioGroupStatusFlag::ContainerSet) == true);
    }

    #[test]
    fn test_status_decode_single_flag() {
        let input: &[u8] = &[0x08, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00];

        let ((_, remaining), status) =
            VfioGroupStatus::from_bytes((input, 0)).expect("Decoding should succeed");
        assert!(remaining == 0);
        assert_eq!(status.argsz, 0x08);
        assert!(status.get_flag(VfioGroupStatusFlag::Viable) == true);
        assert!(status.get_flag(VfioGroupStatusFlag::ContainerSet) == false);
    }

    #[test]
    fn test_status_decode_no_flags() {
        let input: &[u8] = &[0x04, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];

        let ((_, remaining), status) =
            VfioGroupStatus::from_bytes((input, 0)).expect("Decoding should succeed");
        assert!(remaining == 0);
        assert_eq!(status.argsz, 0x04);
        assert!(status.get_flag(VfioGroupStatusFlag::Viable) == false);
        assert!(status.get_flag(VfioGroupStatusFlag::ContainerSet) == false);
    }

    #[test]
    fn test_size() {
        let input: &[u8] = &[0xBE, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
        let ((_, remaining), status) =
            VfioGroupStatus::from_bytes((input, 0)).expect("Decoding should succeed");
        assert!(remaining == 0);
        let b = status.to_bytes().expect("Serialization failed");
        assert!(b.len() == VfioGroupStatus::SERIALIZED_BYTE_SIZE);
    }
}
