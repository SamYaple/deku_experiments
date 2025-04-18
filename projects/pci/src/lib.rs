pub mod ids;
use ids::PciDeviceClass;

use pci_ids::PciIds;
use anyhow::{Result, bail};
use deku::{DekuContainerRead, DekuContainerWrite, DekuRead, DekuWrite};

#[derive(Debug, Default, PartialEq, DekuRead, DekuWrite)]
#[deku(bits = 2, id_type = "u8")]
pub enum PciStatusDevSelTiming {
    #[deku(id = 0x0)] Fast,
    #[deku(id = 0x1)] Medium,
    #[default]
    #[deku(id = 0x2)] Slow,
}

#[derive(Debug, Default, PartialEq, DekuRead, DekuWrite)]
pub struct PciStatusRegister {
    #[deku(bits = 1)] fast_back_to_back_capable: bool,
    #[deku(bits = 1)] _reserved_06: u8,
    #[deku(bits = 1)] _66mhz_capable: bool,
    #[deku(bits = 1)] capabilities_list: bool,
    #[deku(bits = 1)] interupt_status: bool,
    #[deku(bits = 3)] _reserved_02_00: u8,

    #[deku(bits = 1)] deteced_parity_error: bool,
    #[deku(bits = 1)] signalled_system_error: bool,
    #[deku(bits = 1)] received_master_abort: bool,
    #[deku(bits = 1)] received_target_abort: bool,
    #[deku(bits = 1)] signalled_target_abort: bool,
    devsel_timing: PciStatusDevSelTiming,
    #[deku(bits = 1)] master_data_parity_error: bool,
}

#[derive(Debug, Default, PartialEq, DekuRead, DekuWrite)]
pub struct PciCommandRegister {
    #[deku(bits = 1)] _reserved_07: u8,
    #[deku(bits = 1)] parity_error_response: bool,
    #[deku(bits = 1)] vga_palette_snoop: bool,
    #[deku(bits = 1)] memory_write_and_invalidate_enable: bool,
    #[deku(bits = 1)] special_cycles: bool,
    #[deku(bits = 1)] bus_master: bool,
    #[deku(bits = 1)] memory_space: bool,
    #[deku(bits = 1)] io_space: bool,

    #[deku(bits = 5)] _reserved_15_11: u8,
    #[deku(bits = 1)] interrupt_disable: bool,
    #[deku(bits = 1)] fast_back_to_back_enable: bool,
    #[deku(bits = 1)] serr_enable: bool,
}

#[derive(Debug, Default, PartialEq, DekuRead, DekuWrite)]
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
    pub const SERIALIZED_BYTE_SIZE: usize = 16;

    pub fn new(address: &PciAddress) -> Result<Self> {
        // open format!{"/sys/bus/pci/devices/{address}/config"} for bytes
        let p = std::path::PathBuf::from(format!("/sys/bus/pci/devices/{address}/config"));
        let bytes = std::fs::read(&p)?;
        let ((_, remaining), pci_device) = Self::from_bytes((&bytes, 0))?;
        debug_assert!(remaining == 0);
        Ok(pci_device)
    }
}

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
