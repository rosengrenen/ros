#![no_std]
#![no_main]
#![feature(allocator_api)]
#![feature(format_args_nl)]
// TODO: think about if this is necessary
#![deny(unsafe_op_in_unsafe_fn)]

mod acpi;
mod allocator;
mod aml;
mod elf;
mod print;

use crate::{
    acpi::{Fadt, Rsdp},
    allocator::BumpAllocator,
    aml::definition_blocks,
};
use acpi::DefinitionHeader;
use alloc::vec::{PushError, Vec};
use bootloader_api::BootInfo;
use core::{
    alloc::{Allocator, Layout},
    fmt::{Arguments, Write},
};
use elf::{get_elf_entry_point_offset, KernelExecutable};
use parser::error::{FromExternalError, ParseError, ParseErrorKind};
use serial::{SerialPort, COM1_BASE};
use uefi::{
    allocator::UefiAllocator,
    services::{
        boot::{BootServices, Guid, MemoryDescriptor, MemoryMap, MemoryType},
        filesystem::FileSystem,
        graphics::{BltPixel, Graphics},
    },
    string::String16,
};
use x86_64::{
    control::{Cr0, Cr2, Cr3, Cr4},
    flags::RFlags,
    idt::read_cs,
    paging::{FrameAllocator, PageTable, PhysAddr, VirtAddr},
};

#[macro_export]
macro_rules! sprintln {
    ($($arg:tt)*) => {
        $crate::serial_print(format_args_nl!($($arg)*));
    }
}

fn serial_print(args: Arguments) {
    let mut serial = SerialPort::new(COM1_BASE);
    serial.write_fmt(args).unwrap();
}

