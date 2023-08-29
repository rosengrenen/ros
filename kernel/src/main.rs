#![no_std]
#![no_main]
#![feature(allocator_api)]
#![feature(abi_x86_interrupt)]

use bootloader_api::BootInfo;
use core::{
    fmt::{Arguments, Write},
    panic::PanicInfo,
};
use serial::{SerialPort, COM1_BASE};
use uefi::services::graphics::BltPixel;
use x86_64::idt::IdtEntry;

struct Dummy;

impl core::fmt::Write for Dummy {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        Ok(())
    }
}

fn read_eflags() -> u64 {
    let flags;
    unsafe {
        core::arch::asm!("pushf; pop {}", out(reg) flags);
    }
    flags
}

#[derive(Debug)]
#[repr(C)]
struct InterruptStackFrame {
    pub instruction_pointer: u64,
    pub code_segment: u64,
    pub cpu_flags: u64,
    pub stack_pointer: u64,
    pub stack_segment: u64,
}

extern "x86-interrupt" fn interrupt_div0(frame: InterruptStackFrame) {
    let mut serial = SerialPort::new(COM1_BASE);
    serial.write_str("Div 0");
    writeln!(serial, "Div 0: {:?}", frame);
}

extern "x86-interrupt" fn interrupt_breakpoint(frame: InterruptStackFrame) {
    loop {}
    let mut serial = SerialPort::new(COM1_BASE);
    serial.write_str("Breakpoint");
    writeln!(serial, "Breakpoint: {:?}", frame);
}

extern "x86-interrupt" fn interrupt_dbl(frame: InterruptStackFrame, code: u64) {
    let mut serial = SerialPort::new(COM1_BASE);
    serial.write_str("double fault");
    // writeln!(serial, "interrupt {:?}, code: {}", frame, code);
}

extern "x86-interrupt" fn interrupt_gp(frame: InterruptStackFrame, code: u64) {
    // let mut serial = SerialPort::new(COM1_BASE);
    // writeln!(serial, "interrupt {:?}, code: {}", frame, code);
}

fn divide_by_zero() {
    unsafe { core::arch::asm!("mov dx, 0; div dx") }
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

    let idt = unsafe {
        core::slice::from_raw_parts_mut(info.idt as *mut _, 4096 / core::mem::size_of::<IdtEntry>())
    };
    // idt[0x00] = IdtEntry::new(interrupt_div0 as _, 0, 0b1000_0111_0000_000);
    idt[0x03] = IdtEntry::new(interrupt_breakpoint as _, 0, 0b1000_0111_0000_000);
    // idt[0x08] = IdtEntry::new(interrupt_dbl as _, 0, 0b1000_0111_0000_000);
    // idt[0x0d] = IdtEntry::new(interrupt_gp as _, 0, 0b1000_0111_0000_000);
    // writeln!(serial, "{:?}", idt);

    serial.write_str("setting up idt\n");
    unsafe {
        #[derive(Debug)]
        #[repr(C, packed(2))]
        pub struct DescriptorTablePointer {
            /// Size of the DT.
            pub limit: u16,
            /// Pointer to the memory region containing the DT.
            pub base: u64,
        }
        let ptr = DescriptorTablePointer {
            limit: 32,
            base: info.idt,
        };
        // writeln!(serial, "{:x?}", ptr);
        core::arch::asm!("cli");
        core::arch::asm!("lidt [{}]", in(reg) &ptr, options(readonly, nostack, preserves_flags));
        core::arch::asm!("sti");
    }
    serial.write_str("successfully set up idt (?)\n");

    // divide_by_zero();
    unsafe {
        core::arch::asm!("int3");
    }

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
    // let mut d = Dummy;
    // the line below crashes the whole thing, the macro just invokes .write_fmt so they are equivalent
    // serial.write_fmt(format_args!("{}, format_args! works?\n", 2));
    // write!(d, "write! works");

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
