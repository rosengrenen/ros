#![no_std]
#![no_main]
#![feature(allocator_api)]
#![feature(abi_x86_interrupt)]
#![feature(format_args_nl)]
// TODO: think about if this is necessary
#![deny(unsafe_op_in_unsafe_fn)]

mod bitmap;
mod interrupt;
mod msr;
mod spinlock;

use acpi::{
    aml::{Context, LocatedInput, SimpleError, SimpleErrorKind, TermObj},
    tables::DefinitionHeader,
};
use bootloader_api::BootInfo;
use buddy::BuddyAllocator;
use core::{alloc::Allocator, panic::PanicInfo};
use parser::{multi::many::many, parser::Parser};
use serial::{SerialPort, COM1_BASE};
use x86_64::{
    control::Cr3,
    gdt::GdtDesc,
    idt::IdtEntry,
    paging::{FrameAllocError, FrameAllocator, PageTable, PhysAddr, Pml4, VirtAddr},
};

#[macro_export]
macro_rules! sprintln {
    ($($arg:tt)*) => {{
        use serial::{SerialPort, COM1_BASE};
        use core::fmt::Write;
        let mut serial = SerialPort::new(COM1_BASE);
        writeln!(serial, $($arg)*).unwrap();
    }}
}

const UPPER_HALF: u64 = 0xffff_8000_0000_0000;
fn translate_hhdm(phys: PhysAddr) -> VirtAddr {
    VirtAddr::new(UPPER_HALF | phys.inner())
}

struct Buddy(spinlock::Mutex<BuddyAllocator<5, 4096>>);

impl FrameAllocator for Buddy {
    fn allocate_frame(&self) -> Result<u64, x86_64::paging::FrameAllocError> {
        let a = self
            .0
            .lock()
            .allocate_order(1)
            .map_err(|_| FrameAllocError)?;
        Ok(a as _)
    }

    fn deallocate_frame(&self, frame: u64) -> Result<(), x86_64::paging::FrameAllocError> {
        self.0.lock().deallocate_order(frame as _, 1);
        Ok(())
    }
}

#[derive(Debug)]
#[repr(C, packed(2))]
pub struct DescriptorTablePointer {
    /// Size of the DT.
    pub limit: u16,
    /// Pointer to the memory region containing the DT.
    pub base: u64,
}

pub static mut LAPIC: msr::LApic = msr::LApic { base: 0 };

