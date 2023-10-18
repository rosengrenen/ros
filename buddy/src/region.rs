use crate::{layered_bitmap::BuddyBitmap, layout::RegionLayout, util::ilog_ceil};
use alloc::raw_vec::RawVec;

#[derive(Debug)]
pub struct Region<const ORDERS: usize, const FRAME_SIZE: usize> {
    pub usable_frames_base: usize,
    pub usable_frames: usize,
    pub counts: [usize; ORDERS],
    bitmaps: RawVec<BuddyBitmap>,
}

pub struct RegionAllocation {
    pub addr: usize,
    pub allocated_order: usize,
    pub split_order: usize,
}

pub struct RegionDeallocation {
    pub deallocated_order: usize,
    pub merge_order: usize,
}

impl<const ORDERS: usize, const FRAME_SIZE: usize> Region<ORDERS, FRAME_SIZE> {
    pub fn new(base: usize, frames: usize) -> Self {
        assert_eq!(base % FRAME_SIZE, 0);
        // At least one frame is going to be used for metadata
        let max_usable_frames = frames - 1;
        let max_order: usize = ilog_ceil(2, max_usable_frames).min(ORDERS - 1);
        let layout =
            RegionLayout::<ORDERS>::new(FRAME_SIZE, frames, max_usable_frames, max_order).unwrap();
        let (bitmaps, counts) = Self::create_bitmaps(base, layout, max_order);
        let usable_frames_base = base + layout.usable_base_offset;
        Self {
            usable_frames_base,
            usable_frames: layout.usable_frames,
            counts,
            bitmaps,
        }
    }

    pub fn allocate(&mut self, order: usize) -> Option<RegionAllocation> {
        if order >= self.bitmaps.len() {
            return None;
        }

        // 1. Try to allocate from exact order
        let bitmap = self.bitmap_mut(order);
        if let Some(index) = bitmap.find_first_free_index_h() {
            bitmap.clear_free_bit(index);
            bitmap.set_alloc_bit(index);
            self.counts[order] -= 1;
            return Some(RegionAllocation {
                addr: self.addr_from_order_and_index(order, index),
                allocated_order: order,
                split_order: order,
            });
        }

        // 2. If no match was found, search upward and split
        for cur_order in order + 1..=self.bitmaps.len() {
            if let Some(index) = self.bitmap(cur_order).find_first_free_index_h() {
                let index = self.split(cur_order, index, order);
                let bitmap = self.bitmap_mut(order);
                bitmap.set_alloc_bit(index);
                bitmap.clear_free_bit(index);
                self.counts[order] -= 1;
                return Some(RegionAllocation {
                    addr: self.addr_from_order_and_index(order, index),
                    allocated_order: order,
                    split_order: cur_order,
                });
            }
        }

        None
    }

    pub fn deallocate(&mut self, addr: usize) -> Option<RegionDeallocation> {
        let (order, index) = match self.order_and_index_from_addr(addr) {
            Some(res) => res,
            None => {
                // WARNING: deallocate invalid addr
                return None;
            }
        };
        let bitmap = self.bitmap_mut(order);
        bitmap.set_free_bit(index);
        bitmap.clear_alloc_bit(index);
        self.counts[order] += 1;
        Some(RegionDeallocation {
            deallocated_order: order,
            merge_order: self.merge(order, index),
        })
    }

    fn split(&mut self, order: usize, mut index: usize, target_order: usize) -> usize {
        let bitmap = self.bitmap_mut(order);
        bitmap.clear_free_bit(index);
        self.counts[order] -= 1;
        index *= 2;
        for order in (0..order).rev() {
            self.counts[order] += 1;
            let bitmap = self.bitmap_mut(order);
            bitmap.set_free_bit(index + 1);
            if order == target_order {
                bitmap.set_free_bit(index);
                self.counts[order] += 1;
                break;
            }

            index *= 2;
        }

        index
    }

    fn merge(&mut self, order: usize, mut index: usize) -> usize {
        for order in order..(self.bitmaps.len() - 1) {
            index &= !1;
            let buddy_index = index ^ 1;
            let bitmap = self.bitmap_mut(order);
            if let (Some(true), Some(true)) =
                (bitmap.get_free_bit(index), bitmap.get_free_bit(buddy_index))
            {
                bitmap.clear_free_bit(index);
                bitmap.clear_free_bit(buddy_index);
                self.bitmap_mut(order + 1).set_free_bit(index / 2);
                self.counts[order] -= 2;
                self.counts[order + 1] += 1;
                index /= 2;
            } else {
                return order;
            }
        }

        self.bitmaps.len() - 1
    }

    fn bitmap(&self, order: usize) -> &BuddyBitmap {
        &self.bitmaps[order]
    }

    fn bitmap_mut(&mut self, order: usize) -> &mut BuddyBitmap {
        &mut self.bitmaps[order]
    }

    fn addr_from_order_and_index(&self, order: usize, index: usize) -> usize {
        self.usable_frames_base + index * 2usize.pow(order as u32) * FRAME_SIZE
    }

    fn order_and_index_from_addr(&self, addr: usize) -> Option<(usize, usize)> {
        let mut index = (addr - self.usable_frames_base) / FRAME_SIZE;
        let mut order = 0;
        while order < self.bitmaps.len() {
            if let Some(true) = self.bitmap(order).get_alloc_bit(index) {
                return Some((order, index));
            }

            index /= 2;
            order += 1;
        }

        None
    }

    fn create_bitmaps(
        base: usize,
        layout: RegionLayout<ORDERS>,
        max_order: usize,
    ) -> (RawVec<BuddyBitmap>, [usize; ORDERS]) {
        let mut bitmaps = unsafe { RawVec::from_raw_parts(base as *mut BuddyBitmap, max_order) };
        for order in 0..max_order {
            let offset = layout.bitmap_offsets[order].unwrap();
            let bits = layout.usable_frames / 2usize.pow(order as u32);
            let bitmap = unsafe { BuddyBitmap::from_raw_parts((base + offset) as *mut _, bits) };
            bitmaps.push(bitmap).unwrap();
        }

        let mut counts = [0; ORDERS];
        let mut index = 0;
        for order in (0..max_order).rev() {
            let bitmap = &mut bitmaps[order];
            for index in index..bitmap.len() {
                bitmap.set_free_bit(index);
                counts[order] += 1;
            }

            index = bitmap.len() * 2;
        }

        (bitmaps, counts)
    }

    #[cfg(test)]
    pub fn print_free(&self) {
        for order in 0..self.bitmaps.len() {
            let bitmap = self.bitmap(order);
            print!("bitmap {}:", order);
            for index in 0..bitmap.len() {
                print!(
                    " {}",
                    if bitmap.get_free_bit(index).unwrap() {
                        '#'
                    } else {
                        '.'
                    }
                );
                if order > 0 {
                    for _ in 0..2usize.pow(order as u32) - 1 {
                        print!("  ");
                    }
                }
            }

            println!();
        }
    }

    #[cfg(test)]
    pub fn print_allocated(&self) {
        print!("allocate:");
        for i in 0..self.usable_frames {
            if self
                .order_and_index_from_addr(self.usable_frames_base + i * FRAME_SIZE)
                .is_some()
            {
                print!(" #");
            } else {
                print!(" .");
            }
        }

        println!();
    }
}
