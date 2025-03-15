mod capabilities;
mod version;
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
        println!("Region Info: size = 0x{:x}, offset = 0x{:x}", region_info.size, region_info.offset);

        let mapped_ptr = unsafe {
            libc::mmap(
                ptr::null_mut(),
                region_info.size as usize,
                libc::PROT_READ | libc::PROT_WRITE,
                libc::MAP_SHARED,
                device_fd,
                region_info.offset as libc::off_t,
            )
        };
        if mapped_ptr == libc::MAP_FAILED {
            bail! {io::Error::last_os_error()};
        }
        //println!("Mapped BAR region at: {:?}", mapped_ptr);

        let registers = mapped_ptr as *mut NvmeRegisters;
        Ok(Self {
            device,
            registers,
            region_info,
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
