#![no_std]

#[derive(Debug)]
#[repr(C)]
pub struct BootInfo {
    // kernel - code stuffs base and size, and stack
    pub kernel: Kernel,
    // memory regions, ranges and types
    pub memory_regions: MemoryRegions,
    // acpi rsdp
    pub rsdp: *const core::ffi::c_void,
}

#[derive(Debug)]
#[repr(C)]
pub struct Kernel {
    /// Physical address of kernel base
    pub base: u64,
    /// Number of frames the kernel occupies
    pub frames: usize,
    /// Physical address
    pub stack_base: usize,
}

#[derive(Debug)]
#[repr(C)]
pub struct MemoryRegions {
    pub ptr: *const MemoryRegion,
    pub len: usize,
}

#[derive(Debug)]
#[repr(C)]
pub struct MemoryRegion {
    pub ty: MemoryRegionType,
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, PartialEq)]
#[repr(C)]
pub enum MemoryRegionType {
    KernelUsable,
    ReservedMemoryType,
    EfiRuntimeServicesCode,
    EfiRuntimeServicesData,
    UnusableMemory,
    ACPIReclaimMemory,
    ACPIMemoryNVS,
    MemoryMappedIO,
    MemoryMappedIOPortSpace,
    PalCode,
    UnacceptedMemoryType,
}

#[repr(C)]
pub enum ReservedMemoryType {
    KernelCode,
    KernelStack,
    PageTable,
    BootInfo,
}

#[deny(improper_ctypes_definitions)]
extern "C" fn _ffi_safety(_info: BootInfo) {}