#[no_mangle]
pub extern "C" fn _start(info: &'static BootInfo) -> ! {
    sprintln!("Kernel is starting...");

    let mut serial = SerialPort::new(COM1_BASE);
    serial.configure(1);

    sprintln!("Setting up buddy allocator...");
    let mut memory_regions = info
        .memory_regions
        .iter()
        .filter(|region| {
            info.allocated_frame_ranges.iter().any(|range| {
                range.base < region.end && range.base + range.frames as u64 * 4096 > region.start
            })
        })
        .map(|region| {
            (
                region.start as usize,
                (region.end - region.start) as usize / 4096,
            )
        });
    let first_memory_region = memory_regions.next().unwrap();
    let mut buddy_allocator = buddy::BuddyAllocator::<5, 4096>::new(
        first_memory_region.0,
        first_memory_region.1,
        info.memory_regions.len(),
    )
    .unwrap();
    buddy_allocator.add_regions(memory_regions).unwrap();
    let buddy_allocator = Buddy(spinlock::Mutex::new(buddy_allocator));
    // TODO: all usable memory is now identity mapped!!!
    // UPDATE: first 4gb should be mapped
    // sprintln!("Creating init frame allocator...");
    // let init_frame_allocator =
    //     InitFrameAllocator::new(&info.memory_regions[..], &info.allocated_frame_ranges[..]);
    let mut page_table = PageTable::<Pml4>::new(Cr3::read().pba_pml4 as _);

    {
        sprintln!("Expanding stack...");
        let target_pages = 16;
        let current_pages = (info.kernel.stack_start - info.kernel.stack_end + 4095) / 4096;
        for i in 0..target_pages - current_pages {
            let frame = buddy_allocator.allocate_frame().unwrap();
            page_table.map(
                VirtAddr::new(info.kernel.stack_end - (i + 1) * 4096),
                PhysAddr::new(frame),
                &buddy_allocator,
            );
        }
    };

    // sprintln!("Creating page allocator...");
    // let page_allocator = KernelPageAllocator::new(
    //     info.kernel.base + info.kernel.frames as u64 * 4096,
    //     32 * 1024 * 1024,
    //     &init_frame_allocator,
    //     page_table,
    // );
    // sprintln!("Creating frame allocator...");
    // let frame_allocator = KernelFrameAllocator::new(
    //     &info.memory_regions[..],
    //     init_frame_allocator,
    //     &page_allocator,
    //     page_table,
    // );

    let allocated_frames = buddy_allocator.0.lock().allocated_bytes.div_ceil(4096);
    sprintln!("Allocated frames: {:?}(KiB)", allocated_frames);

    let gdt = {
        let frame = buddy_allocator.allocate_frame().unwrap();
        let page = translate_hhdm(PhysAddr::new(frame));
        unsafe {
            core::slice::from_raw_parts_mut(
                page.inner() as *mut u64,
                4096 / core::mem::size_of::<u64>(),
            )
        }
    };

    let idt = {
        let frame = buddy_allocator.allocate_frame().unwrap();
        let page = translate_hhdm(PhysAddr::new(frame));
        unsafe {
            core::slice::from_raw_parts_mut(
                page.inner() as *mut IdtEntry,
                4096 / core::mem::size_of::<IdtEntry>(),
            )
        }
    };

    sprintln!("Setting up GDT...");
    init_gdt(gdt);

    sprintln!("Setting up IDT...");
    interrupt::init(idt);

    unsafe {
        sprintln!("Testing breakpoint interrupt...");
        core::arch::asm!("int3");
    }

    // sprintln!("Creating heap allocator...");
    // let heap = Heap::new(
    //     24 * 1024 * 1024,
    //     &frame_allocator,
    //     &page_allocator,
    //     page_table,
    // );

    // let enabled_timer = false;
    // if enabled_timer {
    //     sprintln!("Setting up Local APIC for timer interrupts...");
    //     unsafe {
    //         LAPIC = {
    //             let lapic = msr::LApic::current();
    //             let page = page_allocator.allocate_pages(1);
    //             page_table.map(
    //                 VirtAddr::new(page),
    //                 PhysAddr::new(lapic.base),
    //                 &frame_allocator,
    //             );
    //             msr::LApic { base: page }
    //         }
    //     };
    //     unsafe {
    //         LAPIC.write_spurious_interrupt_vector((1 << 8) | 0x99);
    //         LAPIC.write_divide_configuration(0b1010);
    //         LAPIC.write_timer_lvt((1 << 17) | 0x20);
    //     }
    // }

    // {
    //     let heap = heap.inner.lock();
    //     sprintln!(
    //         "Heap max: {}K, heap current: {}K",
    //         heap.max_allocated_bytes / 1024,
    //         heap.allocated_bytes / 1024
    //     );
    // }

    // sprintln!("Reading rsdp at {:x?}...", info.rsdp);
    // let rsdp = unsafe { Rsdp::from_addr(info.rsdp as u64) };
    // for table_ptr in rsdp.table_ptrs() {
    //     let header = unsafe { table_ptr.read() };
    //     if &header.signature == b"FACP" {
    //         let ptr = table_ptr as *const Fadt;
    //         let fadt = unsafe { ptr.read_unaligned() };
    //         let dsdt_addr = fadt.dsdt;
    //         sprintln!("Reading dsdt...");
    //         print_dsdt(dsdt_addr as u64, &heap);
    //     }
    // }

    // {
    //     let mut heap = heap.inner.lock();
    //     let heap: &mut HeapInner = &mut heap;
    //     heap.defragment();
    //     let free_bytes = heap
    //         .free_spaces
    //         .iter()
    //         .map(|(start, end)| end - start)
    //         .sum::<u64>();
    //     sprintln!("free bytes: {:x?}", free_bytes);
    //     sprintln!("free bytes: {:x?}", 24 * 1024 * 1024 - free_bytes);
    //     // sprintln!("{:x?}", heap);
    //     sprintln!(
    //         "Heap max: {}K, heap current: {}K",
    //         heap.max_allocated_bytes / 1024,
    //         heap.allocated_bytes / 1024
    //     );
    // }

    sprintln!("Entering halt loop...");
    loop {
        unsafe { core::arch::asm!("hlt") };
        sprintln!("Waking up from halt");
    }
}

