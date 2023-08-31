#![no_std]
#![no_main]
#![feature(allocator_api)]
#![feature(abi_x86_interrupt)]

mod interrupt;

use bootloader_api::BootInfo;
use core::{fmt::Write, panic::PanicInfo};
use serial::{SerialPort, COM1_BASE};
use uefi::services::graphics::BltPixel;
use x86_64::{gdt::GdtDesc, idt::IdtEntry};

#[derive(Debug)]
#[repr(C, packed(2))]
pub struct DescriptorTablePointer {
    /// Size of the DT.
    pub limit: u16,
    /// Pointer to the memory region containing the DT.
    pub base: u64,
}

#[no_mangle]
pub extern "C" fn _start(info: &'static BootInfo) -> ! {
    // Set things up
    // * Set up physical frame manager
    // Load init system

    let mut serial = SerialPort::new(COM1_BASE);
    serial.configure(1);

    let gdt = unsafe { core::slice::from_raw_parts_mut(info.gdt as *mut u64, 3) };
    for entry in gdt.iter_mut() {
        *entry = 0;
    }

    let idt = unsafe { core::slice::from_raw_parts_mut(info.idt as *mut IdtEntry, 256) };
    for entry in idt.iter_mut() {
        *entry = IdtEntry::new(0, 0, 0);
    }

    serial.write_str("setting up gdt\n").unwrap();
    init_gdt(gdt);
    serial.write_str("successfully set up gdt (?)\n").unwrap();

    serial.write_str("setting up idt\n").unwrap();
    interrupt::init(idt);
    serial.write_str("successfully set up idt (?)\n").unwrap();

    unsafe {
        core::arch::asm!("int3");
    }
    // divide_by_zero();
    // cause_page_fault();

    color_loop(info.framebuffer)
}

fn init_gdt(gdt: &mut [u64]) {
    // null segment
    gdt[0] = 0;
    // kernel code segment
    // flags(0x2) = [long mode], access byte(0x9a) = [present, desc type = code/data segment, executable, rw]
    gdt[1] = 0x0020_9a00_0000_0000;
    // kernel data segment
    // flags(0x2) = [long mode], access byte(0x92) = [present, desc type = code/data segment, rw]
    gdt[2] = 0x0020_9200_0000_0000;

    unsafe {
        let ptr = DescriptorTablePointer {
            limit: (gdt.len() * core::mem::size_of::<GdtDesc>() - 1) as u16,
            base: gdt.as_ptr() as _,
        };
        core::arch::asm!("cli");
        core::arch::asm!("lgdt [{}]", in(reg) &ptr);
        core::arch::asm!("sti");
    }

    reload_segments();
}

// TODO: move to x86_64 crate
fn reload_segments() {
    unsafe {
        core::arch::asm!(
                // push the segment selector, index 1 of the gdt
                "push 0x8",
                // load and push the address of the "2" label
                "lea {tmp}, [rip + 2f]",
                "push {tmp}",
                // far return, popping the return address and the new CS value from the stack
                "retfq",
                "2:",
                // set the rest of the segment registers to the data segment in the gdt
                "mov ax, 0x10",
                "mov ds, ax",
                "mov es, ax",
                "mov fs, ax",
                "mov gs, ax",
                "mov ss, ax",
                tmp = lateout(reg) _,
        );
    }
}

#[allow(dead_code)]
fn divide_by_zero() {
    unsafe { core::arch::asm!("mov dx, 0; div dx") }
}

#[allow(dead_code)]
fn cause_page_fault() {
    unsafe {
        *(0xdead_beef as *mut u64) = 5;
    }
}

fn color_loop(framebuffer: bootloader_api::Framebuffer) -> ! {
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

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // println!("{}", info);
    loop {}
}
