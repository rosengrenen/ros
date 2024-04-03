mod addr;
pub mod entry;

use core::fmt::Write;
use core::marker::PhantomData;

use common::addr::PhysAddr;
use common::addr::VirtAddr;
use common::frame::FrameAllocator;

use self::addr::VirtAddrExt;
use self::entry::PageTableEntry;
use crate::control::Cr0;
use crate::control::Cr3;

pub struct MappedPageTable<'a, M: PageTableFrameMapper> {
    level_4_table: &'a mut PageTable,
    table_walker: PageTableWalker<M>,
}

impl<'a, M: PageTableFrameMapper> MappedPageTable<'a, M> {
    pub fn new(level_4_table: &'a mut PageTable, page_table_frame_mapper: M) -> Self {
        Self {
            level_4_table,
            table_walker: PageTableWalker::new(page_table_frame_mapper),
        }
    }

    pub fn translate(&self, page: VirtAddr) -> Result<PhysAddr, ()> {
        let p4 = &self.level_4_table;
        let p4_entry = &p4[page.p4_index()];

        let p3 = self.table_walker.get_next_table(&p4[page.p4_index()])?;
        let p3_entry = &p3[page.p3_index()];
        if p3_entry.is_page() {
            return p3_entry
                .frame()
                .map(|base| base.add(page.as_u64() & 0x3FFF_FFFF));
        }

        let p2 = self.table_walker.get_next_table(&p3[page.p3_index()])?;
        let p2_entry = &p2[page.p2_index()];
        if p2_entry.is_page() {
            return p2_entry
                .frame()
                .map(|base| base.add(page.as_u64() & 0x1F_FFFF));
        }

        let p1 = self.table_walker.get_next_table(&p2[page.p2_index()])?;
        let p1_entry = &p1[page.p1_index()];

        p1_entry.frame().map(|base| base.add(page.as_u64() & 0xFFF))
    }

    pub fn map<F: FrameAllocator>(
        &mut self,
        page: VirtAddr,
        frame: PhysAddr,
        frame_allocator: &F,
        writeable: bool,
    ) -> Result<(), ()> {
        // TODO: check addr alignment, maybe wrap virt and phys addr in
        // page and frame type which are generic over sizes, which provide
        // alignment utils
        let p4 = &mut self.level_4_table;
        let p3 = self
            .table_walker
            .create_next_table(&mut p4[page.p4_index()], frame_allocator)?;
        let p2 = self
            .table_walker
            .create_next_table(&mut p3[page.p3_index()], frame_allocator)?;
        let p1 = self
            .table_walker
            .create_next_table(&mut p2[page.p2_index()], frame_allocator)?;

        let p1_entry = &mut p1[page.p1_index()];

        // TODO: present vs unused, maybe pages are tmp unloaded by data is retained in entry
        if p1_entry.is_present() {
            return Err(());
        }

        p1_entry.set_present(true);
        p1_entry.set_is_page(true);
        p1_entry.set_writable(writeable);
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
        let p4 = &mut self.level_4_table;
        let p3 = self
            .table_walker
            .create_next_table(&mut p4[page.p4_index()], frame_allocator)?;

        let p3_entry = &mut p3[page.p3_index()];
        if p3_entry.is_present() {
            return Err(());
        }

        p3_entry.set_present(true);
        p3_entry.set_is_page(true);
        p3_entry.set_writable(true);
        p3_entry.set_frame(frame);
        Ok(())
    }

    pub fn map_ident<F: FrameAllocator>(
        &mut self,
        page: VirtAddr,
        frame_allocator: &F,
    ) -> Result<(), ()> {
        // TODO: prevent this kind of virt -> phys transformation, might be dangerous
        self.map(page, PhysAddr::new(page.as_u64()), frame_allocator, false)
    }

