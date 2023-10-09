#![no_std]
#![no_main]
#![feature(allocator_api)]
#![feature(abi_x86_interrupt)]
#![feature(format_args_nl)]
// TODO: think about if this is necessary
#![deny(unsafe_op_in_unsafe_fn)]

mod frame_allocator;
mod init_frame_allocator;
mod interrupt;
mod kernel_page_allocator;
mod msr;
mod spinlock;

use crate::{init_frame_allocator::InitFrameAllocator, kernel_page_allocator::KernelPageAllocator};
use bootloader_api::BootInfo;
use core::panic::PanicInfo;
use serial::{SerialPort, COM1_BASE};
use x86_64::{
    control::Cr3,
    gdt::GdtDesc,
    idt::IdtEntry,
    paging::{FrameAllocator, PageTable, PhysAddr, Pml4, VirtAddr},
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
    sprintln!("in the kernel");
    sprintln!("{:#x?}", info);

    let mut serial = SerialPort::new(COM1_BASE);
    serial.configure(1);

    // TODO: all usable memory is now identity mapped!!!
    // UPDATE: first 4gb should be mapped
    let init_frame_allocator =
        InitFrameAllocator::new(&info.memory_regions[..], &info.allocated_frame_ranges[..]);
    let mut page_table = PageTable::<Pml4>::new(Cr3::read().pba_pml4 as _);
    let page_allocator = KernelPageAllocator::new(
        info.kernel.base + info.kernel.frames as u64 * 4096,
        512 * 1024 * 1024,
        &init_frame_allocator,
        page_table,
    );

    let gdt = {
        let frame = init_frame_allocator.allocate_frame().unwrap();
        let page = page_allocator.allocate_pages(1);
        page_table.map(
            VirtAddr::new(page),
            PhysAddr::new(frame),
            &init_frame_allocator,
        );
        unsafe {
            core::slice::from_raw_parts_mut(page as *mut u64, 4096 / core::mem::size_of::<u64>())
        }
    };

    let idt = {
        let frame = init_frame_allocator.allocate_frame().unwrap();
        let page = page_allocator.allocate_pages(1);
        page_table.map(
            VirtAddr::new(page),
            PhysAddr::new(frame),
            &init_frame_allocator,
        );
        unsafe {
            core::slice::from_raw_parts_mut(
                page as *mut IdtEntry,
                4096 / core::mem::size_of::<IdtEntry>(),
            )
        }
    };

    let lapic_info = msr::lapic_info();
    sprintln!("{:x?}", lapic_info);

    sprintln!("setting up gdt");
    init_gdt(gdt);
    sprintln!("successfully set up gdt (?)");

    sprintln!("setting up idt");
    interrupt::init(idt);
    sprintln!("successfully set up idt (?)");

    unsafe {
        core::arch::asm!("int3");
    }
    // divide_by_zero();
    // cause_page_fault();

    unsafe {
        LAPIC = {
            let lapic = msr::LApic::current();
            let page = page_allocator.allocate_pages(1);
            page_table.map(
                VirtAddr::new(page),
                PhysAddr::new(lapic.base),
                &init_frame_allocator,
            );
            msr::LApic { base: page }
        }
    };
    sprintln!("{:x?}", page_table.translate(VirtAddr::new(0xfee0_0000)));
    unsafe {
        LAPIC.write_spurious_interrupt_vector((1 << 8) | 0x99);
        LAPIC.write_divide_configuration(0b0011);
        LAPIC.write_timer_lvt((1 << 17) | 0x20);
        sprintln!(
            "spurious interrupt vector {:x?}",
            LAPIC.read_spurious_interrupt_vector()
        );
        sprintln!(
            "divide configuration {:x?}",
            LAPIC.read_divide_configuration()
        );
        sprintln!("timer lvt {:x?}", LAPIC.read_timer_lvt());
        sprintln!("initial count {:x?}", LAPIC.read_initial_count());
    }

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

#[allow(dead_code)]
fn divide_by_zero() {
    unsafe { core::arch::asm!("mov dx, 0; div dx") }
}

#[allow(dead_code)]
fn cause_page_fault() {
    unsafe {
        *(0xdead_beef as *mut u64) = 5;
    }
}

/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    sprintln!("{}", info);
    loop {}
}
