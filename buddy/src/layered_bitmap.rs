use alloc::raw_vec::RawVec;
use core::alloc::Layout;
use crate::bitmap::BuddyBitmap;

type BitmapVecEntry = u64;

#[derive(Debug)]
pub struct LayeredBuddyBitmap {
    bitmaps: RawVec<BuddyBitmap>,
}

impl BuddyBitmap {
    const FIELDS: usize = 2;
    const FREE_FIELD: usize = 0;
    const ALLOC_FIELD: usize = 1;
    const FREE_FIELDS_MASK: BitmapVecEntry = 0x5555_5555_5555_5555;
    const ENTRY_BITS: usize = 8 * core::mem::size_of::<BitmapVecEntry>();
    const CACHE_ENTRY_BITS: usize = Self::ENTRY_BITS * Self::ENTRY_BITS;

    pub unsafe fn from_raw_parts(base: *mut BuddyBitmap, len: usize) -> Self {
        let vec_entries = Self::vec_entries_from_entries(len);
        let entries_vec_layout = Layout::array::<BitmapVecEntry>(vec_entries).unwrap();
        let mut vec = unsafe { RawVec::from_raw_parts(base.cast::<BitmapVecEntry>(), vec_entries) };
        for _ in 0..vec_entries {
            vec.push(0).unwrap();
        }

        // TODO: set last bits

        let cache_layers =
            ((len as f64).log(Self::CACHE_ENTRY_BITS as f64).ceil() as usize).max(1) - 1;
        let caches_vec_layout = Layout::array::<RawVec<BitmapVecEntry>>(cache_layers).unwrap();

        let (mut combined_layout, caches_vec_offset) =
            entries_vec_layout.extend(caches_vec_layout).unwrap();
        let mut caches = unsafe {
            RawVec::from_raw_parts(
                base.add(caches_vec_offset).cast::<RawVec<BitmapVecEntry>>(),
                cache_layers,
            )
        };

        if cache_layers > 0 {
            let mut cache_entries = Self::cache_entries_from_entries(len);
            for _ in 0..cache_layers {
                let cache_vec_layout = Layout::array::<BitmapVecEntry>(cache_entries).unwrap();
                let (layout, cache_vec_offset) = combined_layout.extend(cache_vec_layout).unwrap();

                let mut cache = unsafe {
                    RawVec::from_raw_parts(
                        base.add(cache_vec_offset).cast::<BitmapVecEntry>(),
                        cache_entries,
                    )
                };
                for _ in 0..cache_entries {
                    cache.push(0);
                }

                // TODO: set last bits

                caches.push(cache);

                combined_layout = layout;
                cache_entries = Self::cache_entries_from_entries(cache_entries);
            }
        }

        Self { vec, caches, len }
    }

    pub fn layout(len: usize) -> Layout {
        let vec_entries = Self::vec_entries_from_entries(len);
        let entries_vec_layout = Layout::array::<BitmapVecEntry>(vec_entries).unwrap();

        let cache_layers =
            ((len as f64).log(Self::CACHE_ENTRY_BITS as f64).ceil() as usize).max(1) - 1;
        let caches_vec_layout = Layout::array::<RawVec<BitmapVecEntry>>(cache_layers).unwrap();

        let mut combined_layout = entries_vec_layout.extend(caches_vec_layout).unwrap().0;
        if cache_layers > 0 {
            let mut cache_entries = Self::cache_entries_from_entries(len);
            for _ in 0..cache_layers {
                let cache_vec_layout = Layout::array::<BitmapVecEntry>(cache_entries).unwrap();
                combined_layout = combined_layout.extend(cache_vec_layout).unwrap().0;
                cache_entries = Self::cache_entries_from_entries(cache_entries);
            }
        }

        combined_layout
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
        let (entry_index, bit_index) = Self::indices(Self::FIELDS * index, field_index);
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
        let (entry_index, bit_index) = Self::indices(Self::FIELDS * index, field_index);
        self.vec[entry_index] |= 1 << bit_index;
    }

    fn clear(&mut self, index: usize, field_index: usize) {
        assert!(index < self.len);
        let (entry_index, bit_index) = Self::indices(Self::FIELDS * index, field_index);
        self.vec[entry_index] &= !(1 << bit_index);
    }

    fn indices(bit_index: usize, field_index: usize) -> (usize, usize) {
        let entry_index = bit_index / Self::ENTRY_BITS;
        let bit_index = bit_index & (Self::ENTRY_BITS - 1);
        (entry_index + field_index, bit_index)
    }

    fn cache_indices(bit_index: usize) -> (usize, usize) {
        let entry_index = bit_index / Self::CACHE_ENTRY_BITS;
        let bit_index = bit_index & (Self::ENTRY_BITS - 1);
        (entry_index, bit_index)
    }

    fn cache_set(&mut self, index: usize) {
        let (entry_index, _) = Self::indices(Self::FIELDS * index, Self::FREE_FIELD);
        if self.vec[entry_index] == !0 {
            let (entry_index, bit_index)
            for cache in self.caches.iter_mut() {
                cache[]
            }            
        }
    }

    fn cache_clear() {}

    fn vec_entries_from_entries(bits: usize) -> usize {
        (bits * Self::FIELDS + Self::ENTRY_BITS - 1) / Self::ENTRY_BITS
    }

    fn cache_entries_from_entries(bits: usize) -> usize {
        (bits + Self::CACHE_ENTRY_BITS - 1) / Self::CACHE_ENTRY_BITS
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    fn create_bitmap(entries: usize) -> BuddyBitmap {
        let layout = BuddyBitmap::layout(entries);
        let mut mem: Vec<u64> = Vec::new();
        mem.resize((layout.size() + 7) / 8, 0);
        let (ptr, _, _) = mem.into_raw_parts();
        unsafe { BuddyBitmap::from_raw_parts(ptr.cast(), entries) }
    }

    #[test]
    fn set() {
        let mut bitmap = create_bitmap(45000);
        bitmap.set_free_bit(bitmap.len() - 1);
        println!("{:?}", BuddyBitmap::layout(64 * 1024 * 1024 * 1024));
        assert_eq!(bitmap.find_first_free_index(), Some(8));
    }

    #[bench]
    fn bench_find_first_64kb(b: &mut Bencher) {
        let mut bitmap = create_bitmap(64 * 1024 / 4096);
        bitmap.set_free_bit(bitmap.len() - 1);
        b.iter(|| bitmap.find_first_free_index());
    }

    #[bench]
    fn bench_find_first_64mb(b: &mut Bencher) {
        let mut bitmap = create_bitmap(64 * 1024 * 1024 / 4096);
        bitmap.set_free_bit(bitmap.len() - 1);
        b.iter(|| bitmap.find_first_free_index());
    }

    #[bench]
    fn bench_find_first_64gb(b: &mut Bencher) {
        let mut bitmap = create_bitmap(64 * 1024 * 1024 * 1024 / 4096);
        bitmap.set_free_bit(bitmap.len() - 1);
        b.iter(|| bitmap.find_first_free_index());
    }
}
