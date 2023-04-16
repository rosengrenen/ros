use core::ffi::c_void;

use crate::{Status, TableHeader};

impl BootServices {
    pub fn get_memory_map(
        &self,
        mut memory_map_size: usize,
        memory_map: *mut c_void,
    ) -> Result<(usize, usize, usize, u32, usize), usize> {
        // let mut memory_map = MemoryDescriptor::default();
        let mut map_key = 0;
        let mut descriptor_size = 0;
        let mut descriptor_version = 0;
        let status = (self.get_memory_map)(
            &mut memory_map_size,
            memory_map as _,
            &mut map_key,
            &mut descriptor_size,
            &mut descriptor_version,
        );
        // if status != 0 {
        //     return Err(status);
        // }

        Ok((
            memory_map_size,
            map_key,
            descriptor_size,
            descriptor_version,
            status,
        ))
    }

    pub fn allocate_pool(&self, pool_type: MemoryType, size: usize) -> Result<*mut c_void, usize> {
        let mut buffer = core::ptr::null_mut();
        let status = (self.allocate_pool)(pool_type, size, &mut buffer);
        if status != 0 {
            return Err(status);
        }

        Ok(buffer)
    }

    pub fn free_pool(&self, buffer: *const c_void) -> Result<(), usize> {
        let status = (self.free_pool)(buffer);
        if status != 0 {
            return Err(status);
        }

        Ok(())
    }
}

#[repr(C)]
pub struct BootServices {
    pub header: TableHeader,

    // Task Priority Services
    pub raise_tpl: extern "efiapi" fn() -> Status, // EFI 1.0+
    pub restore_tpl: extern "efiapi" fn() -> Status, // EFI 1.0+

    // Memory Services
    pub allocate_pages: extern "efiapi" fn() -> Status, // EFI 1.0+
    pub free_pages: extern "efiapi" fn() -> Status,     // EFI 1.0+
    /// UEFI Spec 2.10 section 7.2.3
    pub get_memory_map: extern "efiapi" fn(
        memory_map_size: &mut usize,
        memory_map: *mut MemoryDescriptor,
        map_key: &mut usize,
        descriptor_size: &mut usize,
        descriptor_version: &mut u32,
    ) -> Status, // EFI 1.0+
    pub allocate_pool:
        extern "efiapi" fn(pool_type: MemoryType, size: usize, buffer: *mut *mut c_void) -> Status, // EFI 1.0+
    pub free_pool: extern "efiapi" fn(buffer: *const c_void) -> Status, // EFI 1.0+

    // Event & Timer Services
    pub create_event: extern "efiapi" fn() -> Status, // EFI 1.0+
    pub set_timer: extern "efiapi" fn() -> Status,    // EFI 1.0+
    pub wait_for_event: extern "efiapi" fn() -> Status, // EFI 1.0+
    pub signal_event: extern "efiapi" fn() -> Status, // EFI 1.0+
    pub close_event: extern "efiapi" fn() -> Status,  // EFI 1.0+
    pub check_event: extern "efiapi" fn() -> Status,  // EFI 1.0+

    // Protocol Handler Services
    pub install_protocol_interface: extern "efiapi" fn() -> Status, // EFI 1.
    pub reinstall_protocol_interface: extern "efiapi" fn() -> Status, // EFI 1.
    pub uninstall_protocol_interface: extern "efiapi" fn() -> Status, // EFI 1.
    pub handle_protocol: extern "efiapi" fn() -> Status,            // EFI 1.
    pub reserved: *const c_void,                                    // EFI 1.0+
    pub register_protocol_notify: extern "efiapi" fn() -> Status,   // EFI 1.
    pub locate_handle: extern "efiapi" fn() -> Status,              // EFI 1.
    pub locate_device_path: extern "efiapi" fn() -> Status,         // EFI 1.
    pub install_configuration_table: extern "efiapi" fn() -> Status, // EFI 1.

    // Image Services
    pub load_image: extern "efiapi" fn() -> Status, // EFI 1.0+
    pub start_image: extern "efiapi" fn() -> Status, // EFI 1.0+
    pub exit: extern "efiapi" fn() -> Status,       // EFI 1.0+
    pub unload_image: extern "efiapi" fn() -> Status, // EFI 1.0+
    pub exit_boot_services: extern "efiapi" fn() -> Status, // EFI 1.0+

    // Miscellaneous Services
    pub get_next_monotonic_count: extern "efiapi" fn() -> Status, // EFI 1.0+
    pub stall: extern "efiapi" fn() -> Status,                    // EFI 1.0+
    pub set_watchdog_timer: extern "efiapi" fn() -> Status,       // EFI 1.0+

    // DriverSupport Services
    pub connect_controller: extern "efiapi" fn() -> Status, // EFI 1.1
    pub disconnect_controller: extern "efiapi" fn() -> Status, // EFI 1.1+

    // Open and Close Protocol Services
    pub open_protocol: extern "efiapi" fn() -> Status, // EFI 1.1+
    pub close_protocol: extern "efiapi" fn() -> Status, // EFI 1.1+
    pub open_protocol_information: extern "efiapi" fn() -> Status, // EFI 1.1+

    // Library Services
    pub protocols_per_handle: extern "efiapi" fn() -> Status, // EFI 1.1+
    pub locate_handle_buffer: extern "efiapi" fn() -> Status, // EFI 1.1+
    pub locate_protocol: extern "efiapi" fn() -> Status,      // EFI 1.1+
    pub install_multiple_protocol_interfaces: extern "efiapi" fn() -> Status, // EFI 1.1+
    pub uninstall_multiple_protocol_interfaces: extern "efiapi" fn() -> Status, // EFI 1.1+*

    // 32-bit CRC Services
    pub calculate_crc32: extern "efiapi" fn() -> Status, // EFI 1.1+

    // Miscellaneous Services
    pub copy_mem: extern "efiapi" fn() -> Status, // EFI 1.1+
    pub set_mem: extern "efiapi" fn() -> Status,  // EFI 1.1+
    pub create_event_ex: extern "efiapi" fn() -> Status, // UEFI 2.0+
}

/// UEFI Spec 2.10 section 7.2.3
#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct MemoryDescriptor {
    pub ty: u32,
    pub physical_start: u64,
    pub virtual_start: u64,
    pub number_of_pages: u64,
    pub attribute: u64,
}

/// UEFI Spec 2.10 section 7.2.1
#[repr(C)]
pub enum AllocateType {
    AllocateAnyPages,
    AllocateMaxAddress,
    AllocateAddress,
    MaxAllocateType,
}

/// UEFI Spec 2.10 section 7.2.1
#[repr(C)]
pub enum MemoryType {
    EfiReservedMemoryType,
    EfiLoaderCode,
    EfiLoaderData,
    EfiBootServicesCode,
    EfiBootServicesData,
    EfiRuntimeServicesCode,
    EfiRuntimeServicesData,
    EfiConventionalMemory,
    EfiUnusableMemory,
    EfiACPIReclaimMemory,
    EfiACPIMemoryNVS,
    EfiMemoryMappedIO,
    EfiMemoryMappedIOPortSpace,
    EfiPalCode,
    EfiPersistentMemory,
    EfiUnacceptedMemoryType,
    EfiMaxMemoryType,
}
