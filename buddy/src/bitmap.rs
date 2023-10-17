use alloc::raw_vec::RawVec;
use core::alloc::Layout;

type BitmapVecEntry = u64;

#[derive(Debug)]
pub struct BuddyBitmap {
    vec: RawVec<BitmapVecEntry>,
    len: usize,
}

impl BuddyBitmap {
    const FIELDS: usize = 2;
    const FREE_FIELD: usize = 0;
    const ALLOC_FIELD: usize = 1;
    const FREE_FIELDS_MASK: BitmapVecEntry = 0x5555_5555_5555_5555;
    const ENTRY_BITS: usize = 8 * core::mem::size_of::<BitmapVecEntry>();

    pub unsafe fn from_raw_parts(ptr: *mut BitmapVecEntry, len: usize) -> Self {
        let vec_entries = Self::num_vec_entries_from_entries(len);
        let mut vec = unsafe { RawVec::from_raw_parts(ptr, vec_entries) };
        for _ in 0..vec_entries {
            vec.push(0).unwrap();
        }

        Self { vec, len }
    }

    pub fn layout(bits: usize) -> Layout {
        Layout::array::<BitmapVecEntry>(Self::num_vec_entries_from_entries(bits)).unwrap()
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn get_free_bit(&self, index: usize) -> bool {
        self.get(index, Self::FREE_FIELD)
    }

    pub fn get_free_bit_checked(&self, index: usize) -> Option<bool> {
        self.get_checked(index, Self::FREE_FIELD)
    }

    pub fn set_free_bit(&mut self, index: usize) {
        self.set(index, Self::FREE_FIELD)
    }

    pub fn clear_free_bit(&mut self, index: usize) {
        self.clear(index, Self::FREE_FIELD)
    }

    pub fn get_alloc_bit(&self, index: usize) -> bool {
        self.get(index, Self::ALLOC_FIELD)
    }

    pub fn get_alloc_bit_checked(&self, index: usize) -> Option<bool> {
        self.get_checked(index, Self::ALLOC_FIELD)
    }

    pub fn set_alloc_bit(&mut self, index: usize) {
        self.set(index, Self::ALLOC_FIELD)
    }

    pub fn clear_alloc_bit(&mut self, index: usize) {
        self.clear(index, Self::ALLOC_FIELD)
    }

    pub fn find_first_free_index(&self) -> Option<usize> {
        for vec_index in 0..self.len {
            let bit_index =
                (self.vec[vec_index] & Self::FREE_FIELDS_MASK).trailing_zeros() as usize;
            if bit_index != Self::ENTRY_BITS {
                let entry_index = (Self::ENTRY_BITS * vec_index + bit_index) / Self::FIELDS;
                return Some(entry_index);
            }
        }

        None
    }

    fn get(&self, index: usize, field_index: usize) -> bool {
        assert!(index < self.len);
        let (entry_index, bit_index) = Self::indices(Self::FIELDS * index + field_index);
        self.vec[entry_index] & (1 << bit_index) != 0
    }

    fn get_checked(&self, index: usize, field_index: usize) -> Option<bool> {
        if index >= self.len {
            return None;
        }

        Some(self.get(index, field_index))
    }

    fn set(&mut self, index: usize, field_index: usize) {
        assert!(index < self.len);
        let (entry_index, bit_index) = Self::indices(Self::FIELDS * index + field_index);
        self.vec[entry_index] |= 1 << bit_index;
    }

    fn clear(&mut self, index: usize, field_index: usize) {
        assert!(index < self.len);
        let (entry_index, bit_index) = Self::indices(Self::FIELDS * index + field_index);
        self.vec[entry_index] &= !(1 << bit_index);
    }

    fn indices(bit_index: usize) -> (usize, usize) {
        let entry_index = bit_index / Self::ENTRY_BITS;
        let bit_index = bit_index & (Self::ENTRY_BITS - 1);
        (entry_index, bit_index)
    }

    fn num_vec_entries_from_entries(bits: usize) -> usize {
        (bits * Self::FIELDS + Self::ENTRY_BITS - 1) / Self::ENTRY_BITS
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    fn create_bitmap(entries: usize) -> BuddyBitmap {
        let mut mem: Vec<u64> = Vec::new();
        let vec_entries = (entries + 31) / 32;
        mem.resize(vec_entries, 0);
        let (ptr, _, _) = mem.into_raw_parts();
        unsafe { BuddyBitmap::from_raw_parts(ptr, entries) }
    }

    #[test]
    fn set() {
        let mut bitmap = create_bitmap(45000);
        bitmap.set_free_bit(bitmap.len() - 1);
        assert_eq!(bitmap.find_first_free_index(), Some(8));
    }

    #[bench]
    fn bench_find_first_100(b: &mut Bencher) {
        let mut bitmap = create_bitmap(100);
        bitmap.set_free_bit(bitmap.len() - 1);
        b.iter(|| bitmap.find_first_free_index());
    }

    #[bench]
    fn bench_find_first_100_000(b: &mut Bencher) {
        let mut bitmap = create_bitmap(100_000);
        bitmap.set_free_bit(bitmap.len() - 1);
        b.iter(|| bitmap.find_first_free_index());
    }

    #[bench]
    fn bench_find_first_100_000_000(b: &mut Bencher) {
        let mut bitmap = create_bitmap(100_000_000);
        bitmap.set_free_bit(bitmap.len() - 1);
        b.iter(|| bitmap.find_first_free_index());
    }
}
