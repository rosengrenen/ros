#[derive(Debug)]
pub struct Cr0 {
    // 0 	PE 	Protected Mode Enable
    pe: bool,
    // 1 	MP 	Monitor Co-Processor
    mp: bool,
    // 2 	EM 	Emulation
    em: bool,
    // 3 	TS 	Task Switched
    ts: bool,
    // 4 	ET 	Extension Type
    et: bool,
    // 5 	NE 	Numeric Error
    ne: bool,
    // 6-15 	0 	Reserved
    // 16 	WP 	Write Protect
    wp: bool,
    // 17 	0 	Reserved
    // 18 	AM 	Alignment Mask
    am: bool,
    // 19-28 	0 	Reserved
    // 29 	NW 	Not-Write Through
    nw: bool,
    // 30 	CD 	Cache Disable
    cd: bool,
    // 31 	PG 	Paging
    pg: bool,
    // 32-63 	0 	Reserved
}

impl Cr0 {
    pub fn read() -> Self {
        let cr0: u64;
        unsafe {
            core::arch::asm!("mov {}, cr0", out(reg) cr0);
        }
        Self {
            pe: cr0 & (1 << 0) != 0,
            mp: cr0 & (1 << 1) != 0,
            em: cr0 & (1 << 2) != 0,
            ts: cr0 & (1 << 3) != 0,
            et: cr0 & (1 << 4) != 0,
            ne: cr0 & (1 << 5) != 0,
            wp: cr0 & (1 << 16) != 0,
            am: cr0 & (1 << 18) != 0,
            nw: cr0 & (1 << 29) != 0,
            cd: cr0 & (1 << 30) != 0,
            pg: cr0 & (1 << 31) != 0,
        }
    }
}

#[derive(Debug)]
pub struct Cr2(u64);

impl Cr2 {
    pub fn read() -> Self {
        let cr2: u64;
        unsafe {
            core::arch::asm!("mov {}, cr2", out(reg) cr2);
        }
        Self(cr2)
    }
}

#[derive(Debug)]
pub struct Cr3 {
    // 3 	PWT 	Page-Level Write Through, when CR4.PCIDE = 0
    pwt: bool,
    // 5 	PCD 	Page-Level Cache Disable, when CR4.PCIDE = 0
    pcd: bool,
    //  0-11 	PCID, when CR4.PCIDE = 1
    pcid: u16,
    // 12-63 	Physical Base Address of the PML4
    pba_pml4: u64,
}

impl Cr3 {
    pub fn read() -> Self {
        let cr3: u64;
        unsafe {
            core::arch::asm!("mov {}, cr3", out(reg) cr3);
        };
        Self {
            pwt: cr3 & (1 << 3) != 0,
            pcd: cr3 & (1 << 5) != 0,
            pcid: (cr3 & 0b111_1111_1111) as u16,
            pba_pml4: (cr3 >> 12),
        }
    }
}

#[derive(Debug)]
pub struct Cr4 {
    //  0 	VME 	Virtual-8086 Mode Extensions
    vme: bool,
    // 1 	PVI 	Protected Mode Virtual Interrupts
    pvi: bool,
    // 2 	TSD 	Time Stamp enabled only in ring 0
    tsd: bool,
    // 3 	DE 	Debugging Extensions
    de: bool,
    // 4 	PSE 	Page Size Extension
    pse: bool,
    // 5 	PAE 	Physical Address Extension
    pae: bool,
    // 6 	MCE 	Machine Check Exception
    mce: bool,
    // 7 	PGE 	Page Global Enable
    pge: bool,
    // 8 	PCE 	Performance Monitoring Counter Enable
    pce: bool,
    // 9 	OSFXSR 	OS support for fxsave and fxrstor instructions
    dsfxsr: bool,
    // 10 	OSXMMEXCPT 	OS Support for unmasked simd floating point exceptions
    osxmmexcpt: bool,
    // 11 	UMIP 	User-Mode Instruction Prevention (SGDT, SIDT, SLDT, SMSW, and STR are disabled in user mode)
    uimp: bool,
    // 12 	0 	Reserved
    // 13 	VMXE 	Virtual Machine Extensions Enable
    vmxe: bool,
    // 14 	SMXE 	Safer Mode Extensions Enable
    smxe: bool,
    // 15 	0 	Reserved
    // 16 	FSGSBASE 	Enables the instructions RDFSBASE, RDGSBASE, WRFSBASE, and WRGSBASE
    fsgsbase: bool,
    // 17 	PCIDE 	PCID Enable
    pcide: bool,
    // 18 	OSXSAVE 	XSAVE And Processor Extended States Enable
    osxsave: bool,
    // 19 	0 	Reserved
    // 20 	SMEP 	Supervisor Mode Executions Protection Enable
    smep: bool,
    // 21 	SMAP 	Supervisor Mode Access Protection Enable
    smap: bool,
    // 22 	PKE 	Enable protection keys for user-mode pages
    pke: bool,
    // 23 	CET 	Enable Control-flow Enforcement Technology
    cet: bool,
    // 24 	PKS 	Enable protection keys for supervisor-mode pages
    pks: bool,
    // 25-63 	0 	Reserved
}

impl Cr4 {
    pub fn read() -> Self {
        let cr4: u64;
        unsafe {
            core::arch::asm!("mov {}, cr4", out(reg) cr4);
        };
        Self {
            vme: cr4 & (1 << 0) != 0,
            pvi: cr4 & (1 << 1) != 0,
            tsd: cr4 & (1 << 2) != 0,
            de: cr4 & (1 << 3) != 0,
            pse: cr4 & (1 << 4) != 0,
            pae: cr4 & (1 << 5) != 0,
            mce: cr4 & (1 << 6) != 0,
            pge: cr4 & (1 << 7) != 0,
            pce: cr4 & (1 << 8) != 0,
            dsfxsr: cr4 & (1 << 9) != 0,
            osxmmexcpt: cr4 & (1 << 10) != 0,
            uimp: cr4 & (1 << 11) != 0,
            vmxe: cr4 & (1 << 13) != 0,
            smxe: cr4 & (1 << 14) != 0,
            fsgsbase: cr4 & (1 << 16) != 0,
            pcide: cr4 & (1 << 17) != 0,
            osxsave: cr4 & (1 << 18) != 0,
            smep: cr4 & (1 << 20) != 0,
            smap: cr4 & (1 << 21) != 0,
            pke: cr4 & (1 << 22) != 0,
            cet: cr4 & (1 << 23) != 0,
            pks: cr4 & (1 << 24) != 0,
        }
    }
}
