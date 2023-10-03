use crate::allocator::BumpAllocator;

#[derive(Debug)]
pub struct KernelExecutable {
    pub image_start: u64,
    pub image_end: u64,
    pub frame_addr: u64,
    pub frames: u64,
    pub entry_point: u64,
}

pub struct Elf<'elf> {
    bytes: &'elf [u8],
}

impl<'elf> Elf<'elf> {
    pub fn header(&self) -> ElfHeader {
        ElfHeader::try_from(self.bytes).unwrap()
    }

    pub fn program_headers(&self) -> &[ProgramHeader] {
        let header = self.header();
        assert!(header.program_header_size as usize == core::mem::size_of::<ProgramHeader>());
        let base = unsafe {
            self.bytes
                .as_ptr()
                .cast::<u8>()
                .add(header.program_header_offset as usize)
                .cast::<ProgramHeader>()
        };
        unsafe { core::slice::from_raw_parts(base, header.program_header_count as usize) }
    }
}

pub fn mount_kernel(
    elf: &[u8],
    bump_alloc: &BumpAllocator,
) -> Result<KernelExecutable, &'static str> {
    let elf = Elf { bytes: elf };
    let header = elf.header();
    if header.magic != [0x7F, 0x45, 0x4C, 0x46] {
        return Err("Not an ELF file");
    }

    if header.cpu_format != 2 {
        return Err("Not a 64-bit executable");
    }

    let load_entries = elf.program_headers().iter().filter(|e| e.ty == 1);
    let mut kernel_virt_base = u64::MAX;
    let mut kernel_virt_limit = 0;
    for &entry in load_entries.clone() {
        kernel_virt_base = kernel_virt_base.min(entry.physical_address);
        kernel_virt_limit = kernel_virt_limit.max(entry.physical_address + entry.segment_mem_size);
    }

    kernel_virt_base &= !0xfff;
    kernel_virt_limit += 4093;
    kernel_virt_limit &= !0xfff;
    let kernel_frames = (kernel_virt_limit - kernel_virt_base) / 4096;
    // TODO: allocate zeroed
    let kernel_phys_base = bump_alloc.allocate_frames(kernel_frames).unwrap();
    let kernel = unsafe {
        core::slice::from_raw_parts_mut(kernel_phys_base as *mut u8, kernel_frames as usize * 4096)
    };
    for entry in load_entries {
        let entry_files_offset = entry.segment_file_offset as usize;
        let entry_phys_base = (entry.virtual_address - kernel_virt_base) as usize;
        let size = entry.segment_mem_size as usize;
        kernel[entry_phys_base..(size + entry_phys_base)]
            .copy_from_slice(&elf.bytes[entry_files_offset..(entry_files_offset + size)]);
    }

    Ok(KernelExecutable {
        image_start: kernel_virt_base,
        image_end: kernel_virt_limit,
        frame_addr: kernel_phys_base,
        frames: kernel_frames,
        entry_point: header.entry,
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
