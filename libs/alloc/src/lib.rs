#![no_std]
#![feature(allocator_api)]
#![feature(layout_for_ptr)]
#![feature(ptr_internals)]
// TODO: think about if this is necessary
#![deny(unsafe_op_in_unsafe_fn)]

#[macro_use]
mod macros;

pub mod boxed;
pub mod collections;
pub mod iter;
pub mod raw_vec;
pub mod vec;
