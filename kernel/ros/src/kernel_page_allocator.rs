use crate::{bitmap::Bitmap, spinlock::Mutex};
use x86_64::paging::{FrameAllocator, PageTable, PhysAddr, Pml4, VirtAddr};

pub struct KernelPageAllocator {
    inner: Mutex<KernelPageAllocatorInner>,
}

struct KernelPageAllocatorInner {
    bitmap: Bitmap,
    start_addr: u64,
}

// TODO: buddy allocation
impl KernelPageAllocator {
    pub fn new(
        mut kernel_end: u64,
        heap_size_bytes: u64,
        frame_allocator: &impl FrameAllocator,
        mut page_table: PageTable<Pml4>,
    ) -> Self {
        kernel_end = (kernel_end + 4096) & !0xfff;
        let num_pages = (heap_size_bytes + 4095) / 4096;
        let frames = (num_pages + 4095) / 4096;
        for frame in 0..frames {
            let frame_base = frame_allocator.allocate_frame().unwrap();
            page_table.map(
                VirtAddr::new(kernel_end + 4096 * frame),
                PhysAddr::new(frame_base),
                frame_allocator,
            );
        }

        let bitmap = unsafe { Bitmap::from_raw_parts(kernel_end as *mut u64, num_pages as usize) };

        Self {
            inner: Mutex::new(KernelPageAllocatorInner {
                bitmap,
                start_addr: kernel_end + frames * 4096,
            }),
        }
    }

    pub fn allocate_pages(&self, pages: usize) -> u64 {
        assert!(pages > 0);
        let mut inner = self.inner.lock();

        // Linear search for empty page
        let bitmap = &mut inner.bitmap;
        let mut outer_index = 0;
        while outer_index + pages < bitmap.len() {
            if bitmap.get_bit(outer_index) {
                outer_index += 1;
                continue;
            }

            // Now we have one page that is free
            // Check the following "pages - 1" entries to see if they're also free
            // If not, we fast forward
            'inner: for inner_index in 1..pages {
                if bitmap.get_bit(outer_index + inner_index) {
                    outer_index += inner_index + 1;
                    break 'inner;
                }
            }

            // Now we know that there are "pages" free entries, mark them as occupied
            // and return address
            for inner_index in 0..pages {
                bitmap.set_bit(outer_index + inner_index);
            }

            return inner.start_addr + outer_index as u64 * 4096;
        }

        panic!("no pages left")
    }
}
