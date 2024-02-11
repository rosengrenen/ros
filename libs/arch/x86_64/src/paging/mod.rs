mod addr;
pub mod entry;

use self::{
    addr::{translate_hhdm, VirtAddrExt},
    entry::PageTableEntry,
};
use crate::control::{Cr0, Cr3};
use common::{
    addr::{PhysAddr, VirtAddr},
    frame::FrameAllocator,
};
use core::{fmt::Write, marker::PhantomData};

const NUM_ENTRIES: usize = 512;

// TODO: synchronisation
#[derive(Clone, Copy, Debug)]
#[repr(C, align(4096))]
pub struct PageTable {
    entries: [PageTableEntry; NUM_ENTRIES],
}

impl PageTable {
    fn get_next_table<'a>(entry: &'a PageTableEntry) -> Result<&'a PageTable, ()> {
        if !entry.is_present() {
            return Err(());
        }

        Ok(unsafe { &*translate_hhdm(entry.addr()).as_ptr() })
    }

    fn get_next_table_mut<'a>(entry: &'a mut PageTableEntry) -> Result<&'a mut PageTable, ()> {
        if !entry.is_present() {
            return Err(());
        }

        Ok(unsafe { &mut *translate_hhdm(entry.addr()).as_ptr_mut() })
    }

    fn create_next_table<'a, F: FrameAllocator>(
        entry: &'a mut PageTableEntry,
        frame_allocator: F,
    ) -> Result<&'a mut PageTable, ()> {
        let addr = if !entry.is_present() {
            // TODO: make sure the frame is zerod
            let frame = frame_allocator.allocate_frame().unwrap();
            *entry = PageTableEntry::empty();
            entry.set_present(true);
            entry.set_is_page(false);
            entry.set_addr(frame);
            entry.addr()
        } else {
            entry.addr()
        };

        // TODO: there is no validation that the addr is virt before creating page table ref
        Ok(unsafe { &mut *translate_hhdm(addr).as_ptr_mut() })
    }

    pub fn translate(&self, page: VirtAddr) -> Result<PhysAddr, ()> {
        let p4 = self;
        let p3 = Self::get_next_table(&p4.entries[page.p4_index()])?;
        let p2 = Self::get_next_table(&p3.entries[page.p3_index()])?;
        let p1 = Self::get_next_table(&p2.entries[page.p2_index()])?;

        let p1_entry = &p1.entries[page.p1_index()];

        p1_entry.frame()
    }

    pub fn map<F: FrameAllocator>(
        &mut self,
        page: VirtAddr,
        frame: PhysAddr,
        frame_allocator: &F,
    ) -> Result<(), ()> {
        // TODO: check addr alignment, maybe wrap virt and phys addr in
        // page and frame type which are generic over sizes, which provide
        // alignment utils
        let p4 = self;
        let p3 = Self::create_next_table(&mut p4.entries[page.p4_index()], frame_allocator)?;
        let p2 = Self::create_next_table(&mut p3.entries[page.p3_index()], frame_allocator)?;
        let p1 = Self::create_next_table(&mut p2.entries[page.p2_index()], frame_allocator)?;

        let p1_entry = &mut p1.entries[page.p1_index()];

        // TODO: present vs unused, maybe pages are tmp unloaded by data is retained in entry
        if p1_entry.is_present() {
            return Err(());
        }

        p1_entry.set_present(true);
        p1_entry.set_is_page(true);
        p1_entry.set_frame(frame);
        Ok(())
    }

    pub fn map_1gb<F: FrameAllocator>(
        &mut self,
        page: VirtAddr,
        frame: PhysAddr,
        frame_allocator: &F,
    ) -> Result<(), ()> {
        // TODO: check addr alignment
        let p4 = self;
        let p3 = Self::create_next_table(&mut p4.entries[page.p4_index()], frame_allocator)?;

        let p3_entry = &mut p3.entries[page.p3_index()];

        if p3_entry.is_present() {
            return Err(());
        }

        p3_entry.set_present(true);
        p3_entry.set_is_page(true);
        p3_entry.set_frame(frame);
        Ok(())
    }

    pub fn map_ident<F: FrameAllocator>(
        &mut self,
        page: VirtAddr,
        frame_allocator: &F,
    ) -> Result<(), ()> {
        // TODO: prevent this kind of virt -> phys transformation, might be dangerous
        self.map(page, PhysAddr::new(page.as_u64()), frame_allocator)
    }

    pub fn unmap(
        &mut self,
        page: VirtAddr,
        frame_allocator: &impl FrameAllocator,
    ) -> Result<PhysAddr, ()> {
        // TODO: check addr alignment
        let p4 = self;
        let p3 = Self::get_next_table_mut(&mut p4.entries[page.p4_index()])?;
        let p2 = Self::get_next_table_mut(&mut p3.entries[page.p3_index()])?;
        let p1 = Self::get_next_table_mut(&mut p2.entries[page.p2_index()])?;

        let p1_entry = &mut p1.entries[page.p1_index()];

        // TODO: present vs unused, maybe pages are tmp unloaded by data is retained in entry
        if !p1_entry.is_present() {
            return Err(());
        }

        let frame = p1_entry.frame()?;
        *p1_entry = PageTableEntry::empty();
        Ok(frame)
    }

    pub fn unmap_1gb(
        &mut self,
        page: VirtAddr,
        frame_allocator: &impl FrameAllocator,
    ) -> Result<PhysAddr, ()> {
        // TODO: check addr alignment
        let p4 = self;
        let p3 = Self::get_next_table_mut(&mut p4.entries[page.p4_index()])?;

        let p3_entry = &mut p3.entries[page.p1_index()];

        // TODO: present vs unused, maybe pages are tmp unloaded by data is retained in entry
        if !p3_entry.is_present() {
            return Err(());
        }

        let frame = p3_entry.frame()?;
        *p3_entry = PageTableEntry::empty();
        Ok(frame)
    }
}
