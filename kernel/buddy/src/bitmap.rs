use alloc::raw_vec::RawVec;
use core::alloc::{Layout, LayoutError};

pub type BitmapVecEntry = u64;

#[derive(Debug)]
pub struct Bitmap {
    vec: RawVec<BitmapVecEntry>,
    len: usize,
}

impl Bitmap {
    pub const ENTRY_BITS: usize = 8 * core::mem::size_of::<BitmapVecEntry>();

    pub unsafe fn from_raw_parts(base: *mut BitmapVecEntry, layout: BitmapLayout) -> Self {
        Self {
            vec: Self::create_bitmap(base, layout),
            len: layout.len,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn get_entry(&self, entry_index: usize) -> BitmapVecEntry {
        assert!(entry_index < self.len);
        self.vec[entry_index]
    }

    pub fn get(&self, index: usize) -> bool {
        assert!(index < self.len);
        let (entry_index, bit_index) = self.indices(index);
        self.vec[entry_index] & (1 << bit_index) != 0
    }

    pub fn get_checked(&self, index: usize) -> Option<bool> {
        if index >= self.len {
            return None;
        }

        Some(self.get(index))
    }

    pub fn set(&mut self, index: usize) {
        assert!(index < self.len);
        let (entry_index, bit_index) = self.indices(index);
        self.vec[entry_index] |= 1 << bit_index;
    }

    pub fn clear(&mut self, index: usize) {
        assert!(index < self.len);
        let (entry_index, bit_index) = self.indices(index);
        self.vec[entry_index] &= !(1 << bit_index);
    }

    pub fn find_first_free_index_from(&self, start_entry_index: usize) -> Option<usize> {
        for entry_index in start_entry_index..self.vec.len() {
            let bit_index = self.get_entry(entry_index).trailing_zeros() as usize;
            if bit_index != Bitmap::ENTRY_BITS {
                let found_entry_index = Bitmap::ENTRY_BITS * entry_index + bit_index;
                // Last entry can contain trailing, but unused 1's, resulting in a false positive
                if found_entry_index < self.len {
                    return Some(found_entry_index);
                }
            }
        }

        None
    }

    fn indices(&self, bit_index: usize) -> (usize, usize) {
        let entry_index = Self::entry_index(bit_index);
        let bit_index = Self::bit_index(bit_index);
        (entry_index, bit_index)
    }

    pub fn entry_index(bit_index: usize) -> usize {
        bit_index / Self::ENTRY_BITS
    }

    fn bit_index(bit_index: usize) -> usize {
        bit_index & (Self::ENTRY_BITS - 1)
    }

    fn create_bitmap(base: *mut BitmapVecEntry, layout: BitmapLayout) -> RawVec<BitmapVecEntry> {
        let mut vec = unsafe { RawVec::from_raw_parts(base, layout.entries) };
        for _ in 0..layout.entries {
            let _ = vec.push(0);
        }

        // Clear bits after length
        let extra_bits = layout.entries * Self::ENTRY_BITS - layout.len;
        let last_entry_index = layout.entries - 1;
        for bit_index in Self::ENTRY_BITS - extra_bits..Self::ENTRY_BITS {
            vec[last_entry_index] |= 1 << bit_index;
        }

        vec
    }
}

#[derive(Clone, Copy, Debug)]
pub struct BitmapLayout {
    pub layout: Layout,
    len: usize,
    entries: usize,
}

impl BitmapLayout {
    pub fn new(len: usize) -> Result<Self, LayoutError> {
        let entries = len.div_ceil(Bitmap::ENTRY_BITS);
        Ok(Self {
            layout: Layout::array::<BitmapVecEntry>(entries)?,
            len,
            entries,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_bitmap(entries: usize) -> Bitmap {
        let layout = BitmapLayout::new(entries).unwrap();
        let mut mem: Vec<u64> = Vec::new();
        mem.resize((layout.layout.size() + 7) / 8, 0);
        let (ptr, _, _) = mem.into_raw_parts();
        unsafe { Bitmap::from_raw_parts(ptr.cast(), layout) }
    }

    #[test]
    fn set() {
        let mut bitmap = create_bitmap(7);
        bitmap.set(0);
        bitmap.set(1);
        bitmap.set(2);
        bitmap.set(3);
        bitmap.set(4);
        bitmap.set(6);
        assert_eq!(bitmap.get(0), true);
        assert_eq!(bitmap.get(1), true);
        assert_eq!(bitmap.get(2), true);
        assert_eq!(bitmap.get(3), true);
        assert_eq!(bitmap.get(4), true);
        assert_eq!(bitmap.get(5), false);
        assert_eq!(bitmap.get(6), true);
    }
}
