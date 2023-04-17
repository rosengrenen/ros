#![no_std]
#![no_main]

#[macro_use]
extern crate alloc;

use alloc::vec::Vec;
use uefi::string::CString16;

static mut SYSTEM_TABLE: Option<&'static uefi::SystemTable> = None;

fn system_table() -> &'static uefi::SystemTable {
    unsafe { SYSTEM_TABLE.expect("System table global variable not available") }
}

#[no_mangle]
pub extern "efiapi" fn efi_main(
    _image_handle: uefi::Handle,
    system_table: uefi::SystemTable,
) -> uefi::Status {
    unsafe {
        SYSTEM_TABLE = Some(core::mem::transmute(&system_table));
    }
    uefi::init(&system_table);
    let st = system_table.inner;

    st.con_out.reset(false).unwrap();
    print_str("Hello world!\r\n", None);
    let memory_map = st.boot_services.get_memory_map().unwrap();
    let mut total_ram = 0;
    for desc in memory_map.iter() {
        total_ram += desc.number_of_pages * 4096 / 1024;
    }

    print_str(&format!("Total ram: {}", total_ram), Some((0, 1)));

    st.con_in.reset(false).unwrap();
    loop {
        match st.con_in.read_key() {
            Ok(_key) => break,
            Err(_status) => continue,
        }
    }

    return 0;
}

fn print_str(string: &str, pos: Option<(usize, usize)>) {
    let st = system_table().inner;
    if let Some((col, row)) = pos {
        st.con_out.set_cursor_position(col, row).unwrap();
    }

    let mut buffer = string.encode_utf16().collect::<Vec<_>>();
    buffer.push(0);
    let string = CString16(buffer.as_ptr() as *const _);
    st.con_out.output_string(string).unwrap();
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    print_str(&format!("{}", info), None);
    loop {}
}

// mod uefi2 {

//     use core::ffi::c_void;

//     #[repr(C)]
//     pub struct TableHeader {
//         pub(crate) signature: u64,
//         pub(crate) revision: u32,
//         pub(crate) header_size: u32,
//         pub(crate) crc32: u32,
//         _reserved: u32,
//     }

//     pub type Handle = *const c_void;

//     pub type Status = usize;

//     #[repr(C)]
//     pub struct SystemTable {
//         pub inner: &'static SystemTableImpl,
//     }

//     #[repr(C)]
//     pub struct SystemTableImpl {
//         pub hdr: TableHeader,
//         pub firmware_vendor: *const u16,
//         pub firmware_revision: u32,
//         pub console_in_handle: Handle,
//         pub con_in: &'static ConsoleInput,
//         pub console_out_handle: Handle,
//         pub con_out: ConsoleOutput,
//         pub standard_error_handle: Handle,
//         pub std_err: ConsoleOutput,
//         // pub runtime_services: *mut efi_runtime_services,
//         pub runtime_services: *const c_void,
//         pub boot_services: BootServices,
//         pub number_of_table_entries: usize,
//         // pub configuration_table: *mut efi_configuration_table,
//         pub configuration_table: *const c_void,
//     }

//     #[repr(C)]
//     pub struct ConsoleOutput {
//         pub inner: &'static ConsoleOutputImpl,
//     }

//     impl ConsoleOutput {
//         pub fn reset(&self, extended_verification: bool) -> Result<(), usize> {
//             let status = (self.inner.reset)(self.inner, extended_verification);
//             if status != 0 {
//                 return Err(status);
//             }

//             Ok(())
//         }

//         pub fn output_string(&self, string: *const u16) -> Result<(), usize> {
//             let status = (self.inner.output_string)(self.inner, string);
//             if status != 0 {
//                 return Err(status);
//             }

//             Ok(())
//         }
//     }

//     /// UEFI Spec 2.10 section 12.4.1
//     #[repr(C)]
//     pub struct ConsoleOutputImpl {
//         /// UEFI Spec 2.10 section 12.4.2
//         pub reset: extern "efiapi" fn(&Self, extended_verification: bool) -> Status,
//         /// UEFI Spec 2.10 section 12.4.3
//         pub output_string: extern "efiapi" fn(&Self, string: *const u16) -> Status,
//     }

//     #[derive(Clone, Copy, Default)]
//     #[repr(C)]
//     pub struct Key {
//         pub scan_code: u16,
//         pub unicode_char: u16,
//     }

//     #[repr(C)]
//     pub struct ConsoleInput {
//         /// UEFI Spec 2.10 section 12.3.2
//         pub reset: extern "efiapi" fn(&Self, extended_verification: bool) -> Status,
//         /// UEFI Spec 2.10 section 12.3.3
//         pub read_key_stroke: extern "efiapi" fn(&Self, key: &mut Key) -> Status,
//         /// UEFI Spec 2.10 section 12.3.1
//         pub wait_for_key: *mut c_void,
//     }

//     #[repr(C)]
//     pub struct BootServices(u8);
//     // struct BootServices {
//     //     hdr: TableHeader,

