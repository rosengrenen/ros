#![no_std]
#![no_main]

use core::panic::PanicInfo;

use uefi::services::graphics::BltPixel;

#[no_mangle]
pub extern "sysv64" fn _start(buffer: *mut BltPixel, width: usize, height: usize) -> usize {
    let buffer = unsafe { core::slice::from_raw_parts_mut(buffer, width * height) };
    for x in 200..800 {
        for y in 400..600 {
            buffer[y * width + x] = BltPixel {
                blue: 0,
                green: 255,
                red: 255,
                reserved: 255,
            }
        }
    }
    // loop {}
    return 42;
}

/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    // println!("{}", info);
    loop {}
}
