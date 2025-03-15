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

    pub(crate) fn print_caps(&self) -> () {
        println!("============================");
        println!("Dumping nvme capabilities...");

        if self.cap_cmbs() {
            println!("CMBS is supported");
        }

        if self.cap_pmbs() {
            println!("PMBS is supported");
        }

        let mpsmax = self.cap_mpsmax();
        dbg![mpsmax];

        let mpsmin = self.cap_mpsmin();
        dbg![mpsmin];

        if self.cap_bps() {
            println!("BPS is supported");
        }

        // TODO fix css spec
        //let css = self.cap_css() {}

        if self.cap_nssrs() {
            println!("NNSRS is supported");
        }

        let dstrd = self.cap_dstrd();
        dbg![dstrd];

        let to = self.cap_to();
        dbg![to];
        println!("TO -- Timeout: {} seconds", to as u64 * 500 / 1000);

        let (ams_wrrups, ams_vendor) = self.cap_ams();
        if ams_wrrups {
            println!("AMS supports 'Weighted Round Robin with Urgent Priority Class'");
        }
        if ams_vendor {
            println!("AMS supports a vendor specific arbitration mechanism");
        }
        if self.cap_cqr() {
            println!("CQR is set; the controller requires I/O queues to be physically contiguous");
        } else {
            println!("CQR is unset; the controller does not require I/O queues to be physically contiguous");
        }

        let mqes = self.cap_mqes();
        dbg![mqes];
        println!("============================");
    }

    pub(crate) fn print_caps_table(&self) {
        println!("+----------------------------------------------+");
        println!("|              NVMe Capabilities               |");
        println!("+----------------------------+-----------------+");

        // Example: CMBS
        print_table_row(
            "CMBS",
            if self.cap_cmbs() { "Supported" } else { "Not supported" },
        );

        // PMBS
        print_table_row(
            "PMBS",
            if self.cap_pmbs() { "Supported" } else { "Not supported" },
        );

        // BPS
        print_table_row(
            "BPS",
            if self.cap_bps() { "Supported" } else { "Not supported" },
        );

        // MPSMAX (numeric)
        let mpsmax = self.cap_mpsmax();
        print_table_row("MPSMAX", &mpsmax.to_string());

        // MPSMIN (numeric)
        let mpsmin = self.cap_mpsmin();
        print_table_row("MPSMIN", &mpsmin.to_string());

        // DSTRD (numeric)
        let dstrd = self.cap_dstrd();
        print_table_row("DSTRD", &dstrd.to_string());

        // TO (in seconds)
        let derived_to = format!("{} seconds", (self.cap_to() as u64 * 500 / 1000));
        print_table_row("TO", &derived_to);

        // AMS features
        let (ams_wrrups, ams_vendor) = self.cap_ams();
        print_table_row(
            "AMS (WRRUP)",
            if ams_wrrups { "Yes" } else { "No" },
        );
        print_table_row(
            "AMS (Vendor)",
            if ams_vendor { "Yes" } else { "No" },
        );

        // CQR
        print_table_row(
            "CQR",
            if self.cap_cqr() { "Set" } else { "Unset" },
        );

        // MQES (numeric)
        let mqes = self.cap_mqes();
        print_table_row("MQES", &mqes.to_string());

        println!("+----------------------------+-----------------+");
    }
}

fn print_table_row(name: &str, value: &str) {
    // Adjust the widths below to suit your formatting.
    // This example left-aligns the name in a 26-char field,
    // then left-aligns the value in a 18-char field.
    println!("| {:<26} | {:<15} |", name, value);
}

