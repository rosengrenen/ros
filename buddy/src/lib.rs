// #![no_std]
#![deny(unsafe_op_in_unsafe_fn)]

use alloc::raw_vec::RawVec;
use core::{alloc::Layout, panic};
use std::{alloc::LayoutError, ptr::NonNull};

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
}

#[derive(Debug)]
#[repr(C)]
struct Region<const ORDER: usize, const FRAME_SIZE: usize> {
    next_region: Option<NonNull<Region<ORDER, FRAME_SIZE>>>,
    usable_base: usize,
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
        let max_order: usize = ((max_usable_frames as f64).log2().floor() as usize).min(ORDER);
        let layout =
            RegionLayout::<ORDER>::new(FRAME_SIZE, frames, max_usable_frames, max_order).unwrap();
        let bitmaps = Self::create_bitmaps(base, layout, max_order);
        let usable_frames_base = base + layout.usable_base_offset;
        Self {
            next_region,
            usable_base: usable_frames_base,
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
        for index in 0..bitmap.len {
            if bitmap.get_free_bit(index) {
                bitmap.clear_free_bit(index);
                bitmap.set_alloc_bit(index);
                return Some(self.addr_from_order_and_index(order, index));
            }
        }

        // 2. If no match was found, search upward and split
        for cur_order in order + 1..=self.max_order {
            for index in 0..(self.bitmap(cur_order).len) {
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
        self.usable_base + index * 2usize.pow(order as u32) * FRAME_SIZE
    }

    fn order_and_index_from_addr(&self, addr: usize) -> (usize, usize) {
        let mut index = (addr - self.usable_base) / FRAME_SIZE;
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
            let bitmap = unsafe { BuddyBitmap::from_raw_parts((base + offset) as *mut u8, bits) };
            bitmaps.push(bitmap).unwrap();
        }

        let mut index = 0;
        for order in (0..max_order).rev() {
            let bitmap = &mut bitmaps[order];
            for index in index..bitmap.len {
                bitmap.set_free_bit(index);
            }

            index = bitmap.len * 2;
        }

        bitmaps
    }
}

#[derive(Clone, Copy, Debug)]
struct RegionLayout<const ORDER: usize> {
    bitmaps_offset: usize,
    bitmap_offsets: [Option<usize>; ORDER],
    usable_frames: usize,
    usable_base_offset: usize,
}

impl<const ORDER: usize> RegionLayout<ORDER> {
    fn new(
        frame_size: usize,
        num_frames: usize,
        max_usable_frames: usize,
        max_order: usize,
    ) -> Result<Self, RegionLayoutError> {
        let layout = Layout::new::<Self>();
        let bitmaps_layout = Layout::array::<BuddyBitmap>(max_order)?;
        let (mut layout, bitmaps_offset) = layout.extend(bitmaps_layout)?;
        let mut bitmap_offsets = [None; ORDER];
        for order in 0..ORDER {
            let num_bits = max_usable_frames / 2usize.pow(order as u32);
            if num_bits == 0 {
                break;
            }

            let (bitmap_layout, offset) = layout.extend(BuddyBitmap::layout(num_bits))?;
            bitmap_offsets[order] = Some(offset);
            layout = bitmap_layout;
        }

        let num_meta_frames = (layout.size() + frame_size - 1) / frame_size;
        if num_meta_frames > num_frames {
            return Err(RegionLayoutError::RegionTooSmall);
        }

        Ok(Self {
            bitmaps_offset,
            bitmap_offsets,
            usable_frames: num_frames - num_meta_frames,
            usable_base_offset: num_meta_frames * frame_size,
        })
    }
}

#[derive(Debug)]
enum RegionLayoutError {
    RegionTooSmall,
    LayoutError(LayoutError),
}

impl From<LayoutError> for RegionLayoutError {
    fn from(error: LayoutError) -> Self {
        Self::LayoutError(error)
    }
}

#[derive(Debug)]
#[repr(C)]
struct BuddyBitmap {
    vec: RawVec<u8>,
    len: usize,
}

impl BuddyBitmap {
    const FIELDS: usize = 2;

    pub unsafe fn from_raw_parts(ptr: *mut u8, len: usize) -> Self {
        let bytes = (len + 7) / 8 * Self::FIELDS;
        let mut vec = unsafe { RawVec::from_raw_parts(ptr, bytes) };
        for _ in 0..bytes {
            vec.push(0).unwrap();
        }

        Self { vec, len }
    }

    pub fn layout(bits: usize) -> Layout {
        Layout::array::<u8>((bits + 7) / 8 * Self::FIELDS).unwrap()
    }

    pub fn get_free_bit(&self, index: usize) -> bool {
        self.get(index, 0)
    }

    pub fn get_free_bit_checked(&self, index: usize) -> Option<bool> {
        self.get_checked(index, 0)
    }

    pub fn set_free_bit(&mut self, index: usize) {
        self.set(index, 0)
    }

    pub fn clear_free_bit(&mut self, index: usize) {
        self.clear(index, 0)
    }

    pub fn get_alloc_bit(&self, index: usize) -> bool {
        self.get(index, 1)
    }

    pub fn get_alloc_bit_checked(&self, index: usize) -> Option<bool> {
        self.get_checked(index, 1)
    }

    pub fn set_alloc_bit(&mut self, index: usize) {
        self.set(index, 1)
    }

    pub fn clear_alloc_bit(&mut self, index: usize) {
        self.clear(index, 1)
    }

    fn get(&self, index: usize, field_index: usize) -> bool {
        let (byte_index, bit_index) = Self::indices(Self::FIELDS * index + field_index);
        self.vec[byte_index] & (1 << bit_index) != 0
    }

    fn get_checked(&self, index: usize, field_index: usize) -> Option<bool> {
        if index >= self.len {
            return None;
        }

        Some(self.get(index, field_index))
    }

    fn set(&mut self, index: usize, field_index: usize) {
        let (byte_index, bit_index) = Self::indices(Self::FIELDS * index + field_index);
        self.vec[byte_index] |= 1 << bit_index;
    }

    fn clear(&mut self, index: usize, field_index: usize) {
        let (byte_index, bit_index) = Self::indices(Self::FIELDS * index + field_index);
        self.vec[byte_index] &= !(1 << bit_index);
    }

    fn indices(bit_index: usize) -> (usize, usize) {
        let byte_index = bit_index / 8;
        let bit_index = bit_index & 0b111;
        (byte_index, bit_index)
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
                for index in 0..bitmap.len {
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
                for index in 0..bitmap.len {
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
            stack[i] = region.allocate(0).unwrap();
        }
        let a = region.allocate(2).unwrap();
        p();
        let b = region.allocate(3).unwrap();
        p();
        let c = region.allocate(1).unwrap();
        p();
        region.deallocate(a);
        p();
        region.deallocate(b);
        p();
        region.deallocate(c);
        p();
        for i in 0..10 {
            region.deallocate(stack[i]);
        }
        p();

        assert!(false);
    }
}
