use crate::spinlock::Mutex;
use bootloader_api::{AllocatedFrameRange, MemoryRegion};
use stack_vec::StackVec;
use x86_64::paging::{FrameAllocError, FrameAllocator};

pub struct InitFrameAllocator<'a> {
    inner: Mutex<InitFrameAllocatorInner<'a>>,
}

struct InitFrameAllocatorInner<'a> {
    memory_regions: &'a [MemoryRegion],
    addr: u64,
    index: usize,
    allocated_frame_ranges: StackVec<128, AllocatedFrameRange>,
}

impl<'a> InitFrameAllocator<'a> {
    const FRAME_SIZE: u64 = 4096;

    pub fn new(
        memory_regions: &'a [MemoryRegion],
        // Allocated frame ranges from bootloader
        allocated_frame_ranges_bl: &[AllocatedFrameRange],
    ) -> Self {
        let mut max_allocated_addr = 0;
        for range in allocated_frame_ranges_bl
            .iter()
            // This ignores bootloader only frames, which can be problematic since they are to be
            .filter(|range| !range.bootloader)
        {
            let range_end = range.base + range.frames as u64 * Self::FRAME_SIZE;
            let in_region = memory_regions
                .iter()
                .filter(|region| match region.ty {
                    bootloader_api::MemoryRegionType::KernelUsable => true,
                    _ => false,
                })
                .find(|region| (region.start..region.end).contains(&range_end))
                .is_some();
            if in_region {
                max_allocated_addr = max_allocated_addr.max(range_end);
            }
        }

        let index = memory_regions
            .iter()
            .enumerate()
            .find(|(_, region)| (region.start..region.end).contains(&max_allocated_addr))
            .unwrap()
            .0;

        let mut allocated_frame_ranges = StackVec::new();
        for range in allocated_frame_ranges_bl.iter() {
            allocated_frame_ranges.push(*range);
        }

        Self {
            inner: Mutex::new(InitFrameAllocatorInner {
                memory_regions,
                index,
                addr: max_allocated_addr,
                allocated_frame_ranges,
            }),
        }
    }

    fn next_frame_addr(inner: &mut InitFrameAllocatorInner) -> Result<u64, FrameAllocError> {
        loop {
            let mem_region = inner.memory_regions[inner.index];
            let mem_left_in_region = mem_region.end - inner.addr;
            if mem_left_in_region >= Self::FRAME_SIZE {
                let ptr = inner.addr;
                Self::mark_frame_as_allocated(inner, ptr);
                inner.addr += Self::FRAME_SIZE;
                return Ok(ptr);
            }

            if inner.index >= inner.memory_regions.len() {
                return Err(FrameAllocError);
            }

            inner.index += 1;
            inner.addr = inner.memory_regions[inner.index].start;
        }
    }

    fn mark_frame_as_allocated(inner: &mut InitFrameAllocatorInner, base: u64) {
        let mut existing_index = None;
        for (i, entry) in inner.allocated_frame_ranges.iter().enumerate() {
            let entry_end = entry.base + entry.frames as u64 * Self::FRAME_SIZE;
            if entry_end == base && !entry.bootloader {
                existing_index = Some(i);
                break;
            }
        }

        if let Some(index) = existing_index {
            inner.allocated_frame_ranges[index].frames += 1;
        } else {
            inner.allocated_frame_ranges.push(AllocatedFrameRange {
                base,
                frames: 1,
                bootloader: false,
            });
        }
    }
}

impl<'a> FrameAllocator for InitFrameAllocator<'a> {
    fn allocate_frame(&self) -> Result<u64, FrameAllocError> {
        Self::next_frame_addr(&mut self.inner.lock())
    }

    fn deallocate_frame(&self, _frame: u64) -> Result<(), FrameAllocError> {
        unimplemented!()
    }
}