fn init_gdt(gdt: &mut [u64]) {
    // null segment
    gdt[0] = 0;
    // kernel code segment
    // flags(0x2) = [long mode], access byte(0x9a) = [present, desc type = code/data segment, executable, rw]
    gdt[1] = 0x0020_9a00_0000_0000;
    // kernel data segment
    // flags(0x2) = [long mode], access byte(0x92) = [present, desc type = code/data segment, rw]
    gdt[2] = 0x0020_9200_0000_0000;

    unsafe {
        let ptr = DescriptorTablePointer {
            limit: (gdt.len() * core::mem::size_of::<GdtDesc>() - 1) as u16,
            base: gdt.as_ptr() as _,
        };
        core::arch::asm!("cli");
        core::arch::asm!("lgdt [{}]", in(reg) &ptr);
        core::arch::asm!("sti");
    }

    reload_segments();
}

// TODO: move to x86_64 crate
fn reload_segments() {
    unsafe {
        core::arch::asm!(
                // push the segment selector, index 1 of the gdt
                "push 0x8",
                // load and push the address of the "2" label
                "lea {tmp}, [rip + 2f]",
                "push {tmp}",
                // far return, popping the return address and the new CS value from the stack
                "retfq",
                "2:",
                // set the rest of the segment registers to the data segment in the gdt
                "mov ax, 0x10",
                "mov ds, ax",
                "mov es, ax",
                "mov fs, ax",
                "mov gs, ax",
                "mov ss, ax",
                tmp = lateout(reg) _,
        );
    }
}

fn print_dsdt<A: Allocator>(dsdt_addr: u64, alloc: &A) {
    let ptr = dsdt_addr as *const DefinitionHeader;
    let hdr = unsafe { ptr.read() };
    let dsdt_ptr = dsdt_addr as *const u8;
    let dsdt_len = hdr.length;
    let dsdt_slice = unsafe { core::slice::from_raw_parts(dsdt_ptr, dsdt_len as usize) };
    let input = LocatedInput::new(&dsdt_slice[core::mem::size_of::<DefinitionHeader>()..]);
    let mut context = Context::new(alloc);
    let res = many(TermObj::p::<LocatedInput<&[u8]>, SimpleError<LocatedInput<&[u8]>, _>>).parse(
        input,
        &mut context,
        alloc,
    );
    match res {
        Ok(_ast) => {
            sprintln!("Dsdt ok!");
        }
        Err(e) => match e {
            parser::error::ParserError::Error(_) => sprintln!("err"),
            parser::error::ParserError::Failure(e) => {
                sprintln!("fail");
                for (i, e) in e.errors.iter() {
                    match e {
                        SimpleErrorKind::Context(context) => {
                            sprintln!(
                                "{:x?} {:x?} {:x?}",
                                context,
                                i.span,
                                &i.inner[0..i.inner.len().min(32)]
                            );
                        }
                        SimpleErrorKind::Parser(_) => (),
                    }
                }
            }
        },
    }
}

/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    sprintln!("{}", info);
    loop {}
}
