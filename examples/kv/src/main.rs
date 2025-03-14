mod vfio;
use anyhow::{bail, Result};
use std::io;
use std::ptr;
use std::os::fd::IntoRawFd;
use vfio::{get_region_info, VfioContainer};


// what is up with these names 😨
#[repr(C)]
struct NvmeRegs {
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

fn main() -> Result<()> {
    let vfio_container = VfioContainer::new()?;
    let device = vfio_container.open_device("/dev/vfio/42", "0000:02:00.0")?;
    let device_fd = device.into_raw_fd();
    let region_info = get_region_info(device_fd)?;
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

    // Cast returned pointer as NvmeRegs type
    let nvme_regs = mapped_ptr as *mut NvmeRegs;
    let cap = unsafe { ptr::read_volatile(&(*nvme_regs).cap) };
    let vs = unsafe { ptr::read_volatile(&(*nvme_regs).vs) };
    println!("NVMe Controller Capabilities (CAP): 0x{:016x}", cap);
    println!("NVMe Controller Version (VS): 0x{:08x}", vs);

    // cleanup
    unsafe {
        libc::munmap(mapped_ptr, mapped_size);
    }
    println!("Unmapped BAR region");

    Ok(())
}
