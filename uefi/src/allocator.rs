use crate::services::boot::{BootServices, MemoryType};

static mut BOOT_SERVICES: Option<&'static BootServices> = None;

pub fn enable(boot_services: &'static BootServices) {
    unsafe {
        BOOT_SERVICES = Some(boot_services);
    }
}

// TODO: maybe rename it to exit boot services, since that is when it's called
pub fn disable() {
    unsafe {
        BOOT_SERVICES = None;
    }
}

pub fn allocator_enabled() -> bool {
    unsafe { BOOT_SERVICES.is_some() }
}

struct EfiAllocator;

unsafe impl core::alloc::GlobalAlloc for EfiAllocator {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        let boot_services = match BOOT_SERVICES {
            Some(boot_services) => boot_services,
            None => return core::ptr::null_mut(),
        };

        match boot_services.allocate_pool(MemoryType::EfiLoaderData, layout.size()) {
            Ok(memory) => memory as _,
            Err(_) => core::ptr::null_mut(),
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: core::alloc::Layout) {
        let boot_services = match BOOT_SERVICES {
            Some(boot_services) => boot_services,
            None => return,
        };
        let _ = boot_services.free_pool(ptr as _);
    }
}

#[global_allocator]
static ALLOCATOR: EfiAllocator = EfiAllocator;
