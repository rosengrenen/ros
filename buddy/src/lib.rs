// #![no_std]
#![deny(unsafe_op_in_unsafe_fn)]
#![feature(vec_into_raw_parts)]
#![feature(test)]
#![feature(pointer_byte_offsets)]

extern crate test;

mod bitmap;
mod layered_bitmap;
mod layout;
mod region;

use self::region::Region;
use core::ptr::NonNull;

#[derive(Debug)]
#[repr(C)]
pub struct BuddyAllocator<const ORDER: usize, const FRAME_SIZE: usize> {
    regions_head: Option<NonNull<Region<ORDER, FRAME_SIZE>>>,
}

impl<const ORDER: usize, const FRAME_SIZE: usize> BuddyAllocator<ORDER, FRAME_SIZE> {
    pub fn new() -> Self {
        Self { regions_head: None }
    }

    pub fn add_region(&mut self, base: usize, frames: usize) {
        let region_ptr = unsafe { NonNull::new_unchecked(base as *mut Region<ORDER, FRAME_SIZE>) };
        let region = Region::new(self.regions_head, base, frames);
        unsafe {
            region_ptr.as_ptr().write(region);
        }
        self.regions_head = Some(region_ptr);
    }

    pub fn allocate(&mut self, order: usize) -> Result<usize, ()> {
        let region = self.regions_head;
        while let Some(mut region_ptr) = region {
            let region = unsafe { region_ptr.as_mut() };
            if let Some(addr) = region.allocate(order) {
                return Ok(addr);
            }
        }

        Err(())
    }

    pub fn deallocate(&mut self, addr: usize) {
        let region = self.regions_head;
        while let Some(mut region_ptr) = region {
            let region = unsafe { region_ptr.as_mut() };
            let region_start = region.usable_frames_base;
            let region_end = region_start + region.usable_frames * FRAME_SIZE;
            if (region_start..region_end).contains(&addr) {
                region.deallocate(addr);
                return;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {
        let mut mem: Vec<u8> = Vec::new();
        mem.resize(128 * 1024, 0);
        let mut allocator = BuddyAllocator::<5, 4096>::new();
        let mem_ptr = mem.as_ptr().cast::<u8>();
        let offset = mem_ptr.align_offset(4096);
        let mem_ptr = unsafe { mem_ptr.add(offset) };
        allocator.add_region(mem_ptr as usize, mem.len() / 4096 - 1);
        let p = move || {
            let region = unsafe { allocator.regions_head.unwrap().as_ref() };
            println!("--------------------------------------------------------------------");
            println!("{}", region);
        };
        p();
        let mut stack = [0; 10];
        for i in 0..10 {
            stack[i] = allocator.allocate(0).unwrap();
        }
        p();
        let a = allocator.allocate(2).unwrap();
        p();
        let b = allocator.allocate(3).unwrap();
        p();
        let c = allocator.allocate(1).unwrap();
        p();
        allocator.deallocate(a);
        p();
        allocator.deallocate(b);
        p();
        allocator.deallocate(c);
        p();
        for i in 0..10 {
            allocator.deallocate(stack[i]);
        }
        p();

        assert!(false);
    }
}
