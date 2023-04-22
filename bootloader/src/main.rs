#![no_std]
#![no_main]

#[macro_use]
extern crate alloc;

mod elf;
mod print;
mod x86_64;

use core::fmt::Debug;

use alloc::vec::Vec;
use elf::get_elf_entry_point_offset;
use uefi::{
    allocator,
    services::{
        boot::{AllocateType, MemoryType},
        filesystem::{self, FileSystem},
        graphics::{self, BltPixel, Graphics},
    },
    string::String16,
};

use crate::print::{clear_screen, print_mem_map, print_page_table, print_regs, wait_for_key};

static mut SYSTEM_TABLE: Option<&'static uefi::SystemTable<uefi::Boot>> = None;
static mut SYSTEM_TABLE_RT: Option<&'static uefi::SystemTable<uefi::Runtime>> = None;

pub fn system_table() -> &'static uefi::SystemTable<uefi::Boot> {
    unsafe { SYSTEM_TABLE.expect("System table global variable not available") }
}

pub fn system_table_rt() -> &'static uefi::SystemTable<uefi::Runtime> {
    unsafe { SYSTEM_TABLE_RT.expect("System table global variable not available") }
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
    st: uefi::SystemTable<uefi::Boot>,
) -> uefi::Status {
    unsafe {
        SYSTEM_TABLE = Some(core::mem::transmute(&st));
    }
    let gop = st
        .boot_services()
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
    uefi::init(&st);

    // Set best console output mode
    let modes = (0..st.con_out().mode.max_mode)
        .filter_map(|mode_number| {
            st.con_out()
                .query_mode(mode_number as _)
                .ok()
                .map(|mode| (mode_number, mode))
        })
        .collect::<Vec<_>>();
    let (best_mode_number, _) = modes.last().unwrap();
    st.con_out().set_mode(*best_mode_number as _).unwrap();
    clear_screen();

    // print_regs();
    // print_mem_map();
    // print_page_table();
    wait_for_key();

    // Read kernel executable
    let fs = st
        .boot_services()
        .locate_protocol::<FileSystem>(&filesystem::PROTOCOL_GUID)
        .unwrap();
    let root_fs = unsafe { &*fs.open_volume().unwrap() };
    let file_name: String16 = "ros".parse().unwrap();
    let file = unsafe { &*root_fs.open(&file_name, 0x3, 0x0).unwrap() };
    let info = file.get_info().unwrap();
    let mut buffer = vec![0u8; info.file_size as usize];
    let read_bytes = file.read(&mut buffer).unwrap();
    buffer.truncate(read_bytes);

    let entry_point_fn = get_elf_entry_point_offset(&buffer).unwrap();

    // Allocate 128KiB for stack
    let stack = st
        .boot_services()
        .allocate_pages(
            AllocateType::AllocateAnyPages,
            MemoryType::EfiLoaderData,
            32,
        )
        .unwrap();
    let stack_start = stack;
    let stack_end = stack_start + 128 * 1024;

    clear_screen();
    println!("stack start {}", stack_start);
    wait_for_key();

    // Physical memory that needs to be identity mapped
    //  * Efi structures?
    //  * Kernel?
    //  * Existing page tables?

    // These two have to be called next to each other
    let (st, mem_map) = st.exit_boot_services(image_handle).unwrap();
    unsafe {
        SYSTEM_TABLE = None;
        SYSTEM_TABLE_RT = Some(core::mem::transmute(&st));
    }
    // Jump to kernel
    let g = gfx();
    let stack_end = stack_end;
    unsafe {
        core::arch::asm!("mov rsp, {}; jmp {}",
          in(reg) stack_end,
          in(reg) entry_point_fn,
          in("rdi") g.buffer,
          in("rsi") g.width,
          in("rdx") g.height
        );
    }

    unreachable!("should have jumped to kernel at this point")
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    if allocator::allocator_enabled() {
        println!("{}", info);
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
