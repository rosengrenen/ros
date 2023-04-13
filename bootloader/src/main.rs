#![no_std]
#![no_main]

#[no_mangle]
pub extern "efiapi" fn efi_main(
    image_handle: uefi::Handle,
    system_table: *const uefi::SystemTable,
) -> uefi::Status {
    let text = "Hello world!\r\n";
    let mut buffer = [0u16; 15];
    for (i, b) in text.as_bytes().iter().enumerate() {
        buffer[i] = *b as _;
    }
    buffer[14] = 0;
    unsafe {
        let out = (*system_table).con_out;
        let res = ((*out).output_string)(out, buffer.as_ptr());
    }

    unsafe {
        let conin = (*system_table).con_in;
        ((*conin).reset)(conin, false);
        let not_ready = 0x8000000000000000 | 6;
        let mut key = uefi::InputKey::default();
        while ((*conin).read_key_stroke)(conin, &mut key as *mut _) == not_ready {}
    }

    return 0;
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    loop {}
}

mod uefi {
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
        pub hdr: TableHeader,
        pub firmware_vendor: *const u16,
        pub firmware_revision: u32,
        pub console_in_handle: Handle,
        pub con_in: *const SimpleTextInputProtocol,
        pub console_out_handle: Handle,
        pub con_out: *const SimpleTextOutputProtocol,
        pub standard_error_handle: Handle,
        pub std_err: *const SimpleTextOutputProtocol,
        // pub runtime_services: *mut efi_runtime_services,
        pub runtime_services: *const c_void,
        pub boot_services: BootServices,
        pub number_of_table_entries: usize,
        // pub configuration_table: *mut efi_configuration_table,
        pub configuration_table: *const c_void,
    }

    #[repr(C)]
    pub struct BootServices(u8);
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

    #[derive(Clone, Copy, Default)]
    #[repr(C)]
    pub struct InputKey {
        pub scan_code: u16,
        pub unicode_char: u16,
    }

    #[repr(C)]
    pub struct SimpleTextInputProtocol {
        pub reset: extern "efiapi" fn(
            this: *const SimpleTextInputProtocol,
            extended_verification: bool,
        ) -> Status,
        pub read_key_stroke: fn(this: *const SimpleTextInputProtocol, key: *mut InputKey) -> Status,
        pub wait_for_key: *const c_void,
    }

    #[repr(C)]
    pub struct SimpleTextOutputProtocol {
        pub reset: *const c_void,
        pub output_string:
            extern "efiapi" fn(this: *const SimpleTextOutputProtocol, string: *const u16) -> Status,
        pub test_string: *const c_void,
        pub query_mode: *const c_void,
        pub set_mode: *const c_void,
        pub set_attribute: *const c_void,
        pub clear_screen: *const c_void,
        pub set_cursor_position: *const c_void,
        pub enable_cursor: *const c_void,
        pub mode: *const c_void,
    }
}
