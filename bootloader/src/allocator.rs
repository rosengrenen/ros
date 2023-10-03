use core::{alloc::AllocError, cell::RefCell};
use uefi::services::boot::MemoryDescriptor;
use x86_64::paging::{FrameAllocError, FrameAllocator};

pub struct BumpAllocator {
    memory_map: [Option<MemoryDescriptor>; 128],
    memory_map_len: usize,
    inner: RefCell<BumpAllocatorInner>,
}

struct BumpAllocatorInner {
    descriptor_index: usize,
    addr: usize,
}

impl BumpAllocator {
    pub fn new<'iter>(memory_map_iter: impl Iterator<Item = &'iter MemoryDescriptor>) -> Self {
        let mut memory_map = [None; 128];
        let mut memory_map_len = 0;
        for (i, item) in memory_map_iter
            .filter(|desc| desc.ty.usable_by_loader())
            .filter(|desc| desc.physical_start > 0)
            .take(128)
            .enumerate()
        {
            memory_map[i] = Some(*item);
            memory_map_len += 1;
        }

        Self {
            memory_map,
            memory_map_len,
            inner: RefCell::new(BumpAllocatorInner {
                descriptor_index: 0,
                addr: memory_map[0].unwrap().physical_start as usize,
            }),
        }
    }
}

impl BumpAllocator {
    pub fn allocate_frames(&self, num_pages: usize) -> Result<usize, AllocError> {
        let mut inner = self.inner.borrow_mut();
        loop {
            let mem_desc = &self.memory_map[inner.descriptor_index].unwrap();
            let mem_desc_size = mem_desc.number_of_pages * 4096;
            // align to 4096
            if inner.addr & 0xfff != 0 {
                inner.addr = (inner.addr & !0xfff) + 4096;
            }

            let mem_left_in_desc =
                mem_desc.physical_start as usize + mem_desc_size as usize - inner.addr;
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
                .physical_start as usize;
        }
    }
}

impl FrameAllocator for BumpAllocator {
    fn allocate_frame(&self) -> Result<usize, FrameAllocError> {
        self.allocate_frames(1).map_err(|_| FrameAllocError)
    }

    fn deallocate_frame(&self, _frame: usize) -> Result<(), FrameAllocError> {
        Ok(())
    }
}
