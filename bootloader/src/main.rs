#![no_std]
#![no_main]

extern crate alloc;

use alloc::vec::Vec;
use uefi::{services::boot::MemoryDescriptor, string::CString16};

#[no_mangle]
pub extern "efiapi" fn efi_main(
    image_handle: uefi::Handle,
    system_table: uefi::SystemTable,
) -> uefi::Status {
    uefi::init(&system_table);
    let st = system_table.inner;

    st.con_out.reset(false);
    printstring("Hello world!\r\n", &system_table, 0);

    let memory_map = st.boot_services.get_memory_map().unwrap();
    let buffer_size = memory_map.buffer.len();
    let descs = memory_map.iter().copied().collect::<Vec<_>>();
    let descs_size = descs.len() * core::mem::size_of::<MemoryDescriptor>();
    let mut total_ram = 0;
    for (i, desc) in memory_map.iter().enumerate() {
        total_ram += desc.number_of_pages * 4096 / 1024;
    }
    print10(total_ram as _, &system_table, 0);
    print10(buffer_size as _, &system_table, 1);
    print10(descs_size as _, &system_table, 2);

    st.con_in.reset(false);
    loop {
        match st.con_in.read_key() {
            Ok(_key) => break,
            Err(_status) => continue,
        }
    }

    return 0;
}

fn printmem(mut mem: *const u8, st: &uefi::SystemTable, row: usize) {
    st.inner.con_out.set_cursor_position(0, row);

    let buf = [0; 1];
    for i in 0..100 {
        printchar(b' ' as _, st);

        let c = unsafe { *mem.offset(i) };
        printhexdigit((c >> 4) as _, st);
        printhexdigit((c & 0xF) as _, st);
    }
}

fn printhexdigit(digit: u8, st: &uefi::SystemTable) {
    if digit < 10 {
        printchar((digit + b'0') as _, st);
    } else if digit < 16 {
        printchar((digit + b'a' - 10) as _, st);
    }
}

fn printchar(c: u16, st: &uefi::SystemTable) {
    let buffer = [c, 0x0];
    let string = CString16(buffer.as_ptr() as _);
    st.inner.con_out.output_string(string);
}

fn printstring(string: &str, system_table: &uefi::SystemTable, row: usize) {
    let st = system_table.inner;
    st.con_out.set_cursor_position(0, row);

    let mut buffer = [0u16; 128];
    for (i, u) in string.encode_utf16().enumerate() {
        buffer[i] = u;
    }

    let string = CString16(buffer.as_ptr() as *const _);
    st.con_out.output_string(string);
}

fn print10(value: u64, st: &uefi::SystemTable, row: usize) {
    let st = st.inner;
    st.con_out.set_cursor_position(0, row);
    let mut buffer = [0u16; 32];
    itoa(&mut buffer, value as _, 10);
    let string = CString16(buffer.as_ptr() as *const _);
    st.con_out.output_string(string);
}

fn printmemtype(ty: u32, st: &uefi::SystemTable, row: usize) {
    match ty {
        0 => (),  //printstring("EfiReservedMemoryType", st, row),
        1 => (),  //printstring("EfiLoaderCode", st, row),
        2 => (),  //printstring("EfiLoaderData", st, row),
        3 => (),  //printstring("EfiBootServicesCode", st, row),
        4 => (),  //printstring("EfiBootServicesData", st, row),
        5 => (),  //printstring("EfiRuntimeServicesCode", st, row),
        6 => (),  //printstring("EfiRuntimeServicesData", st, row),
        7 => (),  //printstring("EfiConventionalMemory", st, row),
        8 => (),  //printstring("EfiUnusableMemory", st, row),
        9 => (),  //printstring("EfiACPIReclaimMemory", st, row),
        10 => (), //printstring("EfiACPIMemoryNVS", st, row),
        11 => (), //printstring("EfiMemoryMappedIO", st, row),
        12 => (), //printstring("EfiMemoryMappedIOPortSpace", st, row),
        13 => (), //printstring("EfiPalCode", st, row),
        14 => (), //printstring("EfiPersistentMemory", st, row),
        15 => (), //printstring("EfiUnacceptedMemoryType", st, row),
        16 => (), //printstring("EfiMaxMemoryType", st, row),
        _ => printstring("Unknown", st, row),
    }
}

fn itoa(buffer: &mut [u16], mut value: i64, radix: i64) -> usize {
    let mut tmp = [0u8; 16];
    // char tmp[16];// be careful with the length of the buffer
    // char *tp = tmp;
    // int i;
    // unsigned v;

    let sign = radix == 10 && value < 0;
    if sign {
        value = -value;
    }

    let mut len = 0;
    loop {
        if len != 0 && value == 0 {
            break;
        }
        let digit = (value % radix) as u8;
        value /= radix;
        if digit < 10 {
            tmp[len] = digit + b'0';
        } else {
            tmp[len] = digit + b'a' - 10;
        }

        len += 1;
    }

    if sign {
        tmp[len] = b'-';
        len += 1;
    }

    for (i, b) in tmp[..len].into_iter().rev().enumerate() {
        buffer[i] = *b as _;
    }

    return len;
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
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
