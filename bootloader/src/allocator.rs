use core::alloc::AllocError;
use core::cell::RefCell;

use bootloader_api::AllocatedFrameRange;
use common::addr::PhysAddr;
use common::frame::FrameAllocError;
use common::frame::FrameAllocator;
use stack_vec::StackVec;
use uefi::services::boot::MemoryDescriptor;

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
            .take(128)
        {
            mem_regions.push(*item);
        }

        let addr = mem_regions[0].physical_start.max(4096);
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

    pub fn allocate_frames(&self, num_frames: usize) -> Result<PhysAddr, AllocError> {
        let mut inner = self.inner.borrow_mut();
        loop {
            let mem_desc = self.mem_regions[inner.region_index];
            let mem_desc_size = mem_desc.number_of_pages * Self::FRAME_SIZE;
            let mem_left_in_desc = mem_desc.physical_start + mem_desc_size - inner.addr;
            if mem_left_in_desc >= Self::FRAME_SIZE * num_frames as u64 {
                let ptr = inner.addr;
                Self::reserve_frames(&mut inner, ptr, num_frames);
                inner.addr += 4096 * num_frames as u64;
                return Ok(PhysAddr::new(ptr));
            }

            if inner.region_index >= self.mem_regions.len() {
                return Err(AllocError);
            }

            inner.region_index += 1;
            inner.addr = self.mem_regions[inner.region_index].physical_start;
        }
    }

    fn reserve_frames(inner: &mut BumpAllocatorInner, base: u64, num_frames: usize) {
        let mut existing_index = None;
        for (i, entry) in inner.allocated_frames.iter().enumerate() {
            let entry_end = entry.base + entry.frames as u64 * Self::FRAME_SIZE;
            if entry_end == base {
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
            });
        }
    }
}

impl FrameAllocator for BumpAllocator {
    fn allocate_frames(&self, num_frames: usize) -> Result<PhysAddr, FrameAllocError> {
        self.allocate_frames(num_frames)
            .map_err(|_| FrameAllocError)
    }

    fn deallocate_frames(
        &self,
        _addr: PhysAddr,
        _num_frames: usize,
    ) -> Result<(), FrameAllocError> {
        unimplemented!()
    }
}
