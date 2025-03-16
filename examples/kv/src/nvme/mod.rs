mod capabilities;
mod version;
mod admin;
use anyhow::{bail, Result};
use std::io;
use std::ptr;
use std::os::fd::AsRawFd;
use std::fs::File;
use std::sync::Arc;

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
    device: Arc<File>,
    registers: *mut NvmeRegisters,
}

impl NvmeController {
    pub(crate) fn new(device: File, region_size: u64, region_offset: u64) -> Result<Self> {
        let device = Arc::new(device);
        let device_fd = device.as_raw_fd();
        let mapped_ptr = unsafe {
            libc::mmap(
                ptr::null_mut(),
                region_size as usize,
                libc::PROT_READ | libc::PROT_WRITE,
                libc::MAP_SHARED,
                device_fd,
                region_offset as libc::off_t,
            )
        };
        if mapped_ptr == libc::MAP_FAILED {
            bail! {io::Error::last_os_error()};
        }

        let registers = mapped_ptr as *mut NvmeRegisters;
        Ok(Self {
            device,
            registers,
        })
    }
}


use std::thread::sleep;
use std::time::Duration;

// Controller Configuration (CC) Register Bit Fields
const NVME_CC_EN: u32 = 1 << 0;   // Enable bit
const NVME_CC_IOCQES: u32 = 4 << 20; // Completion Queue Entry Size (4 = 16 bytes)
const NVME_CC_IOSQES: u32 = 6 << 16; // Submission Queue Entry Size (6 = 64 bytes)

// Controller Status (CSTS) Register Bit Fields
const NVME_CSTS_RDY: u32 = 1 << 0; // Controller Ready bit


impl NvmeController {
    /// Enables the NVMe controller
    pub fn enable_controller(&self) -> Result<()> {
        // Ensure the controller is not already enabled
        let current_cc = unsafe { std::ptr::read_volatile(&(*self.registers).cc) };
        if (current_cc & NVME_CC_EN) != 0 {
            bail!("Controller is already enabled");
        }

        // Program the CC register
        let cc_value = NVME_CC_EN | NVME_CC_IOCQES | NVME_CC_IOSQES;
        unsafe {
            std::ptr::write_volatile(&mut (*self.registers).cc, cc_value);
        }
        println!("Controller Enable Command Issued");

        // Poll CSTS register until the Ready bit (RDY) is set
        let mut timeout = 100; // Timeout counter
        while timeout > 0 {
            let csts = unsafe { std::ptr::read_volatile(&(*self.registers).csts) };
            if (csts & NVME_CSTS_RDY) != 0 {
                println!("NVMe Controller is Ready");
                return Ok(());
            }
            sleep(Duration::from_millis(10)); // Short delay before retry
            timeout -= 1;
        }

        bail!("Timeout waiting for NVMe controller to become ready");
    }
}
