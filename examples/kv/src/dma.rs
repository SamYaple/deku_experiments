use anyhow::{bail, Result};
use core::marker::PhantomData;
use libc::posix_memalign;
use std::ptr::NonNull;

pub struct DmaQueue<T> {
    buffer: NonNull<T>,
    capacity: usize,
    head: usize,
    tail: usize,
    _marker: PhantomData<T>,
}

impl<T> DmaQueue<T> {
    pub unsafe fn new(capacity: usize) -> Result<Self> {
        // TODO: improve these checks (or remove them)
        assert!(capacity > 0, "Capacity must be non-zero");
        let buffer = alloc_aligned_4k_dma_buffer(capacity)?;
        Ok(DmaQueue {
            buffer: NonNull::new(buffer).unwrap(),
            capacity,
            head: 0,
            tail: 0,
            _marker: PhantomData,
        })
    }

    pub fn is_empty(&self) -> bool {
        self.head == self.tail
    }

    pub fn is_full(&self) -> bool {
        (self.head + 1) % self.capacity == self.tail
    }

    pub unsafe fn push(&mut self, value: T) -> Result<(), u64> {
        if self.is_full() {
            todo! {"queue is full and we aren't going to see our tail update"};
        }
        std::ptr::write_volatile(self.buffer.as_ptr().add(self.head), value);
        self.head = (self.head + 1) % self.capacity;
        Ok(())
    }

    ///// Pops a value from the queue.
    /////
    ///// # Safety
    /////
    ///// This function performs unsafe reads from the DMA region.
    //pub unsafe fn pop(&mut self) -> Option<u64> {
    //    if self.is_empty() {
    //        return None;
    //    }
    //    let index = self.tail;
    //    let value = std::ptr::read(self.buffer.as_ptr().add(index));
    //    self.tail = (self.tail + 1) % self.capacity;
    //    Some(value)
    //}
}

pub(crate) unsafe fn alloc_aligned_4k_dma_buffer<T>(size: usize) -> Result<*mut T> {
    let mut ptr: *mut T = std::ptr::null_mut();
    if posix_memalign(&mut ptr as *mut *mut T as *mut *mut _, 4096, size) != 0 {
        bail!("Failed to allocate DMA buffer");
    }
    Ok(ptr)
}
