#![no_std]
#![no_main]

#[macro_use]
extern crate alloc;

mod elf;

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

    print!("stack start {}", stack_start);

    let cr0: u64;
    let cr2: u64;
    let cr3: u64;
    let cr4: u64;
    unsafe {
        core::arch::asm!("mov {}, cr0", out(reg) cr0);
        core::arch::asm!("mov {}, cr2", out(reg) cr2);
        core::arch::asm!("mov {}, cr3", out(reg) cr3);
        core::arch::asm!("mov {}, cr4", out(reg) cr4);
    };

    #[derive(Debug)]
    struct Cr0 {
        // 0 	PE 	Protected Mode Enable
        pe: bool,
        // 1 	MP 	Monitor Co-Processor
        mp: bool,
        // 2 	EM 	Emulation
        em: bool,
        // 3 	TS 	Task Switched
        ts: bool,
        // 4 	ET 	Extension Type
        et: bool,
        // 5 	NE 	Numeric Error
        ne: bool,
        // 6-15 	0 	Reserved
        // 16 	WP 	Write Protect
        wp: bool,
        // 17 	0 	Reserved
        // 18 	AM 	Alignment Mask
        am: bool,
        // 19-28 	0 	Reserved
        // 29 	NW 	Not-Write Through
        nw: bool,
        // 30 	CD 	Cache Disable
        cd: bool,
        // 31 	PG 	Paging
        pg: bool,
        // 32-63 	0 	Reserved
    }

    #[derive(Debug)]
    struct Cr2(u64);

    #[derive(Debug)]
    struct Cr3 {
        // 3 	PWT 	Page-Level Write Through, when CR4.PCIDE = 0
        pwt: bool,
        // 5 	PCD 	Page-Level Cache Disable, when CR4.PCIDE = 0
        pcd: bool,
        //  0-11 	PCID, when CR4.PCIDE = 1
        pcid: u16,
        // 12-63 	Physical Base Address of the PML4
        pba_pml4: u64,
    }

    #[derive(Debug)]
    struct Cr4 {
        //  0 	VME 	Virtual-8086 Mode Extensions
        vme: bool,
        // 1 	PVI 	Protected Mode Virtual Interrupts
        pvi: bool,
        // 2 	TSD 	Time Stamp enabled only in ring 0
        tsd: bool,
        // 3 	DE 	Debugging Extensions
        de: bool,
        // 4 	PSE 	Page Size Extension
        pse: bool,
        // 5 	PAE 	Physical Address Extension
        pae: bool,
        // 6 	MCE 	Machine Check Exception
        mce: bool,
        // 7 	PGE 	Page Global Enable
        pge: bool,
        // 8 	PCE 	Performance Monitoring Counter Enable
        pce: bool,
        // 9 	OSFXSR 	OS support for fxsave and fxrstor instructions
        dsfxsr: bool,
        // 10 	OSXMMEXCPT 	OS Support for unmasked simd floating point exceptions
        osxmmexcpt: bool,
        // 11 	UMIP 	User-Mode Instruction Prevention (SGDT, SIDT, SLDT, SMSW, and STR are disabled in user mode)
        uimp: bool,
        // 12 	0 	Reserved
        // 13 	VMXE 	Virtual Machine Extensions Enable
        vmxe: bool,
        // 14 	SMXE 	Safer Mode Extensions Enable
        smxe: bool,
        // 15 	0 	Reserved
        // 16 	FSGSBASE 	Enables the instructions RDFSBASE, RDGSBASE, WRFSBASE, and WRGSBASE
        fsgsbase: bool,
        // 17 	PCIDE 	PCID Enable
        pcide: bool,
        // 18 	OSXSAVE 	XSAVE And Processor Extended States Enable
        osxsave: bool,
        // 19 	0 	Reserved
        // 20 	SMEP 	Supervisor Mode Executions Protection Enable
        smep: bool,
        // 21 	SMAP 	Supervisor Mode Access Protection Enable
        smap: bool,
        // 22 	PKE 	Enable protection keys for user-mode pages
        pke: bool,
        // 23 	CET 	Enable Control-flow Enforcement Technology
        cet: bool,
        // 24 	PKS 	Enable protection keys for supervisor-mode pages
        pks: bool,
        // 25-63 	0 	Reserved
    }

    let cr0 = Cr0 {
        pe: cr0 & (1 << 0) != 0,
        mp: cr0 & (1 << 1) != 0,
        em: cr0 & (1 << 2) != 0,
        ts: cr0 & (1 << 3) != 0,
        et: cr0 & (1 << 4) != 0,
        ne: cr0 & (1 << 5) != 0,
        wp: cr0 & (1 << 16) != 0,
        am: cr0 & (1 << 18) != 0,
        nw: cr0 & (1 << 29) != 0,
        cd: cr0 & (1 << 30) != 0,
        pg: cr0 & (1 << 31) != 0,
    };

    let cr2 = Cr2(cr2);

    let cr3 = Cr3 {
        pwt: cr3 & (1 << 3) != 0,
        pcd: cr3 & (1 << 5) != 0,
        pcid: (cr3 & 0b111_1111_1111) as u16,
        pba_pml4: (cr3 >> 12),
    };

    let cr4 = Cr4 {
        vme: cr4 & (1 << 0) != 0,
        pvi: cr4 & (1 << 1) != 0,
        tsd: cr4 & (1 << 2) != 0,
        de: cr4 & (1 << 3) != 0,
        pse: cr4 & (1 << 4) != 0,
        pae: cr4 & (1 << 5) != 0,
        mce: cr4 & (1 << 6) != 0,
        pge: cr4 & (1 << 7) != 0,
        pce: cr4 & (1 << 8) != 0,
        dsfxsr: cr4 & (1 << 9) != 0,
        osxmmexcpt: cr4 & (1 << 10) != 0,
        uimp: cr4 & (1 << 11) != 0,
        vmxe: cr4 & (1 << 13) != 0,
        smxe: cr4 & (1 << 14) != 0,
        fsgsbase: cr4 & (1 << 16) != 0,
        pcide: cr4 & (1 << 17) != 0,
        osxsave: cr4 & (1 << 18) != 0,
        smep: cr4 & (1 << 20) != 0,
        smap: cr4 & (1 << 21) != 0,
        pke: cr4 & (1 << 22) != 0,
        cet: cr4 & (1 << 23) != 0,
        pks: cr4 & (1 << 24) != 0,
    };

    print!("{:x?}, {:x?}, {:x?}, {:x?}", cr0, cr2, cr3, cr4);

    wait_for_key();

    // These two have to be called next to each other
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

            return 0;
        }
    }

    // let stack_start: usize = 0x1337;
    let stack_end = stack_end;
    let g = gfx();
    unsafe {
        core::arch::asm!("mov rsp, {}; jmp {}",
          in(reg) stack_end,
          in(reg) entry_point_fn,
          in("rdi") g.buffer,
          in("rsi") g.width,
          in("rdx") g.height
        );
    }
    // let _result = entry_point_fn(g.buffer, g.width, g.height);

    // // TODO
    // // * read executable file DONE
    // // * find the location of the main entry point DONE
    // // * do some setup, idk what this is tho

    loop {}
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
        print_str(&format!("{}", info), None);
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
