use core::fmt::Debug;
use core::marker::PhantomData;

use super::PageTable;
use super::PhysAddr;

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct PageTableEntry(u64);

const MASK: u64 = 0x000f_ffff_ffff_f000;

impl PageTableEntry {
    const PRESENT_BIT: usize = 0;
    const WRITABLE_BIT: usize = 1;
    const USER_ACCESIBLE_BIT: usize = 2;
    const PAGE_LEVEL_WRITE_THROUGH_BIT: usize = 3;
    const PAGE_LEVEL_CACHE_BIT: usize = 4;
    const ACCESSED_BIT: usize = 5;
    const DIRTY_BIT: usize = 6;
    const IS_PAGE_BIT: usize = 7;

    pub fn empty() -> Self {
        Self(0)
    }

    pub fn addr(&self) -> PhysAddr {
        PhysAddr::new(self.0 & MASK)
    }

    pub fn set_addr(&mut self, addr: PhysAddr) {
        self.0 &= !MASK;
        self.0 |= addr.as_u64() & MASK;
    }

    pub fn frame(&self) -> Result<PhysAddr, ()> {
        if !self.is_present() {
            return Err(());
        }

        Ok(PhysAddr::new(self.0 & MASK))
    }

    pub fn set_frame(&mut self, addr: PhysAddr) {
        self.0 &= !MASK;
        self.0 |= addr.as_u64() & MASK;
    }

    pub fn is_present(&self) -> bool {
        self.0 & (1 << Self::PRESENT_BIT) != 0
    }

    pub fn set_present(&mut self, value: bool) {
        self.set_bit(Self::PRESENT_BIT, value)
    }

    pub fn writable(&self) -> bool {
        self.0 & (1 << Self::WRITABLE_BIT) != 0
    }

    pub fn set_writable(&mut self, value: bool) {
        self.set_bit(Self::WRITABLE_BIT, value)
    }

    pub fn user_accessible(&self) -> bool {
        self.0 & (1 << Self::USER_ACCESIBLE_BIT) != 0
    }

    pub fn set_user_accessible(&mut self, value: bool) {
        self.set_bit(Self::USER_ACCESIBLE_BIT, value)
    }

    pub fn page_level_write_through(&self) -> bool {
        self.0 & (1 << Self::PAGE_LEVEL_WRITE_THROUGH_BIT) != 0
    }

    pub fn set_page_level_write_through(&mut self, value: bool) {
        self.set_bit(Self::PAGE_LEVEL_WRITE_THROUGH_BIT, value)
    }

    pub fn page_level_cache_disable(&self) -> bool {
        self.0 & (1 << Self::PAGE_LEVEL_CACHE_BIT) != 0
    }

    pub fn set_page_level_cache_disable(&mut self, value: bool) {
        self.set_bit(Self::PAGE_LEVEL_CACHE_BIT, value)
    }

    pub fn accessed(&self) -> bool {
        self.0 & (1 << Self::ACCESSED_BIT) != 0
    }

    pub fn set_accessed(&mut self, value: bool) {
        self.set_bit(Self::ACCESSED_BIT, value)
    }

    pub fn dirty(&self) -> bool {
        self.0 & (1 << Self::DIRTY_BIT) != 0
    }

    pub fn set_dirty(&mut self, value: bool) {
        self.set_bit(Self::DIRTY_BIT, value)
    }

    pub fn is_page(&self) -> bool {
        self.0 & (1 << Self::IS_PAGE_BIT) != 0
    }

    pub fn set_is_page(&mut self, value: bool) {
        self.set_bit(Self::IS_PAGE_BIT, value)
    }

    pub fn user_bits(&self) -> u16 {
        // Bits 9:10
        let low = (self.0 >> 9) & 0b11;
        // Bits 52:58
        let high = (self.0 >> 52) & 0b111_1111;
        ((high << 2) | low) as u16
    }

    pub fn set_user_bits(&mut self, bits: u16) {
        let bits = bits as u64;
        self.0 &= !(0b11 << 9);
        self.0 |= (bits & 0b11) << 9;
        self.0 &= !(0b111_1111 << 52);
        self.0 |= ((bits >> 2) & 0b111_1111) << 52;
    }

    fn set_bit(&mut self, bit: usize, value: bool) {
        if value {
            self.0 |= (1 << bit);
        } else {
            self.0 &= !(1 << bit);
        }
    }
}
