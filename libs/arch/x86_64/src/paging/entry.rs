use core::marker::PhantomData;

use crate::frame::{Frame1GiB, Frame2MiB, Frame4KiB};

use super::{PageTable, PhysAddr, Pml1, Pml2, Pml3, Pml4};

#[derive(Debug)]
#[repr(C)]
pub struct PageTableEntryRaw<S> {
    pub value: usize,
    _marker: PhantomData<S>,
}

impl<S> Copy for PageTableEntryRaw<S> {}
impl<S> Clone for PageTableEntryRaw<S> {
    fn clone(&self) -> Self {
        Self {
            value: self.value,
            _marker: PhantomData,
        }
    }
}

impl<S> PageTableEntryRaw<S> {
    const PRESENT_BIT: usize = 0;
    const WRITABLE_BIT: usize = 1;
    const USER_ACCESIBLE_BIT: usize = 2;
    const PAGE_LEVEL_WRITE_THROUGH_BIT: usize = 3;
    const PAGE_LEVEL_CACHE_BIT: usize = 4;
    const ACCESSED_BIT: usize = 5;
    const DIRTY_BIT: usize = 6;
    const IS_PAGE_BIT: usize = 6;

    pub fn new(value: usize) -> Self {
        Self {
            value,
            _marker: PhantomData,
        }
    }

    pub fn present(&self) -> bool {
        self.value & (1 << Self::PRESENT_BIT) != 0
    }

    pub fn set_present(&mut self, value: bool) {
        self.set_bit(Self::PRESENT_BIT, value)
    }

    pub fn writable(&self) -> bool {
        self.value & (1 << Self::WRITABLE_BIT) != 0
    }

    pub fn set_writable(&mut self, value: bool) {
        self.set_bit(Self::WRITABLE_BIT, value)
    }

    pub fn user_accessible(&self) -> bool {
        self.value & (1 << Self::USER_ACCESIBLE_BIT) != 0
    }

    pub fn set_user_accessible(&mut self, value: bool) {
        self.set_bit(Self::USER_ACCESIBLE_BIT, value)
    }

    pub fn page_level_write_through(&self) -> bool {
        self.value & (1 << Self::PAGE_LEVEL_WRITE_THROUGH_BIT) != 0
    }

    pub fn set_page_level_write_through(&mut self, value: bool) {
        self.set_bit(Self::PAGE_LEVEL_WRITE_THROUGH_BIT, value)
    }

    pub fn page_level_cache_disable(&self) -> bool {
        self.value & (1 << Self::PAGE_LEVEL_CACHE_BIT) != 0
    }

    pub fn set_page_level_cache_disable(&mut self, value: bool) {
        self.set_bit(Self::PAGE_LEVEL_CACHE_BIT, value)
    }

    pub fn accessed(&self) -> bool {
        self.value & (1 << Self::ACCESSED_BIT) != 0
    }

    pub fn set_accessed(&mut self, value: bool) {
        self.set_bit(Self::ACCESSED_BIT, value)
    }

    pub fn dirty(&self) -> bool {
        self.value & (1 << Self::DIRTY_BIT) != 0
    }

    pub fn set_dirty(&mut self, value: bool) {
        self.set_bit(Self::DIRTY_BIT, value)
    }

    pub fn is_page(&self) -> bool {
        self.value & (1 << Self::IS_PAGE_BIT) != 0
    }

    pub fn set_is_page(&mut self, value: bool) {
        self.set_bit(Self::IS_PAGE_BIT, value)
    }

    pub fn user_bits(&self) -> u16 {
        // Bits 9:10
        let low = (self.value >> 9) & 0b11;
        // Bits 52:58
        let high = (self.value >> 52) & 0b111_1111;
        ((high << 2) | low) as u16
    }

    pub fn set_user_bits(&mut self, bits: u16) {
        let bits = bits as usize;
        self.value &= !(0b11 << 9);
        self.value |= (bits & 0b11) << 9;
        self.value &= !(0b111_1111 << 52);
        self.value |= ((bits >> 2) & 0b111_1111) << 52;
    }

    fn set_bit(&mut self, bit: usize, value: bool) {
        if value {
            self.value |= (1 << bit);
        } else {
            self.value &= !(1 << bit);
        }
    }
}

#[derive(Clone, Copy)]
pub enum PageTableEntry<S> {
    Page(PageEntry<S>),
    Table(TableEntry<S>),
}

#[derive(Clone, Copy)]
pub struct PageEntry<S> {
    raw: PageTableEntryRaw<S>,
}

