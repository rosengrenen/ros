use core::fmt::Write;

use serial::SerialPort;
use serial::COM1_BASE;
use x86_64::control::Cr2;
use x86_64::idt::IdtEntry;

use crate::sprintln;
use crate::DescriptorTablePointer;
use crate::LAPIC;

pub fn init(idt: &mut [IdtEntry]) {
    // entry point, index 1 of gdt  (1 << 3) = 8, options(0x8f00) = [present, gate type is trap gate]
    idt[0x00] = IdtEntry::new(interrupt_div0 as _, 0x8, 0x8e00);
    idt[0x03] = IdtEntry::new(interrupt_breakpoint as _, 0x8, 0x8e00);
    idt[0x08] = IdtEntry::new(interrupt_dbl as _, 0x8, 0x8e00);
    idt[0x0e] = IdtEntry::new(interrupt_page_fault as _, 0x8, 0x8e00);
    idt[0x20] = IdtEntry::new(interrupt_timer as _, 0x8, 0x8e00);
    idt[0x21] = IdtEntry::new(interrupt_kb as _, 0x8, 0x8e00);
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
    // let mut serial = SerialPort::new(COM1_BASE);
    // writeln!(
    //     serial,
    //     "{:?} Timer interrupt, frame: {:#x?}",
    //     unsafe { COUNT },
    //     frame,
    // )
    // .unwrap();
    unsafe {
        COUNT += 1;
        LAPIC.write_eoi();
    }
}

extern "x86-interrupt" fn interrupt_kb(frame: InterruptStackFrame) {
    let scancode = unsafe { inb(0x60) };
    // let mut serial = SerialPort::new(COM1_BASE);
    // writeln!(
    //     serial,
    //     "Keyboard interrupt, frame => scancode: {:x}",
    //     scancode
    // )
    // .unwrap();
    print_scancode(scancode);
    unsafe {
        LAPIC.write_eoi();
    }
}