#[no_mangle]
pub extern "efiapi" fn efi_main(
    image_handle: uefi::Handle,
    system_table: uefi::SystemTable<uefi::Uninit>,
) -> uefi::Status {
    let system_table = system_table.init();
    let uefi_allocator = UefiAllocator::new(system_table.boot_services());
    system_table.con_out().reset(false).unwrap();
    SerialPort::new(COM1_BASE).configure(1);

    // This is what the bootloader needs to do:
    // 1. Read the kernel file
    let kernel_executable =
        read_kernel_executable(system_table.boot_services(), &uefi_allocator).unwrap();
    sprintln!("{:#x?}", kernel_executable);

    // 2. Retrieve kernel args from UEFI boot services before it goes out of scope
    let framebuffer = get_framebuffer(system_table.boot_services());

    // Calculate the total size of the boot info struct, including regions which are pointed to
    let boot_info_layout = Layout::new::<BootInfo>();
    let memory_regions_layout = Layout::array::<bootloader_api::MemoryRegion>(0).unwrap();
    let reserved_memory_regions_layout =
        Layout::array::<bootloader_api::ReservedMemoryRegion>(0).unwrap();
    let (boot_info_layout, memory_regions_offset) =
        boot_info_layout.extend(memory_regions_layout).unwrap();
    let (_boot_info_layout, reserved_memory_regions_offset) = boot_info_layout
        .extend(reserved_memory_regions_layout)
        .unwrap();

    const EFI_ACPI_TABLE_GUID: Guid = Guid(
        0x8868e871,
        0xe4f1,
        0x11d3,
        [0xbc, 0x22, 0x00, 0x80, 0xc7, 0x3c, 0x88, 0x81],
    );

    let acpi_entry = system_table
        .configuration_table()
        .iter()
        .find(|entry| entry.vendor_guid == EFI_ACPI_TABLE_GUID)
        .unwrap();
    sprintln!("{:x?}", acpi_entry);

    let rsdp = unsafe { Rsdp::from_addr(acpi_entry.vendor_table as _) };

    // Allocate frames for the boot info
    let boot_info_addr = 0;

    let memory_map = system_table
        .boot_services()
        .get_memory_map(&uefi_allocator)
        .unwrap();

    let bump_allocator = BumpAllocator::new(memory_map.iter());
    let memory_map_key = memory_map.key;
    let _kernel_mem_regions = get_kernel_mem_regions(&memory_map, &bump_allocator).unwrap();
    // sprintln!( "{:#x?}", _kernel_mem_regions);
    core::mem::forget(memory_map);

    // Exit UEFI boot services
    let system_table = system_table
        .exit_boot_services(image_handle, memory_map_key)
        .unwrap();
    let idt = bump_allocator.allocate_pages(1).unwrap();
    let gdt = bump_allocator.allocate_pages(1).unwrap();

    let pml4_frame = bump_allocator.allocate_frame().unwrap();
    let pml4 = PageTable::new(pml4_frame as _);

    // Identity map all boot service regions so that bootloader continues working
    for desc in _kernel_mem_regions.iter() {
        if desc.ty == MemoryType::EfiBootServicesCode
            || desc.ty == MemoryType::EfiBootServicesData
            || desc.ty == MemoryType::EfiACPIReclaimMemory
        {
            for frame_index in 0..desc.number_of_pages {
                pml4.map_ident(
                    VirtAddr::new(desc.physical_start + frame_index * 4096),
                    &bump_allocator,
                );
            }
        }
    }

    // Map kernel to virtual addresses
    for page in 0..kernel_executable.frames {
        pml4.map(
            VirtAddr::new(kernel_executable.image_start + page * 4096),
            PhysAddr::new(kernel_executable.frame_addr + page * 4096),
            &bump_allocator,
        );
    }

    // Identity map framebuffer
    let framebuffer_frames =
        framebuffer.height * framebuffer.width * core::mem::size_of::<BltPixel>() / 4096;
    for page in 0..framebuffer_frames {
        pml4.map_ident(
            VirtAddr::new((framebuffer.base + page * 4096) as u64),
            &bump_allocator,
        );
    }

    for hdr_ptr in rsdp.table_ptrs() {
        let hdr = unsafe { hdr_ptr.read() };
        if &hdr.signature == b"FACP" {
            let ptr = *hdr_ptr as *const Fadt;
            let fadt = unsafe { ptr.read() };
            sprintln!(
                "{} {:#x?}",
                core::str::from_utf8(&hdr.signature).unwrap(),
                fadt
            );
            let dsdt_addr = fadt.dsdt;
            print_dsdt(dsdt_addr as _, &bump_allocator);
        } else if &hdr.signature == b"APIC" {
            // TODO: print the entries
            print_apic(*hdr_ptr);
        } else {
            sprintln!(
                "{} {:?}",
                core::str::from_utf8(&hdr.signature).unwrap(),
                hdr
            );
        }
    }

    // Allocate stack for the kernel and map it to virtual addresses
    let stack_pages = 8;
    let stack_end: u64 = 0xffff_ffff_ffff_fff8;
    let stack_start = (stack_end & !0xfff) - (stack_pages - 1) * 4096;
    for page in 0..stack_pages {
        let frame = bump_allocator.allocate_frame().unwrap();
        pml4.map(
            VirtAddr::new(stack_start + page * 4096),
            PhysAddr::new(frame),
            &bump_allocator,
        );
    }

    // print_page_table(&mut serial, &pml4);

    Cr3::write(pml4_frame);

    // Populate memory regions
    let memory_regions_addr = boot_info_addr + memory_regions_offset;
    let memory_regions_len = 0;

    // Populate reserved memory regions
    // Kernel
    // Page table
    let reserved_memory_regions_addr = boot_info_addr + reserved_memory_regions_offset;
    let reserved_memory_regions_len = 0;

    let info = BootInfo {
        uefi_system_table: system_table,
        framebuffer,
        kernel: bootloader_api::Kernel {
            base: kernel_executable.image_start,
            frames: kernel_executable.frames as _,
            stack_base: stack_end as _,
        },
        memory_regions: bootloader_api::MemoryRegions {
            ptr: memory_regions_addr as _,
            len: memory_regions_len,
        },
        reserved_memory_regions: bootloader_api::ReservedMemoryRegions {
            ptr: reserved_memory_regions_addr as _,
            len: reserved_memory_regions_len,
        },
        idt,
        gdt,
    };
    let info_ptr = &info as *const BootInfo;

    sprintln!("launching kernel!!");
    sprintln!(
        "jumping to {:#x}, new stack {:#x}",
        kernel_executable.entry_point,
        stack_end
    );

    // 4. Call the kernel
    unsafe {
        core::arch::asm!("mov rsp, {}; jmp {}",
          in(reg) stack_end,
          in(reg) kernel_executable.entry_point,
          in("rdi") info_ptr,
        );
    }

    unreachable!("should have jumped to kernel at this point")
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    sprintln!("{:?}", info);
    loop {}
}

fn read_kernel_executable(
    uefi_boot_services: &BootServices,
    uefi_allocator: &UefiAllocator,
) -> Result<KernelExecutable, usize> {
    let fs = uefi_boot_services.locate_protocol::<FileSystem>()?;
    let root_fs = fs.open_volume()?;
    let file_name = String16::from_str("ros", &uefi_allocator).map_err(|_| 99usize)?;
    let file = root_fs.open(file_name.as_raw(), 0x3, 0x0)?;
    let info = file.get_info(&uefi_allocator)?;
    let mut buffer = Vec::with_elem(0u8, info.file_size as usize, &uefi_allocator).unwrap();
    let _read_bytes = file.read(&mut buffer).unwrap();
    // TODO: impl truncate
    // buffer.truncate(read_bytes);
    get_elf_entry_point_offset(uefi_boot_services, &buffer, &uefi_allocator).map_err(|_| 100usize)
}