impl<S> PageEntry<S> {
    pub fn new(mut raw: PageTableEntryRaw<S>) -> Self {
        raw.set_present(true);
        raw.set_is_page(true);
        Self { raw }
    }

    pub fn raw(&self) -> PageTableEntryRaw<S> {
        self.raw
    }

    pub fn writable(&self) -> bool {
        self.raw.writable()
    }

    pub fn set_writable(&mut self, value: bool) {
        self.raw.set_writable(value)
    }

    pub fn user_accessible(&self) -> bool {
        self.raw.user_accessible()
    }

    pub fn set_user_accessible(&mut self, value: bool) {
        self.raw.set_user_accessible(value)
    }

    pub fn page_level_write_through(&self) -> bool {
        self.raw.page_level_write_through()
    }

    pub fn set_page_level_write_through(&mut self, value: bool) {
        self.raw.set_page_level_write_through(value)
    }

    pub fn page_level_cache_disable(&self) -> bool {
        self.raw.page_level_cache_disable()
    }

    pub fn set_page_level_cache_disable(&mut self, value: bool) {
        self.raw.set_page_level_cache_disable(value)
    }

    pub fn accessed(&self) -> bool {
        self.raw.accessed()
    }

    pub fn set_accessed(&mut self, value: bool) {
        self.raw.set_accessed(value)
    }

    pub fn dirty(&self) -> bool {
        self.raw.dirty()
    }

    pub fn set_dirty(&mut self, value: bool) {
        self.raw.set_dirty(value)
    }
}

impl PageEntry<Pml3> {
    pub fn frame(&self) -> Frame1GiB {
        Frame1GiB::new(unsafe { PhysAddr::new(self.raw.value & 0x000f_ffff_c000_0000) })
    }
}

impl PageEntry<Pml2> {
    pub fn frame(&self) -> Frame2MiB {
        Frame2MiB::new(unsafe { PhysAddr::new(self.raw.value & 0x000f_ffff_ffe0_0000) })
    }
}

impl PageEntry<Pml1> {
    pub fn frame(&self) -> Frame4KiB {
        Frame4KiB::new(unsafe { PhysAddr::new(self.raw.value & 0x000f_ffff_ffff_f000) })
    }
}

#[derive(Clone, Copy)]
pub struct TableEntry<S> {
    raw: PageTableEntryRaw<S>,
}

impl<S> TableEntry<S> {
    const ADDR_MASK: usize = 0x000f_ffff_ffff_f000;

    pub fn new(mut raw: PageTableEntryRaw<S>) -> Self {
        raw.set_present(true);
        raw.set_is_page(false);
        Self { raw }
    }

    pub fn raw(&self) -> PageTableEntryRaw<S> {
        self.raw
    }

    pub fn writable(&self) -> bool {
        self.raw.writable()
    }

    pub fn set_writable(&mut self, value: bool) {
        self.raw.set_writable(value)
    }

    pub fn user_accessible(&self) -> bool {
        self.raw.user_accessible()
    }

    pub fn set_user_accessible(&mut self, value: bool) {
        self.raw.set_user_accessible(value)
    }

    pub fn page_level_write_through(&self) -> bool {
        self.raw.page_level_write_through()
    }

    pub fn set_page_level_write_through(&mut self, value: bool) {
        self.raw.set_page_level_write_through(value)
    }

    pub fn page_level_cache_disable(&self) -> bool {
        self.raw.page_level_cache_disable()
    }

    pub fn set_page_level_cache_disable(&mut self, value: bool) {
        self.raw.set_page_level_cache_disable(value)
    }

    pub fn accessed(&self) -> bool {
        self.raw.accessed()
    }

    pub fn set_accessed(&mut self, value: bool) {
        self.raw.set_accessed(value)
    }

    pub fn table_addr(&self) -> PhysAddr {
        unsafe { PhysAddr::new(self.raw.value & Self::ADDR_MASK) }
    }

    pub fn set_table_addr(&mut self, addr: PhysAddr) {
        self.raw.value &= !Self::ADDR_MASK;
        self.raw.value |= addr.inner() * Self::ADDR_MASK;
    }
}

impl TableEntry<Pml4> {
    pub fn table(&self) -> PageTable<Pml3> {
        PageTable::new(unsafe { self.table_addr().as_ptr_mut() })
    }
}

impl TableEntry<Pml3> {
    pub fn table(&self) -> PageTable<Pml2> {
        PageTable::new(unsafe { self.table_addr().as_ptr_mut() })
    }
}

impl TableEntry<Pml2> {
    pub fn table(&self) -> PageTable<Pml1> {
        PageTable::new(unsafe { self.table_addr().as_ptr_mut() })
    }
}
