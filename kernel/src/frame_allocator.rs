use crate::{
    init_frame_allocator::InitFrameAllocator, kernel_page_allocator::KernelPageAllocator,
    spinlock::Mutex,
};
use alloc::raw_vec::RawVec;
use bootloader_api::MemoryRegion;
use core::alloc::Layout;
use x86_64::paging::{FrameAllocError, FrameAllocator, PageTable, PhysAddr, Pml4, VirtAddr};

#[derive(Debug)]
pub struct KernelFrameAllocator {
    inner: Mutex<KernelFrameAllocatorInner>,
}

#[derive(Debug)]
struct KernelFrameAllocatorInner {
    regions: RawVec<(MemoryRegion, RawVec<u64>)>,
}

impl KernelFrameAllocator {
    pub fn new(
        memory_map: &[MemoryRegion],
        frame_allocator: InitFrameAllocator,
        page_allocator: &KernelPageAllocator,
        mut page_table: PageTable<Pml4>,
    ) -> Self {
        let memory_map = memory_map
            .iter()
            .filter(|region| matches!(region.ty, bootloader_api::MemoryRegionType::KernelUsable));
        let mut regions_layout =
            Layout::array::<(MemoryRegion, RawVec<u64>)>(memory_map.clone().count()).unwrap();
        for region in memory_map.clone() {
            let region_frames = (region.end - region.start) as usize / 4096;
            let bitmap_bytes = region_frames / 8;
            let bitmap_u64 = bitmap_bytes / 8 + 1;
            let (layout, _) = regions_layout
                .extend(Layout::array::<u64>(bitmap_u64).unwrap())
                .unwrap();
            regions_layout = layout;
        }

        let frames = (regions_layout.size() + 4095) / 4096;
        let pages = page_allocator.allocate_pages(frames);
        for frame in 0..frames {
            let frame_base = frame_allocator.allocate_frame().unwrap();
            page_table.map(
                VirtAddr::new(pages + 4096 * frame as u64),
                PhysAddr::new(frame_base),
                &frame_allocator,
            );
        }

        let mut regions_layout =
            Layout::array::<(MemoryRegion, RawVec<u64>)>(memory_map.clone().count()).unwrap();
        let mut regions = unsafe {
            RawVec::from_raw_parts(
                pages as *mut (MemoryRegion, RawVec<u64>),
                memory_map.clone().count(),
            )
        };
        for region in memory_map {
            let region_frames = (region.end - region.start) as usize / 4096;
            let bitmap_bytes = region_frames / 8;
            let bitmap_u64 = bitmap_bytes / 4 + 1;
            let (layout, offset) = regions_layout
                .extend(Layout::array::<u64>(bitmap_u64).unwrap())
                .unwrap();
            regions_layout = layout;
            let mut bitmap = unsafe {
                RawVec::from_raw_parts((pages as usize + offset) as *mut u64, bitmap_u64)
            };
            for _ in 0..bitmap_u64 {
                bitmap.push(0).unwrap();
            }

            regions.push((*region, bitmap)).unwrap();
        }

        let mut inner = KernelFrameAllocatorInner { regions };

        // Copy allocated frames from init allocator
        let allocated_frame_ranges = frame_allocator.allocated_frame_ranges();
        for range in allocated_frame_ranges.iter() {
            for frame in 0..range.frames {
                Self::mark_frame_as_allocated(&mut inner, range.base + frame as u64 * 4096);
            }
        }

        // Mark the ends the bitmaps as allocated
        for (region, bitmap) in inner.regions.iter_mut() {
            let region_frames = (region.end - region.start) as usize / 4096;
            let trailing_bits = bitmap.len() * 8 - region_frames;
            if trailing_bits > 0 {
                *bitmap.last_mut().unwrap() = !((1 << (63 - trailing_bits)) - 1);
            }
        }

        Self {
            inner: Mutex::new(inner),
        }
    }

    fn set_frame_state(inner: &mut KernelFrameAllocatorInner, frame_base: u64, allocated: bool) {
        if let Some((region, bitmap)) = inner
            .regions
            .iter_mut()
            .find(|(region, _)| (region.start..region.end).contains(&frame_base))
        {
            let bit_offset = (frame_base - region.start) as usize / 4096;
            let u64_index = bit_offset / 64;
            let bit_index = bit_offset % 64;
            if allocated {
                bitmap[u64_index] |= 1 << bit_index;
            } else {
                bitmap[u64_index] &= !(1 << bit_index);
            }
        }
    }

    fn mark_frame_as_allocated(inner: &mut KernelFrameAllocatorInner, frame_base: u64) {
        Self::set_frame_state(inner, frame_base, true)
    }

    fn mark_frame_as_deallocated(inner: &mut KernelFrameAllocatorInner, frame_base: u64) {
        Self::set_frame_state(inner, frame_base, false)
    }
}

impl FrameAllocator for KernelFrameAllocator {
    fn allocate_frame(&self) -> Result<u64, FrameAllocError> {
        let mut inner = self.inner.lock();

        // Linear search for empty page
        for (region, bitmap) in inner.regions.iter_mut() {
            for (map_index, entry) in bitmap
                .iter_mut()
                .enumerate()
                .filter(|(_, a)| **a != 0xffff_ffff_ffff_ffff)
            {
                for entry_index in 0..64 {
                    if (*entry >> entry_index) & 0x1 == 0 {
                        let frame_base =
                            region.start + (map_index * 64 + entry_index) as u64 * 4096;
                        Self::mark_frame_as_allocated(&mut inner, frame_base);
                        return Ok(frame_base);
                    }
                }
            }
        }

        Err(FrameAllocError)
    }

    fn deallocate_frame(&self, frame: u64) -> Result<(), FrameAllocError> {
        let mut inner = self.inner.lock();
        Self::mark_frame_as_deallocated(&mut inner, frame);
        Ok(())
    }
}
