#![cfg_attr(not(test), no_std)]
#![cfg_attr(test, feature(vec_into_raw_parts))]
#![cfg_attr(test, feature(test))]
#![feature(pointer_byte_offsets)]
#![deny(unsafe_op_in_unsafe_fn)]

#[cfg(test)]
extern crate test;

mod bitmap;
mod layered_bitmap;
mod region;
mod util;

use self::{bitmap::Bitmap, region::Region};
use alloc::raw_vec::RawVec;
use bitmap::BitmapLayout;
use core::{
    alloc::{Layout, LayoutError},
    cmp,
    mem::MaybeUninit,
};
use region::RegionError;

#[derive(Debug)]
pub struct BuddyAllocator<const ORDERS: usize, const FRAME_SIZE: usize> {
    regions: RawVec<Region<ORDERS, FRAME_SIZE>>,
    regions_cache: [Bitmap; ORDERS],
}

#[derive(Clone, Debug)]
pub enum BuddyAllocatorError {
    RegionTooSmall,
    MaxCapacity,
    Layout(LayoutError),
}

impl From<LayoutError> for BuddyAllocatorError {
    fn from(error: LayoutError) -> Self {
        Self::Layout(error)
    }
}

#[derive(Clone, Debug)]
pub enum AllocateError {
    NotEnoughSpace,
}

impl<const ORDERS: usize, const FRAME_SIZE: usize> BuddyAllocator<ORDERS, FRAME_SIZE> {
    pub fn new(
        base: usize,
        frames: usize,
        regions_capacity: usize,
    ) -> Result<Self, BuddyAllocatorError> {
        let layout = BuddyAllocatorLayout::<ORDERS, FRAME_SIZE>::new(regions_capacity)?;
        let meta_frames = layout.meta_frames();
        if meta_frames > frames {
            return Err(BuddyAllocatorError::RegionTooSmall);
        }

        let regions = unsafe {
            RawVec::from_raw_parts(base as *mut Region<ORDERS, FRAME_SIZE>, regions_capacity)
        };
        #[allow(clippy::uninit_assumed_init)]
        let mut regions_cache: [Bitmap; ORDERS] = unsafe { MaybeUninit::uninit().assume_init() };
        for (order, region_cache_layout) in layout.region_caches.iter().enumerate() {
            let bitmap = unsafe {
                Bitmap::from_raw_parts(
                    (base + region_cache_layout.offset) as *mut u64,
                    region_cache_layout.layout,
                )
            };
            regions_cache[order] = bitmap;
        }

        let mut me = Self {
            regions,
            regions_cache,
        };
        me.add_region(base + meta_frames * FRAME_SIZE, frames - meta_frames)?;
        Ok(me)
    }

    pub fn add_region(&mut self, base: usize, frames: usize) -> Result<(), BuddyAllocatorError> {
        self.add_region_inner(base, frames)?;
        self.sort_regions();
        Ok(())
    }

    pub fn add_regions(
        &mut self,
        regions: impl Iterator<Item = (usize, usize)>,
    ) -> Result<(), BuddyAllocatorError> {
        for (base, frames) in regions {
            self.add_region_inner(base, frames)?;
        }

        self.sort_regions();
        Ok(())
    }

    pub fn allocate(&mut self, order: usize) -> Result<usize, AllocateError> {
        for o in order..ORDERS {
            if let Some(region_index) = self.regions_cache[o].find_first_free_index_from(0, 0) {
                let allocation = match self.regions[region_index].allocate(order) {
                    Some(allocation) => allocation,
                    None => {
                        unreachable!("free index should always result in successful allocation")
                    }
                };
                for order in allocation.allocated_order..=allocation.split_order {
                    self.update_region_cache(order, region_index);
                }

                return Ok(allocation.addr);
            }
        }

        Err(AllocateError::NotEnoughSpace)
    }

    pub fn deallocate(&mut self, addr: usize) {
        let region_index = self.regions.binary_search_by(|region| {
            let region_start = region.usable_frames_base;
            let region_end = region_start + region.usable_frames * FRAME_SIZE;
            if addr < region_start {
                cmp::Ordering::Less
            } else if addr > region_end {
                cmp::Ordering::Greater
            } else {
                cmp::Ordering::Equal
            }
        });
        if let Ok(region_index) = region_index {
            let region = &mut self.regions[region_index];
            let deallocation = region.deallocate(addr).unwrap();
            for order in deallocation.deallocated_order..=deallocation.merge_order {
                self.update_region_cache(order, region_index);
            }
        }
    }

