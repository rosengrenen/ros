use crate::{slub::SlabCache, spinlock::Mutex, sprintln};
use common::frame::FrameAllocator;
use core::alloc::{Allocator, Layout};

pub struct KernelAllocator<'f, F: FrameAllocator> {
    slab_32: Mutex<SlabCache<&'f F>>,
    slab_64: Mutex<SlabCache<&'f F>>,
    slab_128: Mutex<SlabCache<&'f F>>,
    slab_256: Mutex<SlabCache<&'f F>>,
    slab_512: Mutex<SlabCache<&'f F>>,
    slab_1k: Mutex<SlabCache<&'f F>>,
    slab_2k: Mutex<SlabCache<&'f F>>,
}

impl<'f, F: FrameAllocator> KernelAllocator<'f, F> {
    pub fn new(frame_allocator: &'f F) -> Self {
        Self {
            slab_32: Mutex::new(SlabCache::new(
                frame_allocator,
                Layout::from_size_align(32, 32).unwrap(),
            )),
            slab_64: Mutex::new(SlabCache::new(
                frame_allocator,
                Layout::from_size_align(64, 64).unwrap(),
            )),
            slab_128: Mutex::new(SlabCache::new(
                frame_allocator,
                Layout::from_size_align(128, 128).unwrap(),
            )),
            slab_256: Mutex::new(SlabCache::new(
                frame_allocator,
                Layout::from_size_align(256, 256).unwrap(),
            )),
            slab_512: Mutex::new(SlabCache::new(
                frame_allocator,
                Layout::from_size_align(512, 512).unwrap(),
            )),
            slab_1k: Mutex::new(SlabCache::new(
                frame_allocator,
                Layout::from_size_align(1024, 1024).unwrap(),
            )),
            slab_2k: Mutex::new(SlabCache::new(
                frame_allocator,
                Layout::from_size_align(2048, 2048).unwrap(),
            )),
        }
    }
}

unsafe impl<'f, F: FrameAllocator> Allocator for KernelAllocator<'f, F> {
    fn allocate(
        &self,
        layout: Layout,
    ) -> Result<core::ptr::NonNull<[u8]>, core::alloc::AllocError> {
        let size = layout.size();
        sprintln!("allocation size: {}, align: {}", size, layout.align());
        let res = if size <= 32 {
            sprintln!("kalloc 32");
            let res = self.slab_32.lock().allocate(layout);
            sprintln!("kalloc 32 done");
            res
        } else if size <= 64 {
            sprintln!("kalloc 64");
            let res = self.slab_64.lock().allocate(layout);
            sprintln!("kalloc 64 done");
            res
        } else if size <= 128 {
            sprintln!("kalloc 128");
            let res = self.slab_128.lock().allocate(layout);
            sprintln!("kalloc 128 done");
            res
        } else if size <= 256 {
            sprintln!("kalloc 256");
            let res = self.slab_256.lock().allocate(layout);
            sprintln!("kalloc 256 done");
            res
        } else if size <= 512 {
            sprintln!("kalloc 512");
            let res = self.slab_512.lock().allocate(layout);
            sprintln!("kalloc 512 done");
            res
        } else if size <= 1024 {
            sprintln!("kalloc 1024");
            let res = self.slab_1k.lock().allocate(layout);
            sprintln!("kalloc 1024 done");
            res
        } else if size <= 2048 {
            sprintln!("kalloc 2048");
            let res = self.slab_2k.lock().allocate(layout);
            sprintln!("kalloc 2048 done");
            res
        } else {
            panic!("kernel allocator does not yet support 2k> allocations");
        };

        if let Ok(res) = &res {
            if res.as_ptr() as *mut u8 as usize % layout.align() != 0 {
                panic!();
            }
        }

        res
    }

    unsafe fn deallocate(&self, ptr: core::ptr::NonNull<u8>, layout: Layout) {
        // TODO: free memory
    }
}
