use crate::spinlock::Mutex;
use alloc::raw_vec::RawVec;
use bootloader_api::{AllocatedFrameRange, MemoryRegion};
use x86_64::paging::{FrameAllocError, FrameAllocator};

#[derive(Debug)]
pub struct InitFrameAllocator<'a> {
    inner: Mutex<InitFrameAllocatorInner<'a>>,
}

#[derive(Debug)]
struct InitFrameAllocatorInner<'a> {
    inner: InitFrameAllocatorInnerInner<'a>,
    allocated_frame_ranges: RawVec<AllocatedFrameRange>,
}

#[derive(Debug)]
struct InitFrameAllocatorInnerInner<'a> {
    memory_regions: &'a [MemoryRegion],
    addr: u64,
    index: usize,
}

impl<'a> InitFrameAllocator<'a> {
    const FRAME_SIZE: u64 = 4096;

    pub fn new(
        memory_regions: &'a [MemoryRegion],
        // Allocated frame ranges from bootloader
        allocated_frame_ranges_bl: &[AllocatedFrameRange],
    ) -> Self {
        let max_allocated_addr = allocated_frame_ranges_bl
            .iter()
            .map(|range| range.base + range.frames as u64 * 4096)
            .max()
            .unwrap();

        let index = memory_regions
            .iter()
            .enumerate()
            .find(|(_, region)| (region.start..=region.end).contains(&max_allocated_addr))
            .unwrap()
            .0;

        let mut inner = InitFrameAllocatorInnerInner {
            memory_regions,
            index,
            addr: max_allocated_addr,
        };
        let frame = Self::next_frame_addr(&mut inner).unwrap();

        let mut allocated_frame_ranges = unsafe {
            RawVec::from_raw_parts(
                frame as *mut AllocatedFrameRange,
                Self::FRAME_SIZE as usize / core::mem::size_of::<AllocatedFrameRange>(),
            )
        };
        for range in allocated_frame_ranges_bl.iter() {
            allocated_frame_ranges.push(*range).unwrap();
        }

        let mut inner = InitFrameAllocatorInner {
            inner,
            allocated_frame_ranges,
        };
        Self::mark_frame_as_allocated(&mut inner, frame);

        Self {
            inner: Mutex::new(inner),
        }
    }

    fn next_frame_addr(inner: &mut InitFrameAllocatorInnerInner) -> Result<u64, FrameAllocError> {
        loop {
            let mem_region = inner.memory_regions[inner.index];
            let mem_left_in_region = mem_region.end - inner.addr;
            if mem_left_in_region >= Self::FRAME_SIZE {
                let ptr = inner.addr;
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
            if entry_end == base {
                existing_index = Some(i);
                break;
            }
        }

        if let Some(index) = existing_index {
            inner.allocated_frame_ranges[index].frames += 1;
        } else {
            inner
                .allocated_frame_ranges
                .push(AllocatedFrameRange { base, frames: 1 })
                .unwrap();
        }
    }

    pub fn allocated_frame_ranges(self) -> RawVec<AllocatedFrameRange> {
        self.inner.into_inner().allocated_frame_ranges
    }
}

impl<'a> FrameAllocator for InitFrameAllocator<'a> {
    fn allocate_frame(&self) -> Result<u64, FrameAllocError> {
        let mut inner = self.inner.lock();
        let base = Self::next_frame_addr(&mut inner.inner)?;
        Self::mark_frame_as_allocated(&mut inner, base);
        Ok(base)
    }

    fn deallocate_frame(&self, _frame: u64) -> Result<(), FrameAllocError> {
        unimplemented!()
    }
}
