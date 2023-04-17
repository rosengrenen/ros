#![no_std]
#![no_main]

#[macro_use]
extern crate alloc;

use uefi::{
    services::filesystem::{self, FileSystem},
    string::String16,
};

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

    let fs = st
        .boot_services
        .locate_protocol::<FileSystem>(&filesystem::PROTOCOL_GUID)
        .unwrap();
    let root_fs = unsafe { &*fs.open_volume().unwrap() };
    let file_name: String16 = "banner.txt".parse().unwrap();
    let file = unsafe { &*root_fs.open(&file_name, 0x3, 0x0).unwrap() };
    let info = file.get_info().unwrap();
    let mut buffer = vec![0u8; info.file_size as usize];
    let read_bytes = file.read(&mut buffer).unwrap();
    let file_string = core::str::from_utf8(&buffer).unwrap();
    // print_str(&format!("{:?}", info), None);
    print_str(
        &format!(
            "buffer size: {}, read bytes: {}, contents: {:?}",
            buffer.len(),
            read_bytes,
            file_string
        ),
        None,
    );

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

    let string: String16 = string.parse().unwrap();
    st.con_out.output_string(&string).unwrap();
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    print_str(&format!("{}", info), None);
    loop {}
}