//     //     // Task Priority Services
//     //     raise_tpl: EFI_RAISE_TPL,     // EFI 1.0+
//     //     restore_tpl: EFI_RESTORE_TPL, // EFI 1.0+

//     //     // Memory Services
//     //     allocate_pages: EFI_ALLOCATE_PAGES, // EFI 1.0+
//     //     free_pages: EFI_FREE_PAGES,         // EFI 1.0+
//     //     get_memory_map: EFI_GET_MEMORY_MAP, // EFI 1.0+
//     //     allocate_pool: EFI_ALLOCATE_POOL,   // EFI 1.0+
//     //     free_pool: EFI_FREE_POOL,           // EFI 1.0+

//     //     // Event & Timer Services
//     //     create_event: EFI_CREATE_EVENT,     // EFI 1.0+
//     //     set_timer: EFI_SET_TIMER,           // EFI 1.0+
//     //     wait_for_event: EFI_WAIT_FOR_EVENT, // EFI 1.0+
//     //     signal_event: EFI_SIGNAL_EVENT,     // EFI 1.0+
//     //     close_event: EFI_CLOSE_EVENT,       // EFI 1.0+
//     //     check_event: EFI_CHECK_EVENT,       // EFI 1.0+

//     //     // Protocol Handler Services
//     //     install_protocol_interface: EFI_INSTALL_PROTOCOL_INTERFACE, // EFI 1.0+
//     //     reinstall_protocol_interface: EFI_REINSTALL_PROTOCOL_INTERFACE, // EFI 1.0+
//     //     uninstall_protocol_interface: EFI_UNINSTALL_PROTOCOL_INTERFACE, // EFI 1.0+
//     //     handle_protocol: EFI_HANDLE_PROTOCOL,                       // EFI 1.0+
//     //     reserved: *mut void,                                        // EFI 1.0+
//     //     register_protocol_notify: EFI_REGISTER_PROTOCOL_NOTIFY,     // EFI  1.0+
//     //     locate_handle: EFI_LOCATE_HANDLE,                           // EFI 1.0+
//     //     locate_device_path: EFI_LOCATE_DEVICE_PATH,                 // EFI 1.0+
//     //     install_configuration_table: EFI_INSTALL_CONFIGURATION_TABLE, // EFI 1.0+

//     //     // Image Services
//     //     load_image: EFI_IMAGE_UNLOAD,               // EFI 1.0+
//     //     start_image: EFI_IMAGE_START,               // EFI 1.0+
//     //     exit: EFI_EXIT,                             // EFI 1.0+
//     //     unload_image: EFI_IMAGE_UNLOAD,             // EFI 1.0+
//     //     exit_boot_services: EFI_EXIT_BOOT_SERVICES, // EFI 1.0+

//     //     // Miscellaneous Services
//     //     get_next_monotonic_count: EFI_GET_NEXT_MONOTONIC_COUNT, // EFI 1.0+
//     //     stall: EFI_STALL,                                       // EFI 1.0+
//     //     set_watchdog_timer: EFI_SET_WATCHDOG_TIMER,             // EFI 1.0+

//     //     // DriverSupport Services
//     //     connect_controller: EFI_CONNECT_CONTROLLER, // EFI 1.1
//     //     disconnect_controller: EFI_DISCONNECT_CONTROLLER, // EFI 1.1+

//     //     // Open and Close Protocol Services
//     //     open_protocol: EFI_OPEN_PROTOCOL,   // EFI 1.1+
//     //     close_protocol: EFI_CLOSE_PROTOCOL, // EFI 1.1+
//     //     open_protocol_information: EFI_OPEN_PROTOCOL_INFORMATION, // EFI 1.1+

//     //     // Library Services
//     //     protocols_per_handle: EFI_PROTOCOLS_PER_HANDLE, // EFI 1.1+
//     //     locate_handle_buffer: EFI_LOCATE_HANDLE_BUFFER, // EFI 1.1+
//     //     locate_protocol: EFI_LOCATE_PROTOCOL,           // EFI 1.1+
//     //     install_multiple_protocol_interfaces: EFI_UNINSTALL_MULTIPLE_PROTOCOL_INTERFACES, // EFI 1.1+
//     //     uninstall_multiple_protocol_interfaces: EFI_UNINSTALL_MULTIPLE_PROTOCOL_INTERFACES, // EFI 1.1+*

//     //     // 32-bit CRC Services
//     //     calculate_crc32: EFI_CALCULATE_CRC32, // EFI 1.1+

//     //     // Miscellaneous Services
//     //     copy_mem: EFI_COPY_MEM,               // EFI 1.1+
//     //     set_mem: EFI_SET_MEM,                 // EFI 1.1+
//     //     create_event_ex: EFI_CREATE_EVENT_EX, // UEFI 2.0+
//     // }

//     // struct ConfigurationTable {
//     //     vendor_guid: EFI_GUID,
//     //     vendor_table: *const void,
//     // }
// }
