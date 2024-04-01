use core::{
    alloc::{Allocator, Layout},
    ptr::NonNull,
};

use common::frame::FrameAllocator;
use x86_64::paging::PageTableFrameMapper;

use crate::{spinlock::Mutex, FRAME_OFFSET_MAPPER};

pub struct KernelAllocator<F> {
    inner: Mutex<KernelAllocatorInner<F>>,
}

pub struct KernelAllocatorInner<F> {
    current_frame: Option<u64>,
    offset: u64,
    frame_allocator: F,
}

impl<F> KernelAllocator<F> {
    pub fn new(frame_allocator: F) -> Self {
        Self {
            inner: Mutex::new(KernelAllocatorInner {
                current_frame: None,
                offset: 0,
                frame_allocator,
            }),
        }
    }
}

unsafe impl<F: FrameAllocator> Allocator for KernelAllocator<F> {
    fn allocate(
        &self,
        layout: Layout,
    ) -> Result<core::ptr::NonNull<[u8]>, core::alloc::AllocError> {
        let mut inner = self.inner.lock();
        let current_frame = match inner.current_frame {
            Some(current_frame) => current_frame,
            None => {
                let frame = inner
                    .frame_allocator
                    .allocate_frame()
                    .map_err(|_| core::alloc::AllocError)?;
                let frame = FRAME_OFFSET_MAPPER.frame_to_page(frame).as_u64();
                inner.current_frame = Some(frame);
                inner.offset = 0;
                frame
            }
        };

        let size = layout.size() as u64;
        if size > 4096 {
            panic!("Cannot allocate more than 4096 bytes");
        }

        if inner.offset + size as u64 > 4096 {
            inner.current_frame = None;
            crate::sprintln!("recurse");
            drop(inner);
            return self.allocate(layout);
        }

        let align = layout.align() as u64;
        if inner.offset % align != 0 {
            inner.offset += align - inner.offset % align;
        }

        crate::sprintln!("{} {} {}", inner.offset, size, align);
        assert_eq!(inner.offset % align, 0);

        let addr = current_frame + inner.offset;
        inner.offset += size as u64;

        Ok(NonNull::new(core::ptr::slice_from_raw_parts_mut(
            addr as *mut u8,
            size as usize,
        ))
        .unwrap())
    }

    unsafe fn deallocate(&self, ptr: core::ptr::NonNull<u8>, layout: Layout) {
        // noop
    }
}
