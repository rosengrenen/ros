#[derive(Debug)]
#[repr(C)]
pub struct RFlags {
    // 0 	    CF 	Carry Flag
    cf: bool,
    // 1 	    1 	Reserved
    // 2 	    PF 	Parity Flag
    pf: bool,
    // 3 	    0 	Reserved
    // 4 	    AF 	Auxiliary Carry Flag
    af: bool,
    // 5 	    0 	Reserved
    // 6 	    ZF 	Zero Flag
    zf: bool,
    // 7 	    SF 	Sign Flag
    sf: bool,
    // 8 	    TF 	Trap Flag
    tf: bool,
    // 9 	    IF 	Interrupt Enable Flag
    if_: bool,
    // 10 	  DF 	Direction Flag
    df: bool,
    // 11 	  OF 	Overflow Flag
    of: bool,
    // 12-13 	IOPL 	I/O Privilege Level
    iopl: u8,
    // 14 	  NT 	Nested Task
    nt: bool,
    // 15 	  0 	Reserved
    // 16 	  RF 	Resume Flag
    rf: bool,
    // 17 	  VM 	Virtual-8086 Mode
    vm: bool,
    // 18 	  AC 	Alignment Check / Access Control
    ac: bool,
    // 19 	  VIF 	Virtual Interrupt Flag
    vif: bool,
    // 20 	  VIP 	Virtual Interrupt Pending
    vip: bool,
    // 21 	  ID 	ID Flag
    id: bool,
    // 22-63 	0 	Reserved
}

impl RFlags {
    pub fn read() -> Self {
        let rflags: u64;
        unsafe { core::arch::asm!("pushf; pop {}", out(reg) rflags) };
        Self {
            cf: rflags & (1 << 0) != 0,
            pf: rflags & (1 << 2) != 0,
            af: rflags & (1 << 4) != 0,
            zf: rflags & (1 << 6) != 0,
            sf: rflags & (1 << 7) != 0,
            tf: rflags & (1 << 8) != 0,
            if_: rflags & (1 << 9) != 0,
            df: rflags & (1 << 10) != 0,
            of: rflags & (1 << 11) != 0,
            iopl: ((rflags << 12) & 0xF) as _,
            nt: rflags & (1 << 14) != 0,
            rf: rflags & (1 << 16) != 0,
            vm: rflags & (1 << 17) != 0,
            ac: rflags & (1 << 18) != 0,
            vif: rflags & (1 << 19) != 0,
            vip: rflags & (1 << 20) != 0,
            id: rflags & (1 << 21) != 0,
        }
    }
}
