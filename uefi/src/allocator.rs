use core::{
    alloc::{AllocError, Allocator, Layout},
    ptr::NonNull,
};

use crate::services::boot::{BootServices, MemoryType};

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

unsafe impl<'bs> core::alloc::GlobalAlloc for UefiAllocator<'bs> {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        match self
            .boot_services
            .allocate_pool(MemoryType::EfiLoaderData, layout.size())
        {
            Ok(memory) => memory as _,
            Err(_) => core::ptr::null_mut(),
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: core::alloc::Layout) {
        let _ = self.boot_services.free_pool(ptr as _);
    }
}