    fn add_region_inner(&mut self, base: usize, frames: usize) -> Result<(), BuddyAllocatorError> {
        let region = Region::new(base, frames).map_err(|e| match e {
            RegionError::TooSmall => BuddyAllocatorError::RegionTooSmall,
        })?;
        for order in 0..ORDERS {
            if region.counts[order] > 0 {
                self.regions_cache[order].set(self.regions.len(), 0);
            }
        }

        self.regions
            .push(region)
            .map_err(|_| BuddyAllocatorError::MaxCapacity)?;
        Ok(())
    }

    fn update_region_cache(&mut self, order: usize, region_index: usize) {
        let region = &mut self.regions[region_index];
        if region.counts[order] == 0 {
            self.regions_cache[order].clear(region_index, 0);
        } else {
            self.regions_cache[order].set(region_index, 0);
        }
    }

    fn sort_regions(&mut self) {
        self.regions
            .sort_unstable_by_key(|region| region.usable_frames_base);
    }
}

#[derive(Clone, Copy, Debug)]
struct CacheBitmapLayout {
    layout: BitmapLayout,
    offset: usize,
}

#[derive(Clone, Copy, Debug)]
struct BuddyAllocatorLayout<const ORDERS: usize, const FRAME_SIZE: usize> {
    layout: Layout,
    region_caches: [CacheBitmapLayout; ORDERS],
}

impl<const ORDERS: usize, const FRAME_SIZE: usize> BuddyAllocatorLayout<ORDERS, FRAME_SIZE> {
    fn new(regions_capacity: usize) -> Result<Self, LayoutError> {
        let regions_layout = Layout::array::<RawVec<Region<ORDERS, FRAME_SIZE>>>(regions_capacity)?;
        let mut combined_layout = regions_layout;
        let region_cache_layout = BitmapLayout::new(regions_capacity, 1)?;
        #[allow(clippy::uninit_assumed_init)]
        let mut region_caches: [CacheBitmapLayout; ORDERS] =
            unsafe { MaybeUninit::uninit().assume_init() };
        for region_cache in region_caches.iter_mut() {
            let (layout, offset) = combined_layout.extend(region_cache_layout.layout)?;
            *region_cache = CacheBitmapLayout {
                layout: region_cache_layout,
                offset,
            };
            combined_layout = layout;
        }

        Ok(Self {
            layout: combined_layout,
            region_caches,
        })
    }

    fn meta_frames(&self) -> usize {
        self.layout.size().div_ceil(FRAME_SIZE)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn print_allocator(allocator: &BuddyAllocator<5, 4096>) {
        let region = &allocator.regions[0];
        println!("--------------------------------------------------------------------");
        region.print_allocated();
        for order in 0..5 {
            println!(
                "has {}: {}",
                order,
                allocator.regions_cache[order].get(0, 0)
            );
        }
        region.print_free();
    }

    #[test]
    fn test_name() {
        let mut mem: Vec<u8> = Vec::new();
        mem.resize(128 * 1024, 0);
        let mem_ptr = mem.as_ptr().cast::<u8>();
        let offset = mem_ptr.align_offset(4096);
        let mem_ptr = unsafe { mem_ptr.add(offset) };
        let mut allocator =
            BuddyAllocator::<5, 4096>::new(mem_ptr as usize, mem.len() / 4096 - 1, 64).unwrap();
        print_allocator(&allocator);
        let mut stack = [0; 10];
        for i in 0..10 {
            stack[i] = allocator.allocate(0).unwrap();
        }
        print_allocator(&allocator);
        let a = allocator.allocate(2).unwrap();
        print_allocator(&allocator);
        let b = allocator.allocate(3).unwrap();
        print_allocator(&allocator);
        let c = allocator.allocate(1).unwrap();
        print_allocator(&allocator);
        allocator.deallocate(a);
        print_allocator(&allocator);
        allocator.deallocate(b);
        print_allocator(&allocator);
        allocator.deallocate(c);
        print_allocator(&allocator);
        for i in 0..10 {
            allocator.deallocate(stack[i]);
        }
        print_allocator(&allocator);

        assert!(false);
    }
}
