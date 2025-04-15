use crate::VfioDevice;
use anyhow::{bail, Result};
use deku::{DekuContainerRead, DekuContainerWrite, DekuRead, DekuWrite};
use std::os::fd::AsRawFd;
use pci::{PciStatusRegister, PciCommandRegister};
use pci::ids::PciDeviceClass;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PciAddress {
    domain: u16,
    bus: u8,
    device: u8,
    function: u8,
}

impl PciAddress {
    pub fn new(bdf: &str) -> Result<Self> {
        let binding: Vec<_> = bdf.split(':').collect();
        let (dom, b, df) = match binding.as_slice() {
            [dom, b, df] => (dom, b, df),
            [b, df] => (&"0000", b, df),
            _ => bail! {format!{"Invalid bdf format -- '{bdf}'"}},
        };

        let binding: Vec<_> = df.split('.').collect();
        let (d, f) = match binding.as_slice() {
            [d, f] => (d, f),
            _ => bail! {format!{"Invalid bdf format -- '{bdf}'"}},
        };

        // TODO: These can still fail when the str is empty and the error message is not helpful
        let domain = u16::from_str_radix(dom, 16)?;
        let bus = u8::from_str_radix(b, 16)?;
        let device = u8::from_str_radix(d, 16)?;
        let function = u8::from_str_radix(f, 16)?;

        // TODO: check if this is defined in the PCI spec or somewhere else
        if device > 31 {
            bail! {format!{"device must be <= 31, we got '{device}'"}};
        }
        if function > 7 {
            bail! {format!{"device function must be <= 7, we got '{function}'"}};
        }

        Ok(Self { domain, bus, device, function })
    }
}

impl std::fmt::Display for PciAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:04x}:{:02x}:{:02x}.{}", self.domain, self.bus, self.device, self.function)
    }
}

#[derive(Debug, Default, DekuRead, DekuWrite)]
pub struct PciDevice {
    vendor_id: u16,
    device_id: u16,
    command: PciCommandRegister,
    status: PciStatusRegister,
    revision_id: u8,
    prog_if: u8,
    subclass: u8,
    class_code: u8,
    cache_line_size: u8,
    latency_timer: u8,
    header_type: u8,
    bist: u8,

    // Any fields below this line are not part of the PCI spec, they are
    // derived based on the context of the previously parsed values.
    // No additional bits are read or written by Deku to create this field,
    #[deku(ctx = "*class_code, *subclass, *prog_if")]
    pci_id: PciDeviceClass,
}

impl PciDevice {
    const SERIALIZED_BYTE_SIZE: usize = 16;

    pub fn new(device: &mut VfioDevice) -> Result<Self> {
        let region_info = device.get_region_info(7)?;
        let mut bytes = Self::default().to_bytes()?;

        let ret = unsafe { libc::pread(device.as_raw_fd(), bytes.as_mut_ptr() as *mut _, Self::SERIALIZED_BYTE_SIZE, region_info.get_offset() as i64) };
        if ret < 0 {
            bail! {std::io::Error::last_os_error()};
        }

        let ((_, remaining), pci_device) = Self::from_bytes((&bytes, 0))?;
        debug_assert!(remaining == 0);
        Ok(pci_device)
    }
}
