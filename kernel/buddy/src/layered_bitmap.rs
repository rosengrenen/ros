use crate::{
    bitmap::{Bitmap, BitmapLayout, BitmapVecEntry},
    util::ilog_ceil,
};
use alloc::raw_vec::RawVec;
use core::alloc::{Layout, LayoutError};

#[derive(Debug)]
pub struct LayeredBitmap {
    layers: RawVec<Bitmap>,
}

impl LayeredBitmap {
    const CACHE_ENTRY_BITS: usize = Bitmap::ENTRY_BITS * Bitmap::ENTRY_BITS;

    pub unsafe fn from_raw_parts(base: *mut Bitmap, layout: LayeredBitmapLayout) -> Self {
        Self {
            layers: Self::create_layers(base, layout),
        }
    }

    pub fn len(&self) -> usize {
        self.layers[0].len()
    }

    #[cfg(test)]
    pub fn find_first_free_index(&self) -> Option<usize> {
        self.layers[0].find_first_free_index_from(0)
    }

    pub fn find_first_free_index_h(&self) -> Option<usize> {
        let mut index = 0;
        for layer in (1..self.layers.len()).rev() {
            match self.layers[layer].find_first_free_index_from(index) {
                Some(found_index) => {
                    index = found_index * Bitmap::ENTRY_BITS;
                }
                None => return None,
            }
        }

        self.layers[0].find_first_free_index_from(index)
    }

    pub fn get(&self, index: usize) -> Option<bool> {
        self.layers[0].get_checked(index)
    }

    pub fn set(&mut self, index: usize) {
        let original_entry = self.layers[0].get_entry(Bitmap::entry_index(index));
        self.layers[0].set(index);
        self.cache_set(index, 0, original_entry);
    }

    pub fn clear(&mut self, index: usize) {
        self.layers[0].clear(index);
        self.cache_clear(index, 0);
    }

    fn cache_set(&mut self, index: usize, layer: usize, original_entry: BitmapVecEntry) {
        if original_entry != 0 {
            return;
        }

        if layer == self.layers.len() - 1 {
            return;
        }

        let entry_index = Bitmap::entry_index(index);
        let higher_layer_bit_index = entry_index / Bitmap::ENTRY_BITS;
        let higher_layer_entry_index = Bitmap::entry_index(higher_layer_bit_index);
        let higher_layer_entry = self.layers[layer + 1].get_entry(higher_layer_entry_index);
        self.layers[layer + 1].set(higher_layer_bit_index);
        self.cache_set(higher_layer_bit_index, layer + 1, higher_layer_entry);
    }

    fn cache_clear(&mut self, index: usize, layer: usize) {
        if layer == self.layers.len() - 1 {
            return;
        }

        let entry_index = Bitmap::entry_index(index);
        let field = self.layers[layer].get_entry(entry_index);
        if field == 0 {
            // It is now full, report upward
            let higher_layer_bit_index = entry_index / Bitmap::ENTRY_BITS;
            self.layers[layer + 1].clear(higher_layer_bit_index);
            self.cache_clear(higher_layer_bit_index, layer + 1);
        }
    }

    fn create_layers(base: *mut Bitmap, layout: LayeredBitmapLayout) -> RawVec<Bitmap> {
        let mut layers = unsafe { RawVec::from_raw_parts(base, layout.num_layers) };
        for layer_meta in layout.layers {
            if let Some(layer_meta) = layer_meta {
                let layer = unsafe {
                    Bitmap::from_raw_parts(
                        base.byte_add(layer_meta.offset).cast(),
                        layer_meta.layout,
                    )
                };
                layers.push(layer).unwrap();
            } else {
                break;
            }
        }

        layers
    }
}

#[derive(Clone, Copy, Debug)]
struct LayerLayout {
    layout: BitmapLayout,
    offset: usize,
}

#[derive(Clone, Copy, Debug)]
pub struct LayeredBitmapLayout {
    pub layout: Layout,
    layers: [Option<LayerLayout>; 5],
    num_layers: usize,
}

