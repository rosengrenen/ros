use core::{
    alloc::{AllocError, Allocator, Layout},
    ptr::NonNull,
};

use alloc::raw_vec::RawVec;
use x86_64::paging::{FrameAllocator, PageTable, PhysAddr, Pml4, VirtAddr};

use crate::{kernel_page_allocator::KernelPageAllocator, spinlock::Mutex};

pub struct Heap {
    inner: Mutex<HeapInner>,
}

pub struct HeapInner {
    free_spaces: RawVec<(u64, u64)>,
}

impl Heap {
    pub fn new(
        size: usize,
        frame_allocator: &impl FrameAllocator,
        page_allocator: &KernelPageAllocator,
        mut page_table: PageTable<Pml4>,
    ) -> Self {
        let mut free_spaces = {
            let frame = frame_allocator.allocate_frame().unwrap();
            let page = page_allocator.allocate_pages(1);
            page_table.map(VirtAddr::new(page), PhysAddr::new(frame), frame_allocator);
            unsafe {
                RawVec::from_raw_parts(
                    page as *mut (u64, u64),
                    4096 / core::mem::size_of::<(u64, u64)>(),
                )
            }
        };

        let frames = size / 4096 + 1;
        let page = page_allocator.allocate_pages(frames);
        for frame_index in 0..frames {
            let frame = frame_allocator.allocate_frame().unwrap();
            page_table.map(
                VirtAddr::new(page + frame_index as u64 * 4096),
                PhysAddr::new(frame),
                frame_allocator,
            );
        }
        free_spaces.push((page, page + size as u64)).unwrap();
        Self {
            inner: Mutex::new(HeapInner { free_spaces }),
        }
    }
}

unsafe impl Allocator for Heap {
    fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
        let mut inner = self.inner.lock();
        for (start, end) in inner.free_spaces.iter_mut() {
            let alignment = layout.align() as u64;
            let offset = (alignment - (*start % alignment)) % alignment;
            let alloc_start = *start + offset;
            let size = layout.size() as u64;
            let alloc_end = alloc_start + size;
            if *end - alloc_start >= size {
                *start = alloc_end;
                let slice =
                    unsafe { core::slice::from_raw_parts_mut(alloc_start as *mut u8, size as _) };
                return Ok(unsafe { NonNull::new_unchecked(slice) });
            }
        }

        Err(AllocError)
    }

    unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
        let mut extended_region = false;
        let mut inner = self.inner.lock();
        let ptr_start = ptr.as_ptr() as u64;
        let size = layout.size() as u64;
        let ptr_end = ptr_start + size;
        for (start, end) in inner.free_spaces.iter_mut() {
            if *end == ptr_start {
                *end += size;
                extended_region = true;
                break;
            }

            if *start == ptr_end {
                *start -= size;
                extended_region = true;
                break;
            }
        }

        if extended_region {
            let mut a = None;
            for (i1, (s1, e1)) in inner.free_spaces.iter().enumerate() {
                for (i2, (s2, e2)) in inner.free_spaces.iter().enumerate() {
                    if s1 == s2 && e1 == e2 {
                        continue;
                    }

                    if s1 == e2 || e1 == s2 {
                        a = Some((i1, i2));
                    }
                }
            }

            if let Some((i1, i2)) = a {
                let (s1, e1) = inner.free_spaces[i1];
                let (s2, e2) = inner.free_spaces[i2];
                inner.free_spaces[i1] = (s1.min(s2), e1.max(e2));
                inner.free_spaces[i2] = (s1.min(s2), s1.min(s2));
            }
        } else {
            if let Some((s, e)) = inner.free_spaces.iter_mut().find(|(e, s)| e == s) {
                *s = ptr_start;
                *e = ptr_end;
            } else {
                inner.free_spaces.push((ptr_start, ptr_end)).unwrap();
            }
        }
    }
}
