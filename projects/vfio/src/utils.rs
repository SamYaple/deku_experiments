use crate::VfioDevice;
use anyhow::{bail, Result};
use deku::{DekuContainerRead, DekuContainerWrite, DekuRead, DekuWrite};
use std::os::fd::AsRawFd;

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

//
// TODO: Move this all somewhere else other than the utils module
//
#[derive(Debug, Default, DekuRead, DekuWrite)]
#[deku(bits = 2, id_type = "u8")]
pub enum PciStatusDevSelTiming {
    #[deku(id = 0x0)]
    Fast,
    #[deku(id = 0x1)]
    Medium,

    #[default]
    #[deku(id = 0x2)]
    Slow,
}

#[derive(Debug, Default, DekuRead, DekuWrite)]
pub struct PciStatusRegister {
    #[deku(bits = 1)]
    fast_back_to_back_capable: bool,

    #[deku(bits = 1)]
    _reserved_06: u8,

    #[deku(bits = 1)]
    _66mhz_capable: bool,

    #[deku(bits = 1)]
    capabilities_list: bool,

    #[deku(bits = 1)]
    interupt_status: bool,

    #[deku(bits = 3)]
    _reserved_02_00: u8,

    #[deku(bits = 1)]
    deteced_parity_error: bool,

    #[deku(bits = 1)]
    signalled_system_error: bool,

    #[deku(bits = 1)]
    received_master_abort: bool,

    #[deku(bits = 1)]
    received_target_abort: bool,

    #[deku(bits = 1)]
    signalled_target_abort: bool,

    devsel_timing: PciStatusDevSelTiming,

    #[deku(bits = 1)]
    master_data_parity_error: bool,
}

#[derive(Debug, Default, DekuRead, DekuWrite)]
pub struct PciCommandRegister {
    #[deku(bits = 1)]
    _reserved_07: u8,

    #[deku(bits = 1)]
    parity_error_response: bool,

    #[deku(bits = 1)]
    vga_palette_snoop: bool,

    #[deku(bits = 1)]
    memory_write_and_invalidate_enable: bool,

    #[deku(bits = 1)]
    special_cycles: bool,

    #[deku(bits = 1)]
    bus_master: bool,

    #[deku(bits = 1)]
    memory_space: bool,

    #[deku(bits = 1)]
    io_space: bool,

    #[deku(bits = 5)]
    _reserved_15_11: u8,

    #[deku(bits = 1)]
    interrupt_disable: bool,

    #[deku(bits = 1)]
    fast_back_to_back_enable: bool,

    #[deku(bits = 1)]
    serr_enable: bool,
}

#[derive(Debug, DekuRead, DekuWrite)]
#[deku(id = "prog_if", ctx = "prog_if: u8")]
pub enum NonVolatileMemoryProgIf {
    #[deku(id = 0x01)]
    NVMHCI,

    #[deku(id = 0x02)]
    NVMExpress,
}

#[derive(Debug, DekuRead, DekuWrite)]
#[deku(id = "subclass", ctx = "subclass: u8, prog_if: u8")]
pub enum MassStorageSubtype {
    #[deku(id = 0x00)]
    Scsi,

    #[deku(id = 0x01)]
    Ide,

    #[deku(id = 0x08)]
    NonVolatileMemory(#[deku(ctx = "prog_if")] NonVolatileMemoryProgIf),
}

#[derive(Debug, Default, DekuRead, DekuWrite)]
#[deku(id = "class_code", ctx = "class_code: u8, subclass: u8, prog_if: u8")]
pub enum PciDeviceClass {
    #[deku(id = 0x00)]
    UnclassifiedDevice,
    #[deku(id = 0x01)]
    MassStorageController(#[deku(ctx = "subclass, prog_if")] MassStorageSubtype),
    #[deku(id = 0x02)]
    NetworkController,
    #[deku(id = 0x03)]
    DisplayController,
    #[deku(id = 0x04)]
    MultimediaController,
    #[deku(id = 0x05)]
    MemoryController,
    #[deku(id = 0x06)]
    Bridge,
    #[deku(id = 0x07)]
    CommunicationController,
    #[deku(id = 0x08)]
    GenericSystemPeripheral,
    #[deku(id = 0x09)]
    InputDeviceController,
    #[deku(id = 0x0A)]
    DockingStation,
    #[deku(id = 0x0B)]
    Process,
    #[deku(id = 0x0C)]
    SerialBusController,
    #[deku(id = 0x0D)]
    WirelessController,
    #[deku(id = 0x0E)]
    IntelligentController,
    #[deku(id = 0x0F)]
    SatelliteCommunicationsController,
    #[deku(id = 0x10)]
    EncryptionController,
    #[deku(id = 0x11)]
    SignalProcessingController,
    #[deku(id = 0x12)]
    ProcessingAccelerators,
    #[deku(id = 0x13)]
    NonEssentialInstrumentation,
    #[deku(id = 0x40)]
    Coprocessor,
    #[default]
    #[deku(id = 0xFF)]
    UnassignedClass,
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
