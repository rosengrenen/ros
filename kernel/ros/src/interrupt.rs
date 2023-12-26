use core::fmt::Write;
use serial::{SerialPort, COM1_BASE};
use x86_64::{control::Cr2, idt::IdtEntry};

use crate::{DescriptorTablePointer, LAPIC};

pub fn init(idt: &mut [IdtEntry]) {
    // entry point, index 1 of gdt  (1 << 3) = 8, options(0x8f00) = [present, gate type is trap gate]
    idt[0x00] = IdtEntry::new(interrupt_div0 as _, 0x8, 0x8e00);
    idt[0x03] = IdtEntry::new(interrupt_breakpoint as _, 0x8, 0x8e00);
    idt[0x08] = IdtEntry::new(interrupt_dbl as _, 0x8, 0x8e00);
    idt[0x0e] = IdtEntry::new(interrupt_page_fault as _, 0x8, 0x8e00);
    idt[0x20] = IdtEntry::new(interrupt_timer as _, 0x8, 0x8e00);
    unsafe {
        let ptr = DescriptorTablePointer {
            limit: (core::mem::size_of_val(idt) - 1) as u16,
            base: idt.as_ptr() as _,
        };
        core::arch::asm!("cli");
        core::arch::asm!("lidt [{}]", in(reg) &ptr);
        core::arch::asm!("sti");
    }
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
    writeln!(serial, "Div 0, frame: {:#x?}", frame).unwrap();
    loop {}
}

extern "x86-interrupt" fn interrupt_breakpoint(frame: InterruptStackFrame) {
    let mut serial = SerialPort::new(COM1_BASE);
    writeln!(serial, "Breakpoint, frame: {:#x?}", frame).unwrap();
}

extern "x86-interrupt" fn interrupt_dbl(frame: InterruptStackFrame, code: u64) {
    let mut serial = SerialPort::new(COM1_BASE);
    writeln!(
        serial,
        "Double fault, frame: {:#x?}. code: {:#x}",
        frame, code
    )
    .unwrap();
}

extern "x86-interrupt" fn interrupt_page_fault(frame: InterruptStackFrame, code: u64) {
    let mut serial = SerialPort::new(COM1_BASE);
    writeln!(
        serial,
        "Page fault, frame: {:#x?}, code: {:#x}, trying to access: {:#x?}",
        frame,
        code,
        Cr2::read()
    )
    .unwrap();
    loop {}
}

static mut COUNT: usize = 0;

extern "x86-interrupt" fn interrupt_timer(frame: InterruptStackFrame) {
    let mut serial = SerialPort::new(COM1_BASE);
    writeln!(
        serial,
        "{:?} Timer interrupt, frame: {:#x?}",
        unsafe { COUNT },
        frame,
    )
    .unwrap();
    unsafe {
        COUNT += 1;
        LAPIC.write_eoi();
    }
}
