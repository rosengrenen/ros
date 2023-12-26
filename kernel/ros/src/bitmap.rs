use alloc::raw_vec::RawVec;
use core::fmt;

pub struct Bitmap {
    vec: RawVec<u64>,
    len: usize,
}

impl fmt::Debug for Bitmap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}

impl Bitmap {
    pub unsafe fn from_raw_parts(ptr: *mut u64, len: usize) -> Self {
        let vec_len = len.div_ceil(64);
        let mut vec = unsafe { RawVec::from_raw_parts(ptr, vec_len) };
        for _ in 0..vec_len {
            vec.push(0).unwrap();
        }

        let trailing_bits = vec_len * 64 - len;
        if trailing_bits > 0 {
            vec[vec_len - 1] = !((1 << (64 - trailing_bits)) - 1);
        }

        Self { vec, len }
    }

    pub fn get_bit(&self, index: usize) -> bool {
        assert!(index < self.len);
        let (entry_index, bit_index) = Self::indices(index);
        self.vec[entry_index] & 1 << bit_index != 0
    }

    pub fn set_bit(&mut self, index: usize) {
        assert!(index < self.len);
        let (entry_index, bit_index) = Self::indices(index);
        self.vec[entry_index] |= 1 << bit_index;
    }

    pub fn clear_bit(&mut self, index: usize) {
        assert!(index < self.len);
        let (entry_index, bit_index) = Self::indices(index);
        self.vec[entry_index] &= !(1 << bit_index);
    }

    pub fn find_free_index(&self) -> Option<usize> {
        self.vec
            .iter()
            .enumerate()
            .filter(|(_, entry)| **entry != 0xffff_ffff_ffff_ffff)
            .map(|(entry_index, entry)| {
                let index = Self::binary_bit_search_64(*entry);
                entry_index * 64 + index
            })
            .next()
    }

    pub fn iter(&self) -> Iter {
        Iter {
            bitmap: self,
            index: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    fn binary_bit_search_64(entry: u64) -> usize {
        if entry & 0xffff_ffff != 0xffff_ffff {
            Self::binary_bit_search_32(entry as u32)
        } else {
            32 + Self::binary_bit_search_32((entry >> 32) as u32)
        }
    }

    fn binary_bit_search_32(entry: u32) -> usize {
        if entry & 0xffff != 0xffff {
            Self::binary_bit_search_16(entry as u16)
        } else {
            16 + Self::binary_bit_search_16((entry >> 16) as u16)
        }
    }

    fn binary_bit_search_16(entry: u16) -> usize {
        if entry & 0xff != 0xff {
            Self::binary_bit_search_8(entry as u8)
        } else {
            8 + Self::binary_bit_search_8((entry >> 8) as u8)
        }
    }

    fn binary_bit_search_8(entry: u8) -> usize {
        for bit_index in 0..8 {
            if entry & 1 << bit_index == 0 {
                return bit_index;
            }
        }

        unreachable!()
    }

    fn indices(index: usize) -> (usize, usize) {
        (index / 64, index % 64)
    }
}

pub struct Iter<'iter> {
    bitmap: &'iter Bitmap,
    index: usize,
}

impl<'iter> Iterator for Iter<'iter> {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index == self.bitmap.len {
            return None;
        }

        let value = self.bitmap.get_bit(self.index);
        self.index += 1;
        Some(value)
    }
}
