// https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html volume 3A contains paging information

use crate::control::{Cr0, Cr3};

#[derive(Debug)]
pub struct PhysAddr(u64);

impl PhysAddr {
    pub fn new(addr: u64) -> Self {
        Self(addr)
    }
}

impl core::fmt::Display for PhysAddr {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:x}", self.0)
    }
}

#[derive(Clone, Copy)]
pub struct VirtAddr(u64);

impl VirtAddr {
    pub fn new(addr: u64) -> Self {
        Self(addr)
    }

    pub fn pml4_index(&self) -> u64 {
        self.0 >> 39 & 0x1ff
    }

    pub fn pml3_index(&self) -> u64 {
        self.0 >> 30 & 0x1ff
    }

    pub fn pml2_index(&self) -> u64 {
        self.0 >> 21 & 0x1ff
    }

    pub fn pml1_index(&self) -> u64 {
        self.0 >> 12 & 0x1ff
    }

    pub fn page_1gb_offset(&self) -> u64 {
        self.0 & 0x3fffffff
    }

    pub fn page_2mb_offset(&self) -> u64 {
        self.0 & 0x1fffff
    }

    pub fn page_4kb_offset(&self) -> u64 {
        self.0 & 0xfff
    }
}

impl core::fmt::Debug for VirtAddr {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("VirtAddr")
            .field("address", &self.0)
            .field("pml4_index", &self.pml4_index())
            .field("pml3_index", &self.pml3_index())
            .field("pml2_index", &self.pml2_index())
            .field("pml1_index", &self.pml1_index())
            .finish()
    }
}

impl core::fmt::Display for VirtAddr {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:x}", self.0)
    }
}

trait FrameAllocator {
    fn allocate_frame(&self) -> PhysAddr;

    fn deallocate_frame(&self, frame: PhysAddr);
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct PageTableEntry(pub u64);

impl PageTableEntry {
    pub fn present(&self) -> bool {
        self.0 & 0x1 == 0x1
    }

    pub fn writable(&self) -> bool {
        self.0 & 0x2 == 0x2
    }

    pub fn user_accessible(&self) -> bool {
        self.0 & 0x4 == 0x0
    }

    pub fn page_level_write_through(&self) -> bool {
        self.0 & 0x8 == 0x8
    }

    pub fn page_level_cache_disable(&self) -> bool {
        self.0 & 0x10 == 0x10
    }

    pub fn accessed(&self) -> bool {
        self.0 & 0x20 == 0x20
    }

    pub fn dirty(&self) -> bool {
        self.0 & 0x40 == 0x40
    }

    pub fn is_page(&self) -> bool {
        self.0 & 0x80 == 0x80
    }

    pub fn global(&self) -> bool {
        self.0 & 0x100 == 0x100
    }

    pub fn page_addr_1gb(&self) -> u64 {
        self.0 & 0x0000_ffff_c000_0000
    }

    pub fn page_addr_2mb(&self) -> u64 {
        self.0 & 0x0000_ffff_ffe0_0000
    }

    pub fn page_addr_4kb(&self) -> u64 {
        self.0 & 0x0000_ffff_ffff_f000
    }

    pub fn page_table(&self) -> PageTable {
        PageTable::new((self.0 & 0x0000_ffff_ffff_f000) as _)
    }
}

// TODO: synchronisation

#[derive(Clone, Copy, Debug)]
#[repr(C, align(4096))]
pub struct PageTable {
    pub entries: *const PageTableEntry,
}

impl PageTable {
    pub fn new(base: *const PageTableEntry) -> Self {
        Self { entries: base }
    }

    pub fn get_index(&self, index: usize) -> Option<&PageTableEntry> {
        // TODO: make sure index is between 0 and 512
        unsafe {
            let entry_ptr = self.entries.offset(index as isize);
            let entry = *entry_ptr;
            if !entry.present() {
                return None;
            }

            Some(&*entry_ptr)
        }
    }

    pub fn translate(&self, virt_addr: VirtAddr) -> Option<PhysAddr> {
        let indices = [
            virt_addr.pml4_index(),
            virt_addr.pml3_index(),
            virt_addr.pml2_index(),
        ];

        let pml4 = *self;
        let pml4_index = virt_addr.pml4_index();
        let pml4_entry = pml4.get_index(pml4_index as _)?;

        let pml3 = pml4_entry.page_table();
        let pml3_index = virt_addr.pml3_index();
        let pml3_entry = pml3.get_index(pml3_index as _)?;
        if pml3_entry.is_page() {
            return Some(PhysAddr::new(
                pml3_entry.page_addr_1gb() + virt_addr.page_1gb_offset(),
            ));
        }

        let pml2 = pml3_entry.page_table();
        let pml2_index = virt_addr.pml2_index();
        let pml2_entry = pml2.get_index(pml2_index as _)?;
        if pml2_entry.is_page() {
            return Some(PhysAddr::new(
                pml2_entry.page_addr_2mb() + virt_addr.page_2mb_offset(),
            ));
        }

        let pml1 = pml2_entry.page_table();
        let pml1_index = virt_addr.pml1_index();
        let pml1_entry = pml1.get_index(pml1_index as _)?;
        Some(PhysAddr::new(
            pml1_entry.page_addr_4kb() + virt_addr.page_4kb_offset(),
        ))
    }
}
