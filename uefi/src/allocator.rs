use crate::services::boot::{BootServices, MemoryType};

#[derive(Clone, Copy)]
pub struct UefiAllocator<'a> {
    boot_services: &'a BootServices,
}

impl<'a> UefiAllocator<'a> {
    pub fn new(boot_services: &'a BootServices) -> Self {
        Self { boot_services }
    }
}

unsafe impl<'a> core::alloc::GlobalAlloc for UefiAllocator<'a> {
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
