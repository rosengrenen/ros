use alloc::raw_vec::RawVec;
use core::alloc::{Layout, LayoutError};

pub type BitmapVecEntry = u64;

#[derive(Debug)]
pub struct Bitmap {
    vec: RawVec<BitmapVecEntry>,
    len: usize,
    fields: usize,
    entries_per_field: usize,
}

impl Bitmap {
    pub const ENTRY_BITS: usize = 8 * core::mem::size_of::<BitmapVecEntry>();

    pub unsafe fn from_raw_parts(base: *mut BitmapVecEntry, layout: BitmapLayout) -> Self {
        Self {
            vec: Self::create_bitmap(base, layout),
            len: layout.len,
            fields: layout.fields,
            entries_per_field: layout.entries_per_field,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn get_entry(&self, entry_index: usize, field_index: usize) -> BitmapVecEntry {
        assert!(entry_index < self.len);
        self.vec[self.field_entry_index(entry_index, field_index)]
    }

    pub fn get(&self, index: usize, field_index: usize) -> bool {
        assert!(index < self.len);
        assert!(field_index < self.fields);
        let (entry_index, bit_index) = self.indices(index, field_index);
        self.vec[entry_index] & (1 << bit_index) != 0
    }

    pub fn get_checked(&self, index: usize, field_index: usize) -> Option<bool> {
        assert!(field_index < self.fields);
        if index >= self.len {
            return None;
        }

        Some(self.get(index, field_index))
    }

    pub fn set(&mut self, index: usize, field_index: usize) {
        assert!(index < self.len);
        assert!(field_index < self.fields);
        let (entry_index, bit_index) = self.indices(index, field_index);
        self.vec[entry_index] |= 1 << bit_index;
    }

    pub fn clear(&mut self, index: usize, field_index: usize) {
        assert!(index < self.len);
        assert!(field_index < self.fields);
        let (entry_index, bit_index) = self.indices(index, field_index);
        self.vec[entry_index] &= !(1 << bit_index);
    }

    pub fn find_first_free_index_from(
        &self,
        start_entry_index: usize,
        field_index: usize,
    ) -> Option<usize> {
        for vec_index in start_entry_index..self.entries_per_field {
            let bit_index = self.get_entry(vec_index, field_index).trailing_zeros() as usize;
            if bit_index != Bitmap::ENTRY_BITS {
                let entry_index = Bitmap::ENTRY_BITS * vec_index + bit_index;
                // Last entry can contain trailing, but unused 1's, resulting in a false positive
                if entry_index < self.len {
                    return Some(entry_index);
                }
            }
        }

        None
    }

    fn indices(&self, bit_index: usize, field_index: usize) -> (usize, usize) {
        let entry_index = Self::entry_index(bit_index);
        let bit_index = Self::bit_index(bit_index);
        (self.field_entry_index(entry_index, field_index), bit_index)
    }

    pub fn entry_index(bit_index: usize) -> usize {
        bit_index / Self::ENTRY_BITS
    }

    fn bit_index(bit_index: usize) -> usize {
        bit_index & (Self::ENTRY_BITS - 1)
    }

    fn field_entry_index(&self, entry_index: usize, field_index: usize) -> usize {
        self.entries_per_field * field_index + entry_index
    }

    fn create_bitmap(base: *mut BitmapVecEntry, layout: BitmapLayout) -> RawVec<BitmapVecEntry> {
        let mut vec = unsafe { RawVec::from_raw_parts(base, layout.entries()) };
        for _ in 0..layout.entries() {
            let _ = vec.push(0);
        }

        // Clear bits after length
        let extra_bits = layout.entries_per_field * Self::ENTRY_BITS - layout.len;
        for field in 0..layout.fields {
            let entry_index = (field + 1) * layout.entries_per_field - 1;
            for bit_index in Self::ENTRY_BITS - extra_bits..Self::ENTRY_BITS {
                vec[entry_index] |= 1 << bit_index;
            }
        }

        vec
    }
}

#[derive(Clone, Copy, Debug)]
pub struct BitmapLayout {
    pub layout: Layout,
    len: usize,
    fields: usize,
    entries_per_field: usize,
}

impl BitmapLayout {
    pub fn new(len: usize, fields: usize) -> Result<Self, LayoutError> {
        let entries_per_field = len.div_ceil(Bitmap::ENTRY_BITS);
        let entries = entries_per_field * fields;
        Ok(Self {
            layout: Layout::array::<BitmapVecEntry>(entries)?,
            len,
            fields,
            entries_per_field,
        })
    }

    fn entries(&self) -> usize {
        self.fields * self.entries_per_field
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_bitmap(entries: usize, fields: usize) -> Bitmap {
        let layout = BitmapLayout::new(entries, fields).unwrap();
        let mut mem: Vec<u64> = Vec::new();
        mem.resize((layout.layout.size() + 7) / 8, 0);
        let (ptr, _, _) = mem.into_raw_parts();
        unsafe { Bitmap::from_raw_parts(ptr.cast(), layout) }
    }

    #[test]
    fn set() {
        let mut bitmap = create_bitmap(200, 2);
        bitmap.set(0, 0);
        bitmap.set(1, 0);
        bitmap.set(2, 0);
        bitmap.set(3, 0);
        bitmap.set(0, 1);
        bitmap.set(2, 1);
        bitmap.set(4, 1);
        bitmap.set(6, 1);
        assert_eq!(bitmap.get(0, 0), true);
        assert_eq!(bitmap.get(1, 0), true);
        assert_eq!(bitmap.get(2, 0), true);
        assert_eq!(bitmap.get(3, 0), true);
        assert_eq!(bitmap.get(4, 0), false);
        assert_eq!(bitmap.get(5, 0), false);
        assert_eq!(bitmap.get(6, 0), false);
        assert_eq!(bitmap.get(0, 1), true);
        assert_eq!(bitmap.get(1, 1), false);
        assert_eq!(bitmap.get(2, 1), true);
        assert_eq!(bitmap.get(3, 1), false);
        assert_eq!(bitmap.get(4, 1), true);
        assert_eq!(bitmap.get(5, 1), false);
        assert_eq!(bitmap.get(6, 1), true);
    }
}
