#![no_std]

pub mod services;
pub mod string;
mod table;

use core::ffi::c_void;

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

// struct BootServices {
//     hdr: TableHeader,

//     // Task Priority Services
//     raise_tpl: EFI_RAISE_TPL,     // EFI 1.0+
//     restore_tpl: EFI_RESTORE_TPL, // EFI 1.0+

//     // Memory Services
//     allocate_pages: EFI_ALLOCATE_PAGES, // EFI 1.0+
//     free_pages: EFI_FREE_PAGES,         // EFI 1.0+
//     get_memory_map: EFI_GET_MEMORY_MAP, // EFI 1.0+
//     allocate_pool: EFI_ALLOCATE_POOL,   // EFI 1.0+
//     free_pool: EFI_FREE_POOL,           // EFI 1.0+

//     // Event & Timer Services
//     create_event: EFI_CREATE_EVENT,     // EFI 1.0+
//     set_timer: EFI_SET_TIMER,           // EFI 1.0+
//     wait_for_event: EFI_WAIT_FOR_EVENT, // EFI 1.0+
//     signal_event: EFI_SIGNAL_EVENT,     // EFI 1.0+
//     close_event: EFI_CLOSE_EVENT,       // EFI 1.0+
//     check_event: EFI_CHECK_EVENT,       // EFI 1.0+

//     // Protocol Handler Services
//     install_protocol_interface: EFI_INSTALL_PROTOCOL_INTERFACE, // EFI 1.0+
//     reinstall_protocol_interface: EFI_REINSTALL_PROTOCOL_INTERFACE, // EFI 1.0+
//     uninstall_protocol_interface: EFI_UNINSTALL_PROTOCOL_INTERFACE, // EFI 1.0+
//     handle_protocol: EFI_HANDLE_PROTOCOL,                       // EFI 1.0+
//     reserved: *mut void,                                        // EFI 1.0+
//     register_protocol_notify: EFI_REGISTER_PROTOCOL_NOTIFY,     // EFI  1.0+
//     locate_handle: EFI_LOCATE_HANDLE,                           // EFI 1.0+
//     locate_device_path: EFI_LOCATE_DEVICE_PATH,                 // EFI 1.0+
//     install_configuration_table: EFI_INSTALL_CONFIGURATION_TABLE, // EFI 1.0+

//     // Image Services
//     load_image: EFI_IMAGE_UNLOAD,               // EFI 1.0+
//     start_image: EFI_IMAGE_START,               // EFI 1.0+
//     exit: EFI_EXIT,                             // EFI 1.0+
//     unload_image: EFI_IMAGE_UNLOAD,             // EFI 1.0+
//     exit_boot_services: EFI_EXIT_BOOT_SERVICES, // EFI 1.0+

//     // Miscellaneous Services
//     get_next_monotonic_count: EFI_GET_NEXT_MONOTONIC_COUNT, // EFI 1.0+
//     stall: EFI_STALL,                                       // EFI 1.0+
//     set_watchdog_timer: EFI_SET_WATCHDOG_TIMER,             // EFI 1.0+

//     // DriverSupport Services
//     connect_controller: EFI_CONNECT_CONTROLLER, // EFI 1.1
//     disconnect_controller: EFI_DISCONNECT_CONTROLLER, // EFI 1.1+

//     // Open and Close Protocol Services
//     open_protocol: EFI_OPEN_PROTOCOL,   // EFI 1.1+
//     close_protocol: EFI_CLOSE_PROTOCOL, // EFI 1.1+
//     open_protocol_information: EFI_OPEN_PROTOCOL_INFORMATION, // EFI 1.1+

//     // Library Services
//     protocols_per_handle: EFI_PROTOCOLS_PER_HANDLE, // EFI 1.1+
//     locate_handle_buffer: EFI_LOCATE_HANDLE_BUFFER, // EFI 1.1+
//     locate_protocol: EFI_LOCATE_PROTOCOL,           // EFI 1.1+
//     install_multiple_protocol_interfaces: EFI_UNINSTALL_MULTIPLE_PROTOCOL_INTERFACES, // EFI 1.1+
//     uninstall_multiple_protocol_interfaces: EFI_UNINSTALL_MULTIPLE_PROTOCOL_INTERFACES, // EFI 1.1+*

//     // 32-bit CRC Services
//     calculate_crc32: EFI_CALCULATE_CRC32, // EFI 1.1+

//     // Miscellaneous Services
//     copy_mem: EFI_COPY_MEM,               // EFI 1.1+
//     set_mem: EFI_SET_MEM,                 // EFI 1.1+
//     create_event_ex: EFI_CREATE_EVENT_EX, // UEFI 2.0+
// }

// struct ConfigurationTable {
//     vendor_guid: EFI_GUID,
//     vendor_table: *const void,
// }
