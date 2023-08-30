// https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html volume 3A contains paging information

use crate::control::{Cr0, Cr3};

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct PageMapLevel4TableEntry(pub u64);

impl PageMapLevel4TableEntry {
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

    pub fn page_directory_pointer(&self) -> PageDirPointerTable {
        PageDirPointerTable::new((self.0 & 0x0000_ffff_ffff_f000) as _)
    }

    pub fn execute_disable(&self) -> bool {
        self.0 & 0x8000 == 0x8000
    }
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct PageDirPointerTableEntry(pub u64);

impl PageDirPointerTableEntry {
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

    pub fn page_size(&self) -> bool {
        self.0 & 0x80 == 0x80
    }

    pub fn global(&self) -> bool {
        self.0 & 0x100 == 0x100
    }

    pub fn page_addr_1gb(&self) -> u64 {
        self.0 & 0x0000_ffff_c000_0000
    }

    pub fn page_directory(&self) -> PageDirTable {
        PageDirTable::new((self.0 & 0x0000_ffff_ffff_f000) as _)
    }
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct PageDirTableEntry(pub u64);

impl PageDirTableEntry {
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

    pub fn page_size(&self) -> bool {
        self.0 & 0x80 == 0x80
    }

    pub fn global(&self) -> bool {
        self.0 & 0x100 == 0x100
    }

    pub fn page_addr_2mb(&self) -> u64 {
        self.0 & 0x0000_ffff_ffe0_0000
    }

    pub fn page_table(&self) -> PageTable {
        PageTable::new((self.0 & 0x0000_ffff_ffff_f000) as _)
    }
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

    pub fn global(&self) -> bool {
        self.0 & 0x100 == 0x100
    }

    pub fn page_addr_4kb(&self) -> u64 {
        self.0 & 0x0000_ffff_ffff_f000
    }

    pub fn page_table(&self) -> PageTable {
        PageTable::new((self.0 & 0x0000_ffff_ffff_f000) as _)
    }
}

// TODO: synchronisation
#[derive(Debug)]
#[repr(C, align(4096))]
pub struct PageMapLevel4Table {
    pub entries: *const PageMapLevel4TableEntry,
}

impl PageMapLevel4Table {
    pub fn new(base: *const PageMapLevel4TableEntry) -> Self {
        Self { entries: base }
    }

    pub fn from_cr3() -> Self {
        let cr3 = Cr3::read();
        let base = cr3.pba_pml4;
        Self { entries: base as _ }
    }

    pub fn get_index(&self, index: usize) -> Option<*const PageMapLevel4TableEntry> {
        // TODO: make sure index is between 0 and 512
        unsafe {
            let entry_ptr = self.entries.offset(index as isize);
            let entry = *entry_ptr;
            if !entry.present() {
                return None;
            }

            Some(entry_ptr)
        }
    }
}

#[derive(Debug)]
#[repr(C, align(4096))]
pub struct PageDirPointerTable {
    pub entries: *const PageDirPointerTableEntry,
}

impl PageDirPointerTable {
    pub fn new(base: *const PageDirPointerTableEntry) -> Self {
        Self { entries: base }
    }

    pub fn get_index(&self, index: usize) -> Option<*const PageDirPointerTableEntry> {
        // TODO: make sure index is between 0 and 512
        unsafe {
            let entry_ptr = self.entries.offset(index as isize);
            let entry = *entry_ptr;
            if !entry.present() {
                return None;
            }

            Some(entry_ptr)
        }
    }
}

#[derive(Debug)]
#[repr(C, align(4096))]
pub struct PageDirTable {
    pub entries: *const PageDirTableEntry,
}

impl PageDirTable {
    pub fn new(base: *const PageDirTableEntry) -> Self {
        Self { entries: base }
    }

    pub fn get_index(&self, index: usize) -> Option<*const PageDirTableEntry> {
        // TODO: make sure index is between 0 and 512
        unsafe {
            let entry_ptr = self.entries.offset(index as isize);
            let entry = *entry_ptr;
            if !entry.present() {
                return None;
            }

            Some(entry_ptr)
        }
    }
}

#[derive(Debug)]
#[repr(C, align(4096))]
pub struct PageTable {
    pub entries: *const PageTableEntry,
}

impl PageTable {
    pub fn new(base: *const PageTableEntry) -> Self {
        Self { entries: base }
    }

    pub fn get_index(&self, index: usize) -> Option<*const PageTableEntry> {
        // TODO: make sure index is between 0 and 512
        unsafe {
            let entry_ptr = self.entries.offset(index as isize);
            let entry = *entry_ptr;
            if !entry.present() {
                return None;
            }

            Some(entry_ptr)
        }
    }
}
