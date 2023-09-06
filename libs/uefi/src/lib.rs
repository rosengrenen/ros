// CRC32 The 32-bit CRC for the entire table. This value is computed by setting
// this field to 0, and computing the 32-bit CRC for HeaderSize bytes.

// NOTE: The capabilities found in the EFI system table, runtime table and boot
// services table may change over time. The first field in each of these tables
// is an EFI_TABLE_HEADER. This header’s Revision field is incremented when new
// capabilities and functions are added to the functions in the table. When
// checking for capabilities, code should verify that Revision is greater than
// or equal to the revision level of the table at the point when the
// capabilities were added to the UEFI specification.

// NOTE: The size of the system table, runtime services table, and boot
//  services table may increase over time. It is very important to always use
// the HeaderSize field of the EFI_TABLE_HEADER to determine the size of these
// tables.

#![no_std]
#![feature(allocator_api)]

pub mod allocator;
pub mod services;
pub mod string;
mod table;

use core::{ffi::c_void, marker::PhantomData};

use services::boot::Guid;

/// UEFI Spec 2.10 section 4.2.1
#[derive(Clone, Copy, Debug)]
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

pub struct Uninit;

pub struct Boot;

pub struct Runtime;

#[repr(C)]
pub struct SystemTable<S> {
    pub inner: &'static SystemTableImpl,
    _marker: PhantomData<S>,
}

impl SystemTable<Uninit> {
    pub fn init(self) -> SystemTable<Boot> {
        unsafe { core::mem::transmute(self) }
    }
}

impl SystemTable<Boot> {
    pub fn con_in(&self) -> &services::console::ConsoleInput {
        self.inner.con_in
    }

    pub fn con_out(&self) -> &services::console::ConsoleOutput {
        self.inner.con_out
    }

    pub fn std_err(&self) -> &services::console::ConsoleOutput {
        self.inner.std_err
    }

    pub fn boot_services(&self) -> &services::boot::BootServices {
        self.inner.boot_services
    }

    pub fn configuration_table(&self) -> &[ConfigurationTable] {
        unsafe {
            core::slice::from_raw_parts(
                self.inner.configuration_table,
                self.inner.number_of_table_entries,
            )
        }
    }

    pub fn exit_boot_services(
        self,
        image_handle: Handle,
        memory_map_key: usize,
    ) -> Result<SystemTable<Runtime>, usize> {
        self.boot_services()
            .exit_boot_services(image_handle, memory_map_key)?;
        Ok(unsafe { core::mem::transmute(self) })
    }
}

/// UEFI Spec 2.10 section 4.3.1
#[repr(C)]
pub struct SystemTableImpl {
    header: TableHeader,
    firmware_vendor: *const u16,
    firmware_revision: u32,
    console_in_handle: Handle,
    con_in: &'static services::console::ConsoleInput,
    console_out_handle: Handle,
    con_out: &'static services::console::ConsoleOutput,
    standard_error_handle: Handle,
    std_err: &'static services::console::ConsoleOutput,
    pub runtime_services: *const c_void,
    boot_services: &'static services::boot::BootServices,
    number_of_table_entries: usize,
    configuration_table: *const ConfigurationTable,
}

#[derive(Debug)]
#[repr(C)]
pub struct ConfigurationTable {
    pub vendor_guid: Guid,
    pub vendor_table: *const c_void,
}
