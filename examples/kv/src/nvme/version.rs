use super::NvmeController;
use anyhow::{bail, Result};
use deku::prelude::*;

#[derive(Debug, DekuRead)]
#[deku(endian = "big")]
pub(crate) struct NvmeSpecVersion {
    mjr: u16,
    mnr: u8,
    ter: u8,
}

impl NvmeSpecVersion {
    pub(crate) fn from_raw(val: u32) -> Result<Self> {
        let bytes = val.to_be_bytes();
        let ((_, remaining), version) = NvmeSpecVersion::from_bytes((&bytes, 0))?;
        if remaining > 0 {
            bail! {"failed to consume all data when parsing NvmeSpecVersion"};
        }
        Ok(version)
    }
}

impl NvmeController<'_> {
    pub(crate) fn get_spec_version(&self) -> Result<NvmeSpecVersion> {
        let val = unsafe { std::ptr::read_volatile(&self.registers.as_ref().vs) };
        NvmeSpecVersion::from_raw(val)
    }

    pub(crate) fn print_spec_version(&self) -> Result<()> {
        let ver = self.get_spec_version()?;
        println!("NVMe spec version: {}.{}.{}", ver.mjr, ver.mnr, ver.ter);
        Ok(())
    }
}
