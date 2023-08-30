#![no_std]
#![no_main]
#![feature(allocator_api)]
#![feature(abi_x86_interrupt)]

mod elf;

use alloc::{iter::IteratorCollectIn, vec::Vec};
use bootloader_api::BootInfo;
use core::{
    alloc::{AllocError, Allocator, Layout},
    fmt::Write,
    ops::DerefMut,
    ptr::NonNull,
};
use elf::get_elf_entry_point_offset;
use serial::{SerialPort, COM1_BASE};
use uefi::{
    allocator::UefiAllocator,
    services::{
        boot::{AllocateType, MemoryDescriptor, MemoryType},
        console::serial::Serial,
        filesystem::FileSystem,
        graphics::{BltPixel, Graphics},
    },
    string::String16,
};
use x86_64::{
    control::{Cr0, Cr2, Cr3, Cr4},
    gdt::{GdtDesc, Gdtr},
    idt::{read_cs, IdtEntry},
    paging::PageMapLevel4Table,
};

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

fn read_eflags() -> u64 {
    let flags;
    unsafe {
        core::arch::asm!("pushf; pop {}", out(reg) flags);
    }
    flags
}

#[derive(Debug)]
#[repr(C)]
struct InterruptStackFrame {
    pub instruction_pointer: u64,
    pub code_segment: u64,
    pub cpu_flags: u64,
    pub stack_pointer: u64,
    pub stack_segment: u64,
}

extern "x86-interrupt" fn interrupt_div0(frame: InterruptStackFrame) {
    let mut serial = SerialPort::new(COM1_BASE);
    serial.write_str("Div 0");
    writeln!(serial, "Div 0: {:?}", frame);
}

extern "x86-interrupt" fn interrupt_breakpoint(frame: InterruptStackFrame) {
    loop {}
    let mut serial = SerialPort::new(COM1_BASE);
    serial.write_str("Breakpoint");
    writeln!(serial, "Breakpoint: {:?}", frame);
}

extern "x86-interrupt" fn interrupt_dbl(frame: InterruptStackFrame, code: u64) {
    let mut serial = SerialPort::new(COM1_BASE);
    serial.write_str("double fault");
    // writeln!(serial, "interrupt {:?}, code: {}", frame, code);
}

extern "x86-interrupt" fn interrupt_gp(frame: InterruptStackFrame, code: u64) {
    // let mut serial = SerialPort::new(COM1_BASE);
    // writeln!(serial, "interrupt {:?}, code: {}", frame, code);
}

fn divide_by_zero() {
    unsafe { core::arch::asm!("mov dx, 0; div dx") }
}

#[derive(Debug)]
#[repr(C, packed(2))]
pub struct DescriptorTablePointer {
    /// Size of the DT.
    pub limit: u16,
    /// Pointer to the memory region containing the DT.
    pub base: u64,
}

