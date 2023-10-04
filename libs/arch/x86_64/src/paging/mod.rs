mod addr;
mod entry;

pub use self::addr::{PhysAddr, VirtAddr};

use self::entry::{PageTableEntry, PageTableEntryRaw};
use crate::{
    control::{Cr0, Cr3},
    frame::{Frame1GiB, Frame4KiB},
    paging::entry::{PageEntry, TableEntry},
};
use core::{fmt::Write, marker::PhantomData};

#[derive(Debug)]
pub struct FrameAllocError;

// TODO: is this going here?
pub trait FrameAllocator {
    fn allocate_frame(&self) -> Result<u64, FrameAllocError>;

    fn deallocate_frame(&self, frame: u64) -> Result<(), FrameAllocError>;
}

// Page map levels
#[derive(Clone, Copy, Debug)]
pub struct Pml4;
#[derive(Clone, Copy, Debug)]
pub struct Pml3;
#[derive(Clone, Copy, Debug)]
pub struct Pml2;
#[derive(Clone, Copy, Debug)]
pub struct Pml1;

// TODO: synchronisation

#[derive(Clone, Copy, Debug)]
#[repr(C, align(4096))]
pub struct PageTable<S> {
    pub entries: *mut PageTableEntryRaw<S>,
}

impl<S> PageTable<S> {
    pub fn new(base: *mut PageTableEntryRaw<S>) -> Self {
        Self { entries: base }
    }

    pub fn entries(&self) -> &[PageTableEntryRaw<S>] {
        unsafe { core::slice::from_raw_parts(self.entries, 512) }
    }

    pub fn entries_mut(&mut self) -> &mut [PageTableEntryRaw<S>] {
        unsafe { core::slice::from_raw_parts_mut(self.entries, 512) }
    }

    pub fn get(&self, index: u64) -> Option<PageTableEntry<S>> {
        assert!(index < 512);
        let entry = &self.entries()[index as usize];
        let entry = *entry;
        if !entry.present() {
            return None;
        }

        Some(match entry.is_page() {
            true => PageTableEntry::Page(PageEntry::from_raw(entry)),
            false => PageTableEntry::Table(TableEntry::from_raw(entry)),
        })
    }

    pub fn get_or_create(
        &mut self,
        index: u64,
        frame_allocator: &impl FrameAllocator,
    ) -> PageTableEntry<S> {
        if let Some(entry) = self.get(index) {
            entry
        } else {
            let frame = frame_allocator.allocate_frame().unwrap();
            let mut table_entry = TableEntry::new();
            table_entry.set_writable(true);
            table_entry.set_table_addr(PhysAddr::new(frame));
            self.entries_mut()[index as usize] = table_entry.raw();
            PageTableEntry::Table(table_entry)
        }
    }
}

impl PageTable<Pml4> {
    pub fn translate(&self, virt_addr: VirtAddr) -> Option<PhysAddr> {
        let pml4_index = virt_addr.pml4_index();
        let pml4_entry = self.get(pml4_index)?;
        let pml3 = match pml4_entry {
            PageTableEntry::Page(page) => unreachable!(),
            PageTableEntry::Table(table) => table.table(),
        };

        let pml3_index = virt_addr.pml3_index();
        let pml3_entry = pml3.get(pml3_index)?;
        let pml2 = match pml3_entry {
            PageTableEntry::Page(page) => {
                return Some(unsafe { page.frame().with_offset(virt_addr) })
            }
            PageTableEntry::Table(table) => table.table(),
        };

        let pml2_index = virt_addr.pml2_index();
        let pml2_entry = pml2.get(pml2_index)?;
        let pml1 = match pml2_entry {
            PageTableEntry::Page(page) => {
                return Some(unsafe { page.frame().with_offset(virt_addr) })
            }
            PageTableEntry::Table(table) => table.table(),
        };

        let pml1_index = virt_addr.pml1_index();
        let pml1_entry = pml1.get(pml1_index)?;
        match pml1_entry {
            PageTableEntry::Page(page) => Some(unsafe { page.frame().with_offset(virt_addr) }),
            PageTableEntry::Table(table) => unreachable!("{:?}", table),
        }
    }

    pub fn map(
        &mut self,
        virt_addr: VirtAddr,
        phys_addr: PhysAddr,
        frame_allocator: &impl FrameAllocator,
    ) -> bool {
        assert!(virt_addr.inner() & 0xfff == 0);
        assert!(phys_addr.inner() & 0xfff == 0);

        let pml4_index = virt_addr.pml4_index();
        let pml4_entry = self.get_or_create(pml4_index, frame_allocator);
        let mut pml3 = match pml4_entry {
            PageTableEntry::Page(page) => unreachable!(),
            PageTableEntry::Table(table) => table.table(),
        };

        let pml3_index = virt_addr.pml3_index();
        let pml3_entry = pml3.get_or_create(pml3_index, frame_allocator);
        let mut pml2 = match pml3_entry {
            PageTableEntry::Page(page) => return false,
            PageTableEntry::Table(table) => table.table(),
        };

        let pml2_index = virt_addr.pml2_index();
        let pml2_entry = pml2.get_or_create(pml2_index, frame_allocator);
        let mut pml1 = match pml2_entry {
            PageTableEntry::Page(_) => return false,
            PageTableEntry::Table(table) => table.table(),
        };

        let pml1_index = virt_addr.pml1_index();
        if let Some(entry) = pml1.get(pml1_index) {
            false
        } else {
            let frame = frame_allocator.allocate_frame().unwrap();
            let mut page_entry = PageEntry::<Pml1>::new();
            page_entry.set_writable(true);
            page_entry.set_frame(Frame4KiB::new(phys_addr));
            pml1.entries_mut()[pml1_index as usize] = page_entry.raw();
            true
        }
    }

    pub fn map_ident(
        &mut self,
        virt_addr: VirtAddr,
        frame_allocator: &impl FrameAllocator,
    ) -> bool {
        self.map(
            virt_addr,
            unsafe { PhysAddr::new(virt_addr.inner()) },
            frame_allocator,
        )
    }
}
