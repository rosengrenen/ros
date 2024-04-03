use core::alloc::Allocator;
use core::alloc::Layout;
use core::ptr::NonNull;

use common::frame::FrameAllocator;
use x86_64::paging::PageTableFrameMapper;

use crate::spinlock::Mutex;
use crate::FRAME_OFFSET_MAPPER;

// This is a simple slab allocator, and only works on a single cpu for now.
// Basically just a number of freestanding frames that have fixed size slots of a particular size.
// Empty slabs are returned to frame allocator.
// Partially full slabs are allocated from
// Full slabs are ignored, and become partailly full when freed from.
#[derive(Debug)]
pub struct SlabCache<F: FrameAllocator> {
    pub inner: Mutex<SlabCacheInner>,
    pub frame_allocator: F,
    pub object_layout: Layout,
}

impl<F: FrameAllocator> SlabCache<F> {
    pub fn new(frame_allocator: F, object_layout: Layout) -> Self {
        Self {
            inner: Mutex::new(SlabCacheInner::new()),
            frame_allocator,
            object_layout,
        }
    }
}

unsafe impl<F: FrameAllocator> Allocator for SlabCache<F> {
    fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, core::alloc::AllocError> {
        let mut inner = self.inner.lock();
        let mut active = if let Some(active) = inner.active {
            active
        } else {
            // TODO: use page allocator not a frame allocator
            let frame = self.frame_allocator.allocate_frame().unwrap();
            let slab_layout = Layout::new::<Slab>();
            let (_, offset) = slab_layout.extend(layout).unwrap();
            // TODO: make generic over page allocator instead of frame allocator
            let mut page = FRAME_OFFSET_MAPPER.frame_to_page(frame);
            let mut slab_ptr = NonNull::new(page.as_ptr_mut::<Slab>()).unwrap();
            unsafe {
                *slab_ptr.as_mut() = Slab::new(
                    page.as_u64() + offset as u64,
                    4096 - offset as u64,
                    self.object_layout,
                );
            }
            inner.active = Some(slab_ptr);
            slab_ptr
        };
        let active = unsafe { active.as_mut() };
        let ptr = active.allocate(layout);
        if active.freelist.is_none() {
            inner.active = None;
        }

        Ok(ptr)
    }

    unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
        todo!()
    }
}

#[derive(Debug)]
pub struct SlabCacheInner {
    active: Option<NonNull<Slab>>,
    partial: Option<NonNull<Slab>>,
    full: Option<NonNull<Slab>>,
}

impl SlabCacheInner {
    pub fn new() -> Self {
        Self {
            active: None,
            partial: None,
            full: None,
        }
    }
}

#[derive(Debug)]
#[repr(C)]
struct Slab {
    freelist: Option<NonNull<Freelist>>,
    next: Option<NonNull<Slab>>,
}

impl Slab {
    pub fn new(base: u64, size: u64, object_layout: Layout) -> Self {
        let u64_layout = Layout::new::<u64>();
        let object_size = u64_layout.size().max(object_layout.size());
        let mut freelist = None;
        for i in 0..size / object_size as u64 {
            let cur =
                unsafe { NonNull::new_unchecked((base + i * object_size as u64) as *mut Freelist) };
            unsafe {
                cur.write(Freelist { next: freelist });
            };
            freelist = Some(cur);
        }

        Self {
            freelist,
            next: None,
        }
    }

    fn allocate(&mut self, layout: Layout) -> NonNull<[u8]> {
        if let Some(freelist) = self.freelist {
            self.freelist = unsafe { freelist.as_ref() }.next;
            unsafe {
                NonNull::new_unchecked(core::ptr::slice_from_raw_parts_mut(
                    freelist.cast().as_mut(),
                    layout.size(),
                ))
            }
        } else {
            panic!()
        }
    }
}

#[derive(Debug)]
#[repr(C)]
struct Freelist {
    next: Option<NonNull<Freelist>>,
}
