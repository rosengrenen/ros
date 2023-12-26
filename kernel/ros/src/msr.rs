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

pub struct LApic {
    pub base: u64,
}

impl LApic {
    const EOI: u64 = 0x00b0;
    const SPURIOUS_VECTOR_INTERRUPT: u64 = 0x00f0;
    const TIMER_LVT: u64 = 0x0320;
    const INITIAL_COUNT: u64 = 0x0380;
    const CURRENT_COUNT: u64 = 0x0390;
    const DIVIDE_CONFIGURATION: u64 = 0x03e0;

    pub fn current() -> Self {
        Self {
            base: lapic_info().base_addr(),
        }
    }

    pub fn write_eoi(&self) {
        self.write(Self::EOI, 0)
    }

    pub fn read_spurious_interrupt_vector(&self) -> u32 {
        self.read(Self::SPURIOUS_VECTOR_INTERRUPT)
    }

    pub fn write_spurious_interrupt_vector(&self, value: u32) {
        self.write(Self::SPURIOUS_VECTOR_INTERRUPT, value)
    }

    pub fn read_divide_configuration(&self) -> u32 {
        self.read(Self::DIVIDE_CONFIGURATION)
    }

    pub fn write_divide_configuration(&self, value: u32) {
        self.write(Self::DIVIDE_CONFIGURATION, value)
    }

    pub fn read_timer_lvt(&self) -> u32 {
        self.read(Self::TIMER_LVT)
    }

    pub fn write_timer_lvt(&self, value: u32) {
        self.write(Self::TIMER_LVT, value)
    }

    pub fn read_initial_count(&self) -> u32 {
        self.read(Self::INITIAL_COUNT)
    }

    pub fn write_initial_count(&self, value: u32) {
        self.write(Self::INITIAL_COUNT, value)
    }

    pub fn read_current_count(&self) -> u32 {
        self.read(Self::CURRENT_COUNT)
    }

    fn read(&self, offset: u64) -> u32 {
        unsafe { ((self.base + offset) as *const u32).read() }
    }

    fn write(&self, offset: u64, value: u32) {
        unsafe { ((self.base + offset) as *mut u32).write(value) }
    }
}
