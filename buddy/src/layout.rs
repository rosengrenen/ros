use crate::layered_bitmap::BuddyBitmap;
use core::alloc::{Layout, LayoutError};

#[derive(Clone, Copy, Debug)]
pub struct RegionLayout<const ORDER: usize> {
    pub bitmap_offsets: [Option<usize>; ORDER],
    pub usable_frames: usize,
    pub usable_base_offset: usize,
}

impl<const ORDERS: usize> RegionLayout<ORDERS> {
    pub fn new(
        frame_size: usize,
        num_frames: usize,
        max_usable_frames: usize,
        max_order: usize,
    ) -> Result<Self, RegionLayoutError> {
        let mut layout = Layout::array::<BuddyBitmap>(max_order)?;
        let mut bitmap_offsets = [None; ORDERS];
        for order in 0..ORDERS {
            let num_entries = max_usable_frames / 2usize.pow(order as u32);
            if num_entries == 0 {
                break;
            }

            let (bitmap_layout, offset) = layout.extend(BuddyBitmap::layout(num_entries))?;
            bitmap_offsets[order] = Some(offset);
            layout = bitmap_layout;
        }

        let num_meta_frames = (layout.size() + frame_size - 1) / frame_size;
        if num_meta_frames > num_frames {
            return Err(RegionLayoutError::RegionTooSmall);
        }

        Ok(Self {
            bitmap_offsets,
            usable_frames: num_frames - num_meta_frames,
            usable_base_offset: num_meta_frames * frame_size,
        })
    }
}

#[derive(Debug)]
pub enum RegionLayoutError {
    RegionTooSmall,
    LayoutError(LayoutError),
}

impl From<LayoutError> for RegionLayoutError {
    fn from(error: LayoutError) -> Self {
        Self::LayoutError(error)
    }
}
