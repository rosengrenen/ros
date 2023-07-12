#![no_std]
#![no_main]

use core::panic::PanicInfo;

use uefi::services::graphics::BltPixel;

#[no_mangle]
pub extern "C" fn _start(buffer: *mut BltPixel, width: usize, height: usize) -> ! {
    let buffer = unsafe { core::slice::from_raw_parts_mut(buffer, width * height) };
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
