#[derive(Debug)]
#[repr(C)]
pub struct IdtEntry {
    fn_ptr_low: u16,  // [0:15]	The lower bits of the pointer to the handler function.
    gdt: u16,         // selector	Selector of a code segment in the global descriptor table.
    options: u16,     //	(see below)
    fn_ptr_mid: u16,  // [16:31]	The middle bits of the pointer to the handler function.
    fn_ptr_high: u32, // [32:63]	The remaining bits of the pointer to the handler function.
    _reserved: u32,   //
}

impl IdtEntry {
    pub fn new(fn_ptr: u64, gdt: u16, options: u16) -> Self {
        Self {
            fn_ptr_low: fn_ptr as u16,
            gdt: 0x8, //read_cs(),
            options: 0x8e,
            fn_ptr_mid: (fn_ptr >> 16) as u16,
            fn_ptr_high: (fn_ptr >> 32) as u32,
            _reserved: 0,
        }
    }
}

pub fn read_cs() -> u16 {
    let mut segment: u16 = 0;
    unsafe {
        core::arch::asm!("mov {0:x}, cs", out(reg) segment, options(nomem, nostack, preserves_flags));
    }
    segment
}
