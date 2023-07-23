use alloc::vec::Vec;
use uefi::string::String16;

use crate::{
    print, println,
    x86_64::{
        control, efer, flags, gdt,
        paging::{PageDirPointerTable, PageDirTable, PageMapLevel4Table, PageTable},
    },
};

#[derive(Debug)]
pub enum PageSize {
    Size4KiB,
    Size2MiB,
    Size1GiB,
}

#[allow(dead_code)]
pub fn translate_addr(virt: u64) -> Option<u64> {
    let (base_addr, page_size) = addr_page_table_indices(virt)?;
    match page_size {
        PageSize::Size1GiB => Some(base_addr | (virt & 0x3FFF_FFFF)),
        PageSize::Size2MiB => Some(base_addr | (virt & 0x001F_FFFF)),
        PageSize::Size4KiB => Some(base_addr | (virt & 0xFFF)),
    }
}

pub fn addr_page_table_indices(virt: u64) -> Option<(u64, PageSize)> {
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
        return Some((
            page_dir_ptr_entry.0 & 0x000F_FFFF_C000_0000,
            PageSize::Size1GiB,
        ));
    }
    let page_dir_table_addr = page_dir_ptr_entry.0 & mask;
    // println!("page_dir_table_addr: {}", page_dir_table_addr);
    let page_dir_table = unsafe { &*(page_dir_table_addr as *const PageDirTable) };
    let page_dir_entry = page_dir_table.entries[page_dir_index as usize];
    if page_dir_entry.0 & 0x1 == 0 {
        return None;
    }

    if page_dir_entry.0 & (1 << 7) != 0 {
        return Some((page_dir_entry.0 & 0x000F_FFFF_FFE0_0000, PageSize::Size2MiB));
    }
    let page_table_addr = page_dir_entry.0 & mask;
    // println!("page_table_addr: {}", page_table_addr);
    let page_table = unsafe { &*(page_table_addr as *const PageTable) };
    let page_entry = page_table.entries[page_index as usize];
    if page_entry.0 & 0x1 == 0 {
        return None;
    }

    Some((page_entry.0 & mask, PageSize::Size4KiB))
}

#[allow(dead_code)]
pub fn print_page_table() {
    let mask = 0x000F_FFFF_FFFF_F000;
    let pml4_table_addr = control::Cr3::read().pba_pml4;
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
pub fn print_regs() {
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
pub fn print_mem_map() {
    let st = system_table();
    let memory_map = st.boot_services().get_memory_map().unwrap();
    let mut total_ram_kb = 0;
    let mut rt_usable_ram_kb = 0;
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
    st.con_out().set_cursor_position(0, 2).unwrap();
    let mut prev_end = 0;
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
            st.con_out().set_cursor_position(0, 2).unwrap();
        }

        let start = desc.physical_start;
        let end = desc.physical_start + desc.number_of_pages * 4096;
        println!("{:x} {:x} {:x} {:#x?}", start - prev_end, start, end, desc,);
        total_ram_kb += desc.number_of_pages * EFI_PAGE_SIZE_BYTES / KB_IN_BYTES;
        let usable = match desc.ty {
            uefi::services::boot::MemoryType::EfiReservedMemoryType => false,
            uefi::services::boot::MemoryType::EfiLoaderCode => true,
            uefi::services::boot::MemoryType::EfiLoaderData => true,
            uefi::services::boot::MemoryType::EfiBootServicesCode => true,
            uefi::services::boot::MemoryType::EfiBootServicesData => true,
            uefi::services::boot::MemoryType::EfiRuntimeServicesCode => false,
            uefi::services::boot::MemoryType::EfiRuntimeServicesData => false,
            uefi::services::boot::MemoryType::EfiConventionalMemory => true,
            uefi::services::boot::MemoryType::EfiUnusableMemory => false,
            uefi::services::boot::MemoryType::EfiACPIReclaimMemory => true,
            uefi::services::boot::MemoryType::EfiACPIMemoryNVS => false,
            uefi::services::boot::MemoryType::EfiMemoryMappedIO => false,
            uefi::services::boot::MemoryType::EfiMemoryMappedIOPortSpace => false,
            uefi::services::boot::MemoryType::EfiPalCode => false,
            uefi::services::boot::MemoryType::EfiPersistentMemory => true,
            uefi::services::boot::MemoryType::EfiUnacceptedMemoryType => false,
            uefi::services::boot::MemoryType::EfiMaxMemoryType => false,
        };
        if usable {
            rt_usable_ram_kb += desc.number_of_pages * EFI_PAGE_SIZE_BYTES / KB_IN_BYTES;
        } else {
            println!(
                "Unusable {:?} {}KiB",
                desc.ty,
                desc.number_of_pages * EFI_PAGE_SIZE_BYTES / KB_IN_BYTES
            )
        }
        prev_end = end;
    }

    wait_for_key();
    clear_screen();
    println!(
        "Total ram: {}, usable: {}, non-usable: {}",
        total_ram_kb,
        rt_usable_ram_kb,
        total_ram_kb - rt_usable_ram_kb
    );
    wait_for_key();
}

pub fn print_str(string: &str, pos: Option<(usize, usize)>) {
    let st = system_table();
    if let Some((col, row)) = pos {
        st.con_out().set_cursor_position(col, row).unwrap();
    }

    let parts = string.split('\n').collect::<Vec<_>>();
    for (i, part) in parts.iter().enumerate() {
        if !part.is_empty() {
            let string: String16 = part.parse().unwrap();
            st.con_out().output_string(&string).unwrap();
        }

        if i != parts.len() - 1 {
            let string: String16 = "\r\n".parse().unwrap();
            st.con_out().output_string(&string).unwrap();
        }
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::print::print_str(&format!("{}", format_args!($($arg)*)), None));
}

#[macro_export]
macro_rules! println {
    () => (print!("\n"));
    ($($arg:tt)*) => (print!("{}\n", format_args!($($arg)*)));
}

pub fn clear_screen() {
    let st = system_table();
    st.con_out().reset(false).unwrap();
}

pub fn wait_for_key() {
    let st = system_table();
    st.con_in().reset(false).unwrap();
    loop {
        match st.con_in().read_key() {
            Ok(_key) => break,
            Err(_status) => continue,
        }
    }
}