    pub fn unmap(
        &mut self,
        page: VirtAddr,
        frame_allocator: &impl FrameAllocator,
    ) -> Result<PhysAddr, ()> {
        // TODO: check addr alignment
        let p4 = &mut self.level_4_table;
        let p3 = self
            .table_walker
            .get_next_table_mut(&mut p4[page.p4_index()])?;
        let p2 = self
            .table_walker
            .get_next_table_mut(&mut p3[page.p3_index()])?;
        let p1 = self
            .table_walker
            .get_next_table_mut(&mut p2[page.p2_index()])?;

        let p1_entry = &mut p1[page.p1_index()];

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
        let p4 = &mut self.level_4_table;
        let p3 = self
            .table_walker
            .get_next_table_mut(&mut p4[page.p4_index()])?;

        let p3_entry = &mut p3[page.p1_index()];

        // TODO: present vs unused, maybe pages are tmp unloaded by data is retained in entry
        if !p3_entry.is_present() {
            return Err(());
        }

        let frame = p3_entry.frame()?;
        *p3_entry = PageTableEntry::empty();
        Ok(frame)
    }
}

const NUM_ENTRIES: usize = 512;

// TODO: synchronisation
#[derive(Clone, Copy, Debug)]
#[repr(C, align(4096))]
pub struct PageTable {
    pub entries: [PageTableEntry; NUM_ENTRIES],
}

impl core::ops::Index<usize> for PageTable {
    type Output = PageTableEntry;

    fn index(&self, index: usize) -> &Self::Output {
        &self.entries[index]
    }
}

impl core::ops::IndexMut<usize> for PageTable {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.entries[index]
    }
}

struct PageTableWalker<M: PageTableFrameMapper> {
    page_table_frame_mapper: M,
}

impl<M: PageTableFrameMapper> PageTableWalker<M> {
    fn new(page_table_frame_mapper: M) -> Self {
        Self {
            page_table_frame_mapper,
        }
    }

    fn get_next_table<'a>(&self, entry: &'a PageTableEntry) -> Result<&'a PageTable, ()> {
        if !entry.is_present() {
            return Err(());
        }

        Ok(unsafe {
            &*self
                .page_table_frame_mapper
                .frame_to_page(entry.addr())
                .as_ptr()
        })
    }

    fn get_next_table_mut<'a>(
        &mut self,
        entry: &'a PageTableEntry,
    ) -> Result<&'a mut PageTable, ()> {
        if !entry.is_present() {
            return Err(());
        }

        Ok(unsafe {
            &mut *self
                .page_table_frame_mapper
                .frame_to_page(entry.addr())
                .as_ptr_mut()
        })
    }

    fn create_next_table<'a, F: FrameAllocator>(
        &mut self,
        entry: &'a mut PageTableEntry,
        frame_allocator: F,
    ) -> Result<&'a mut PageTable, ()> {
        let frame = if !entry.is_present() {
            // TODO: make sure the frame is zerod
            let frame = frame_allocator.allocate_frame().unwrap();
            *entry = PageTableEntry::empty();
            entry.set_present(true);
            entry.set_is_page(false);
            entry.set_writable(true);
            entry.set_addr(frame);
            entry.addr()
        } else {
            entry.addr()
        };

        // TODO: there is no validation that the addr is virt before creating page table ref
        Ok(unsafe {
            &mut *self
                .page_table_frame_mapper
                .frame_to_page(frame)
                .as_ptr_mut()
        })
    }
}

pub trait PageTableFrameMapper {
    fn frame_to_page(&self, frame: PhysAddr) -> VirtAddr;
}

#[derive(Clone, Copy, Default)]
pub struct PageTableFrameOffsetMapper {
    offset: u64,
}

impl PageTableFrameOffsetMapper {
    pub const fn new(offset: u64) -> Self {
        Self { offset }
    }
}

impl PageTableFrameMapper for PageTableFrameOffsetMapper {
    fn frame_to_page(&self, frame: PhysAddr) -> VirtAddr {
        VirtAddr::new(frame.as_u64() + self.offset)
    }
}
