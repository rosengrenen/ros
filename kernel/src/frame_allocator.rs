use bootloader_api::MemoryRegion;
use core::alloc::AllocError;
use x86_64::paging::{FrameAllocError, FrameAllocator};

use crate::spinlock::Mutex;

pub struct InitFrameAllocator<'a> {
    inner: Mutex<InitFrameAllocatorInner<'a>>,
}

struct InitFrameAllocatorInner<'a> {
    memory_regions: &'a [MemoryRegion],
    descriptor_index: usize,
    addr: u64,
}

impl<'a> InitFrameAllocator<'a> {
    pub fn new(memory_regions: &'a [MemoryRegion]) -> Self {
        Self {
            inner: Mutex::new(InitFrameAllocatorInner {
                memory_regions,
                descriptor_index: 0,
                addr: memory_regions[0].start,
            }),
        }
    }
}

impl<'a> InitFrameAllocator<'a> {
    pub fn allocate_frames(&self, num_frames: u64) -> Result<u64, AllocError> {
        let mut lock = self.inner.lock();
        loop {
            let mem_region = lock.memory_regions[lock.descriptor_index];
            let mem_left_in_region = mem_region.end - lock.addr;
            if mem_left_in_region >= 4096 * num_frames {
                let ptr = lock.addr;
                lock.addr += 4096 * num_frames;
                return Ok(ptr);
            }

            if lock.descriptor_index >= lock.memory_regions.len() {
                return Err(AllocError);
            }

            lock.descriptor_index += 1;
            lock.addr = lock.memory_regions[lock.descriptor_index].start;
        }
    }

    pub fn add_allocated_frames(&self, base: u64, num_frames: usize) {
        let mut lock = self.inner.lock();
        let end = base + num_frames as u64 * 4096;
        if end < lock.addr {
            return;
        }

        while end > lock.memory_regions[lock.descriptor_index].end {
            lock.descriptor_index += 1;
        }

        if lock.memory_regions[lock.descriptor_index].start < end {
            lock.addr = end;
        }
    }
}

impl<'a> FrameAllocator for InitFrameAllocator<'a> {
    fn allocate_frame(&self) -> Result<u64, FrameAllocError> {
        self.allocate_frames(1).map_err(|_| FrameAllocError)
    }

    fn allocate_frame_zeroed(&self) -> Result<u64, FrameAllocError> {
        let base = self.allocate_frame()?;
        // TODO: make helper?
        let frame = unsafe {
            core::slice::from_raw_parts_mut(base as *mut u64, 4096 / core::mem::size_of::<u64>())
        };
        for part in frame {
            *part = 0;
        }
        Ok(base)
    }

    fn deallocate_frame(&self, _frame: u64) -> Result<(), FrameAllocError> {
        Ok(())
    }

    fn deallocate_frame_zeroed(&self, _frame: u64) -> Result<(), FrameAllocError> {
        Ok(())
    }
}

pub struct BitmapFrameAllocator {
    // regions
}
