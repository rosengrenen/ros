use crate::{layered_bitmap::BuddyBitmap, layout::RegionLayout, util::ilog_ceil};
use alloc::raw_vec::RawVec;
use core::ptr::NonNull;

#[derive(Debug)]
pub struct Region<const ORDER: usize, const FRAME_SIZE: usize> {
    pub next_region: Option<NonNull<Region<ORDER, FRAME_SIZE>>>,
    pub usable_frames_base: usize,
    pub usable_frames: usize,
    bitmaps: RawVec<BuddyBitmap>,
    counts: [usize; ORDER],
}

impl<const ORDER: usize, const FRAME_SIZE: usize> Region<ORDER, FRAME_SIZE> {
    pub fn new(
        next_region: Option<NonNull<Region<ORDER, FRAME_SIZE>>>,
        base: usize,
        frames: usize,
    ) -> Self {
        assert_eq!(base % FRAME_SIZE, 0);
        // At least one frame is going to be used for metadata
        let max_usable_frames = frames - 1;
        let mut max_order: usize = ilog_ceil(2, max_usable_frames).min(ORDER - 1);
        let layout =
            RegionLayout::<ORDER>::new(FRAME_SIZE, frames, max_usable_frames, max_order).unwrap();
        let (bitmaps, counts) = Self::create_bitmaps(base, layout, max_order);
        let usable_frames_base = base + layout.usable_base_offset;
        Self {
            next_region,
            usable_frames_base,
            usable_frames: layout.usable_frames,
            bitmaps,
            counts,
        }
    }

    pub fn allocate(&mut self, order: usize) -> Option<usize> {
        if order >= self.bitmaps.len() {
            return None;
        }

        // 1. Try to allocate from exact order
        let bitmap = self.bitmap_mut(order);
        if let Some(index) = bitmap.find_first_free_index_h() {
            bitmap.clear_free_bit(index);
            bitmap.set_alloc_bit(index);
            self.counts[order] -= 1;
            return Some(self.addr_from_order_and_index(order, index));
        }

        // 2. If no match was found, search upward and split
        for cur_order in order + 1..=self.bitmaps.len() {
            if let Some(index) = self.bitmap(cur_order).find_first_free_index_h() {
                let index = self.split(cur_order, index, order);
                let bitmap = self.bitmap_mut(order);
                bitmap.set_alloc_bit(index);
                bitmap.clear_free_bit(index);
                self.counts[order] -= 1;
                return Some(self.addr_from_order_and_index(order, index));
            }
        }

        None
    }

    pub fn deallocate(&mut self, addr: usize) {
        let (order, index) = match self.order_and_index_from_addr(addr) {
            Some(res) => res,
            None => {
                // WARNING: deallocate invalid addr
                return;
            }
        };
        let bitmap = self.bitmap_mut(order);
        bitmap.set_free_bit(index);
        bitmap.clear_alloc_bit(index);
        self.counts[order] += 1;
        self.merge(order, index);
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

    fn merge(&mut self, order: usize, mut index: usize) {
        for order in order..(self.bitmaps.len() - 1) {
            index &= !1;
            let buddy_index = index ^ 1;
            let bitmap = self.bitmap_mut(order);
            if let (Some(true), Some(true)) = (
                bitmap.get_free_bit_checked(index),
                bitmap.get_free_bit_checked(buddy_index),
            ) {
                bitmap.clear_free_bit(index);
                bitmap.clear_free_bit(buddy_index);
                self.bitmap_mut(order + 1).set_free_bit(index / 2);
                self.counts[order] -= 2;
                self.counts[order + 1] += 1;
                index /= 2;
            } else {
                break;
            }
        }
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
            if let Some(true) = self.bitmap(order).get_alloc_bit_checked(index) {
                return Some((order, index));
            }

            index /= 2;
            order += 1;
        }

        None
    }

    fn create_bitmaps(
        base: usize,
        layout: RegionLayout<ORDER>,
        max_order: usize,
    ) -> (RawVec<BuddyBitmap>, [usize; ORDER]) {
        let mut bitmaps = unsafe {
            RawVec::from_raw_parts(
                (base + layout.bitmaps_offset) as *mut BuddyBitmap,
                max_order,
            )
        };
        for order in 0..max_order {
            let offset = layout.bitmap_offsets[order].unwrap();
            let bits = layout.usable_frames / 2usize.pow(order as u32);
            let bitmap = unsafe { BuddyBitmap::from_raw_parts((base + offset) as *mut _, bits) };
            bitmaps.push(bitmap).unwrap();
        }

        let mut counts = [0; ORDER];
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
                print!(" {}", if bitmap.get_free_bit(index) { '#' } else { '.' });
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
