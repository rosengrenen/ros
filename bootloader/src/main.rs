#![no_std]
#![no_main]
#![feature(allocator_api)]
#![feature(format_args_nl)]
// TODO: think about if this is necessary
#![deny(unsafe_op_in_unsafe_fn)]

mod allocator;
mod elf;
mod stack_vec;

use crate::{allocator::BumpAllocator, elf::mount_kernel};
use alloc::{raw_vec::RawVec, vec::Vec};
use bootloader_api::{AllocatedFrameRange, BootInfo, MemoryRegion, MemoryRegionType};
use core::{
    alloc::Layout,
    fmt::{Arguments, Write},
};
use serial::{SerialPort, COM1_BASE};
use uefi::{
    allocator::UefiAllocator,
    services::{
        boot::{BootServices, Guid, MemoryMap, MemoryType},
        filesystem::FileSystem,
    },
    string::String16,
};
use x86_64::{
    control::Cr3,
    paging::{FrameAllocator, PageTable, PhysAddr, VirtAddr},
};

#[macro_export]
macro_rules! sprintln {
    ($($arg:tt)*) => {
        $crate::serial_print(format_args_nl!($($arg)*))
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
    system_table.con_out().reset(false).unwrap();
    let uefi_allocator = UefiAllocator::new(system_table.boot_services());

    let kernel_executable =
        read_kernel_executable(system_table.boot_services(), &uefi_allocator).unwrap();

    let rsdp = get_rsdp(&system_table);

    let memory_map = system_table
        .boot_services()
        .get_memory_map(&uefi_allocator)
        .unwrap();
    let efi_main_region = get_efi_main_region(&memory_map);

    let stack_pointer = get_stack_pointer();
    let efi_stack_region = get_efi_stack_region(&memory_map, stack_pointer);

    let memory_map_key = memory_map.key;
    let bump_allocator = BumpAllocator::new(memory_map.iter());

    let kernel_memory_map = optimize_memory_map(&memory_map, &bump_allocator);
    core::mem::forget(memory_map);
    let kernel = mount_kernel(&kernel_executable, &bump_allocator).unwrap();
    core::mem::forget(kernel_executable);

    // Exit UEFI boot services
    system_table
        .exit_boot_services(image_handle, memory_map_key)
        .unwrap();

    let pml4_frame = bump_allocator.allocate_frame().unwrap();
    let mut pml4 = PageTable::new(pml4_frame as _);

    // Identity map region in which main function resides, so that bootloader continues working after enabling paing, but mark the frames as usable for the kernel
    bump_allocator.reserve_frames(
        efi_main_region.physical_start,
        efi_main_region.number_of_pages,
        true,
    );
    for frame_index in 0..efi_main_region.number_of_pages {
        pml4.map_ident(
            PhysAddr::new(efi_main_region.physical_start + frame_index * 4096),
            &bump_allocator,
        );
    }

    // Do the same with the stack
    bump_allocator.reserve_frames(
        efi_stack_region.physical_start,
        efi_stack_region.number_of_pages,
        true,
    );
    for frame_index in 0..efi_stack_region.number_of_pages {
        pml4.map_ident(
            PhysAddr::new(efi_stack_region.physical_start + frame_index * 4096),
            &bump_allocator,
        );
    }

    sprintln!("{:#x?}", efi_main_region);
    sprintln!(
        "{:x?}",
        efi_main_region.physical_start + efi_main_region.number_of_pages * 4096
    );
    sprintln!("{:#x?}", efi_stack_region);
    sprintln!(
        "{:x?}",
        efi_stack_region.physical_start + efi_stack_region.number_of_pages * 4096
    );

    // Map kernel to virtual addresses
    for page in 0..kernel.frames {
        pml4.map(
            VirtAddr::new(kernel.image_start + page * 4096),
            PhysAddr::new(kernel.frame_addr + page * 4096),
            &bump_allocator,
        );
    }

    // Allocate stack for the kernel and map it to virtual addresses
    let stack_end: u64 = 0xffff_ffff_ffff_fff8;
    let stack_frame = bump_allocator.allocate_frame().unwrap();
    pml4.map(
        VirtAddr::new(stack_end & !0xfff),
        PhysAddr::new(stack_frame as u64),
        &bump_allocator,
    );

    let boot_info_frame = bump_allocator.allocate_frame().unwrap();
    pml4.map_ident(PhysAddr::new(boot_info_frame as u64), &bump_allocator);
    let allocated_frame_ranges = bump_allocator.inner.into_inner().allocated_frames;

    let boot_info_layout = Layout::new::<BootInfo>().align_to(4096).unwrap();
    let memory_regions_layout = Layout::array::<MemoryRegion>(kernel_memory_map.len()).unwrap();
    let allocated_frame_ranges_layout =
        Layout::array::<AllocatedFrameRange>(allocated_frame_ranges.len()).unwrap();
    let (boot_info_layout, memory_regions_offset) =
        boot_info_layout.extend(memory_regions_layout).unwrap();
    let (_, allocated_frame_ranges_offset) = boot_info_layout
        .extend(allocated_frame_ranges_layout)
        .unwrap();
    let boot_info = boot_info_frame as *mut BootInfo;

    // Copy memory regions
    let boot_info_memory_regions = unsafe {
        core::slice::from_raw_parts_mut(
            (boot_info_frame as usize + memory_regions_offset) as *mut MemoryRegion,
            kernel_memory_map.len(),
        )
    };
    for (to_entry, from_entry) in boot_info_memory_regions
        .iter_mut()
        .zip(kernel_memory_map.iter())
    {
        *to_entry = *from_entry;
    }

    // Copy memory regions
    let boot_info_allocated_frame_ranges = unsafe {
        core::slice::from_raw_parts_mut(
            (boot_info_frame as usize + allocated_frame_ranges_offset) as *mut AllocatedFrameRange,
            allocated_frame_ranges.len(),
        )
    };
    for (to_entry, from_entry) in boot_info_allocated_frame_ranges
        .iter_mut()
        .zip(allocated_frame_ranges.iter())
    {
        *to_entry = *from_entry;
    }

    unsafe {
        boot_info.write(BootInfo {
            kernel: bootloader_api::Kernel {
                base: kernel.image_start,
                frames: kernel.frames as _,
                stack_base: stack_end as _,
            },
            memory_regions: bootloader_api::MemoryRegions {
                ptr: boot_info_memory_regions.as_ptr(),
                len: boot_info_memory_regions.len(),
            },
            allocated_frame_ranges: bootloader_api::AllocatedFrameRanges {
                ptr: boot_info_allocated_frame_ranges.as_ptr(),
                len: boot_info_allocated_frame_ranges.len(),
            },
            rsdp,
        })
    }

    sprintln!("Bootloader is launching kernel");

    // Set new page table
    Cr3::write(pml4_frame as u64);

    // Jump to kernel
    unsafe {
        core::arch::asm!("mov rsp, {}; jmp {}",
          in(reg) stack_end,
          in(reg) kernel.entry_point,
          in("rdi") boot_info_frame,
        );
    }

    unreachable!("should have jumped to kernel at this point")
}

fn get_efi_stack_region(
    memory_map: &MemoryMap<&UefiAllocator<'_>>,
    stack_pointer: u64,
) -> uefi::services::boot::MemoryDescriptor {
    let efi_stack_region = *memory_map
        .iter()
        .find(|desc| {
            (desc.physical_start..desc.physical_start + desc.number_of_pages * 4096)
                .contains(&stack_pointer)
        })
        .unwrap();
    efi_stack_region
}

fn get_efi_main_region(
    memory_map: &MemoryMap<&UefiAllocator<'_>>,
) -> uefi::services::boot::MemoryDescriptor {
    let efi_main_region = *memory_map
        .iter()
        .find(|desc| {
            (desc.physical_start..desc.physical_start + desc.number_of_pages * 4096)
                .contains(&(efi_main as u64))
        })
        .unwrap();
    efi_main_region
}

fn get_stack_pointer() -> u64 {
    let stack_pointer: u64;
    unsafe {
        core::arch::asm!("mov {}, rsp", out(reg) stack_pointer);
    }
    stack_pointer
}

fn get_rsdp(system_table: &uefi::SystemTable<uefi::Boot>) -> *const core::ffi::c_void {
    const EFI_ACPI_TABLE_GUID: Guid = Guid(
        0x8868e871,
        0xe4f1,
        0x11d3,
        [0xbc, 0x22, 0x00, 0x80, 0xc7, 0x3c, 0x88, 0x81],
    );
    system_table
        .configuration_table()
        .iter()
        .find(|entry| entry.vendor_guid == EFI_ACPI_TABLE_GUID)
        .unwrap()
        .vendor_table
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    sprintln!("{:?}", info);
    loop {}
}

fn read_kernel_executable<'uefi>(
    uefi_boot_services: &BootServices,
    uefi_allocator: &'uefi UefiAllocator,
) -> Result<Vec<u8, &'uefi UefiAllocator<'uefi>>, usize> {
    let fs = uefi_boot_services.locate_protocol::<FileSystem>()?;
    let root_fs = fs.open_volume()?;
    let file_name = String16::from_str("ros", uefi_allocator).map_err(|_| 99usize)?;
    let file = root_fs.open(file_name.as_raw(), 0x3, 0x0)?;
    let info = file.get_info(uefi_allocator)?;
    let mut buffer = Vec::with_size_default(info.file_size as usize, uefi_allocator).unwrap();
    let _read_bytes = file.read(&mut buffer).unwrap();
    Ok(buffer)
}

