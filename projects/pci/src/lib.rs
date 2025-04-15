pub mod ids;
use deku::{DekuContainerRead, DekuContainerWrite, DekuRead, DekuWrite};

#[derive(Debug, Default, DekuRead, DekuWrite)]
#[deku(bits = 2, id_type = "u8")]
pub enum PciStatusDevSelTiming {
    #[deku(id = 0x0)] Fast,
    #[deku(id = 0x1)] Medium,
    #[default]
    #[deku(id = 0x2)] Slow,
}

#[derive(Debug, Default, DekuRead, DekuWrite)]
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

#[derive(Debug, Default, DekuRead, DekuWrite)]
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

