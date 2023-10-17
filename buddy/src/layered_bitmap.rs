use crate::{
    bitmap::{Bitmap, BitmapVecEntry},
    util::ilog_ceil,
};
use alloc::raw_vec::RawVec;
use core::alloc::Layout;

#[derive(Debug)]
pub struct BuddyBitmap {
    layers: RawVec<Bitmap>,
}

impl BuddyBitmap {
    const FIELDS: usize = 2;
    const FREE_FIELD: usize = 0;
    const ALLOC_FIELD: usize = 1;
    const CACHE_ENTRY_BITS: usize = Bitmap::ENTRY_BITS * Bitmap::ENTRY_BITS;

    pub unsafe fn from_raw_parts(ptr: *mut Bitmap, len: usize) -> Self {
        let num_layers = Self::num_layers_from_len(len);
        let layers_layout = Layout::array::<Bitmap>(num_layers).unwrap();
        let mut layers = unsafe { RawVec::from_raw_parts(ptr, num_layers) };

        let main_layer_layout = Bitmap::layout(len, Self::FIELDS);
        let (mut combined_layout, offset) = layers_layout.extend(main_layer_layout).unwrap();
        let main_layer =
            unsafe { Bitmap::from_raw_parts(ptr.byte_add(offset).cast(), len, Self::FIELDS) };
        layers.push(main_layer).unwrap();

        if num_layers > 1 {
            let mut cache_layer_entries = Self::cache_entries_from_entries(len);
            for _ in 1..num_layers {
                let cache_layer_layout = Bitmap::layout(cache_layer_entries, 1);
                let (layout, offset) = combined_layout.extend(cache_layer_layout).unwrap();
                let cache_layer = unsafe {
                    Bitmap::from_raw_parts(ptr.byte_add(offset).cast(), cache_layer_entries, 1)
                };
                layers.push(cache_layer).unwrap();

                combined_layout = layout;
                cache_layer_entries = Self::cache_entries_from_entries(cache_layer_entries);
            }
        }

        Self { layers }
    }

    pub fn layout(len: usize) -> Layout {
        let num_layers = Self::num_layers_from_len(len);
        let layers_layout = Layout::array::<Bitmap>(num_layers).unwrap();
        let main_layer_layout = Bitmap::layout(len, Self::FIELDS);

        let mut combined_layout = layers_layout.extend(main_layer_layout).unwrap().0;
        if num_layers > 1 {
            let mut cache_layer_entries = Self::cache_entries_from_entries(len);
            for _ in 1..num_layers {
                let cache_vec_layout =
                    Layout::array::<BitmapVecEntry>(cache_layer_entries).unwrap();
                combined_layout = combined_layout.extend(cache_vec_layout).unwrap().0;
                cache_layer_entries = Self::cache_entries_from_entries(cache_layer_entries);
            }
        }

        combined_layout
    }

    pub fn len(&self) -> usize {
        self.layers[0].len()
    }

    pub fn get_free_bit(&self, index: usize) -> bool {
        self.get(index, Self::FREE_FIELD)
    }

    pub fn get_free_bit_checked(&self, index: usize) -> Option<bool> {
        self.get_checked(index, Self::FREE_FIELD)
    }

    pub fn set_free_bit(&mut self, index: usize) {
        let original_entry = self.layers[0].get_entry(Bitmap::entry_index(index), Self::FREE_FIELD);
        self.set(index, Self::FREE_FIELD);
        self.free_bit_cache_set(index, 0, original_entry);
    }

    pub fn clear_free_bit(&mut self, index: usize) {
        self.clear(index, Self::FREE_FIELD);
        self.free_bit_cache_clear(index, 0);
    }

    pub fn get_alloc_bit(&self, index: usize) -> bool {
        self.get(index, Self::ALLOC_FIELD)
    }

    pub fn get_alloc_bit_checked(&self, index: usize) -> Option<bool> {
        self.get_checked(index, Self::ALLOC_FIELD)
    }

    pub fn set_alloc_bit(&mut self, index: usize) {
        self.set(index, Self::ALLOC_FIELD);
    }

    pub fn clear_alloc_bit(&mut self, index: usize) {
        self.clear(index, Self::ALLOC_FIELD);
    }

    pub fn find_first_free_index(&self) -> Option<usize> {
        self.layers[0].find_first_free_index_from(0, Self::FREE_FIELD)
    }

    pub fn find_first_free_index_h(&self) -> Option<usize> {
        let mut index = 0;
        for layer in (1..self.layers.len()).rev() {
            match self.layers[layer].find_first_free_index_from(0, index) {
                Some(found_index) => {
                    index = found_index * Bitmap::ENTRY_BITS;
                }
                None => return None,
            }
        }

        self.layers[0].find_first_free_index_from(index, Self::FREE_FIELD)
    }

