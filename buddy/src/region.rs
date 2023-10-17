use crate::{layered_bitmap::BuddyBitmap, layout::RegionLayout};
use alloc::raw_vec::RawVec;
use core::fmt;
use std::ptr::NonNull;

#[derive(Debug)]
pub struct Region<const ORDER: usize, const FRAME_SIZE: usize> {
    next_region: Option<NonNull<Region<ORDER, FRAME_SIZE>>>,
    pub usable_frames_base: usize,
    pub usable_frames: usize,
    bitmaps: RawVec<BuddyBitmap>,
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
        let max_order: usize = ((max_usable_frames as f64).log2().floor() as usize).min(ORDER - 1);
        let layout =
            RegionLayout::<ORDER>::new(FRAME_SIZE, frames, max_usable_frames, max_order).unwrap();
        let bitmaps = Self::create_bitmaps(base, layout, max_order);
        let usable_frames_base = base + layout.usable_base_offset;
        Self {
            next_region,
            usable_frames_base,
            usable_frames: layout.usable_frames,
            bitmaps,
        }
    }

    pub fn allocate(&mut self, order: usize) -> Option<usize> {
        if order >= self.bitmaps.len() {
            return None;
        }

        // 1. Try to allocate from exact order
        let bitmap = self.bitmap_mut(order);
        for index in 0..bitmap.len() {
            if bitmap.get_free_bit(index) {
                bitmap.clear_free_bit(index);
                bitmap.set_alloc_bit(index);
                return Some(self.addr_from_order_and_index(order, index));
            }
        }

        // 2. If no match was found, search upward and split
        for cur_order in order + 1..=self.bitmaps.len() {
            for index in 0..self.bitmap(cur_order).len() {
                if self.bitmap(cur_order).get_free_bit(index) {
                    let index = self.split(cur_order, index, order);
                    let bitmap = self.bitmap_mut(order);
                    bitmap.set_alloc_bit(index);
                    bitmap.clear_free_bit(index);
                    return Some(self.addr_from_order_and_index(order, index));
                }
            }
        }

        None
    }

    pub fn deallocate(&mut self, addr: usize) {
        let (order, index) = self.order_and_index_from_addr(addr);
        let bitmap = self.bitmap_mut(order);
        bitmap.set_free_bit(index);
        bitmap.clear_alloc_bit(index);
        self.merge(order, index);
    }

    fn split(&mut self, order: usize, mut index: usize, target_order: usize) -> usize {
        let bitmap = self.bitmap_mut(order);
        bitmap.clear_free_bit(index);
        index *= 2;
        for order in (0..order).rev() {
            let bitmap = self.bitmap_mut(order);
            bitmap.set_free_bit(index + 1);
            if order == target_order {
                bitmap.set_free_bit(index);
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

    fn order_and_index_from_addr(&self, addr: usize) -> (usize, usize) {
        let mut index = (addr - self.usable_frames_base) / FRAME_SIZE;
        let mut order = 0;
        while order < self.bitmaps.len() {
            if self.bitmap(order).get_alloc_bit(index) {
                return (order, index);
            }

            index /= 2;
            order += 1;
        }

        panic!("that's an odd address");
    }

    fn create_bitmaps(
        base: usize,
        layout: RegionLayout<ORDER>,
        max_order: usize,
    ) -> RawVec<BuddyBitmap> {
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

        let mut index = 0;
        for order in (0..max_order).rev() {
            let bitmap = &mut bitmaps[order];
            for index in index..bitmap.len() {
                bitmap.set_free_bit(index);
            }

            index = bitmap.len() * 2;
        }

        bitmaps
    }
}

impl<const ORDER: usize, const FRAME_SIZE: usize> fmt::Display for Region<ORDER, FRAME_SIZE> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for order in 0..self.bitmaps.len() {
            let bitmap = self.bitmap(order);
            write!(f, "bitmap {}:", order)?;
            for index in 0..bitmap.len() {
                write!(f, " {}", if bitmap.get_free_bit(index) { '#' } else { '.' })?;
                if order > 0 {
                    for _ in 0..2usize.pow(order as u32) - 1 {
                        write!(f, "  ")?;
                    }
                }
            }

            writeln!(f)?;
        }

        Ok(())
    }
}
