mod addr;
pub mod entry;

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
// TODO: the alignment shouldn't make any difference for now, but can't be created on the stack if alignment is that high
// #[repr(C, align(4096))]
#[repr(C)]
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
    ) -> GetOrCreate<S> {
        if let Some(entry) = self.get(index) {
            GetOrCreate::Found(entry)
        } else {
            let frame = frame_allocator.allocate_frame().unwrap();
            for a in unsafe {
                core::slice::from_raw_parts_mut(
                    frame as *mut u64,
                    4096 / core::mem::size_of::<u64>(),
                )
            } {
                *a = 0;
            }
            // TODO: clear the thing, needs to be mapped tho :(
            let mut table_entry = TableEntry::new();
            table_entry.set_writable(true);
            table_entry.set_table_addr(PhysAddr::new(frame));
            self.entries_mut()[index as usize] = table_entry.raw();
            GetOrCreate::Created(table_entry)
        }
    }

    pub fn iter(&self) -> Iter<'_, S> {
        Iter {
            table: self,
            index: 0,
        }
    }
}

pub enum GetOrCreate<S> {
    Found(PageTableEntry<S>),
    Created(TableEntry<S>),
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
        let mut pml3 = match self.get_or_create(pml4_index, frame_allocator) {
            GetOrCreate::Found(entry) => match entry {
                PageTableEntry::Page(page) => unreachable!(),
                PageTableEntry::Table(table) => table.table(),
            },
            GetOrCreate::Created(entry) => entry.table(),
        };

        let pml3_index = virt_addr.pml3_index();
        let mut pml2 = match pml3.get_or_create(pml3_index, frame_allocator) {
            GetOrCreate::Found(entry) => match entry {
                PageTableEntry::Page(page) => return false,
                PageTableEntry::Table(table) => table.table(),
            },
            GetOrCreate::Created(entry) => entry.table(),
        };

        let pml2_index = virt_addr.pml2_index();
        let mut pml1 = match pml2.get_or_create(pml2_index, frame_allocator) {
            GetOrCreate::Found(entry) => match entry {
                PageTableEntry::Page(page) => return false,
                PageTableEntry::Table(table) => table.table(),
            },
            GetOrCreate::Created(entry) => entry.table(),
        };

        let pml1_index = virt_addr.pml1_index();
        if let Some(entry) = pml1.get(pml1_index) {
            false
        } else {
            let mut page_entry = PageEntry::<Pml1>::new();
            page_entry.set_writable(true);
            page_entry.set_frame(Frame4KiB::new(phys_addr));
            pml1.entries_mut()[pml1_index as usize] = page_entry.raw();
            true
        }
    }

    pub fn map_1gb(
        &mut self,
        virt_addr: VirtAddr,
        phys_addr: PhysAddr,
        frame_allocator: &impl FrameAllocator,
    ) -> bool {
        assert!(virt_addr.inner() & 0x3fff_ffff == 0);
        assert!(phys_addr.inner() & 0x3fff_ffff == 0);

        let pml4_index = virt_addr.pml4_index();
        let mut pml3 = match self.get_or_create(pml4_index, frame_allocator) {
            GetOrCreate::Found(entry) => match entry {
                PageTableEntry::Page(page) => unreachable!(),
                PageTableEntry::Table(table) => table.table(),
            },
            GetOrCreate::Created(entry) => entry.table(),
        };

        let pml3_index = virt_addr.pml3_index();
        if let Some(entry) = pml3.get(pml3_index) {
            false
        } else {
            let mut page_entry = PageEntry::<Pml3>::new();
            page_entry.set_writable(true);
            page_entry.set_frame(Frame1GiB::new(phys_addr));
            pml3.entries_mut()[pml3_index as usize] = page_entry.raw();
            true
        }
    }

    pub fn map_ident(
        &mut self,
        phys_addr: PhysAddr,
        frame_allocator: &impl FrameAllocator,
    ) -> bool {
        self.map(VirtAddr::new(phys_addr.inner()), phys_addr, frame_allocator)
    }

    pub fn unmap(
        &mut self,
        virt_addr: VirtAddr,
        frame_allocator: &impl FrameAllocator,
    ) -> Result<PhysAddr, ()> {
        assert!(virt_addr.inner() & 0xfff == 0);
        let pml4_index = virt_addr.pml4_index();
        let mut pml3 = match self.get(pml4_index) {
            Some(entry) => match entry {
                PageTableEntry::Page(page) => unreachable!(),
                PageTableEntry::Table(table) => table.table(),
            },
            None => return Err(()),
        };

        let pml3_index = virt_addr.pml3_index();
        let mut pml2 = match pml3.get(pml3_index) {
            Some(entry) => match entry {
                PageTableEntry::Page(page) => return Err(()),
                PageTableEntry::Table(table) => table.table(),
            },
            None => return Err(()),
        };

        let pml2_index = virt_addr.pml2_index();
        let mut pml1 = match pml2.get(pml2_index) {
            Some(entry) => match entry {
                PageTableEntry::Page(page) => return Err(()),
                PageTableEntry::Table(table) => table.table(),
            },
            None => return Err(()),
        };

        let pml1_index = virt_addr.pml1_index();
        if let Some(entry) = pml1.get(pml1_index) {
            let frame = match entry {
                PageTableEntry::Page(page) => page.frame(),
                PageTableEntry::Table(_) => unreachable!(),
            };
            pml1.entries_mut()[pml1_index as usize] = PageTableEntryRaw::empty();
            // TODO: check if page tables are empty and need to be removed
            Ok(frame.addr())
        } else {
            Err(())
        }
    }

    pub fn unmap_1gb(
        &mut self,
        virt_addr: VirtAddr,
        frame_allocator: &impl FrameAllocator,
    ) -> Result<PhysAddr, ()> {
        assert!(virt_addr.inner() & 0x3fff_ffff == 0);
        let pml4_index = virt_addr.pml4_index();
        let mut pml3 = match self.get(pml4_index) {
            Some(entry) => match entry {
                PageTableEntry::Page(page) => unreachable!(),
                PageTableEntry::Table(table) => table.table(),
            },
            None => return Err(()),
        };

        let pml3_index = virt_addr.pml3_index();
        if let Some(entry) = pml3.get(pml3_index) {
            let frame = match entry {
                PageTableEntry::Page(page) => page.frame(),
                PageTableEntry::Table(_) => unreachable!(),
            };
            pml3.entries_mut()[pml3_index as usize] = PageTableEntryRaw::empty();
            // TODO: check if page tables are empty and need to be removed
            Ok(frame.addr())
        } else {
            Err(())
        }
    }
}

pub struct Iter<'iter, S> {
    table: &'iter PageTable<S>,
    index: u64,
}

impl<'iter, S> Iterator for Iter<'iter, S> {
    type Item = (usize, PageTableEntry<S>);

    fn next(&mut self) -> Option<Self::Item> {
        while self.index < 512 {
            let cur_index = self.index;
            self.index += 1;
            if let Some(entry) = self.table.get(cur_index) {
                return Some((cur_index as usize, entry));
            }
        }

        None
    }
}