    fn get(&self, index: usize, field_index: usize) -> bool {
        self.layers[0].get(index, field_index)
    }

    fn get_checked(&self, index: usize, field_index: usize) -> Option<bool> {
        self.layers[0].get_checked(index, field_index)
    }

    fn set(&mut self, index: usize, field_index: usize) {
        self.layers[0].set(index, field_index);
    }

    fn clear(&mut self, index: usize, field_index: usize) {
        self.layers[0].clear(index, field_index);
    }

    fn free_bit_cache_set(&mut self, index: usize, layer: usize, original_entry: BitmapVecEntry) {
        if original_entry != 0 {
            return;
        }

        if layer == self.layers.len() - 1 {
            return;
        }

        let entry_index = Bitmap::entry_index(index);
        let higher_layer_bit_index = entry_index / Bitmap::ENTRY_BITS;
        let higher_layer_entry_index = Bitmap::entry_index(higher_layer_bit_index);
        let higher_layer_entry = self.layers[layer + 1].get_entry(higher_layer_entry_index, 0);
        self.layers[layer + 1].set(higher_layer_bit_index, 0);
        self.free_bit_cache_set(higher_layer_bit_index, layer + 1, higher_layer_entry);
    }

    fn free_bit_cache_clear(&mut self, index: usize, layer: usize) {
        if layer == self.layers.len() - 1 {
            return;
        }

        let entry_index = Bitmap::entry_index(index);
        let field = self.layers[layer].get_entry(entry_index, Self::FREE_FIELD);
        if field == 0 {
            // It is now full, report upward
            let higher_layer_bit_index = entry_index / Bitmap::ENTRY_BITS;
            self.layers[layer + 1].clear(higher_layer_bit_index, 0);
            self.free_bit_cache_clear(higher_layer_bit_index, layer + 1);
        }
    }

    fn cache_entries_from_entries(bits: usize) -> usize {
        (bits + Self::CACHE_ENTRY_BITS - 1) / Self::CACHE_ENTRY_BITS
    }

    fn num_layers_from_len(len: usize) -> usize {
        ilog_ceil(Self::CACHE_ENTRY_BITS, len).max(1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    fn create_bitmap(entries: usize) -> BuddyBitmap {
        let layout = BuddyBitmap::layout(entries);
        println!("{:?}", layout);
        let mut mem: Vec<u64> = Vec::new();
        mem.resize((layout.size() + 7) / 8, 0);
        let (ptr, _, _) = mem.into_raw_parts();
        unsafe { BuddyBitmap::from_raw_parts(ptr.cast(), entries) }
    }

    #[test]
    fn set() {
        let mut bitmap = create_bitmap(200);
        bitmap.set_free_bit(127);
        assert_eq!(bitmap.find_first_free_index(), Some(127));
        assert_eq!(bitmap.find_first_free_index_h(), Some(127));
        bitmap.set_free_bit(7);
        assert_eq!(bitmap.find_first_free_index(), Some(7));
        assert_eq!(bitmap.find_first_free_index_h(), Some(7));
    }

    #[test]
    fn lol() {
        let mut bitmap = create_bitmap(64 * 1024 * 1024 * 1024 / 4096);
        println!("{:?}", bitmap);
        bitmap.set_free_bit(bitmap.len() - 1);
        assert_eq!(bitmap.find_first_free_index(), Some(bitmap.len() - 1));
        assert_eq!(bitmap.find_first_free_index_h(), Some(bitmap.len() - 1));
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

    #[bench]
    fn bench_find_first_64kb_h(b: &mut Bencher) {
        let mut bitmap = create_bitmap(64 * 1024 / 4096);
        bitmap.set_free_bit(bitmap.len() - 1);
        b.iter(|| bitmap.find_first_free_index_h());
    }

    #[bench]
    fn bench_find_first_64mb_h(b: &mut Bencher) {
        let mut bitmap = create_bitmap(64 * 1024 * 1024 / 4096);
        bitmap.set_free_bit(bitmap.len() - 1);
        b.iter(|| bitmap.find_first_free_index_h());
    }

    #[bench]
    fn bench_find_first_64gb_h(b: &mut Bencher) {
        let mut bitmap = create_bitmap(64 * 1024 * 1024 * 1024 / 4096);
        bitmap.set_free_bit(bitmap.len() - 1);
        b.iter(|| bitmap.find_first_free_index_h());
    }
}
