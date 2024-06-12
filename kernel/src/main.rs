#![no_std]
#![no_main]
#![feature(allocator_api)]
#![feature(abi_x86_interrupt)]
#![feature(format_args_nl)]
#![feature(non_null_convenience)]
// TODO: think about if this is necessary
#![deny(unsafe_op_in_unsafe_fn)]

// mod bitmap;
mod interrupt;
mod kalloc;
mod msr;
mod slub;
mod spinlock;

use core::alloc::Allocator;
use core::panic::PanicInfo;

use acpi::tables::DefinitionHeader;
use acpi::tables::Fadt;
use acpi::tables::Rsdp;
use acpi2::aml::context::Context;
use acpi2::aml::parser::Input;
use acpi2::parse_term_objs;
use bootloader_api::BootInfo;
use bootloader_api::MemoryRegionType;
use buddy::BuddyAllocator;
use common::addr::PhysAddr;
use common::addr::VirtAddr;
use common::frame::FrameAllocError;
use common::frame::FrameAllocator;
use kalloc::KernelAllocator;
use serial::SerialPort;
use serial::COM1_BASE;
use x86_64::control::Cr3;
use x86_64::gdt::GdtDesc;
use x86_64::idt::IdtEntry;
use x86_64::paging::MappedPageTable;
use x86_64::paging::PageTable;
use x86_64::paging::PageTableFrameMapper;
use x86_64::paging::PageTableFrameOffsetMapper;

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
pub const FRAME_OFFSET_MAPPER: PageTableFrameOffsetMapper =
    PageTableFrameOffsetMapper::new(UPPER_HALF);

fn ilog_ceil(base: usize, value: usize) -> usize {
    let log = value.ilog(base) as usize;
    if value > base.pow(log as _) {
        log + 1
    } else {
        log
    }
}

#[derive(Debug)]
struct Buddy(spinlock::Mutex<BuddyAllocator<5, 4096>>);

impl FrameAllocator for Buddy {
    fn allocate_frames(&self, num_frames: usize) -> Result<PhysAddr, FrameAllocError> {
        let a = self
            .0
            .lock()
            // TODO: f64log2 not in core, only std
            .allocate_order(ilog_ceil(2, num_frames) + 1)
            .map_err(|_| FrameAllocError)?;
        Ok(PhysAddr::new(a as u64))
    }