impl LayeredBitmapLayout {
    pub fn new(len: usize) -> Result<Self, LayoutError> {
        let num_layers = ilog_ceil(LayeredBitmap::CACHE_ENTRY_BITS, len).max(1);
        let layers_layout = Layout::array::<Bitmap>(num_layers)?;
        let main_layer_layout = BitmapLayout::new(len)?;
        let mut layers = [None; 5];
        let (mut combined_layout, offset) = layers_layout.extend(main_layer_layout.layout)?;
        layers[0] = Some(LayerLayout {
            layout: main_layer_layout,
            offset,
        });
        if num_layers > 1 {
            let mut cache_layer_entries = Self::cache_entries_from_entries(len);
            for layer in layers.iter_mut().take(num_layers).skip(1) {
                let cache_layer_layout = BitmapLayout::new(cache_layer_entries)?;
                let (layout, offset) = combined_layout.extend(cache_layer_layout.layout)?;
                *layer = Some(LayerLayout {
                    layout: cache_layer_layout,
                    offset,
                });
                combined_layout = layout;
                cache_layer_entries = Self::cache_entries_from_entries(cache_layer_entries);
            }
        }

        Ok(Self {
            layout: combined_layout,
            layers,
            num_layers,
        })
    }

    fn cache_entries_from_entries(bits: usize) -> usize {
        bits.div_ceil(LayeredBitmap::CACHE_ENTRY_BITS)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    fn create_bitmap(entries: usize) -> LayeredBitmap {
        let layout = LayeredBitmapLayout::new(entries).unwrap();
        let mut mem: Vec<u64> = Vec::new();
        mem.resize((layout.layout.size() + 7) / 8, 0);
        let (ptr, _, _) = mem.into_raw_parts();
        unsafe { LayeredBitmap::from_raw_parts(ptr.cast(), layout) }
    }

    #[test]
    fn set() {
        let mut bitmap = create_bitmap(200);
        bitmap.set(127);
        assert_eq!(bitmap.find_first_free_index(), Some(127));
        assert_eq!(bitmap.find_first_free_index_h(), Some(127));
        bitmap.set(7);
        assert_eq!(bitmap.find_first_free_index(), Some(7));
        assert_eq!(bitmap.find_first_free_index_h(), Some(7));
    }

    #[test]
    fn lol() {
        let mut bitmap = create_bitmap(64 * 1024 * 1024 * 1024 / 4096);
        println!("{:?}", bitmap);
        bitmap.set(bitmap.len() - 1);
        assert_eq!(bitmap.find_first_free_index(), Some(bitmap.len() - 1));
        assert_eq!(bitmap.find_first_free_index_h(), Some(bitmap.len() - 1));
    }

    #[bench]
    fn bench_find_first_64kb_best(b: &mut Bencher) {
        let mut bitmap = create_bitmap(64 * 1024 / 4096);
        bitmap.set(0);
        b.iter(|| bitmap.find_first_free_index());
    }

    #[bench]
    fn bench_find_first_64mb_best(b: &mut Bencher) {
        let mut bitmap = create_bitmap(64 * 1024 * 1024 / 4096);
        bitmap.set(0);
        b.iter(|| bitmap.find_first_free_index());
    }

    #[bench]
    fn bench_find_first_64gb_best(b: &mut Bencher) {
        let mut bitmap = create_bitmap(64 * 1024 * 1024 * 1024 / 4096);
        bitmap.set(0);
        b.iter(|| bitmap.find_first_free_index());
    }

    #[bench]
    fn bench_find_first_64kb_avg(b: &mut Bencher) {
        let mut bitmap = create_bitmap(64 * 1024 / 4096);
        bitmap.set(bitmap.len() / 2);
        b.iter(|| bitmap.find_first_free_index());
    }

    #[bench]
    fn bench_find_first_64mb_avg(b: &mut Bencher) {
        let mut bitmap = create_bitmap(64 * 1024 * 1024 / 4096);
        bitmap.set(bitmap.len() / 2);
        b.iter(|| bitmap.find_first_free_index());
    }

    #[bench]
    fn bench_find_first_64gb_avg(b: &mut Bencher) {
        let mut bitmap = create_bitmap(64 * 1024 * 1024 * 1024 / 4096);
        bitmap.set(bitmap.len() / 2);
        b.iter(|| bitmap.find_first_free_index());
    }

    #[bench]
    fn bench_find_first_64kb_worst(b: &mut Bencher) {
        let mut bitmap = create_bitmap(64 * 1024 / 4096);
        bitmap.set(bitmap.len() - 1);
        b.iter(|| bitmap.find_first_free_index());
    }

    #[bench]
    fn bench_find_first_64mb_worst(b: &mut Bencher) {
        let mut bitmap = create_bitmap(64 * 1024 * 1024 / 4096);
        bitmap.set(bitmap.len() - 1);
        b.iter(|| bitmap.find_first_free_index());
    }

    #[bench]
    fn bench_find_first_64gb_worst(b: &mut Bencher) {
        let mut bitmap = create_bitmap(64 * 1024 * 1024 * 1024 / 4096);
        bitmap.set(bitmap.len() - 1);
        b.iter(|| bitmap.find_first_free_index());
    }

    #[bench]
    fn bench_find_first_64kb_h_best(b: &mut Bencher) {
        let mut bitmap = create_bitmap(64 * 1024 / 4096);
        bitmap.set(0);
        b.iter(|| bitmap.find_first_free_index_h());
    }

    #[bench]
    fn bench_find_first_64mb_h_best(b: &mut Bencher) {
        let mut bitmap = create_bitmap(64 * 1024 * 1024 / 4096);
        bitmap.set(0);
        b.iter(|| bitmap.find_first_free_index_h());
    }

    #[bench]
    fn bench_find_first_64gb_h_best(b: &mut Bencher) {
        let mut bitmap = create_bitmap(64 * 1024 * 1024 * 1024 / 4096);
        bitmap.set(0);
        b.iter(|| bitmap.find_first_free_index_h());
    }

    #[bench]
    fn bench_find_first_64kb_h_avg(b: &mut Bencher) {
        let mut bitmap = create_bitmap(64 * 1024 / 4096);
        bitmap.set(bitmap.len() / 2);
        b.iter(|| bitmap.find_first_free_index_h());
    }

    #[bench]
    fn bench_find_first_64mb_h_avg(b: &mut Bencher) {
        let mut bitmap = create_bitmap(64 * 1024 * 1024 / 4096);
        bitmap.set(bitmap.len() / 2);
        b.iter(|| bitmap.find_first_free_index_h());
    }

    #[bench]
    fn bench_find_first_64gb_h_avg(b: &mut Bencher) {
        let mut bitmap = create_bitmap(64 * 1024 * 1024 * 1024 / 4096);
        bitmap.set(bitmap.len() / 2);
        b.iter(|| bitmap.find_first_free_index_h());
    }

    #[bench]
    fn bench_find_first_64kb_h_worst(b: &mut Bencher) {
        let mut bitmap = create_bitmap(64 * 1024 / 4096);
        bitmap.set(bitmap.len() - 1);
        b.iter(|| bitmap.find_first_free_index_h());
    }

    #[bench]
    fn bench_find_first_64mb_h_worst(b: &mut Bencher) {
        let mut bitmap = create_bitmap(64 * 1024 * 1024 / 4096);
        bitmap.set(bitmap.len() - 1);
        b.iter(|| bitmap.find_first_free_index_h());
    }

    #[bench]
    fn bench_find_first_64gb_h_worst(b: &mut Bencher) {
        let mut bitmap = create_bitmap(64 * 1024 * 1024 * 1024 / 4096);
        bitmap.set(bitmap.len() - 1);
        b.iter(|| bitmap.find_first_free_index_h());
    }
}
