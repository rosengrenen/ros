// #![no_std]
#![deny(unsafe_op_in_unsafe_fn)]
#![feature(vec_into_raw_parts)]
#![feature(test)]
#![feature(pointer_byte_offsets)]

extern crate test;

mod bitmap;
mod layered_bitmap;
mod layout;

use self::{layered_bitmap::BuddyBitmap, layout::RegionLayout};
use alloc::raw_vec::RawVec;
use core::ptr::NonNull;

#[derive(Debug)]
#[repr(C)]
pub struct BuddyAllocator<const ORDER: usize, const FRAME_SIZE: usize> {
    regions_head: Option<NonNull<Region<ORDER, FRAME_SIZE>>>,
}

impl<const ORDER: usize, const FRAME_SIZE: usize> BuddyAllocator<ORDER, FRAME_SIZE> {
    pub fn new() -> Self {
        Self { regions_head: None }
    }

    pub fn add_region(&mut self, base: usize, frames: usize) {
        let region_ptr = unsafe { NonNull::new_unchecked(base as *mut Region<ORDER, FRAME_SIZE>) };
        let region = Region::new(self.regions_head, base, frames);
        unsafe {
            region_ptr.as_ptr().write(region);
        }
        self.regions_head = Some(region_ptr);
    }

    pub fn allocate(&mut self, order: usize) -> Result<usize, ()> {
        let region = self.regions_head;
        while let Some(mut region_ptr) = region {
            let region = unsafe { region_ptr.as_mut() };
            if let Some(addr) = region.allocate(order) {
                return Ok(addr);
            }
        }

        Err(())
    }

    pub fn deallocate(&mut self, addr: usize) {
        let region = self.regions_head;
        while let Some(mut region_ptr) = region {
            let region = unsafe { region_ptr.as_mut() };
            let region_start = region.usable_frames_base;
            let region_end = region_start + region.usable_frames * FRAME_SIZE;
            if (region_start..region_end).contains(&addr) {
                region.deallocate(addr);
                return;
            }
        }
    }
}

#[derive(Debug)]
#[repr(C)]
struct Region<const ORDER: usize, const FRAME_SIZE: usize> {
    next_region: Option<NonNull<Region<ORDER, FRAME_SIZE>>>,
    usable_frames_base: usize,
    usable_frames: usize,
    bitmaps: RawVec<BuddyBitmap>,
    max_order: usize,
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
            max_order,
        }
    }

    fn allocate(&mut self, order: usize) -> Option<usize> {
        if order >= self.max_order {
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
        for cur_order in order + 1..=self.max_order {
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

    fn deallocate(&mut self, addr: usize) {
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
        for order in order..(self.max_order - 1) {
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
        while order < self.max_order {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {
        let mut mem: Vec<u8> = Vec::new();
        mem.resize(128 * 1024, 0);
        let mut allocator = BuddyAllocator::<5, 4096>::new();
        let mem_ptr = mem.as_ptr().cast::<u8>();
        let offset = mem_ptr.align_offset(4096);
        let mem_ptr = unsafe { mem_ptr.add(offset) };
        allocator.add_region(mem_ptr as usize, mem.len() / 4096 - 1);
        let p = move || {
            let region = unsafe { allocator.regions_head.unwrap().as_ref() };
            println!("--------------------------------------------------------------------");
            for order in 0..region.max_order {
                let bitmap = region.bitmap(order);
                print!("order {}:", order);
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

            for order in 0..region.max_order {
                let bitmap = region.bitmap(order);
                print!("order {}:", order);
                for index in 0..bitmap.len() {
                    print!(
                        " {}",
                        if bitmap.get_alloc_bit(index) {
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

            println!("--------------------------------------------------------------------");
        };
        p();
        let region = unsafe { allocator.regions_head.unwrap().as_mut() };
        let mut stack = [0; 10];
        for i in 0..10 {
            stack[i] = allocator.allocate(0).unwrap();
        }
        let a = allocator.allocate(2).unwrap();
        p();
        let b = allocator.allocate(3).unwrap();
        p();
        let c = allocator.allocate(1).unwrap();
        p();
        allocator.deallocate(a);
        p();
        allocator.deallocate(b);
        p();
        allocator.deallocate(c);
        p();
        for i in 0..10 {
            allocator.deallocate(stack[i]);
        }
        p();

        assert!(false);
    }
}
