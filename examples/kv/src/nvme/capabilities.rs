use anyhow::{Result, bail};
use deku::prelude::*;
use super::NvmeController;

#[derive(Debug, DekuRead)]
#[deku(endian = "big")]
pub(crate) struct NvmeCapabilities {
    #[deku(bits = 6)]
    _reserved_63_58: u8,

    #[deku(bits = 1)]
    cmbs: bool,

    #[deku(bits = 1)]
    pmrs: bool,

    #[deku(bits = 4)]
    mpsmax: u8,

    #[deku(bits = 4)]
    mpsmin: u8,

    #[deku(bits = 2)]
    _reserved_47_46: u8,

    #[deku(bits = 1)]
    bps: bool,

    /// (Command Sets Supported) I/O Command Set
    /// If css_io is set to true, then the I/O command set is available. If css_io is set to false,
    /// then only the Admin Command Set is available.
    #[deku(bits = 1, map = "|b: bool| -> Result<_, DekuError> { Ok(!b) }")]
    css_io: bool,

    #[deku(bits = 6)]
    _reserved_43_38: u8,

    /// (Command Sets Supported) NVM command set
    #[deku(bits = 1)]
    css_nvm: bool,

    #[deku(bits = 1)]
    nssrs: bool,

    #[deku(bits = 4)]
    dstrd: u8,

    /// (TO) Timeout in 500ms units
    #[deku(bits = 8)]
    to: u8,

    #[deku(bits = 5)]
    _reserved_23_19: u8,

    #[deku(bits = 1)]
    ams_wrrups: bool,

    #[deku(bits = 1)]
    ams_vendor: bool,

    #[deku(bits = 1)]
    cqr: bool,

    #[deku(bits = 16)]
    mqes: u16,
}

impl NvmeController {
    pub(crate) fn get_capabilities(&self) -> Result<NvmeCapabilities> {
        let val = unsafe { std::ptr::read_volatile(&self.registers.as_ref().cap) };
        let bytes = val.to_be_bytes();
        let ((_, remaining), caps) = NvmeCapabilities::from_bytes((&bytes, 0))?;
        if remaining > 0 {
            bail!{"failed to consume all data"};
        }
        Ok(caps)
    }

    pub(crate) fn print_caps_table(&self) -> Result<()> {
        let caps = self.get_capabilities()?;

        println!("+-----------------------------------------------------+");
        println!("| NVMe Capabilities                                   |");
        println!("+--------+-------+------------------------------------+");
        println!("| Name   | Value | Description                        |");
        println!("+--------+-------+------------------------------------+");

        print_table_row("CMBS", caps.cmbs, "Controller Memory Buffer Supported");
        print_table_row("PMRS", caps.pmrs, "Persistent Memory Region Supported");
        print_table_row("MPSMAX", caps.mpsmax, "Memory Page Size Maximum");
        print_table_row("MPSMIN", caps.mpsmin, "Memory Page Size Minimum");
        print_table_row("BPS", caps.bps, "Boot Partition Support");

        // TODO: CSS
        // print_table_row("CSS", caps.css, "Command Sets Supported");
        print_table_row("NSSRS", caps.nssrs, "NVM Subsystem Reset Supported");
        print_table_row("DSTRD", caps.dstrd, "Doorbell Stride");
        print_table_row("TO", caps.to, "Timeout (500ms units)");

        // TODO: print small bit table somehow?
        //let (ams_wrrups, ams_vendor) = caps.ams();
        //print_table_row(
        //    "AMS (WRRUP)",
        //    if ams_wrrups { "Y" } else { "N" },
        //    "Weighted Round Robin with Urgent Priority Class",
        //);
        //print_table_row(
        //    "AMS (Vendor)",
        //    if ams_vendor { "Y" } else { "N" },
        //    "Vendor Specific",
        //);

        print_table_row("CQR", caps.cqr, "Contiguous Queues Required");
        print_table_row("MQES", caps.mqes, "Maximum Queue Entries Supported");
        println!("+--------+-------+------------------------------------+");

        Ok(())
    }
}

fn print_table_row<T: ToString>(name: &str, value: T, description: &str) {
    // {:<6} is left aligned with at least 6 chars. Shorter values are padded
    println!("| {:<6} | {:>5} | {:<34} |", name, value.to_string(), description);
}
