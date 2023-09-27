#![no_std]
#![feature(allocator_api)]
#![feature(layout_for_ptr)]
#![feature(ptr_internals)]

#[macro_use]
mod macros;

pub mod boxed;
pub mod iter;
pub mod vec;
