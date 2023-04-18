#![no_std]
#![no_main]

#[macro_use]
extern crate alloc;

mod elf;

use alloc::vec::Vec;
use uefi::{
    allocator,
    services::{
        filesystem::{self, FileSystem},
        graphics::{self, BltPixel, Graphics, PixelFormat},
    },
    string::String16,
};

use crate::elf::get_elf_entry_point_offset;

static mut SYSTEM_TABLE: Option<&'static uefi::SystemTable> = None;

pub fn system_table() -> &'static uefi::SystemTable {
    unsafe { SYSTEM_TABLE.expect("System table global variable not available") }
}

#[derive(Clone, Copy, Debug)]
pub struct GraphicsBuffer {
    pub buffer: *mut BltPixel,
    pub width: usize,
    pub height: usize,
}

impl GraphicsBuffer {
    pub fn buf(&self) -> &'static mut [BltPixel] {
        unsafe { core::slice::from_raw_parts_mut(self.buffer, self.width * self.height) }
    }
}

static mut GRAPHICS_BUFFER: Option<GraphicsBuffer> = None;

pub fn gfx() -> GraphicsBuffer {
    unsafe { GRAPHICS_BUFFER.unwrap() }
}

#[no_mangle]
pub extern "efiapi" fn efi_main(
    image_handle: uefi::Handle,
    system_table: uefi::SystemTable,
) -> uefi::Status {
    let st = system_table.inner;
    unsafe {
        SYSTEM_TABLE = Some(core::mem::transmute(&system_table));
    }
    let gop = st
        .boot_services
        .locate_protocol::<Graphics>(&graphics::PROTOCOL_GUID)
        .unwrap();
    let gfx_buffer = gop.mode.frame_buffer_base as *mut BltPixel;
    let buffer_w = gop.mode.info.horizontal_resolution as usize;
    let buffer_h = gop.mode.info.vertical_resolution as usize;
    unsafe {
        GRAPHICS_BUFFER = Some(GraphicsBuffer {
            buffer: gfx_buffer,
            width: buffer_w,
            height: buffer_h,
        })
    }
    // core::mem::forget(&gfx_buffer);
    // let mut buf = vec![0u8; core::mem::size_of::<GraphicsBuffer>()];
    // core::mem::forget(&buf);
    // let buf = buf.as_mut_ptr() as *mut GraphicsBuffer;
    // unsafe {
    //     *buf = GraphicsBuffer {
    //         buffer: core::mem::transmute(gfx_buffer),
    //         width: buffer_w,
    //         height: buffer_h,
    //     };
    //     GRAPHICS_BUFFER = buf;
    // }
    uefi::init(&system_table);

    st.con_out.reset(false).unwrap();
    print_str("Hello world!\r\n", None);
    // print_mem_map();
    st.con_out.reset(false).unwrap();

    let fs = st
        .boot_services
        .locate_protocol::<FileSystem>(&filesystem::PROTOCOL_GUID)
        .unwrap();
    let root_fs = unsafe { &*fs.open_volume().unwrap() };
    let file_name: String16 = "ros".parse().unwrap();
    let file = unsafe { &*root_fs.open(&file_name, 0x3, 0x0).unwrap() };
    let info = file.get_info().unwrap();
    let mut buffer = vec![0u8; info.file_size as usize];
    let read_bytes = file.read(&mut buffer).unwrap();
    buffer.truncate(read_bytes);

    gop.blt(
        BltPixel {
            red: 100,
            green: 0,
            blue: 255,
            reserved: 255,
        },
        0,
        0,
        200,
        100,
    )
    .unwrap();

    wait_for_key();

    let mem_map = st.boot_services.get_memory_map().unwrap();
    match st
        .boot_services
        .exit_boot_services(image_handle, mem_map.key)
    {
        Ok(_) => {
            for x in 250..300 {
                for y in 300..600 {
                    gfx().buf()[buffer_w * y + x] = BltPixel {
                        blue: 0,
                        green: 255,
                        red: 0,
                        reserved: 255,
                    };
                }
            }
        }
        Err(_) => {
            for x in 250..300 {
                for y in 300..600 {
                    gfx().buf()[buffer_w * y + x] = BltPixel {
                        blue: 0,
                        green: 0,
                        red: 255,
                        reserved: 255,
                    };
                }
            }
        }
    }

    // let entry_point_offset = get_elf_entry_point_offset(&buffer).unwrap();
    // print_str(
    //     &format!("Entry point offset: {}", entry_point_offset),
    //     Some((5, 5)),
    // );

    // // TODO
    // // * read executable file DONE
    // // * find the location of the main entry point
    // // * do some setup, idk what this is tho
    // // *

    // wait_for_key();

    return 0;
}

