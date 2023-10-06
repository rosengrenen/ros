use crate::{spinlock::Mutex, sprintln};
use alloc::raw_vec::RawVec;
use x86_64::paging::{FrameAllocator, PageTable, PhysAddr, Pml4, VirtAddr};

pub struct KernelPageAllocator {
    inner: Mutex<KernelPageAllocatorInner>,
}

struct KernelPageAllocatorInner {
    bitmap: RawVec<u64>,
    page_table: PageTable<Pml4>,
    start_addr: u64,
}

impl KernelPageAllocator {
    pub fn new(
        mut kernel_end: u64,
        heap_size_bytes: u64,
        frame_allocator: &impl FrameAllocator,
        mut page_table: PageTable<Pml4>,
    ) -> Self {
        kernel_end = (kernel_end + 4096) & !0xfff;
        let frames = heap_size_bytes / 4096 / 8 / 4096;
        for frame in 0..frames {
            let frame_base = frame_allocator.allocate_frame().unwrap();
            sprintln!("{:x?}", frame_base);
            page_table.map(
                VirtAddr::new(kernel_end + 4096 * frame),
                PhysAddr::new(frame_base),
                frame_allocator,
            );
        }
        loop {}

        let bitmap = unsafe {
            RawVec::from_raw_parts(
                kernel_end as *mut u64,
                frames as usize * 4096 / core::mem::size_of::<u64>(),
            )
        };
        Self {
            inner: Mutex::new(KernelPageAllocatorInner {
                bitmap,
                page_table,
                start_addr: kernel_end + frames as u64 * 4096,
            }),
        }
    }

    pub fn allocate_pages(&self, pages: usize) -> u64 {
        assert!(pages > 0);
        let mut inner = self.inner.lock();

        // Linear search for empty page
        let mask = (1 << pages) - 1;
        for (map_index, entry) in inner
            .bitmap
            .iter_mut()
            .enumerate()
            .filter(|(_, a)| **a != 0xffff_ffff_ffff_ffff)
        {
            for entry_index in 0..=64 - pages {
                if (*entry >> entry_index) & mask == 0 {
                    *entry |= mask << entry_index;
                    return inner.start_addr + (map_index * 64 + entry_index) as u64 * 4096;
                }
            }
        }

        panic!("no pages left")
    }
}