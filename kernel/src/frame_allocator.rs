use crate::{
    bitmap::Bitmap, init_frame_allocator::InitFrameAllocator,
    kernel_page_allocator::KernelPageAllocator, spinlock::Mutex,
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
    regions: RawVec<(MemoryRegion, Bitmap)>,
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
            Layout::array::<(MemoryRegion, Bitmap)>(memory_map.clone().count()).unwrap();
        for region in memory_map.clone() {
            let region_frames = (region.end - region.start) as usize / 4096;
            let bitmap_u64 = (region_frames + 63) / 64;
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
            Layout::array::<(MemoryRegion, Bitmap)>(memory_map.clone().count()).unwrap();
        let mut regions = unsafe {
            RawVec::from_raw_parts(
                pages as *mut (MemoryRegion, Bitmap),
                memory_map.clone().count(),
            )
        };
        for region in memory_map {
            let region_frames = (region.end - region.start) as usize / 4096;
            let bitmap_u64 = (region_frames + 63) / 64;
            let (layout, offset) = regions_layout
                .extend(Layout::array::<u64>(bitmap_u64).unwrap())
                .unwrap();
            regions_layout = layout;
            let bitmap = unsafe {
                Bitmap::from_raw_parts((pages as usize + offset) as *mut u64, region_frames)
            };
            regions.push((*region, bitmap)).unwrap();
        }

        let mut inner = KernelFrameAllocatorInner { regions };

        // Copy allocated frames from init allocator
        let allocated_frame_ranges = frame_allocator.allocated_frame_ranges();
        for range in allocated_frame_ranges.iter() {
            // We assume that a range is contained within a region
            let (region, bitmap) = inner.get_region_mut(range.base).unwrap();
            let base_index = (range.base - region.start) as usize / 4096;
            for frame_index in 0..range.frames {
                bitmap.set_bit(base_index + frame_index);
            }
        }

        let me = Self {
            inner: Mutex::new(inner),
        };

        me.deallocate_frame(allocated_frame_ranges.as_ptr() as u64)
            .unwrap();

        me
    }

    pub fn allocated_frames(&self) -> usize {
        let inner = self.inner.lock();
        inner
            .regions
            .iter()
            .map(|(_, bitmap)| bitmap.iter().filter(|bit| *bit).count())
            .sum()
    }
}

impl KernelFrameAllocatorInner {
    fn get_region_mut(&mut self, frame_base: u64) -> Option<&mut (MemoryRegion, Bitmap)> {
        self.regions
            .iter_mut()
            .find(|(region, _)| (region.start..region.end).contains(&frame_base))
    }
}

impl FrameAllocator for KernelFrameAllocator {
    fn allocate_frame(&self) -> Result<u64, FrameAllocError> {
        let mut inner = self.inner.lock();

        // Linear search for empty page
        for (region, bitmap) in inner.regions.iter_mut() {
            if let Some(index) = bitmap.find_free_index() {
                let frame_base = region.start + index as u64 * 4096;
                bitmap.set_bit(index);
                return Ok(frame_base);
            }
        }

        Err(FrameAllocError)
    }

    fn deallocate_frame(&self, frame: u64) -> Result<(), FrameAllocError> {
        let mut inner = self.inner.lock();
        let (region, bitmap) = inner.get_region_mut(frame).unwrap();
        let index = (frame - region.start) as usize / 4096;
        if !bitmap.get_bit(index) {
            panic!("Frame is not allocated");
        }

        bitmap.clear_bit(index);
        Ok(())
    }
}
