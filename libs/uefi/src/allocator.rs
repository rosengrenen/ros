use crate::services::boot::{BootServices, MemoryType};
use core::{
    alloc::{AllocError, Allocator, Layout},
    ptr::NonNull,
};

#[derive(Clone, Copy)]
pub struct UefiAllocator<'bs> {
    boot_services: &'bs BootServices,
}

impl<'bs> UefiAllocator<'bs> {
    pub fn new(boot_services: &'bs BootServices) -> Self {
        Self { boot_services }
    }
}

unsafe impl<'bs> Allocator for UefiAllocator<'bs> {
    fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
        match self
            .boot_services
            .allocate_pool(MemoryType::EfiLoaderData, layout.size())
        {
            Ok(memory) => unsafe {
                let ptr = NonNull::new_unchecked(memory as *mut u8);
                Ok(NonNull::slice_from_raw_parts(ptr, layout.size()))
            },
            Err(1) => Err(AllocError),
            Err(_) => panic!(),
        }
    }

    unsafe fn deallocate(&self, ptr: NonNull<u8>, _layout: Layout) {
        self.boot_services
            .free_pool(ptr.as_ptr() as *const _)
            .unwrap();
    }
}
