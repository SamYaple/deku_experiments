use anyhow::{Result, bail};
use super::NvmeController;
use libc::posix_memalign;

// Constants
const ADMIN_QUEUE_DEPTH: usize = 32;  // Depth of ASQ and ACQ
const PAGE_SIZE: usize = 4096; // Standard 4KB page size

impl NvmeController {
    /// Allocates and initializes admin queues
    pub fn setup_admin_queues(&self) -> Result<()> {
        let queue_size = ADMIN_QUEUE_DEPTH * std::mem::size_of::<u64>();

        // Allocate page-aligned memory for the queues
        let asq = unsafe { Self::alloc_dma_buffer(queue_size)? } as u64;
        let acq = unsafe { Self::alloc_dma_buffer(queue_size)? } as u64;

        println!(
            "Allocated Admin Submission Queue at: 0x{:x}, size: {}",
            asq, queue_size
        );
        println!(
            "Allocated Admin Completion Queue at: 0x{:x}, size: {}",
            acq, queue_size
        );

        // Write queue attributes (AQA register)
        let aqa_value = ((ADMIN_QUEUE_DEPTH as u32 - 1) << 16) | (ADMIN_QUEUE_DEPTH as u32 - 1);
        unsafe {
            std::ptr::write_volatile(&mut (*self.registers).aqa, aqa_value);
        }

        // Write ASQ base address
        unsafe {
            std::ptr::write_volatile(&mut (*self.registers).asq, asq);
        }

        // Write ACQ base address
        unsafe {
            std::ptr::write_volatile(&mut (*self.registers).acq, acq);
        }

        println!("Admin Queues Set Up Successfully");

        Ok(())
    }

    /// Allocates a page-aligned DMA buffer for queue memory
    unsafe fn alloc_dma_buffer(size: usize) -> Result<*mut u8> {
        let mut ptr: *mut u8 = std::ptr::null_mut();
        if posix_memalign(&mut ptr as *mut *mut u8 as *mut *mut _, PAGE_SIZE, size) != 0 {
            bail!("Failed to allocate DMA buffer");
        }
        Ok(ptr)
    }
}