#[no_mangle]
pub extern "efiapi" fn efi_main(
    image_handle: uefi::Handle,
    system_table: uefi::SystemTable<uefi::Uninit>,
) -> uefi::Status {
    let system_table = system_table.init();
    let uefi_allocator = UefiAllocator::new(system_table.boot_services());
    system_table.con_out().reset(false).unwrap();

    let serial = system_table
        .boot_services()
        .locate_protocol::<Serial>()
        .unwrap();

    // This is what the bootloader needs to do:
    // 1. Read the kernel file
    let (kernel_fn, kernel_addr, kernel_pages) = {
        let fs = system_table
            .boot_services()
            .locate_protocol::<FileSystem>()
            .unwrap();
        let root_fs = fs.open_volume().unwrap();
        let file_name = String16::from_str("ros", &uefi_allocator).unwrap();
        let file = unsafe { &*root_fs.open(file_name.as_raw(), 0x3, 0x0).unwrap() };
        let info = file.get_info(&uefi_allocator).unwrap();
        let mut buffer = Vec::with_elem(0u8, info.file_size as usize, &uefi_allocator).unwrap();
        let _read_bytes = file.read(&mut buffer).unwrap();

        // TODO: impl truncate
        // buffer.truncate(read_bytes);
        get_elf_entry_point_offset(system_table.boot_services(), &buffer, &uefi_allocator).unwrap()
    };

    // 2. Retrieve kernel args from UEFI boot services before it goes out of scope
    let framebuffer = {
        let graphics = system_table
            .boot_services()
            .locate_protocol::<Graphics>()
            .unwrap();
        bootloader_api::Framebuffer {
            base: graphics.mode.frame_buffer_base as _,
            width: graphics.mode.info.horizontal_resolution as _,
            height: graphics.mode.info.vertical_resolution as _,
        }
    };

    // 2. Allocate stack for the kernel
    const STACK_PAGES: usize = 1;
    let stack = system_table
        .boot_services()
        .allocate_pages(
            AllocateType::AllocateAnyPages,
            MemoryType::EfiLoaderData,
            STACK_PAGES,
        )
        .unwrap();
    let stack_start = stack;
    let stack_end = stack_start + 4096 * STACK_PAGES as u64;

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

    // Allocate frames for the boot info
    let boot_info_addr = 0;

    let memory_map = system_table
        .boot_services()
        .get_memory_map(&uefi_allocator)
        .unwrap();

    struct BumpAllocator {
        memory_map: [Option<MemoryDescriptor>; 128],
        memory_map_len: usize,
        inner: BumpAllocatorInner,
    }

    struct BumpAllocatorInner {
        descriptor_index: usize,
        addr: u64,
    }

    impl BumpAllocator {
        pub fn new<'iter>(memory_map_iter: impl Iterator<Item = &'iter MemoryDescriptor>) -> Self {
            let mut memory_map = [None; 128];
            let mut memory_map_len = 0;
            for (i, item) in memory_map_iter
                .filter(|desc| desc.ty.usable_by_loader())
                .filter(|desc| desc.physical_start > 0)
                .enumerate()
            {
                if i >= 128 {
                    break;
                }

                memory_map[i] = Some(*item);
                memory_map_len += 1;
            }

            Self {
                memory_map,
                memory_map_len,
                inner: BumpAllocatorInner {
                    descriptor_index: 0,
                    addr: memory_map[0].unwrap().physical_start,
                },
            }
        }
    }

    impl BumpAllocator {
        fn allocate_pages(&self, num_pages: u64) -> Result<u64, AllocError> {
            let inner = unsafe {
                let inner = (&self.inner) as *const BumpAllocatorInner;
                let inner = inner as *mut BumpAllocatorInner;
                &mut *inner
            };

            loop {
                let mem_desc = &self.memory_map[inner.descriptor_index].unwrap();
                let mem_desc_size = mem_desc.number_of_pages * 4096;
                // align to 4096
                inner.addr = (inner.addr & !0xfff) + 4096;
                let mem_left_in_desc = mem_desc.physical_start + mem_desc_size - inner.addr;

                if mem_left_in_desc >= 4096 * num_pages {
                    let ptr = inner.addr;
                    inner.addr += 4096 * num_pages;
                    return Ok(ptr);
                }

                if inner.descriptor_index >= self.memory_map_len {
                    return Err(AllocError);
                }

                inner.descriptor_index += 1;
                inner.addr = self.memory_map[inner.descriptor_index]
                    .unwrap()
                    .physical_start;
            }
        }
    }

    unsafe impl Allocator for BumpAllocator {
        fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
            let inner = unsafe {
                let inner = (&self.inner) as *const BumpAllocatorInner;
                let inner = inner as *mut BumpAllocatorInner;
                &mut *inner
            };

            loop {
                let mem_desc = &self.memory_map[inner.descriptor_index].unwrap();
                let mem_desc_size = mem_desc.number_of_pages * 4096;
                let mem_left_in_desc = mem_desc.physical_start + mem_desc_size - inner.addr;

                let size = layout.size() as u64;
                let align = layout.align() as u64;
                let align_offset = if align % inner.addr == 0 {
                    0
                } else {
                    align - align % inner.addr
                };

                if mem_left_in_desc >= size + align_offset {
                    let ptr = (inner.addr + align_offset) as *mut u8;
                    inner.addr += align_offset + size;
                    let slice = unsafe { core::slice::from_raw_parts_mut(ptr, size as _) };
                    return Ok(unsafe { NonNull::new_unchecked(slice) });
                }

                if inner.descriptor_index >= self.memory_map_len {
                    return Err(AllocError);
                }

                inner.descriptor_index += 1;
                inner.addr = self.memory_map[inner.descriptor_index]
                    .unwrap()
                    .physical_start;
            }
        }

        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {
            // Deallocating is a noop
        }
    }

    let bump_allocator = BumpAllocator::new(memory_map.iter());
    let memory_map_key = memory_map.key;
    let _memory_descs = memory_map
        .iter()
        .cloned()
        .collect_in::<Vec<_, _>, _>(&bump_allocator)
        .unwrap();
    let mut kernel_mem_regions: Vec<MemoryDescriptor, _> = Vec::new(&bump_allocator);
    for desc in _memory_descs.iter() {
        if !desc.ty.usable_by_kernel() {
            kernel_mem_regions.push(*desc);
            continue;
        }

        if let Some(last) = kernel_mem_regions.last_mut() {
            if last.ty.usable_by_kernel()
                && last.physical_start + last.number_of_pages * 4096 == desc.physical_start
            {
                last.number_of_pages += desc.number_of_pages;
            } else {
                kernel_mem_regions.push(*desc).unwrap();
            }
        } else {
            kernel_mem_regions.push(*desc).unwrap();
        }
    }
    core::mem::forget(memory_map);
    writeln!(serial, "uefi").unwrap();
    writeln!(serial, "mem descs").unwrap();
    for desc in _memory_descs.iter() {
        writeln!(
            serial,
            "{:?} {:x} {:x}",
            desc.ty,
            desc.physical_start,
            desc.physical_start + desc.number_of_pages * 4096
        )
        .unwrap();
    }

    writeln!(serial, "kernel mem regions").unwrap();
    for desc in kernel_mem_regions.iter() {
        writeln!(
            serial,
            "{:?} {:x} {:x}",
            desc.ty,
            desc.physical_start,
            desc.physical_start + desc.number_of_pages * 4096
        )
        .unwrap();
    }

    writeln!(serial, "{:x?}", Cr0::read());
    writeln!(serial, "{:x?}", Cr2::read());
    writeln!(serial, "{:x?}", Cr3::read());
    writeln!(serial, "{:x?}", Cr4::read());
    writeln!(serial, "EFLAGS: {:x?}", read_eflags());
    writeln!(serial, "CS: {:x?}", read_cs());

    // Exit UEFI boot services
    let system_table = system_table
        .exit_boot_services(image_handle, memory_map_key)
        .unwrap();

    let mut serial = SerialPort::new(COM1_BASE);

    let idt = bump_allocator.allocate_pages(1).unwrap();
    let gdt = bump_allocator.allocate_pages(1).unwrap();

    writeln!(serial, "idt at {:x}", idt);
    writeln!(serial, "gdt at {:x}", gdt);
    writeln!(serial, "{:x?}", Cr0::read());
    writeln!(serial, "{:x?}", Cr2::read());
    writeln!(serial, "{:x?}", Cr3::read());
    writeln!(serial, "{:x?}", Cr4::read());
    writeln!(serial, "EFLAGS: {:x?}", read_eflags());
    writeln!(serial, "CS: {:x?}", read_cs());

    let gdt = unsafe { core::slice::from_raw_parts_mut(gdt as *mut u64, 64) };
    for e in gdt.iter_mut() {
        *e = 0;
    }
    // kernel code segment
    // let access = 0b1001_1010;
    gdt[1] = 0x00af_9a00_0000_ffff;
    // kernel data segment
    // let access = 0b1001_0010;
    gdt[2] = 0x00af_9200_0000_ffff;

    // serial.write_str("setting up gdt\n");
    // unsafe {
    //     let ptr = DescriptorTablePointer {
    //         limit: gdt.len() as _,
    //         base: gdt.as_ptr() as _,
    //     };
    //     core::arch::asm!("cli");
    //     core::arch::asm!("lgdt [{}]", in(reg) &ptr, options(readonly, nostack, preserves_flags));
    //     core::arch::asm!("sti");
    // }
    // serial.write_str("successfully set up gdt (?)\n");

    writeln!(serial, "GDTR: {:x?}", Gdtr::read());
    for a in GdtDesc::table_iter() {
        writeln!(serial, "{:x?}", a);
    }

    // Populate memory regions
    let memory_regions_addr = boot_info_addr + memory_regions_offset;
    let memory_regions_len = 0;

    // Populate reserved memory regions
    // Kernel
    // Page table
    //
    let reserved_memory_regions_addr = boot_info_addr + reserved_memory_regions_offset;
    let reserved_memory_regions_len = 0;

    let info = BootInfo {
        uefi_system_table: system_table,
        framebuffer,
        kernel: bootloader_api::Kernel {
            base: kernel_addr,
            frames: kernel_pages,
            stack_base: stack_end as usize,
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
        gdt: gdt.as_ptr() as _,
    };
    let info_ptr = &info as *const BootInfo;

    writeln!(serial, "launching kernel!!").unwrap();
    writeln!(serial, "jumping to {:x}", kernel_fn as usize).unwrap();

    // let pml4 = PageMapLevel4Table::from_cr3();
    // writeln!(serial, "pml4: {:x?}", pml4);
    // for pml4_index in 0..512 {
    //     if let Some(pml4_entry) = pml4.get_index(pml4_index) {
    //         let pml4_entry = unsafe { *pml4_entry };
    //         let pml3 = pml4_entry.page_directory_pointer();
    //         writeln!(serial, "pml3: {:x?}", pml3);
    //         for pml3_index in 0..512 {
    //             if let Some(pml3_entry) = pml3.get_index(pml3_index) {
    //                 let pml3_entry = unsafe { *pml3_entry };
    //                 if pml3_entry.page_size() {
    //                     writeln!(serial, "pml3 page: {:x?}", pml3_entry.page_addr_1gb());
    //                 } else {
    //                     let pml2 = pml3_entry.page_directory();
    //                     writeln!(serial, "pml2: {:x?}", pml2);
    //                 }
    //             }
    //         }
    //     }
    // }

    // writeln!(serial, "fine");

    // panic!();

    // 4. Call the kernel
    unsafe {
        core::arch::asm!("mov rsp, {}; jmp {}",
          in(reg) stack_end - 16,
          in(reg) kernel_fn,
          in("rdi") info_ptr,
        );
    }

    unreachable!("should have jumped to kernel at this point")
}

/// The kernel needs to know what regions of the memory are occupied and which aren't.
/// Before exiting uefi boot services uefi handles memory allocation. But after exiting
/// the boot services and before handing over control to the kernel memory needs to be
/// allocated and kept track of. The frame allocator uses the uefi memory map and
/// allows for allocation of individual frames, starting from the beginning of the memory
/// and progressing upward. With the information the frame allocator stores about
/// allocated frames a new memory map can be built that can be handed over to the kernel.
///
/// The frame allocator does not allocate frames that are part of the uefi boot services,
/// but memory used by the uefi boot services is usable by the kernel, so everything
/// that is possibly in the boot service memory regions need to be moves elsewhere before
/// handing control to the kernel
///
/// The frame allocator does not support deallocations, as it only keeps track of the
/// address of the latest sequentially allocated frame, i.e. does not keep track of
/// invidual frames.
// struct FrameAllocator {}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
