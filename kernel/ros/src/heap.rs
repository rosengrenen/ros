use core::{
    alloc::{AllocError, Allocator, Layout},
    ptr::NonNull,
};

use alloc::raw_vec::RawVec;
use x86_64::paging::{FrameAllocator, PageTable, PhysAddr, Pml4, VirtAddr};

use crate::{kernel_page_allocator::KernelPageAllocator, spinlock::Mutex, sprintln};

#[derive(Debug)]
pub struct Heap {
    pub inner: Mutex<HeapInner>,
}

#[derive(Debug)]
pub struct HeapInner {
    pub free_spaces: RawVec<(u64, u64)>,
    pub allocated_bytes: usize,
    pub max_allocated_bytes: usize,
    pub num_allocations: usize,
    pub num_deallocations: usize,
}

impl Heap {
    pub fn new(
        size: usize,
        frame_allocator: &impl FrameAllocator,
        page_allocator: &KernelPageAllocator,
        mut page_table: PageTable<Pml4>,
    ) -> Self {
        let mut free_spaces = {
            let num_frames = 16;
            let page = page_allocator.allocate_pages(num_frames);
            for frame in 0..num_frames {
                let frame_base = frame_allocator.allocate_frame().unwrap();
                page_table.map(
                    VirtAddr::new(page + frame as u64 * 4096),
                    PhysAddr::new(frame_base),
                    frame_allocator,
                );
            }
            unsafe {
                RawVec::from_raw_parts(
                    page as *mut (u64, u64),
                    num_frames * 4096 / core::mem::size_of::<(u64, u64)>(),
                )
            }
        };

        let frames = (size + 4095) / 4096;
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
            inner: Mutex::new(HeapInner {
                free_spaces,
                allocated_bytes: 0,
                max_allocated_bytes: 0,
                num_allocations: 0,
                num_deallocations: 0,
            }),
        }
    }
}

unsafe impl Allocator for Heap {
    fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
        let mut inner = self.inner.lock();
        let mut new_free_space = None;
        let mut ptr = None;
        for (start, end) in inner.free_spaces.iter_mut() {
            let alignment = layout.align() as u64;
            let offset = (alignment - (*start % alignment)) % alignment;
            let alloc_start = *start + offset;
            let size = layout.size() as u64;
            let alloc_end = alloc_start + size;
            if *end >= alloc_start + size {
                *start = alloc_end;
                if alloc_start > *start {
                    new_free_space = Some((*start, alloc_start));
                }

                let slice =
                    unsafe { core::slice::from_raw_parts_mut(alloc_start as *mut u8, size as _) };
                ptr = Some(unsafe { NonNull::new_unchecked(slice) });
                break;
            }
        }

        if let Some(entry) = new_free_space {
            sprintln!("pushgin {:?}", entry);
            inner.free_spaces.push(entry).unwrap();
        }

        match ptr {
            Some(ptr) => {
                inner.allocated_bytes += layout.size();
                inner.max_allocated_bytes = inner.max_allocated_bytes.max(inner.allocated_bytes);
                inner.num_allocations += 1;
                Ok(ptr)
            }
            None => Err(AllocError),
        }
    }

    unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
        let mut inner = self.inner.lock();
        inner.allocated_bytes -= layout.size();
        inner.num_deallocations += 1;
        let ptr_start = ptr.as_ptr() as u64;
        let size = layout.size() as u64;
        let ptr_end = ptr_start + size;
        if inner
            .free_spaces
            .iter()
            .find(|(start, end)| ptr_start >= *start && ptr_end <= *end)
            .is_some()
        {
            sprintln!(
                "WARNING {:x?} {:?} has already been freed",
                ptr.as_ptr() as u64,
                layout
            );
        }
        match inner
            .free_spaces
            .iter_mut()
            .find(|(start, end)| *start == ptr_end || *end == ptr_start)
        {
            Some((start, end)) => {
                *start = (*start).min(ptr_start);
                *end = (*end).max(ptr_end);
            }
            None => inner.free_spaces.push((ptr_start, ptr_end)).unwrap(),
        }

        if inner.free_spaces.len() > inner.free_spaces.cap() / 2 {
            inner.defragment();
        }
    }
}

impl HeapInner {
    pub fn defragment(&mut self) {
        let mut len = self.free_spaces.len();
        let mut outer_index = 0;
        while outer_index < len {
            // Remove empty
            let (outer_start, outer_end) = self.free_spaces[outer_index];
            if outer_start == outer_end {
                self.free_spaces[outer_index] = self.free_spaces[len - 1];
                self.free_spaces.pop();
                len -= 1;
                continue;
            }

            // Merge
            let mut merged = false;
            for inner_index in 0..len {
                if inner_index == outer_index {
                    continue;
                }

                let (inner_start, inner_end) = self.free_spaces[inner_index];
                if outer_start == inner_end || outer_end == inner_start {
                    self.free_spaces[inner_index] =
                        (outer_start.min(inner_start), outer_end.max(inner_end));
                    self.free_spaces[outer_index] = (outer_start, outer_start);
                    merged = true;
                    break;
                }
            }

            if !merged {
                outer_index += 1;
            }
        }
    }
}
