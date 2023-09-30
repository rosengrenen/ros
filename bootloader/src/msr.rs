pub unsafe fn rdmsr(msr: u32) -> u64 {
    let low: u32;
    let high: u32;
    unsafe {
        core::arch::asm!("rdmsr", out("eax") low, out("edx") high, in("ecx") msr);
    }
    ((high as u64) << 32) | (low as u64)
}

pub struct LApicInfo(u64);

impl core::fmt::Debug for LApicInfo {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("LApicInfo")
            .field("bsp", &self.bsp())
            .field("global_enable", &self.global_enable())
            .field("base_addr", &self.base_addr())
            .finish()
    }
}

impl LApicInfo {
    pub fn bsp(&self) -> bool {
        self.0 & (1 << 8) != 0
    }

    pub fn global_enable(&self) -> bool {
        self.0 & (1 << 11) != 0
    }

    pub fn base_addr(&self) -> u64 {
        self.0 & 0x0000_000f_ffff_f000
    }
}

pub fn lapic_info() -> LApicInfo {
    LApicInfo(unsafe { rdmsr(0x1b) })
}
