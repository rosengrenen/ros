use bootloader_api::AllocatedFrameRange;
use core::{alloc::AllocError, cell::RefCell};
use stack_vec::StackVec;
use uefi::services::boot::MemoryDescriptor;
use x86_64::paging::{FrameAllocError, FrameAllocator};

pub struct BumpAllocator {
    mem_regions: StackVec<128, MemoryDescriptor>,
    pub inner: RefCell<BumpAllocatorInner>,
}

pub struct BumpAllocatorInner {
    region_index: usize,
    addr: u64,
    pub allocated_frames: StackVec<128, AllocatedFrameRange>,
}

impl BumpAllocator {
    pub fn new<'iter>(memory_map_iter: impl Iterator<Item = &'iter MemoryDescriptor>) -> Self {
        let mut mem_regions = StackVec::new();
        for item in memory_map_iter
            .filter(|desc| desc.ty.usable_by_loader())
            .filter(|desc| desc.physical_start > 0)
            .take(128)
        {
            mem_regions.push(*item);
        }

        let addr = mem_regions[0].physical_start;
        Self {
            mem_regions,
            inner: RefCell::new(BumpAllocatorInner {
                region_index: 0,
                addr,
                allocated_frames: StackVec::new(),
            }),
        }
    }
}

impl BumpAllocator {
    const FRAME_SIZE: u64 = 4096;

    pub fn allocate_frames(&self, num_frames: u64, bootloader: bool) -> Result<u64, AllocError> {
        let mut inner = self.inner.borrow_mut();
        loop {
            let mem_desc = self.mem_regions[inner.region_index];
            let mem_desc_size = mem_desc.number_of_pages * Self::FRAME_SIZE;
            let mem_left_in_desc = mem_desc.physical_start + mem_desc_size - inner.addr;
            if mem_left_in_desc >= Self::FRAME_SIZE * num_frames {
                let base = inner.addr;
                Self::reserve_frames_inner(&mut inner, base, num_frames, bootloader);
                let ptr = inner.addr;
                inner.addr += 4096 * num_frames;
                return Ok(ptr);
            }

            if inner.region_index >= self.mem_regions.len() {
                return Err(AllocError);
            }

            inner.region_index += 1;
            inner.addr = self.mem_regions[inner.region_index].physical_start;
        }
    }

    pub fn reserve_frames(&self, base: u64, num_frames: u64, bootloader: bool) {
        let mut inner = self.inner.borrow_mut();
        Self::reserve_frames_inner(&mut inner, base, num_frames, bootloader)
    }

    fn reserve_frames_inner(
        inner: &mut BumpAllocatorInner,
        base: u64,
        num_frames: u64,
        bootloader: bool,
    ) {
        let mut existing_index = None;
        for (i, entry) in inner.allocated_frames.iter().enumerate() {
            let entry_end = entry.base + entry.frames as u64 * Self::FRAME_SIZE;
            if entry_end == base && entry.bootloader == bootloader {
                existing_index = Some(i);
                break;
            }
        }

        if let Some(index) = existing_index {
            inner.allocated_frames[index].frames += num_frames as usize;
        } else {
            inner.allocated_frames.push(AllocatedFrameRange {
                base,
                frames: num_frames as usize,
                bootloader,
            });
        }
    }
}

impl FrameAllocator for BumpAllocator {
    fn allocate_frame(&self) -> Result<u64, FrameAllocError> {
        self.allocate_frames(1, false).map_err(|_| FrameAllocError)
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
}