fn optimize_memory_map<'uefi>(
    memory_map: &MemoryMap<&UefiAllocator<'uefi>>,
    bump_allocator: &BumpAllocator,
) -> RawVec<MemoryRegion> {
    let ptr = bump_allocator.allocate_frame().unwrap();
    let mut mem_regions = unsafe {
        RawVec::from_raw_parts(
            ptr as *mut MemoryRegion,
            4096 / core::mem::size_of::<MemoryRegion>(),
        )
    };
    for desc in memory_map.iter() {
        let mem_type = map_mem_type(desc.ty);
        if let Some(last) = mem_regions.last_mut() {
            let overlap = last.end as u64 == desc.physical_start;
            if overlap
                && last.ty == MemoryRegionType::KernelUsable
                && mem_type == MemoryRegionType::KernelUsable
            {
                last.end += desc.number_of_pages * 4096;
            } else {
                mem_regions
                    .push(MemoryRegion {
                        ty: mem_type,
                        start: desc.physical_start,
                        end: desc.physical_start + desc.number_of_pages * 4096,
                    })
                    .unwrap();
            }
        } else {
            mem_regions
                .push(MemoryRegion {
                    ty: mem_type,
                    start: desc.physical_start,
                    end: desc.physical_start + desc.number_of_pages * 4096,
                })
                .unwrap();
        }
    }

    mem_regions
}

