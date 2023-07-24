#![no_std]
#![no_main]

extern crate alloc;

use core::panic::PanicInfo;

use bootloader_api::BootInfo;
use uefi::services::graphics::BltPixel;

#[no_mangle]
pub extern "C" fn _start(info: &'static BootInfo) -> ! {
    let framebuffer = &info.framebuffer;
    let memory_regions =
        unsafe { core::slice::from_raw_parts(info.memory_regions.ptr, info.memory_regions.len) };
    // Set things up
    // * Set up physical frame manager
    // * Set up paging
    // * Set up interrupt handlers

    // Load init system
    let buffer = unsafe {
        core::slice::from_raw_parts_mut(
            framebuffer.base as *mut BltPixel,
            framebuffer.width * framebuffer.height,
        )
    };
    let mut red = 255;
    let mut green = 0;
    let mut blue = 0;
    loop {
        if red == 255 {
            if blue > 0 {
                blue -= 15;
            } else if green == 255 {
                red -= 15;
            } else {
                green += 15;
            }
        } else if green == 255 {
            if red > 0 {
                red -= 15;
            } else if blue == 255 {
                green -= 15;
            } else {
                blue += 15;
            }
        } else if blue == 255 {
            if green > 0 {
                green -= 15;
            } else if red == 255 {
                blue -= 15;
            } else {
                red += 15;
            }
        }

        for y in 0..framebuffer.height {
            for x in 0..framebuffer.width {
                buffer[y * framebuffer.width + x] = BltPixel {
                    blue,
                    green,
                    red,
                    reserved: 255,
                }
            }
        }
    }
}

struct FrameAllocator {
    //
}

impl FrameAllocator {}

// TODO: bitfield
struct Frame(pub u8);

impl Frame {
    fn allocated(&self) -> bool {
        self.0 & 0x1 != 0
    }

    fn reserved(&self) -> bool {
        self.0 & 0x2 != 0
    }
}

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // println!("{}", info);
    loop {}
}

// We can't rely on a global allocator in the kernel, but one must be
// provided since we use the alloc crate
struct DummyAllocator;

unsafe impl alloc::alloc::GlobalAlloc for DummyAllocator {
    unsafe fn alloc(&self, _layout: core::alloc::Layout) -> *mut u8 {
        unimplemented!()
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: core::alloc::Layout) {
        unimplemented!()
    }
}

#[global_allocator]
static GLOBAL_ALLOCATOR: DummyAllocator = DummyAllocator;
