#![no_std]
#![no_main]
#![feature(allocator_api)]

use bootloader_api::BootInfo;
use core::{
    fmt::{Arguments, Write},
    panic::PanicInfo,
};
use serial::{SerialPort, COM1_BASE};
use uefi::services::graphics::BltPixel;

struct Dummy;

impl core::fmt::Write for Dummy {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        Ok(())
    }
}

#[no_mangle]
pub extern "C" fn _start(info: &'static BootInfo) -> ! {
    let framebuffer = &info.framebuffer;
    let _memory_regions =
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

    let mut serial = SerialPort::new(COM1_BASE);
    serial.configure(1);

    for y in 0..framebuffer.height {
        for x in 0..framebuffer.width {
            buffer[y * framebuffer.width + x] = BltPixel {
                blue: 0,
                green: 0,
                red: 128,
                reserved: 255,
            }
        }
    }

    serial.serial_write(b"serial_write works\n");
    serial.write_str("write_str works\n");
    let mut d = Dummy;
    // the line below crashes the whole thing, the macro just invokes .write_fmt so they are equivalent
    serial.write_fmt(format_args!("{}, format_args! works?\n", 2));
    write!(d, "write! works");

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

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // println!("{}", info);
    loop {}
}
