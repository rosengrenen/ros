#[derive(Debug)]
pub struct Efer {
    // 0 	SCE 	System Call Extensions
    sce: bool,
    // 1-7 	0 	Reserved
    // 8 	LME 	Long Mode Enable
    lme: bool,
    // 10 	LMA 	Long Mode Active
    lma: bool,
    // 11 	NXE 	No-Execute Enable
    nxe: bool,
    // 12 	SVME 	Secure Virtual Machine Enable
    svme: bool,
    // 13 	LMSLE 	Long Mode Segment Limit Enable
    lmsle: bool,
    // 14 	FFXSR 	Fast FXSAVE/FXRSTOR
    ffxsr: bool,
    // 15 	TCE 	Translation Cache Extension
    tce: bool,
    // 16-63 	0 	Reserved
}

impl Efer {
    pub fn read() -> Self {
        let efer: u32;
        unsafe {
            core::arch::asm!(
                "rdmsr",
                in("ecx") 0xC0000080u32,
                out("eax") efer
            );
        }
        Self {
            sce: efer & (1 << 0) != 0,
            lme: efer & (1 << 8) != 0,
            lma: efer & (1 << 10) != 0,
            nxe: efer & (1 << 11) != 0,
            svme: efer & (1 << 12) != 0,
            lmsle: efer & (1 << 13) != 0,
            ffxsr: efer & (1 << 14) != 0,
            tce: efer & (1 << 15) != 0,
        }
    }
}
