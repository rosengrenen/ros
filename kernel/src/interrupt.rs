use core::fmt::Write;
use serial::{SerialPort, COM1_BASE};
use x86_64::{control::Cr2, idt::IdtEntry};

use crate::DescriptorTablePointer;

pub fn init(idt: &mut [IdtEntry]) {
    // entry point, index 1 of gdt  (1 << 3) = 8, options(0x8f00) = [present, gate type is trap gate]
    idt[0x0] = IdtEntry::new(interrupt_div0 as _, 0x8, 0x8e00);
    idt[0x3] = IdtEntry::new(interrupt_breakpoint as _, 0x8, 0x8e00);
    idt[0x8] = IdtEntry::new(interrupt_dbl as _, 0x8, 0x8e00);
    idt[0xe] = IdtEntry::new(interrupt_page_fault as _, 0x8, 0x8e00);
    unsafe {
        let ptr = DescriptorTablePointer {
            limit: (idt.len() * core::mem::size_of::<IdtEntry>() - 1) as u16,
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
}
