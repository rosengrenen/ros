#![no_std]

#[macro_use]
extern crate alloc;

mod allocator;
pub mod services;
pub mod string;
mod table;

use core::ffi::c_void;

/// UEFI Spec 2.10 section 4.2.1
#[repr(C)]
pub struct TableHeader {
    pub signature: u64,
    pub revision: u32,
    pub header_size: u32,
    pub crc32: u32,
    _reserved: u32,
}

pub type Handle = *const c_void;

pub type Status = usize;

#[repr(C)]
pub struct SystemTable {
    pub inner: &'static SystemTableImpl,
}

/// UEFI Spec 2.10 section 4.3.1
#[repr(C)]
pub struct SystemTableImpl {
    pub header: TableHeader,
    pub firmware_vendor: *const u16,
    pub firmware_revision: u32,
    console_in_handle: Handle,
    pub con_in: &'static services::console::ConsoleInput,
    console_out_handle: Handle,
    pub con_out: &'static services::console::ConsoleOutput,
    standard_error_handle: Handle,
    pub std_err: &'static services::console::ConsoleOutput,
    pub runtime_services: *const c_void,
    pub boot_services: &'static services::boot::BootServices,
    pub number_of_table_entries: usize,
    // pub configuration_table: *mut efi_configuration_table,
    pub configuration_table: *const c_void,
}

pub fn init(system_table: &SystemTable) {
    allocator::enable(system_table.inner.boot_services);
}
