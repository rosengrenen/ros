#![no_std]
#![no_main]

use core::panic::PanicInfo;

use uefi::services::graphics::BltPixel;

#[no_mangle]
pub extern "sysv64" fn _start(buffer: *mut BltPixel, width: usize, height: usize) -> ! {
    let buffer = unsafe { core::slice::from_raw_parts_mut(buffer, width * height) };
    let mut red = 255;
    let mut green = 0;
    let mut blue = 0;
    loop {
        if red == 255 {
            if blue > 0 {
                blue -= 5;
            } else if green == 255 {
                red -= 5;
            } else {
                green += 5;
            }
        } else if green == 255 {
            if red > 0 {
                red -= 5;
            } else if blue == 255 {
                green -= 5;
            } else {
                blue += 5;
            }
        } else if blue == 255 {
            if green > 0 {
                green -= 5;
            } else if red == 255 {
                blue -= 5;
            } else {
                red += 5;
            }
        }

        for x in 0..width {
            for y in 0..height {
                buffer[y * width + x] = BltPixel {
                    blue,
                    green,
                    red,
                    reserved: 255,
                }
            }
        }
    }
}

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // println!("{}", info);
    loop {}
}
