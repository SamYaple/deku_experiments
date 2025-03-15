use anyhow::{bail, Result};
use std::io;
use std::ptr;
use std::os::fd::AsRawFd;
use std::fs::File;

#[repr(C)]
struct NvmeRegisters {
    cap:    u64,
    vs:     u32,
    intms:  u32,
    intmc:  u32,
    cc:     u32,
    rsvd:   u32,
    csts:   u32,
    nssr:   u32,
    aqa:    u32,
    asq:    u64,
    acq:    u64,
    cmbloc: u32,
    cmbsz:  u32,
}

pub(crate) struct NvmeController {
    // this is the file handle for the pcie device (through vfio)
    device: File,

    // see the nvme spec for the registers available
    registers: *mut NvmeRegisters,

    // TODO: enum for this if we ever get beyond vfio access
    region_info: crate::vfio::vfio_region_info,
}

impl NvmeController {
    pub(crate) fn new(device: File) -> Result<Self> {
        let device_fd = device.as_raw_fd();
        let region_info = crate::vfio::get_region_info(device_fd)?;
        println!(
            "Region Info: size = 0x{:x}, offset = 0x{:x}",
            region_info.size, region_info.offset
        );

        // Memory-map the BAR region.
        let mapped_size = region_info.size as usize;
        let mapped_ptr = unsafe {
            libc::mmap(
                ptr::null_mut(),
                mapped_size,
                libc::PROT_READ | libc::PROT_WRITE,
                libc::MAP_SHARED,
                device_fd,
                region_info.offset as libc::off_t,
            )
        };
        if mapped_ptr == libc::MAP_FAILED {
            bail! {io::Error::last_os_error()};
        }
        println!("Mapped BAR region at: {:?}", mapped_ptr);

        let registers = mapped_ptr as *mut NvmeRegisters;
        Ok(Self {
            device,
            registers,
            region_info,
        })
    }

    fn read_vs(&self) -> u32 {
        let vs = unsafe { ptr::read_volatile(&(*self.registers).vs) };
        vs
    }

    // prior to nvme spec 1.2.1, the "ter" version number is reserved space and not used
    pub(crate) fn get_version(&self) -> (u16, u8, u8) {
        let mjr = ((self.read_vs() >> 16) & 0b1111_1111_1111_1111) as u16;
        let mnr = ((self.read_vs() >> 8)  & 0b1111_1111) as u8;
        let ter = ((self.read_vs() >> 0)  & 0b1111_1111) as u8;
        (mjr, mnr, ter)
    }

    fn read_cap(&self) -> u64 {
        let cap = unsafe { ptr::read_volatile(&(*self.registers).cap) };
        cap
    }

    fn cap_reserved_63_58(&self) -> u8 {
        ((self.read_cap() >> 58) & 0b11_1111) as u8
    }

    pub(crate) fn cap_cmbs(&self) -> bool {
        ((self.read_cap() >> 57) & 0b1) != 0
    }

    pub(crate) fn cap_pmbs(&self) -> bool {
        ((self.read_cap() >> 56) & 0b1) != 0
    }

    pub(crate) fn cap_mpsmax(&self) -> u8 {
        ((self.read_cap() >> 52) & 0b1111) as u8
    }

    pub(crate) fn cap_mpsmin(&self) -> u8 {
        ((self.read_cap() >> 48) & 0b1111) as u8
    }

    fn cap_reserved_47_46(&self) -> u8 {
        ((self.read_cap() >> 46) & 0b11) as u8
    }

    pub(crate) fn cap_bps(&self) -> bool {
        ((self.read_cap() >> 45) & 0b1) != 0
    }

    pub(crate) fn cap_css(&self) -> u8 {
        ((self.read_cap() >> 37) & 0b1111_1111) as u8
    }

    pub(crate) fn cap_nssrs(&self) -> bool {
        ((self.read_cap() >> 36) & 0b1) != 0
    }

    pub(crate) fn cap_dstrd(&self) -> u8 {
        ((self.read_cap() >> 32) & 0b1111) as u8
    }

    pub(crate) fn cap_to(&self) -> u8 {
        ((self.read_cap() >> 24) & 0b1111_1111) as u8
    }

    fn cap_reserved_23_19(&self) -> u8 {
        ((self.read_cap() >> 19) & 0b1_1111) as u8
    }

    pub(crate) fn cap_ams(&self) -> (bool, bool) {
        // Weighted Round Robin with Urgent Priority Class
        let wrrups = ((self.read_cap() >> 18) & 0b1) != 0;

        // TODO: do i have anything that sets this bit?
        // Vendor specific bit likely meant to enable a vendor specific arbitration mechanism
        let vendor = ((self.read_cap() >> 17) & 0b1) != 0;

        (wrrups, vendor)
    }

    pub(crate) fn cap_cqr(&self) -> bool {
        ((self.read_cap() >> 16) & 0b1) != 0
    }

    pub(crate) fn cap_mqes(&self) -> u16 {
        ((self.read_cap() >> 0) & 0b1111_1111_1111_1111) as u16
    }

    pub(crate) fn print_caps_table(&self) {
        println!("+-----------------------------------------------------+");
        println!("| NVMe Capabilities                                   |");
        println!("+--------+-------+------------------------------------+");
        println!("| Name   | Value | Description                        |");
        println!("+--------+-------+------------------------------------+");

        print_table_row("CMBS", self.cap_cmbs(), "Controller Memory Buffer Supported");
        print_table_row("PMBS", self.cap_pmbs(), "Persistent Memory Region Supported");
        print_table_row("MPSMAX", self.cap_mpsmax(), "Memory Page Size Maximum");
        print_table_row("MPSMIN", self.cap_mpsmin(), "Memory Page Size Minimum");
        print_table_row("BPS", self.cap_bps(), "Boot Partition Support");

        // TODO: CSS
        // print_table_row("CSS", self.cap_css(), "Command Sets Supported");
        print_table_row("NSSRS", self.cap_nssrs(), "NVM Subsystem Reset Supported");
        print_table_row("DSTRD", self.cap_dstrd(), "Doorbell Stride");
        print_table_row("TO", self.cap_to(), "Timeout (500ms units)");

        // TODO: print small bit table somehow?
        //let (ams_wrrups, ams_vendor) = self.cap_ams();
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

        print_table_row("CQR", self.cap_cqr(), "Contiguous Queues Required");
        print_table_row("MQES", self.cap_mqes(), "Maximum Queue Entries Supported");
        println!("+--------+-------+------------------------------------+");
    }
}

fn print_table_row<T: ToString>(name: &str, value: T, description: &str) {
    // {:<6} is left aligned with at least 6 chars. Shorter values are padded
    println!("| {:<6} | {:>5} | {:<34} |", name, value.to_string(), description);
}
