#![no_std]
#![no_main]

#[macro_use]
extern crate alloc;

mod elf;

use alloc::vec::Vec;
use uefi::{
    services::filesystem::{self, FileSystem},
    string::String16,
};

use crate::elf::get_elf_entry_point_offset;

static mut SYSTEM_TABLE: Option<&'static uefi::SystemTable> = None;

pub fn system_table() -> &'static uefi::SystemTable {
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

    let entry_point_offset = get_elf_entry_point_offset(&buffer).unwrap();
    print_str(
        &format!("Entry point offset: {}", entry_point_offset),
        Some((5, 5)),
    );

    // TODO
    // * read executable file DONE
    // * find the location of the main entry point
    // * do some setup, idk what this is tho
    // *

    wait_for_key();

    return 0;
}

// static efi_status_t load_elf_image(struct elf_app *elf)
// {
// 	uint64_t size = elf->page_size + (elf->image_end - elf->image_begin);
// 	uint64_t addr;
// 	uint16_t start_msg[] = u"Loading ELF image...\r\n";
// 	uint16_t finish_msg[] = u"Loaded ELF image\r\n";
// 	efi_status_t status;

// 	// Allocate the required number of pages
// 	status = elf->system->boot->allocate_pages(
// 		EFI_ALLOCATE_ANY_PAGES, EFI_LOADER_DATA, size / elf->page_size, &addr);
// 	if (status != EFI_SUCCESS)
// 		return status;

// 	// Save some bookkeeping information for cleanup in case of errors
// 	elf->image_pages = size / elf->page_size;
// 	elf->image_addr = addr;

// 	// Entry point has to be adjusted, given that the ELF image might not
// 	// be loaded at the addresses stored in program headers
// 	elf->image_entry = elf->image_addr + elf->page_size
// 		+ elf->header.e_entry - elf->image_begin;

// 	// Fill in everything with zeros, whatever data is not read from the
// 	// ELF file itself has to be zero-initialized.
// 	memset((void *)elf->image_addr, 0, size);

// 	// Go over all program headers and load their content in memory
// 	for (size_t i = 0; i < elf->header.e_phnum; ++i) {
// 		const struct elf64_phdr *phdr = &elf->program_headers[i];
// 		uint64_t phdr_addr;

// 		if (phdr->p_type != PT_LOAD)
// 			continue;

// 		phdr_addr = elf->image_addr + elf->page_size
// 			+ phdr->p_vaddr - elf->image_begin;
// 		status = efi_read_fixed(
// 			elf->kernel, phdr->p_offset, phdr->p_filesz, (void *)phdr_addr);
// 		if (status != EFI_SUCCESS)
// 			return status;
// 	}

// 	return elf->system->out->output_string(elf->system->out, finish_msg);
// }

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
    print_str(&format!("{}", info), None);
    loop {}
}