fn print_mem_map() {
    let st = system_table().inner;
    let memory_map = st.boot_services.get_memory_map().unwrap();
    let mut total_ram_kb = 0;
    // Note: EFI page sizes are always 4096 bytes
    const EFI_PAGE_SIZE_BYTES: u64 = 4096;
    const KB_IN_BYTES: u64 = 1024;

    let mut memory_map: Vec<_> = memory_map.iter().copied().collect();
    memory_map.sort_by_key(|m| m.physical_start);

    let memory_map_len = memory_map.len();
    st.con_out.reset(false).unwrap();
    print_str(
        &format!(
            "Showing {}-{} / {}",
            1,
            20.min(memory_map_len),
            memory_map_len
        ),
        Some((0, 0)),
    );
    let mut prev_end = 0;
    for (i, desc) in memory_map.iter().enumerate() {
        if i != 0 && i % 20 == 0 {
            wait_for_key();
            st.con_out.reset(false).unwrap();
            print_str(
                &format!(
                    "Showing {}-{} / {}",
                    i + 1,
                    (i + 20).min(memory_map_len),
                    memory_map_len
                ),
                Some((0, 0)),
            );
        }

        let end = desc.physical_start + desc.number_of_pages * EFI_PAGE_SIZE_BYTES;
        let mut size = desc.number_of_pages * EFI_PAGE_SIZE_BYTES / KB_IN_BYTES;
        let unit = if size < 1024 {
            "K"
        } else if size < 1024 * 1024 {
            size /= 1024;
            "M !"
        } else {
            size /= 1024 * 1024;
            "G !!!"
        };
        print_str(
            &format!(
                "{:<30}: {:#010x} {:#010x} {:#010x} {:<6}{}",
                mem_type_str(desc.ty),
                desc.physical_start,
                end,
                desc.physical_start - prev_end,
                size,
                unit
            ),
            Some((0, 3 + i % 20)),
        );

        total_ram_kb += desc.number_of_pages * EFI_PAGE_SIZE_BYTES / KB_IN_BYTES;

        prev_end = end;
    }

    wait_for_key();
    st.con_out.reset(false).unwrap();
    print_str(&format!("Total ram: {}", total_ram_kb), Some((0, 1)));
    wait_for_key();
}

fn print_str(string: &str, pos: Option<(usize, usize)>) {
    let st = system_table().inner;
    if let Some((col, row)) = pos {
        st.con_out.set_cursor_position(col, row).unwrap();
    }

    let string: String16 = string.parse().unwrap();
    st.con_out.output_string(&string).unwrap();
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::print_str(&format!("{}", format_args!($($arg)*)), None));
}

fn mem_type_str(mem_type: u32) -> &'static str {
    match mem_type {
        0 => "EfiReservedMemoryType",
        1 => "EfiLoaderCode",
        2 => "EfiLoaderData",
        3 => "EfiBootServicesCode",
        4 => "EfiBootServicesData",
        5 => "EfiRuntimeServicesCode",
        6 => "EfiRuntimeServicesData",
        7 => "EfiConventionalMemory",
        8 => "EfiUnusableMemory",
        9 => "EfiACPIReclaimMemory",
        10 => "EfiACPIMemoryNVS",
        11 => "EfiMemoryMappedIO",
        12 => "EfiMemoryMappedIOPortSpace",
        13 => "EfiPalCode",
        14 => "EfiPersistentMemory",
        15 => "EfiUnacceptedMemoryType",
        16 => "EfiMaxMemoryType",
        _ => "",
    }
}

fn wait_for_key() {
    let st = system_table().inner;
    st.con_in.reset(false).unwrap();
    loop {
        match st.con_in.read_key() {
            Ok(_key) => break,
            Err(_status) => continue,
        }
    }
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    if allocator::allocator_enabled() {
        // print_str(&format!("{}", info), None);
    } else {
        let g = gfx();
        for i in 0..g.width * g.height {
            g.buf()[i] = BltPixel {
                blue: 0,
                green: 0,
                red: 255,
                reserved: 255,
            }
        }
    }

    loop {}
}
