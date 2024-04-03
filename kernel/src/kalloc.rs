use core::alloc::Allocator;
use core::alloc::Layout;

use common::frame::FrameAllocator;

use crate::slub::SlabCache;
use crate::spinlock::Mutex;

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
        if size <= 32 {
            self.slab_32.lock().allocate(layout)
        } else if size <= 64 {
            self.slab_64.lock().allocate(layout)
        } else if size <= 128 {
            self.slab_128.lock().allocate(layout)
        } else if size <= 256 {
            self.slab_256.lock().allocate(layout)
        } else if size <= 512 {
            self.slab_512.lock().allocate(layout)
        } else if size <= 1024 {
            self.slab_1k.lock().allocate(layout)
        } else if size <= 2048 {
            self.slab_2k.lock().allocate(layout)
        } else {
            panic!("kernel allocator does not yet support 2k> allocations");
        }
    }

    unsafe fn deallocate(&self, ptr: core::ptr::NonNull<u8>, layout: Layout) {
        // TODO: free memory
    }
}
