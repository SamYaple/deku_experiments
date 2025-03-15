mod capabilities;
mod version;
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

    fn read_cap(&self) -> u64 {
        let cap = unsafe { ptr::read_volatile(&(*self.registers).cap) };
        cap
    }

    fn read_vs(&self) -> u32 {
        let vs = unsafe { ptr::read_volatile(&(*self.registers).vs) };
        vs
    }
}
