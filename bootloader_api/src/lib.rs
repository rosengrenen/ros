#![no_std]

use core::fmt;

#[repr(C)]
pub struct BootInfo {
    // kernel - code stuffs base and size, and stack
    pub kernel: Kernel,
    // memory regions, ranges and types
    pub memory_regions: MemoryRegions,
    pub allocated_frame_ranges: AllocatedFrameRanges,
    // acpi rsdp
    pub rsdp: *const core::ffi::c_void,
}

impl fmt::Debug for BootInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("BootInfo")
            .field("kernel", &self.kernel)
            .field("memory_regions", &&self.memory_regions[..])
            .field("allocated_frame_ranges", &&self.allocated_frame_ranges[..])
            .field("rsdp", &self.rsdp)
            .finish()
    }
}

#[derive(Debug)]
#[repr(C)]
pub struct Kernel {
    /// Physical address of kernel base
    pub base: u64,
    /// Number of frames the kernel occupies
    pub num_frames: usize,
    /// Physical address
    pub stack_start: u64,
    pub stack_end: u64,
}

#[derive(Debug)]
#[repr(C)]
pub struct MemoryRegions {
    pub ptr: *const MemoryRegion,
    pub len: usize,
}

impl core::ops::Deref for MemoryRegions {
    type Target = [MemoryRegion];

    fn deref(&self) -> &Self::Target {
        unsafe { core::slice::from_raw_parts(self.ptr, self.len) }
    }
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct MemoryRegion {
    pub ty: MemoryRegionType,
    pub start: u64,
    pub end: u64,
}

#[derive(Clone, Copy, Debug, PartialEq)]
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

#[derive(Debug)]
#[repr(C)]
pub struct AllocatedFrameRanges {
    pub ptr: *const AllocatedFrameRange,
    pub len: usize,
}

impl core::ops::Deref for AllocatedFrameRanges {
    type Target = [AllocatedFrameRange];

    fn deref(&self) -> &Self::Target {
        unsafe { core::slice::from_raw_parts(self.ptr, self.len) }
    }
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]

pub struct AllocatedFrameRange {
    // TOOD: PhysAddr
    pub base: u64,
    pub frames: usize,
}

#[deny(improper_ctypes_definitions)]
extern "C" fn _ffi_safety(_info: BootInfo) {}
