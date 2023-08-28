#![no_std]
#![no_main]
#![feature(allocator_api)]
#![feature(abi_x86_interrupt)]

mod elf;
// mod print;
mod x86_64;

use alloc::{iter::IteratorCollectIn, vec::Vec};
use bootloader_api::BootInfo;
use core::{
    alloc::{AllocError, Allocator, Layout},
    fmt::Write,
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

use crate::x86_64::idt::IdtEntry;

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

#[derive(Debug)]
#[repr(C)]
struct InterruptStackFrame {
    pub instruction_pointer: u64,
    pub code_segment: u64,
    pub cpu_flags: u64,
    pub stack_pointer: u64,
    pub stack_segment: u64,
}

extern "x86-interrupt" fn interrupt_breakpoint(frame: InterruptStackFrame) {
    // let mut serial = SerialPort::new(COM1_BASE);
    // writeln!(serial, "{:?}", frame);
}

extern "x86-interrupt" fn interrupt_dbl(frame: InterruptStackFrame, code: u64) {
    // let mut serial = SerialPort::new(COM1_BASE);
    // writeln!(serial, "interrupt {:?}, code: {}", frame, code);
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

    let idt = system_table
        .boot_services()
        .allocate_pages(AllocateType::AllocateAnyPages, MemoryType::EfiLoaderData, 1)
        .unwrap() as *mut IdtEntry;
    let idt =
        unsafe { core::slice::from_raw_parts_mut(idt, 4096 / core::mem::size_of::<IdtEntry>()) };
    for e in idt.iter_mut() {
        *e = IdtEntry::new(interrupt_breakpoint as _, 0, 0b1000_0111_0000_000);
    }
    // idt[8] = IdtEntry::new(interrupt_dbl as _, 0, 0b1000_1000_1000_000);
    unsafe {
        #[repr(C, packed(2))]
        pub struct DescriptorTablePointer {
            /// Size of the DT.
            pub limit: u16,
            /// Pointer to the memory region containing the DT.
            pub base: u64,
        }
        let ptr = DescriptorTablePointer {
            limit: idt.len() as _,
            base: idt.as_ptr() as _,
        };
        core::arch::asm!("lidt [{}]; sti", in(reg) &ptr, options(readonly, nostack, preserves_flags));
    }

    unsafe {
        core::arch::asm!("int3");
    }

    write!(serial, "hello").unwrap();

    loop {}

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
                .filter(|desc| match desc.ty {
                    MemoryType::EfiReservedMemoryType => false,
                    MemoryType::EfiLoaderCode => true,
                    MemoryType::EfiLoaderData => true,
                    MemoryType::EfiBootServicesCode => true,
                    MemoryType::EfiBootServicesData => true,
                    MemoryType::EfiRuntimeServicesCode => false,
                    MemoryType::EfiRuntimeServicesData => false,
                    MemoryType::EfiConventionalMemory => true,
                    MemoryType::EfiUnusableMemory => false,
                    MemoryType::EfiACPIReclaimMemory => false,
                    MemoryType::EfiACPIMemoryNVS => false,
                    MemoryType::EfiMemoryMappedIO => false,
                    MemoryType::EfiMemoryMappedIOPortSpace => false,
                    MemoryType::EfiPalCode => false,
                    MemoryType::EfiPersistentMemory => false,
                    MemoryType::EfiUnacceptedMemoryType => false,
                    MemoryType::EfiMaxMemoryType => false,
                })
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

    let new_allocator = BumpAllocator::new(memory_map.iter());
    let memory_map_key = memory_map.key;
    let _memory_descs = memory_map
        .iter()
        .cloned()
        .collect_in::<Vec<_, _>, _>(&new_allocator)
        .unwrap();
    core::mem::forget(memory_map);
    writeln!(serial, "uefi").unwrap();

    // Exit UEFI boot services
    let system_table = system_table
        .exit_boot_services(image_handle, memory_map_key)
        .unwrap();

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
    };
    let info_ptr = &info as *const BootInfo;

    let mut serial = SerialPort::new(COM1_BASE);
    writeln!(serial, "launching kernel!!").unwrap();
    writeln!(serial, "jumping to {:x}", kernel_fn as usize).unwrap();

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