    fn deallocate_frames(&self, frame: PhysAddr, num_frames: usize) -> Result<(), FrameAllocError> {
        self.0
            .lock()
            .deallocate_order(frame.as_u64() as usize, ilog_ceil(2, num_frames) + 1);
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
pub extern "C" fn _start2() -> ! {
    sprintln!("Cpu is starting...");
    loop {
        unsafe {
            core::arch::asm!(
                "
            mov al, 115;
            mov dx, 0x03f8;
            out dx, al;
        "
            );
        }
    }

    loop {}
}

#[no_mangle]
pub extern "C" fn _start(info: &'static BootInfo) -> ! {
    sprintln!("Kernel is starting...");

    sprintln!("Address of _start2 is {:x}", _start2 as u64);

    sprintln!("{:#x?}", info);

    let mut serial = SerialPort::new(COM1_BASE);
    serial.configure(1);

    sprintln!("Setting up buddy allocator...");
    let mut memory_regions = info
        .memory_regions
        .iter()
        .filter(|region| region.ty == MemoryRegionType::KernelUsable)
        .filter(|region| {
            info.allocated_frame_ranges.iter().all(|range| {
                !(region.start..=region.end).contains(&range.base)
                    && !(region.start..=region.end)
                        .contains(&(range.base + range.frames as u64 * 4096))
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
    let trampoline_frame = buddy_allocator.allocate_frame().unwrap().as_u64();

    let kalloc = KernelAllocator::new(&buddy_allocator);

    let page_table = FRAME_OFFSET_MAPPER
        .frame_to_page(PhysAddr::new(Cr3::read().pba_pml4))
        .as_ref_mut::<PageTable>();
    let mut mapped_page_table = MappedPageTable::new(page_table, FRAME_OFFSET_MAPPER);

    {
        sprintln!("Expanding stack...");
        let current_pages = (info.kernel.stack_start - info.kernel.stack_end + 4095) / 4096;
        let target_pages = 64.max(current_pages);
        for i in 0..target_pages - current_pages {
            let frame = buddy_allocator.allocate_frame().unwrap();
            mapped_page_table
                .map(
                    VirtAddr::new(info.kernel.stack_end - (i + 1) * 4096),
                    frame,
                    &buddy_allocator,
                    true,
                )
                .unwrap();
        }
    };

    let allocated_frames = buddy_allocator.0.lock().allocated_bytes.div_ceil(4096);
    sprintln!("Allocated frames: {:?}(KiB)", allocated_frames);

    let gdt = {
        let frame = buddy_allocator.allocate_frame().unwrap();
        let mut page = FRAME_OFFSET_MAPPER.frame_to_page(frame);
        page.as_slice_mut::<u64>(4096 / core::mem::size_of::<u64>())
    };

    let idt = {
        let frame = buddy_allocator.allocate_frame().unwrap();
        let mut page = FRAME_OFFSET_MAPPER.frame_to_page(frame);
        page.as_slice_mut::<IdtEntry>(4096 / core::mem::size_of::<IdtEntry>())
    };

    sprintln!("Setting up GDT...");
    init_gdt(gdt);

    sprintln!("Setting up IDT...");
    interrupt::init(idt);

    unsafe {
        sprintln!("Testing breakpoint interrupt...");
        core::arch::asm!("int3");
    }

    sprintln!("Setting up Local APIC for timer interrupts...");
    unsafe {
        LAPIC = {
            let lapic = msr::LApic::current();
            let phys_addr = PhysAddr::new(lapic.base);
            let virt_addr = FRAME_OFFSET_MAPPER.frame_to_page(phys_addr);
            msr::LApic {
                base: virt_addr.as_u64(),
            }
        }
    };
    unsafe {
        // This line enables the lapic (i think), so not specific to timers
        LAPIC.write_spurious_interrupt_vector((1 << 8) | 0x99);
        let timer_enabled = false;
        if timer_enabled {
            LAPIC.write_divide_configuration(0b1010);
            LAPIC.write_timer_lvt((1 << 17) | 0x20);
        }
    }

    {
        // TODO: get these from MADT
        // Define I/O APIC registers addresses
        const IOAPIC_REGSEL: u64 = 0xFEC00000;
        const IOAPIC_REGWIN: u64 = 0xFEC00010;

        // Function to write to an I/O APIC register
        fn ioapic_write(reg: u32, value: u32) {
            unsafe {
                (IOAPIC_REGSEL as *mut u32).write(reg);
                (IOAPIC_REGWIN as *mut u32).write(value);
            }
        }

        // Route PS/2 interrupt (IRQ 0x12) to vector 0x21
        let ioapic_irq_low: u32 = 0x21 | (1 << 15); // Set the destination CPU (in this case, CPU 0)
        let ioapic_irq_high: u32 = 0; // Set the delivery mode and other flags to default (0)

        // Configure the I/O APIC entry for IRQ 0x12
        ioapic_write(0x12, ioapic_irq_low);
        ioapic_write(0x13, ioapic_irq_high);

        sprintln!("PS/2 interrupt (IRQ 0x1) routed to vector 0x21 successfully");
    }

    unsafe {
        LAPIC.write_icr_low(0x000C4500);
        if trampoline_frame >= 0x100000 {
            panic!(
                "trampoline must be loaded in a frame below 1MB {:x?}",
                trampoline_frame
            );
        }

        let trampoline_code = include_bytes!("../trampoline.bin");

        let slice = core::slice::from_raw_parts_mut(trampoline_frame as *mut u8, 4096);
        for (i, b) in trampoline_code.iter().enumerate() {
            slice[i] = *b;
        }

        let stack = buddy_allocator.allocate_frame().unwrap();
        sprintln!("trampoline segment {:#x?}", trampoline_frame / 4096);
        sprintln!(
            "stack addr {:#x?}",
            FRAME_OFFSET_MAPPER.frame_to_page(stack)
        );
        LAPIC.write_icr_low(0x000C4600 | (trampoline_frame as u32 / 4096));
    }

    loop {}
    let rsdp_addr = FRAME_OFFSET_MAPPER
        .frame_to_page(PhysAddr::new(info.rsdp as u64))
        .as_u64();
    sprintln!("Reading rsdp at {:x?}...", rsdp_addr);
    let rsdp = unsafe { Rsdp::from_addr(rsdp_addr) };

    // TODO: table_ptrs is not offset_mapped
    for table_ptr in rsdp.table_ptrs() {
        let table_ptr = FRAME_OFFSET_MAPPER
            .frame_to_page(PhysAddr::new(table_ptr as u64))
            .as_ptr::<DefinitionHeader>();
        let header = unsafe { table_ptr.read() };
        let sig = unsafe { core::str::from_utf8_unchecked(&header.signature) };
        sprintln!("{:?}", sig);
        if sig == "FACP" {
            let ptr = table_ptr as *const Fadt;
            let fadt = unsafe { ptr.read_unaligned() };
            let dsdt_addr = FRAME_OFFSET_MAPPER
                .frame_to_page(PhysAddr::new(fadt.dsdt as u64))
                .as_u64();
            sprintln!("Reading dsdt...");
            print_dsdt(dsdt_addr, &kalloc);
        }

        if sig == "APIC" {
            let mut offset: usize = 44;
            while offset < header.length as usize {
                let ptr = unsafe { table_ptr.cast::<u8>().add(offset) };
                let ty = unsafe { ptr.read() };
                let len = unsafe { ptr.add(1).read() };
                match ty {
                    0 => {
                        let uid = unsafe { ptr.add(2).read() };
                        let apic_id = unsafe { ptr.add(3).read() };
                        let flags = unsafe { ptr.add(4).cast::<u32>().read_unaligned() };
                        sprintln!(
                            "Processor local apic => uid: {}, apic id: {}, flags: {:#b}",
                            uid,
                            apic_id,
                            flags
                        );
                    }
                    1 => {
                        let apic_id = unsafe { ptr.add(2).read() };
                        let io_apic_addr = unsafe { ptr.add(4).cast::<u32>().read_unaligned() };
                        let global_system_interrupt_base =
                            unsafe { ptr.add(8).cast::<u32>().read_unaligned() };
                        sprintln!(
                            "I/O apic => apic id: {}, ip apic addr: {:#x}, global system interrupt base: {}",
                            apic_id,
                            io_apic_addr,
                            global_system_interrupt_base
                        );
                    }
                    2 => {
                        let bus = unsafe { ptr.add(2).read() };
                        let source = unsafe { ptr.add(3).read() };
                        let global_system_interrupt =
                            unsafe { ptr.add(4).cast::<u32>().read_unaligned() };
                        let flags = unsafe { ptr.add(8).cast::<u16>().read_unaligned() };
                        sprintln!(
                            "Interrupt source override => bus: {}, source: {}, global system interrupt: {}, flags: {:#b}",
                            bus,
                            source,
                            global_system_interrupt,
                            flags
                        );
                    }
                    _ => (),
                }
                offset += len as usize;
            }
        }
    }

    sprintln!("Entering halt loop...");
    loop {
        unsafe { core::arch::asm!("hlt") };
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
    let input = Input::new(&dsdt_slice[core::mem::size_of::<DefinitionHeader>()..]);
    let mut context = Context::new(alloc);
    let res = parse_term_objs(input, &mut context, alloc);
    match res {
        Ok(ast) => {
            sprintln!("Dsdt ok!");
        }
        Err(e) => sprintln!("Dsdt err... {:?}", e),
    }
}

/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    sprintln!("{}", info);
    loop {}
}
