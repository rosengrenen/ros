#![no_std]
#![no_main]

#[macro_use]
extern crate alloc;

mod elf;
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

use crate::x86_64::{
    control, efer, flags, gdt,
    paging::{PageDirPointerTable, PageDirTable, PageMapLevel4Table, PageTable},
};

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
    uefi::init(&system_table);

    // Set best console output mode
    let modes = (0..st.con_out.mode.max_mode)
        .filter_map(|mode_number| {
            st.con_out
                .query_mode(mode_number as _)
                .ok()
                .map(|mode| (mode_number, mode))
        })
        .collect::<Vec<_>>();
    let (best_mode_number, _) = modes.last().unwrap();
    st.con_out.set_mode(*best_mode_number as _).unwrap();
    clear_screen();

    // print_regs();
    // print_mem_map();
    // print_page_table();
    wait_for_key();

    // Read kernel executable
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

    let entry_point_fn = get_elf_entry_point_offset(&buffer).unwrap();

    // Allocate 128KiB for stack
    let stack = st
        .boot_services
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

    // These two have to be called next to each other
    let mem_map = st.boot_services.get_memory_map().unwrap();
    st.boot_services
        .exit_boot_services(image_handle, mem_map.key)
        .unwrap();
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

fn translate_addr(virt: u64) -> Option<u64> {
    let mask = 0x000F_FFFF_FFFF_F000;
    let pml4_index = virt >> 39 & 0x1FF;
    let page_dir_ptr_index = virt >> 30 & 0x1FF;
    let page_dir_index = virt >> 21 & 0x1FF;
    let page_index = virt >> 12 & 0x1FF;
    let pml4_table_addr = control::Cr3::read().pba_pml4;
    let pml4_table = unsafe { &*(pml4_table_addr as *const PageMapLevel4Table) };
    let pml4_entry = pml4_table.entries[pml4_index as usize];
    if pml4_entry.0 & 0x1 == 0 {
        return None;
    }

    let page_dir_ptr_table_addr = pml4_entry.0 & mask;
    // println!("page_dir_ptr_table_addr: {}", page_dir_ptr_table_addr);
    let page_dir_ptr_table = unsafe { &*(page_dir_ptr_table_addr as *const PageDirPointerTable) };
    let page_dir_ptr_entry = page_dir_ptr_table.entries[page_dir_ptr_index as usize];
    if page_dir_ptr_entry.0 & 0x1 == 0 {
        return None;
    }

    if page_dir_ptr_entry.0 & (1 << 7) != 0 {
        return Some(page_dir_ptr_entry.0 & 0x000F_FFFF_C000_0000 | (virt & 0x3FFF_FFFF));
    }
    let page_dir_table_addr = page_dir_ptr_entry.0 & mask;
    // println!("page_dir_table_addr: {}", page_dir_table_addr);
    let page_dir_table = unsafe { &*(page_dir_table_addr as *const PageDirTable) };
    let page_dir_entry = page_dir_table.entries[page_dir_index as usize];
    if page_dir_entry.0 & 0x1 == 0 {
        return None;
    }

    if page_dir_entry.0 & (1 << 7) != 0 {
        return Some(page_dir_entry.0 & 0x000F_FFFF_FFE0_0000 | (virt & 0x001F_FFFF));
    }
    let page_table_addr = page_dir_entry.0 & mask;
    // println!("page_table_addr: {}", page_table_addr);
    let page_table = unsafe { &*(page_table_addr as *const PageTable) };
    let page_entry = page_table.entries[page_index as usize];
    if page_entry.0 & 0x1 == 0 {
        return None;
    }

    Some(page_entry.0 & mask | (virt & 0xFFF))
}

#[allow(dead_code)]
fn print_page_table() {
    let mask = 0x000F_FFFF_FFFF_F000;
    let pml4_table_addr = control::Cr3::read().pba_pml4;
    println!("{:x?}", pml4_table_addr);
    let pml4_table = unsafe { &*(pml4_table_addr as *const PageMapLevel4Table) };
    let mut num_1gb_pages = 0;
    let mut num_2mb_pages = 0;
    let mut num_4kb_pages = 0;
    // One pml4 table
    let mut total_size_bytes = 4096;
    for (i, pml4_entry) in pml4_table
        .entries
        .iter()
        .enumerate()
        .filter(|(_, e)| e.0 & 0x1 != 0)
    {
        let page_dir_ptr_table_addr = pml4_entry.0 & mask;
        let table = unsafe { &*(page_dir_ptr_table_addr as *const PageDirPointerTable) };
        total_size_bytes += 4096;
        for (j, page_dir_pointer_entry) in table
            .entries
            .iter()
            .enumerate()
            .filter(|(_, e)| (e.0 & 0x1) != 0)
        {
            if page_dir_pointer_entry.0 & (1 << 7) != 0 {
                num_1gb_pages += 1;
                // let virt_base_addr = (i << 39) | (j << 30);
                // let phys_base_addr = page_dir_pointer_entry.0 & 0x000F_FFFF_C000_0000;
                // println!("maps a 1gb page {:x} {:x}", virt_base_addr, phys_base_addr);
            } else {
                let page_dir_table_addr = page_dir_pointer_entry.0 & mask;
                let page_dir_table = unsafe { &*(page_dir_table_addr as *const PageDirTable) };
                total_size_bytes += 4096;
                for (k, page_dir_entry) in page_dir_table
                    .entries
                    .iter()
                    .enumerate()
                    .filter(|(_, e)| (e.0 & 0x1) != 0)
                {
                    if page_dir_entry.0 & (1 << 7) != 0 {
                        num_2mb_pages += 1;
                        // let virt_base_addr = (i << 39) | (j << 30) | (k << 21);
                        // let phys_base_addr = page_dir_entry.0 & 0x000F_FFFF_FFE0_0000;
                        // println!("maps a 2mb page {:x} {:x}", virt_base_addr, phys_base_addr);
                    } else {
                        let page_table_addr = page_dir_entry.0 & mask;
                        let page_table = unsafe { &*(page_table_addr as *const PageTable) };
                        total_size_bytes += 4096;
                        for (l, page_entry) in page_table
                            .entries
                            .iter()
                            .enumerate()
                            .filter(|(_, e)| (e.0 & 0x1) != 0)
                        {
                            num_4kb_pages += 1;
                            // let virt_base_addr = (i << 39) | (j << 30) | (k << 21) | (l << 12);
                            // let phys_base_addr = (page_entry.0 & mask) as usize;
                            // println!("maps as 4kb page {:#x} {:#x}", virt_base_addr, phys_base_addr);
                        }
                    }
                }
            }
        }
    }
    println!(
        "num page tables: {}, {}, {}",
        num_1gb_pages, num_2mb_pages, num_4kb_pages
    );
    println!("paging uses {} bytes", total_size_bytes);
}

