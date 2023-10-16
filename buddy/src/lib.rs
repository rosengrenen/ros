// #![no_std]
#![deny(unsafe_op_in_unsafe_fn)]

use alloc::raw_vec::RawVec;
use core::alloc::Layout;

#[derive(Debug)]
#[repr(C)]
pub struct BuddyAllocator<const ORDER: usize, const FRAME_SIZE: usize> {
    regions_head: Option<&'static Region<ORDER, FRAME_SIZE>>,
}

impl<const ORDER: usize, const FRAME_SIZE: usize> BuddyAllocator<ORDER, FRAME_SIZE> {
    pub fn new() -> Self {
        Self { regions_head: None }
    }

    pub fn add_region(&mut self, base: usize, frames: usize) {
        let region_ptr = base as *mut Region<ORDER, FRAME_SIZE>;
        let region = Region::new(self.regions_head, base, frames);
        let region = unsafe {
            region_ptr.write(region);
            &mut *region_ptr
        };
        self.regions_head = Some(region);
    }

    // pub fn allocate_order(&mut self, order: usize) -> usize {}
    // allocate_order / allocate_size

    // pub fn deallocate()
}

#[derive(Debug)]
#[repr(C)]
struct Region<const ORDER: usize, const FRAME_SIZE: usize> {
    next_region: Option<&'static Region<ORDER, FRAME_SIZE>>,
    base: usize,
    bitmaps: [Option<Bitmap>; ORDER],
}

impl<const ORDER: usize, const FRAME_SIZE: usize> Region<ORDER, FRAME_SIZE> {
    pub fn new(
        next_region: Option<&'static Region<ORDER, FRAME_SIZE>>,
        base: usize,
        frames: usize,
    ) -> Self {
        assert_eq!(base % FRAME_SIZE, 0);
        let mut layout = Layout::new::<Self>();
        let mut bitmap_offsets = [None; ORDER];
        for order in 0..ORDER {
            let bits = (frames - 1) / 2usize.pow(order as u32);
            if bits == 0 {
                break;
            }

            let (bitmap_layout, offset) = layout.extend(Bitmap::layout(bits)).unwrap();
            bitmap_offsets[order] = Some(offset);
            layout = bitmap_layout;
        }

        let meta_frames = (layout.size() + FRAME_SIZE - 1) / FRAME_SIZE;
        if meta_frames > frames {
            panic!();
        }

        const BITMAP_NONE: Option<Bitmap> = None;
        let mut bitmaps = [BITMAP_NONE; ORDER];
        for (order, offset) in bitmap_offsets.into_iter().enumerate() {
            if let Some(offset) = offset {
                let bits = (frames - meta_frames) / 2usize.pow(order as u32);
                if bits == 0 {
                    break;
                }

                bitmaps[order] =
                    Some(unsafe { Bitmap::from_raw_parts((base + offset) as *mut u8, bits) });
            } else {
                break;
            }
        }

        let mut bit_index = 0;
        for order in (0..ORDER).rev() {
            let bitmap = &mut bitmaps[order];
            if let Some(bitmap) = bitmap {
                for i in bit_index..bitmap.len {
                    bitmap.set(i);
                }

                bit_index = bitmap.len * 2;
            }
        }

        let usable_frames_base = base + meta_frames * FRAME_SIZE;
        Self {
            next_region,
            base: usable_frames_base,
            bitmaps,
        }
    }

    // alloc

    // dealloc
}

#[derive(Debug)]
#[repr(C)]
struct Bitmap {
    vec: RawVec<u8>,
    len: usize,
}

impl Bitmap {
    pub unsafe fn from_raw_parts(ptr: *mut u8, len: usize) -> Self {
        let bytes = (len + 7) / 8;
        let mut vec = unsafe { RawVec::from_raw_parts(ptr, bytes) };
        for _ in 0..bytes {
            vec.push(0).unwrap();
        }
        Self { vec, len }
    }

    pub fn layout(bits: usize) -> Layout {
        Layout::array::<u8>((bits + 7) / 8).unwrap()
    }

    pub fn get(&self, bit_index: usize) -> bool {
        assert!(bit_index < self.len);
        let (byte_index, bit_index) = Self::indices(bit_index);
        self.vec[byte_index] & (1 << bit_index) != 0
    }

    pub fn set(&mut self, bit_index: usize) {
        assert!(bit_index < self.len);
        let (byte_index, bit_index) = Self::indices(bit_index);
        println!("{}", bit_index);
        self.vec[byte_index] |= 1 << bit_index;
    }

    pub fn clear(&mut self, bit_index: usize) {
        assert!(bit_index < self.len);
        let (byte_index, bit_index) = Self::indices(bit_index);
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
        println!("{:x?}", &mem[offset..offset + 512]);
        println!("{:#?}", allocator);
        assert!(false);
    }
}
