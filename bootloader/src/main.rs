#![no_std]
#![no_main]

#[macro_use]
extern crate alloc;

mod elf;
// mod print;
mod x86_64;

use core::alloc::Layout;

use bootloader_api::BootInfo;
use elf::get_elf_entry_point_offset;
use uefi::{
    allocator::UefiAllocator,
    services::{
        boot::{AllocateType, MemoryType},
        filesystem::FileSystem,
        graphics::{BltPixel, Graphics},
    },
    string::String16,
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

#[no_mangle]
pub extern "efiapi" fn efi_main(
    image_handle: uefi::Handle,
    system_table: uefi::SystemTable<uefi::Uninit>,
) -> uefi::Status {
    let system_table = system_table.init();
    let uefi_allocator = UefiAllocator::new(system_table.boot_services());
    system_table.con_out().reset(false).unwrap();
    // This is what the bootloader needs to do:
    // 1. Read the kernel file
    let (kernel_fn, kernel_addr, kernel_pages) = {
        let fs = system_table
            .boot_services()
            .locate_protocol::<FileSystem>()
            .unwrap();
        let root_fs = fs.open_volume().unwrap();
        let file_name: String16 = "ros".parse().unwrap();

        let file = unsafe { &*root_fs.open(&file_name, 0x3, 0x0).unwrap() };
        let info = file.get_info().unwrap();
        let mut buffer = vec![0u8; info.file_size as usize];
        let read_bytes = file.read(&mut buffer).unwrap();
        buffer.truncate(read_bytes);
        get_elf_entry_point_offset(system_table.boot_services(), &buffer).unwrap()
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
    let stack_pages = 1;
    let stack = system_table
        .boot_services()
        .allocate_pages(
            AllocateType::AllocateAnyPages,
            MemoryType::EfiLoaderData,
            stack_pages,
        )
        .unwrap();
    let stack_start = stack;
    let stack_end = stack_start + 4096 * stack_pages as u64;

    // Calculate the total size of the boot info struct, including regions which are pointed to
    let boot_info_layout = Layout::new::<BootInfo>();
    let memory_regions_layout = Layout::array::<bootloader_api::MemoryRegion>(0).unwrap();
    let reserved_memory_regions_layout =
        Layout::array::<bootloader_api::ReservedMemoryRegion>(0).unwrap();
    let (boot_info_layout, memory_regions_offset) =
        boot_info_layout.extend(memory_regions_layout).unwrap();
    let (boot_info_layout, reserved_memory_regions_offset) = boot_info_layout
        .extend(reserved_memory_regions_layout)
        .unwrap();

    // Allocate frames for the boot info
    let boot_info_addr = 0;

    // Exit UEFI boot services
    let (system_table, mem_map) = system_table.exit_boot_services(image_handle).unwrap();

    // Populate memory regions
    let memory_regions_addr = boot_info_addr + memory_regions_offset;
    let memory_regions_len = 0;

    // Populate reserved memory regions
    // Kernel
    // Page table
    //
    let reserved_memory_regions_addr = boot_info_addr + reserved_memory_regions_offset;
    let reserved_memory_regions_len = 0;

    let stack_end = stack_end;
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
struct FrameAllocator {}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    loop {}
}

// We can't rely on a global allocator in the bootloader, but one must be
// provided since we use the alloc crate
struct DummyAllocator;

unsafe impl alloc::alloc::GlobalAlloc for DummyAllocator {
    unsafe fn alloc(&self, _layout: Layout) -> *mut u8 {
        unimplemented!()
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        unimplemented!()
    }
}

#[global_allocator]
static GLOBAL_ALLOCATOR: DummyAllocator = DummyAllocator;