#[allow(dead_code)]
fn print_regs() {
    clear_screen();
    println!("{:x?}", flags::RFlags::read());
    wait_for_key();

    clear_screen();
    println!("{:x?}", control::Cr0::read());
    wait_for_key();

    clear_screen();
    println!("{:x?}", control::Cr2::read());
    wait_for_key();

    clear_screen();
    println!("{:x?}", control::Cr3::read());
    wait_for_key();

    clear_screen();
    println!("{:x?}", control::Cr4::read());
    wait_for_key();

    clear_screen();
    println!("{:x?}", efer::Efer::read());
    wait_for_key();

    clear_screen();
    print!("{:?}", gdt::Gdtr::read());
    wait_for_key();

    for desc in gdt::GdtDesc::table_iter() {
        clear_screen();
        println!("{:#x?}", desc);
        wait_for_key();
    }

    let pcid: u64;
    unsafe {
        core::arch::asm!("mov eax, 0x1; cpuid; mov {}, rax", out(reg) pcid);
    }
    let pcid_supported = pcid & (1 << 17) != 0;
    println!("PCID supported: {}", pcid_supported);
}

#[allow(dead_code)]
fn print_mem_map() {
    let st = system_table().inner;
    let memory_map = st.boot_services.get_memory_map().unwrap();
    let mut total_ram_kb = 0;
    // Note: EFI page sizes are always 4096 bytes
    const EFI_PAGE_SIZE_BYTES: u64 = 4096;
    const KB_IN_BYTES: u64 = 1024;

    let mut memory_map: Vec<_> = memory_map.iter().copied().collect();
    memory_map.sort_by_key(|m| m.physical_start);

    const ENTRIES_PER_PAGE: usize = 3;

    let memory_map_len = memory_map.len();
    clear_screen();
    println!(
        "Showing {}-{} / {}",
        1,
        ENTRIES_PER_PAGE.min(memory_map_len),
        memory_map_len
    );
    st.con_out.set_cursor_position(0, 2).unwrap();
    for (i, desc) in memory_map.iter().enumerate() {
        if i != 0 && i % ENTRIES_PER_PAGE == 0 {
            wait_for_key();
            clear_screen();
            println!(
                "Showing {}-{} / {}",
                i + 1,
                (i + ENTRIES_PER_PAGE).min(memory_map_len),
                memory_map_len
            );
            st.con_out.set_cursor_position(0, 2).unwrap();
        }

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
        let i = desc.physical_start >> 39 & 0x1FF;
        let j = desc.physical_start >> 30 & 0x1FF;
        let k = desc.physical_start >> 21 & 0x1FF;
        let l = desc.physical_start >> 12 & 0x1FF;
        println!("{:#x?} ({:?})", desc, (i, j, k, l));

        total_ram_kb += desc.number_of_pages * EFI_PAGE_SIZE_BYTES / KB_IN_BYTES;
    }

    wait_for_key();
    clear_screen();
    print_str(&format!("Total ram: {}", total_ram_kb), Some((0, 1)));
    wait_for_key();
}

fn print_str(string: &str, pos: Option<(usize, usize)>) {
    let st = system_table().inner;
    if let Some((col, row)) = pos {
        st.con_out.set_cursor_position(col, row).unwrap();
    }

    let parts = string.split('\n').collect::<Vec<_>>();
    for (i, part) in parts.iter().enumerate() {
        if !part.is_empty() {
            let string: String16 = part.parse().unwrap();
            st.con_out.output_string(&string).unwrap();
        }

        if i != parts.len() - 1 {
            let string: String16 = "\r\n".parse().unwrap();
            st.con_out.output_string(&string).unwrap();
        }
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::print_str(&format!("{}", format_args!($($arg)*)), None));
}

#[macro_export]
macro_rules! println {
    () => (print!("\n"));
    ($($arg:tt)*) => (print!("{}\n", format_args!($($arg)*)));
}

fn clear_screen() {
    let st = system_table().inner;
    st.con_out.reset(false).unwrap();
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