fn get_kernel_mem_regions<'alloc, A: Allocator>(
    memory_map: &MemoryMap<UefiAllocator>,
    alloc: &'alloc A,
) -> Result<Vec<'alloc, MemoryDescriptor, A>, PushError> {
    let mut kernel_mem_regions: Vec<MemoryDescriptor, _> = Vec::new(alloc);
    for desc in memory_map.iter() {
        if let Some(last) = kernel_mem_regions.last_mut() {
            let overlap = last.physical_start + last.number_of_pages * 4096 == desc.physical_start;
            if last.ty.usable_by_kernel() && desc.ty.usable_by_kernel() && overlap {
                last.number_of_pages += desc.number_of_pages;
            } else {
                kernel_mem_regions.push(*desc)?;
            }
        } else {
            kernel_mem_regions.push(*desc)?;
        }
    }

    Ok(kernel_mem_regions)
}

fn get_framebuffer(uefi_boot_services: &BootServices) -> bootloader_api::Framebuffer {
    let graphics = uefi_boot_services.locate_protocol::<Graphics>().unwrap();
    bootloader_api::Framebuffer {
        base: graphics.mode.frame_buffer_base as _,
        width: graphics.mode.info.horizontal_resolution as _,
        height: graphics.mode.info.vertical_resolution as _,
    }
}

#[allow(dead_code)]
fn dump_registers() {
    sprintln!("{:x?}", RFlags::read());
    sprintln!("{:x?}", Cr0::read());
    sprintln!("{:x?}", Cr2::read());
    sprintln!("{:x?}", Cr3::read());
    sprintln!("{:x?}", Cr4::read());
    sprintln!("CS: {:x?}", read_cs());
}

fn print_apic(hdr_ptr: *const DefinitionHeader) {
    let hdr = unsafe { hdr_ptr.read() };
    let byte_ptr = unsafe { hdr_ptr.add(1) }.cast::<u8>();
    let local_interrupts_controller_address = unsafe { byte_ptr.cast::<u32>().read() };
    let flags = unsafe { byte_ptr.add(1).cast::<u32>().read() };
    sprintln!(
        "{:#x?}, local interrupts controller addr: {:x}, flags: {:x}",
        hdr,
        local_interrupts_controller_address,
        flags
    );

    let mut entry_start = unsafe { byte_ptr.add(8) };
    loop {
        let ty = unsafe { entry_start.read() };
        let len = unsafe { entry_start.add(1).read() };
        match ty {
            0 => {
                #[derive(Debug)]
                #[repr(C, packed)]
                struct LocalApic {
                    processor_uid: u8,
                    apic_id: u8,
                    flags: u32,
                }
                sprintln!("{:x?}", unsafe {
                    entry_start.add(2).cast::<LocalApic>().read()
                });
            }
            1 => {
                #[derive(Debug)]
                #[repr(C, packed)]
                struct IoApic {
                    apic_id: u8,
                    _reserved: u8,
                    apic_addr: u32,
                    global_system_interrupt_base: u32,
                }
                sprintln!("{:x?}", unsafe {
                    entry_start.add(2).cast::<IoApic>().read()
                });
            }
            2 => {
                #[derive(Debug)]
                #[repr(C, packed)]
                struct InterruptSourceOverride {
                    bus: u8,
                    source: u8,
                    global_system_interrupts: u32,
                    flags: u16,
                }
                sprintln!("{:x?}", unsafe {
                    entry_start.add(2).cast::<InterruptSourceOverride>().read()
                });
            }
            4 => {
                #[derive(Debug)]
                #[repr(C, packed)]
                struct LocalApicNmi {
                    processor_uid: u8,
                    flags: u16,
                    local_apic_lint: u8,
                }
                sprintln!("{:x?}", unsafe {
                    entry_start.add(2).cast::<LocalApicNmi>().read()
                });
            }
            _ => {
                sprintln!("{:x?} {:x} {:x}", entry_start, ty, len);
            }
        }

        entry_start = unsafe { entry_start.add(len as _) };
        if entry_start as usize >= hdr_ptr as usize + hdr.length as usize {
            break;
        }
    }
}

fn print_dsdt<A: Allocator>(dsdt_addr: u64, alloc: &A) {
    let ptr = dsdt_addr as *const DefinitionHeader;
    let hdr = unsafe { ptr.read() };
    let aml_ptr = unsafe { ptr.add(1) }.cast::<u8>();
    let aml_len = hdr.length - 36;
    let aml_slice = unsafe { core::slice::from_raw_parts(aml_ptr, aml_len as _) };
    let res = definition_blocks::<SimpleError<&[u8]>, _>(aml_slice, alloc);
    sprintln!("{:?}", res);
    loop {}
}

#[derive(Debug)]
struct SimpleError<I> {
    input: I,
    kind: ParseErrorKind,
}

impl<I> ParseError<I> for SimpleError<I> {
    fn from_error_kind(input: I, kind: ParseErrorKind) -> Self {
        Self { input, kind }
    }

    fn append(self, _other: Self) -> Self {
        self
    }
}

impl<I, E> FromExternalError<I, E> for SimpleError<I> {
    fn from_external_error(input: I, kind: ParseErrorKind, _error: E) -> Self {
        Self { input, kind }
    }
}
