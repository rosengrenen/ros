#![no_std]

use uefi::{Runtime, SystemTable};

#[repr(C)]
pub struct BootInfo {
    pub uefi_system_table: SystemTable<Runtime>,
    // framebuffer
    pub framebuffer: Framebuffer,
    // kernel - code stuffs base and size, and stack
    pub kernel: Kernel,
    // memory regions
    pub memory_regions: MemoryRegions,
    pub reserved_memory_regions: ReservedMemoryRegions,
}

#[repr(C)]
pub struct Framebuffer {
    pub base: usize,
    pub width: usize,
    pub height: usize,
    // Pixel type?
}

#[repr(C)]
pub struct Kernel {
    /// Physical address of kernel base
    pub base: u64,
    /// Number of frames the kernel occupies
    pub frames: usize,
    /// Physical address
    pub stack_base: usize,
}

#[repr(C)]
pub struct MemoryRegions {
    pub ptr: *const MemoryRegion,
    pub len: usize,
}

#[repr(C)]
pub struct MemoryRegion {
    pub start: usize,
    pub end: usize,
}

#[repr(C)]
pub struct ReservedMemoryRegions {
    pub ptr: *const ReservedMemoryRegion,
    pub len: usize,
}

#[repr(C)]
pub struct ReservedMemoryRegion {
    pub start: usize,
    pub end: usize,
    pub ty: ReservedMemoryType,
}

#[repr(C)]
pub enum ReservedMemoryType {
    KernelCode,
    KernelStack,
    UefiPageTable,
    BootInfo,
    Framebuffer,
}

#[deny(improper_ctypes_definitions)]
extern "C" fn _ffi_safety(_info: BootInfo) {}