fn print_scancode(b: u8) {
    let special_case_string = match b {
        0x00 => Some("Key detection error or internal buffer overrun"),
        0xAA => Some("Self test passed (sent after \"0xFF (reset)\" command or keyboard power up)"),
        0xEE => Some("Response to \"0xEE (echo)\" command"),
        0xFA => Some("Command acknowledged (ACK)"),
        0xFC => Some("Self test failed (sent after \"0xFF (reset)\" command or keyboard power up"),
        0xFD => Some("Self test failed (sent after \"0xFF (reset)\" command or keyboard power up"),
        0xFE => Some("Resend (keyboard wants controller to repeat last command it sent"),
        0xFF => Some("Key detection error or internal buffer overrun"),
        _ => None,
    };
    if let Some(msg) = special_case_string {
        sprintln!("{}", msg);
        return;
    }

    let string = match b {
        0x01 => "escape down",
        0x02 => "1 down",
        0x03 => "2 down",
        0x04 => "3 down",
        0x05 => "4 down",
        0x06 => "5 down",
        0x07 => "6 down",
        0x08 => "7 down",
        0x09 => "8 down",
        0x0a => "9 down",
        0x0b => "0 down",
        0x0c => "- down",
        0x0d => "= down",
        0x0e => "backspace down",
        0x0f => "tab down",
        0x10 => "q down",
        0x11 => "w down",
        0x12 => "e down",
        0x13 => "r down",
        0x14 => "t down",
        0x15 => "y down",
        0x16 => "u down",
        0x17 => "i down",
        0x18 => "o down",
        0x19 => "p down",
        0x1a => "[ down",
        0x1b => "] down",
        0x1c => "enter down",
        0x1d => "left ctrl down",
        0x1e => "a down",
        0x1f => "s down",
        0x20 => "d down",
        0x21 => "f down",
        0x22 => "g down",
        0x23 => "h down",
        0x24 => "j down",
        0x25 => "k down",
        0x26 => "l down",
        0x27 => "; down",
        0x28 => "' down",
        0x29 => "` down",
        0x2a => "left shift down",
        0x2b => "\\ down",
        0x2c => "z down",
        0x2d => "x down",
        0x2e => "c down",
        0x2f => "v down",
        0x30 => "b down",
        0x31 => "n down",
        0x32 => "m down",
        0x33 => ", down",
        0x34 => ". down",
        0x35 => "/ down",
        0x36 => "right shift down",
        0x37 => "(keypad) * down",
        0x38 => "left alt down",
        0x39 => "space down",
        0x3a => "caps down",
        0x3b => "f1 down",
        0x3c => "f2 down",
        0x3d => "f3 down",
        0x3e => "f4 down",
        0x3f => "f5 down",
        0x40 => "f6 down",
        0x41 => "f7 down",
        0x42 => "f8 down",
        0x43 => "f9 down",
        0x44 => "f10 down",
        0x45 => "num lock down",
        0x46 => "scroll lock down",
        0x47 => "(keypad) 7 down",
        0x48 => "(keypad) 8 down",
        0x49 => "(keypad) 9 down",
        0x4a => "(keypad) - down",
        0x4b => "(keypad) 4 down",
        0x4c => "(keypad) 5 down",
        0x4d => "(keypad) 6 down",
        0x4e => "(keypad) + down",
        0x4f => "(keypad) 0 down",
        0x50 => "(keypad) 1 down",
        0x51 => "(keypad) 2 down",
        0x52 => "(keypad) 3 down",
        0x53 => "(keypad) . down",
        0x57 => "f11 down",
        0x58 => "f12 down",
        0x81 => "escape up",
        0x82 => "1 up",
        0x83 => "2 up",
        0x84 => "3 up",
        0x85 => "4 up",
        0x86 => "5 up",
        0x87 => "6 up",
        0x88 => "7 up",
        0x89 => "8 up",
        0x8a => "9 up",
        0x8b => "0 up",
        0x8c => "- up",
        0x8d => "= up",
        0x8e => "backspace up",
        0x8f => "tab up",
        0x90 => "q up",
        0x91 => "w up",
        0x92 => "e up",
        0x93 => "r up",
        0x94 => "t up",
        0x95 => "y up",
        0x96 => "u up",
        0x97 => "i up",
        0x98 => "o up",
        0x99 => "p up",
        0x9a => "[ up",
        0x9b => "] up",
        0x9c => "enter up",
        0x9d => "left ctrl up",
        0x9e => "a up",
        0x9f => "s up",
        0xa0 => "d up",
        0xa1 => "f up",
        0xa2 => "g up",
        0xa3 => "h up",
        0xa4 => "j up",
        0xa5 => "k up",
        0xa6 => "l up",
        0xa7 => "; up",
        0xa8 => "' up",
        0xa9 => "` up",
        0xaa => "left shift up",
        0xab => "\\ up",
        0xac => "z up",
        0xad => "x up",
        0xae => "c up",
        0xaf => "v up",
        0xb0 => "b up",
        0xb1 => "n up",
        0xb2 => "m up",
        0xb3 => ", up",
        0xb4 => ". up",
        0xb5 => "/ up",
        0xb6 => "right shift up",
        0xb7 => "(keypad) * up",
        0xb8 => "left alt up",
        0xb9 => "space up",
        0xba => "caps up",
        0xbb => "f1 up",
        0xbc => "f2 up",
        0xbd => "f3 up",
        0xbe => "f4 up",
        0xbf => "f5 up",
        0xc0 => "f6 up",
        0xc1 => "f7 up",
        0xc2 => "f8 up",
        0xc3 => "f9 up",
        0xc4 => "f10 up",
        0xc5 => "num lock up",
        0xc6 => "scroll lock up",
        0xc7 => "(keypad) 7 up",
        0xc8 => "(keypad) 8 up",
        0xc9 => "(keypad) 9 up",
        0xca => "(keypad) - up",
        0xcb => "(keypad) 4 up",
        0xcc => "(keypad) 5 up",
        0xcd => "(keypad) 6 up",
        0xce => "(keypad) + up",
        0xcf => "(keypad) 0 up",
        0xd0 => "(keypad) 1 up",
        0xd1 => "(keypad) 2 up",
        0xd2 => "(keypad) 3 up",
        0xd3 => "(keypad) . up",
        0xd7 => "f11 up",
        0xd8 => "f12 up",
        _ => "unknown scancode",
    };
    sprintln!("{}", string);
}

/// Write byte to port
unsafe fn outb(port: u16, data: u8) {
    unsafe {
        core::arch::asm!("out dx, al",
          in("al") data,
          in("dx") port
        );
    }
}

unsafe fn inb(port: u16) -> u8 {
    unsafe {
        // Trust me bro
        #[allow(unused_assignments)]
        let mut data = 0;
        core::arch::asm!("in al, dx",
          in("dx") port,
          inout("al") data
        );
        data
    }
}
