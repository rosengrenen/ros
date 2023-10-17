use alloc::raw_vec::RawVec;
use core::alloc::Layout;

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

    pub unsafe fn from_raw_parts(base: *mut u64, len: usize, fields: usize) -> Self {
        let entries_per_field = Self::entries_per_field(len);
        let vec_entries = entries_per_field * fields;
        let mut vec = unsafe { RawVec::from_raw_parts(base.cast::<BitmapVecEntry>(), vec_entries) };
        for _ in 0..vec_entries {
            vec.push(0).unwrap();
        }

        // Clear bits after length
        let extra_bits = vec_entries / fields * Self::ENTRY_BITS - len;
        for field in 0..fields {
            let entry_index = (field + 1) * entries_per_field - 1;
            for bit_index in Self::ENTRY_BITS - extra_bits..Self::ENTRY_BITS {
                vec[entry_index] |= 1 << bit_index;
            }
        }

        Self {
            vec,
            len,
            fields,
            entries_per_field,
        }
    }

    pub fn layout(len: usize, fields: usize) -> Layout {
        let vec_entries = Self::entries_per_field(len) * fields;
        Layout::array::<BitmapVecEntry>(vec_entries).unwrap()
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
        for vec_index in start_entry_index..self.len {
            let bit_index = self.get_entry(vec_index, field_index).trailing_zeros() as usize;
            if bit_index != Bitmap::ENTRY_BITS {
                let entry_index = Bitmap::ENTRY_BITS * vec_index + bit_index;
                return Some(entry_index);
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

    fn entries_per_field(len: usize) -> usize {
        (len + Self::ENTRY_BITS - 1) / Self::ENTRY_BITS
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_bitmap(entries: usize, fields: usize) -> Bitmap {
        let layout = Bitmap::layout(entries, fields);
        let mut mem: Vec<u64> = Vec::new();
        mem.resize((layout.size() + 7) / 8, 0);
        let (ptr, _, _) = mem.into_raw_parts();
        unsafe { Bitmap::from_raw_parts(ptr.cast(), entries, fields) }
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
