use alloc::{iter::IteratorCollectIn, vec::Vec};
use core::alloc::Allocator;
use uefi::services::boot::{AllocateType, BootServices, MemoryType};

#[derive(Debug)]
pub struct KernelExecutable {
    pub image_start: u64,
    pub image_end: u64,
    pub frame_addr: u64,
    pub frames: u64,
    pub entry_point: u64,
}

pub fn get_elf_entry_point_offset<A: Allocator>(
    boot_services: &BootServices,
    elf: &[u8],
    alloc: &A,
) -> Result<KernelExecutable, &'static str> {
    let header = ElfHeader::try_from(elf).unwrap();
    if header.magic != [0x7F, 0x45, 0x4C, 0x46] {
        return Err("Not an ELF file");
    }

    if header.cpu_format != 2 {
        return Err("Not a 64-bit executable");
    }

    const PROGRAM_HEADER_SIZE: usize = core::mem::size_of::<ProgramHeader>();
    assert!(header.program_header_size as usize == PROGRAM_HEADER_SIZE);

    // TODO: no alloc, just iter
    let mut program_header_entries = Vec::with_elem(
        ProgramHeader::default(),
        header.program_header_count as _,
        alloc,
    )
    .unwrap();
    for i in 0..header.program_header_count {
        let entry_start = header.program_header_offset as usize + i as usize * PROGRAM_HEADER_SIZE;
        program_header_entries[i as usize] =
            ProgramHeader::try_from(&elf[entry_start..entry_start + PROGRAM_HEADER_SIZE]).unwrap();
    }

    let load_entries: Vec<_, _> = program_header_entries
        .into_iter()
        .filter(|e| e.ty == 1)
        .collect_in(alloc)
        .unwrap();
    let mut image_start = u64::MAX;
    let mut image_end = 0;
    for &entry in &load_entries {
        image_start = image_start.min(entry.virtual_address);
        image_end = image_end.max(entry.virtual_address + entry.segment_mem_size);
    }

    let image_size = image_end - image_start;
    let kernel_pages = image_size as usize / 4096 + 1;
    let kernel_addr = boot_services
        .allocate_pages(
            AllocateType::AllocateAnyPages,
            MemoryType::EfiLoaderData,
            kernel_pages,
        )
        .unwrap();
    let page =
        unsafe { core::slice::from_raw_parts_mut(kernel_addr as *mut u8, kernel_pages * 4096) };

    for entry in &load_entries {
        let file_start_addr = entry.segment_file_offset as usize;
        let loaded_start_addr = (entry.virtual_address - image_start) as usize;
        let size = entry.segment_mem_size as usize;
        page[loaded_start_addr..(size + loaded_start_addr)]
            .copy_from_slice(&elf[file_start_addr..(size + file_start_addr)]);
    }

    let entry_point_offset = header.entry - image_start;
    let entry_point = &page[entry_point_offset as usize] as *const _ as _;
    Ok(KernelExecutable {
        image_start,
        image_end,
        frame_addr: kernel_addr,
        frames: kernel_pages as _,
        entry_point,
    })
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct ElfHeader {
    magic: [u8; 4],
    cpu_format: u8,
    endianess: u8,
    elf_version: u8,
    os_abi: u8,
    abi_version: u8,
    _pad: [u8; 7],
    ty: u16,
    machine: u16,
    version: u32,
    entry: u64,
    program_header_offset: u64,
    section_header_offset: u64,
    flags: u32,
    elf_header_size: u16,
    program_header_size: u16,
    program_header_count: u16,
    section_header_size: u16,
    section_header_count: u16,
    section_header_index_containing_section_names: u16,
}

impl TryFrom<&[u8]> for ElfHeader {
    type Error = ();

    fn try_from(slice: &[u8]) -> Result<Self, Self::Error> {
        const HEADER_SIZE: usize = core::mem::size_of::<ElfHeader>();
        if slice.len() < HEADER_SIZE {
            return Err(());
        }

        let slice: [u8; core::mem::size_of::<ElfHeader>()] = slice[0..HEADER_SIZE]
            .try_into()
            .expect("convert slice into array");

        Ok(unsafe { core::mem::transmute_copy(&slice) })
    }
}

#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct ProgramHeader {
    ty: u32,
    flags: u32,
    segment_file_offset: u64,
    virtual_address: u64,
    physical_address: u64,
    segment_file_size: u64,
    segment_mem_size: u64,
    align: u64,
}

impl TryFrom<&[u8]> for ProgramHeader {
    type Error = ();

    fn try_from(slice: &[u8]) -> Result<Self, Self::Error> {
        const HEADER_SIZE: usize = core::mem::size_of::<ProgramHeader>();
        if slice.len() < HEADER_SIZE {
            return Err(());
        }

        let slice: [u8; HEADER_SIZE] = slice[0..HEADER_SIZE]
            .try_into()
            .expect("convert slice into array");

        Ok(unsafe { core::mem::transmute_copy(&slice) })
    }
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct SectionHeader {
    ty: u32,
    flags: u32,
    offset: u64,
    virtual_address: u64,
    physical_address: u64,
    segment_image_size: u64,
    segment_mem_size: u64,
    align: u64,
}
