#![no_std]
#![allow(unused)]

use common::addr::PhysAddr;

pub mod control;
pub mod efer;
pub mod flags;
pub mod gdt;
pub mod idt;
pub mod paging;
