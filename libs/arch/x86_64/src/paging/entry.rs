use core::{fmt::Debug, marker::PhantomData};

use crate::frame::{Frame1GiB, Frame2MiB, Frame4KiB};

use super::{PageTable, PhysAddr, Pml1, Pml2, Pml3, Pml4};

#[derive(Debug)]
#[repr(C)]
pub struct PageTableEntryRaw<S> {
    pub value: u64,
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
    const PRESENT_BIT: u64 = 0;
    const WRITABLE_BIT: u64 = 1;
    const USER_ACCESIBLE_BIT: u64 = 2;
    const PAGE_LEVEL_WRITE_THROUGH_BIT: u64 = 3;
    const PAGE_LEVEL_CACHE_BIT: u64 = 4;
    const ACCESSED_BIT: u64 = 5;
    const DIRTY_BIT: u64 = 6;
    const IS_PAGE_BIT: u64 = 6;

    pub fn new() -> Self {
        Self {
            value: 0,
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
        let bits = bits as u64;
        self.value &= !(0b11 << 9);
        self.value |= (bits & 0b11) << 9;
        self.value &= !(0b111_1111 << 52);
        self.value |= ((bits >> 2) & 0b111_1111) << 52;
    }

    fn set_bit(&mut self, bit: u64, value: bool) {
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

impl<S> Debug for PageTableEntry<S>
where
    PageEntry<S>: Debug,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Page(page) => f.debug_tuple("Page").field(page).finish(),
            Self::Table(table) => f.debug_tuple("Table").field(table).finish(),
        }
    }
}

#[derive(Clone, Copy)]
pub struct PageEntry<S> {
    raw: PageTableEntryRaw<S>,
}

impl<S> PageEntry<S> {
    pub fn new() -> Self {
        let mut raw = PageTableEntryRaw::new();
        raw.set_present(true);
        raw.set_is_page(true);
        Self { raw }
    }

    pub fn from_raw(raw: PageTableEntryRaw<S>) -> Self {
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

impl Debug for PageEntry<Pml4> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("PageEntry")
            .field("writable", &self.writable())
            .field("user_accessible", &self.user_accessible())
            .field("page_level_write_through", &self.page_level_write_through())
            .field("page_level_cache_disable", &self.page_level_cache_disable())
            .field("accessed", &self.accessed())
            .field("dirty", &self.dirty())
            .field("marker", &self.raw._marker)
            .finish()
    }
}

impl PageEntry<Pml3> {
    const ADDR_MASK: u64 = 0x000f_ffff_c000_0000;

    pub fn frame(&self) -> Frame1GiB {
        Frame1GiB::new(PhysAddr::new(self.raw.value & Self::ADDR_MASK))
    }

    pub fn set_frame(&mut self, frame: Frame1GiB) {
        self.raw.value &= !Self::ADDR_MASK;
        self.raw.value |= frame.addr().inner() & Self::ADDR_MASK;
    }
}

impl Debug for PageEntry<Pml3> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("PageEntry")
            .field("writable", &self.writable())
            .field("user_accessible", &self.user_accessible())
            .field("page_level_write_through", &self.page_level_write_through())
            .field("page_level_cache_disable", &self.page_level_cache_disable())
            .field("accessed", &self.accessed())
            .field("dirty", &self.dirty())
            .field("frame", &self.frame())
            .field("marker", &self.raw._marker)
            .finish()
    }
}

impl PageEntry<Pml2> {
    const ADDR_MASK: u64 = 0x000f_ffff_ffe0_0000;

    pub fn frame(&self) -> Frame2MiB {
        Frame2MiB::new(PhysAddr::new(self.raw.value & Self::ADDR_MASK))
    }

    pub fn set_frame(&mut self, frame: Frame2MiB) {
        self.raw.value &= !Self::ADDR_MASK;
        self.raw.value |= frame.addr().inner() & Self::ADDR_MASK;
    }
}

impl Debug for PageEntry<Pml2> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("PageEntry")
            .field("writable", &self.writable())
            .field("user_accessible", &self.user_accessible())
            .field("page_level_write_through", &self.page_level_write_through())
            .field("page_level_cache_disable", &self.page_level_cache_disable())
            .field("accessed", &self.accessed())
            .field("dirty", &self.dirty())
            .field("frame", &self.frame())
            .field("marker", &self.raw._marker)
            .finish()
    }
}

impl PageEntry<Pml1> {
    const ADDR_MASK: u64 = 0x000f_ffff_ffff_f000;

    pub fn frame(&self) -> Frame4KiB {
        Frame4KiB::new(PhysAddr::new(self.raw.value & Self::ADDR_MASK))
    }

    pub fn set_frame(&mut self, frame: Frame4KiB) {
        self.raw.value &= !Self::ADDR_MASK;
        self.raw.value |= frame.addr().inner() & Self::ADDR_MASK;
    }
}

impl Debug for PageEntry<Pml1> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("PageEntry")
            .field("writable", &self.writable())
            .field("user_accessible", &self.user_accessible())
            .field("page_level_write_through", &self.page_level_write_through())
            .field("page_level_cache_disable", &self.page_level_cache_disable())
            .field("accessed", &self.accessed())
            .field("dirty", &self.dirty())
            .field("frame", &self.frame())
            .field("marker", &self.raw._marker)
            .finish()
    }
}

#[derive(Clone, Copy)]
pub struct TableEntry<S> {
    raw: PageTableEntryRaw<S>,
}

impl<S> Debug for TableEntry<S> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("TableEntry")
            .field("writable", &self.writable())
            .field("user_accessible", &self.user_accessible())
            .field("page_level_write_through", &self.page_level_write_through())
            .field("page_level_cache_disable", &self.page_level_cache_disable())
            .field("accessed", &self.accessed())
            .field("addr", &self.table_addr())
            .field("marker", &self.raw._marker)
            .finish()
    }
}

impl<S> TableEntry<S> {
    const ADDR_MASK: u64 = 0x000f_ffff_ffff_f000;

    pub fn new() -> Self {
        let mut raw = PageTableEntryRaw::new();
        raw.set_present(true);
        raw.set_is_page(false);
        Self { raw }
    }

    pub fn from_raw(raw: PageTableEntryRaw<S>) -> Self {
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
        self.raw.value |= addr.inner() & Self::ADDR_MASK;
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
