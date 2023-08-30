use core::{
    alloc::{AllocError, Allocator, Layout},
    ptr::NonNull,
};

use uefi::services::boot::MemoryDescriptor;

pub struct BumpAllocator {
    memory_map: [Option<MemoryDescriptor>; 128],
    memory_map_len: usize,
    inner: BumpAllocatorInner,
}

struct BumpAllocatorInner {
    descriptor_index: usize,
    addr: u64,
}

impl BumpAllocator {
    pub fn new<'iter>(memory_map_iter: impl Iterator<Item = &'iter MemoryDescriptor>) -> Self {
        let mut memory_map = [None; 128];
        let mut memory_map_len = 0;
        for (i, item) in memory_map_iter
            .filter(|desc| desc.ty.usable_by_loader())
            .filter(|desc| desc.physical_start > 0)
            .enumerate()
        {
            if i >= 128 {
                break;
            }

            memory_map[i] = Some(*item);
            memory_map_len += 1;
        }

        Self {
            memory_map,
            memory_map_len,
            inner: BumpAllocatorInner {
                descriptor_index: 0,
                addr: memory_map[0].unwrap().physical_start,
            },
        }
    }
}

impl BumpAllocator {
    pub fn allocate_pages(&self, num_pages: u64) -> Result<u64, AllocError> {
        let inner = unsafe {
            let inner = (&self.inner) as *const BumpAllocatorInner;
            let inner = inner as *mut BumpAllocatorInner;
            &mut *inner
        };

        loop {
            let mem_desc = &self.memory_map[inner.descriptor_index].unwrap();
            let mem_desc_size = mem_desc.number_of_pages * 4096;
            // align to 4096
            inner.addr = (inner.addr & !0xfff) + 4096;
            let mem_left_in_desc = mem_desc.physical_start + mem_desc_size - inner.addr;

            if mem_left_in_desc >= 4096 * num_pages {
                let ptr = inner.addr;
                inner.addr += 4096 * num_pages;
                return Ok(ptr);
            }

            if inner.descriptor_index >= self.memory_map_len {
                return Err(AllocError);
            }

            inner.descriptor_index += 1;
            inner.addr = self.memory_map[inner.descriptor_index]
                .unwrap()
                .physical_start;
        }
    }
}

unsafe impl Allocator for BumpAllocator {
    fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
        let inner = unsafe {
            let inner = (&self.inner) as *const BumpAllocatorInner;
            let inner = inner as *mut BumpAllocatorInner;
            &mut *inner
        };

        loop {
            let mem_desc = &self.memory_map[inner.descriptor_index].unwrap();
            let mem_desc_size = mem_desc.number_of_pages * 4096;
            let mem_left_in_desc = mem_desc.physical_start + mem_desc_size - inner.addr;

            let size = layout.size() as u64;
            let align = layout.align() as u64;
            let align_offset = if align % inner.addr == 0 {
                0
            } else {
                align - align % inner.addr
            };

            if mem_left_in_desc >= size + align_offset {
                let ptr = (inner.addr + align_offset) as *mut u8;
                inner.addr += align_offset + size;
                let slice = unsafe { core::slice::from_raw_parts_mut(ptr, size as _) };
                return Ok(unsafe { NonNull::new_unchecked(slice) });
            }

            if inner.descriptor_index >= self.memory_map_len {
                return Err(AllocError);
            }

            inner.descriptor_index += 1;
            inner.addr = self.memory_map[inner.descriptor_index]
                .unwrap()
                .physical_start;
        }
    }

    unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {
        // Deallocating is a noop
    }
}
