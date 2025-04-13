mod command;
pub(crate) mod controller;
pub(crate) use command::{Command, Completion};
mod capabilities;
mod version;
use anyhow::{bail, Result};
use std::io;
use std::os::fd::AsRawFd;
use std::ptr::NonNull;
use vfio::VfioDevice;

#[repr(C)]
struct NvmeRegisters {
    cap: u64,
    vs: u32,
    intms: u32,
    intmc: u32,
    cc: u32,
    rsvd: u32,
    csts: u32,
    nssr: u32,
    aqa: u32,
    asq: u64,
    acq: u64,
    cmbloc: u32,
    cmbsz: u32,
}

type NvmeCommand = [u8; Command::SIZE];
type NvmeCompletion = [u8; Completion::SIZE];

pub(crate) struct NvmeController<'dev> {
    // this is the file handle for the pcie device (through vfio)
    device: &'dev VfioDevice,
    registers: NonNull<NvmeRegisters>,
    admin_submission_queue: NonNull<NvmeCommand>,
    admin_completion_queue: NonNull<NvmeCompletion>,
    admin_submission_queue_tail: u16,
    admin_completion_queue_head: u16,
}

impl<'dev> NvmeController<'dev> {
    pub(crate) fn new(device: &'dev VfioDevice) -> Result<Self> {
        let region_info = device.get_region_info()?;
        let device_fd = device.as_raw_fd();
        let mapped_ptr = unsafe {
            libc::mmap(
                std::ptr::null_mut(),
                region_info.get_size(),
                libc::PROT_READ | libc::PROT_WRITE,
                libc::MAP_SHARED,
                device_fd,
                region_info.get_offset() as libc::off_t,
            )
        };
        if mapped_ptr == libc::MAP_FAILED {
            bail! {io::Error::last_os_error()};
        }
        let registers = mapped_ptr as *mut NvmeRegisters;

        // setup admin queues for dma command transfer/response
        let asq_size = 32;
        let acq_size = 16;
        let asq_ptr = unsafe {
            crate::dma::alloc_aligned_4k_dma_buffer::<NvmeCommand>(
                asq_size * std::mem::size_of::<NvmeCommand>(),
            )?
        };
        let acq_ptr = unsafe {
            crate::dma::alloc_aligned_4k_dma_buffer::<NvmeCompletion>(
                acq_size * std::mem::size_of::<NvmeCompletion>(),
            )?
        };

        // // TODO: (SUBMISSION_QUEUE_DEPTH << 16) | COMPLETION_QUEUE_DEPTH
        // //       seems to work like a netmask, 0s then 1s for valid values
        // let aqa = 0b0000_0000_0011_1111_0000_0000_0011_1111 as u32;
        let aqa = ((asq_size as u32 - 1) << 16) | (acq_size as u32 - 1);
        let asq = unsafe { asq_ptr } as u64;
        let acq = unsafe { acq_ptr } as u64;

        unsafe {
            std::ptr::write_volatile(&mut (*registers).aqa, aqa);
            std::ptr::write_volatile(&mut (*registers).asq, asq);
            std::ptr::write_volatile(&mut (*registers).acq, acq);
        }

        let registers = NonNull::new(registers).expect("the registers pointer is null");
        let admin_submission_queue = NonNull::new(asq_ptr).expect("the pointer is still null");
        let admin_completion_queue = NonNull::new(acq_ptr).expect("the pointer is still null");

        Ok(Self {
            device,
            registers,
            admin_submission_queue,
            admin_completion_queue,
            admin_submission_queue_tail: 0,
            admin_completion_queue_head: 0,
        })
    }
}