fn map_mem_type(memory_type: MemoryType) -> MemoryRegionType {
    match memory_type {
        MemoryType::EfiReservedMemoryType => MemoryRegionType::ReservedMemoryType,
        MemoryType::EfiLoaderCode => MemoryRegionType::KernelUsable,
        MemoryType::EfiLoaderData => MemoryRegionType::KernelUsable,
        MemoryType::EfiBootServicesCode => MemoryRegionType::KernelUsable,
        MemoryType::EfiBootServicesData => MemoryRegionType::KernelUsable,
        MemoryType::EfiRuntimeServicesCode => MemoryRegionType::EfiRuntimeServicesCode,
        MemoryType::EfiRuntimeServicesData => MemoryRegionType::EfiRuntimeServicesData,
        MemoryType::EfiConventionalMemory => MemoryRegionType::KernelUsable,
        MemoryType::EfiUnusableMemory => MemoryRegionType::UnusableMemory,
        MemoryType::EfiACPIReclaimMemory => MemoryRegionType::ACPIReclaimMemory,
        MemoryType::EfiACPIMemoryNVS => MemoryRegionType::ACPIMemoryNVS,
        MemoryType::EfiMemoryMappedIO => MemoryRegionType::MemoryMappedIO,
        MemoryType::EfiMemoryMappedIOPortSpace => MemoryRegionType::MemoryMappedIOPortSpace,
        MemoryType::EfiPalCode => MemoryRegionType::PalCode,
        // TODO: maybe kernel wants to know this information?
        MemoryType::EfiPersistentMemory => MemoryRegionType::KernelUsable,
        MemoryType::EfiUnacceptedMemoryType => MemoryRegionType::UnacceptedMemoryType,
    }
}
