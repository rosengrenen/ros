#![allow(dead_code)]

use x86_64::paging::{PageTable, PhysAddr, VirtAddr};

use crate::sprintln;

#[allow(dead_code)]
pub fn print_page_table(pml4: &PageTable) {
    let mut num_1gb_pages: u64 = 0;
    let mut num_2mb_pages: u64 = 0;
    let mut num_4kb_pages: u64 = 0;

    // level 4 page
    let mut total_size_bytes: u64 = 4096;

    sprintln!("---- Page table ----");

    for pml4_index in 0..512 {
        if let Some(entry) = pml4.get_index(pml4_index) {
            // level 3 page
            total_size_bytes += 4096;
            let pml3 = entry.page_table();
            for pml3_index in 0..512 {
                if let Some(entry) = pml3.get_index(pml3_index) {
                    if entry.is_page() {
                        num_1gb_pages += 1;
                        sprintln!(
                            "{} => {} (1gb)",
                            VirtAddr::new((pml4_index << 39 | pml3_index << 30) as _),
                            PhysAddr::new(entry.page_addr_1gb() as _)
                        );
                        continue;
                    }

                    // level 2 page
                    total_size_bytes += 4096;

                    let pml2 = entry.page_table();
                    for pml2_index in 0..512 {
                        if let Some(entry) = pml2.get_index(pml2_index) {
                            if entry.is_page() {
                                num_2mb_pages += 1;
                                sprintln!(
                                    "{} => {} (2mb)",
                                    VirtAddr::new(
                                        (pml4_index << 39 | pml3_index << 30 | pml2_index << 21)
                                            as _
                                    ),
                                    PhysAddr::new(entry.page_addr_2mb() as _)
                                );
                                continue;
                            }

                            // level 1 page
                            total_size_bytes += 4096;

                            let pml1 = entry.page_table();
                            for pml1_index in 0..512 {
                                if let Some(page) = pml1.get_index(pml1_index) {
                                    num_4kb_pages += 1;
                                    sprintln!(
                                        "{} => {} (4kb)",
                                        VirtAddr::new(
                                            (pml4_index << 39
                                                | pml3_index << 30
                                                | pml2_index << 21
                                                | pml1_index << 12)
                                                as _
                                        ),
                                        PhysAddr::new(page.page_addr_4kb() as _)
                                    );
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    sprintln!(
        "Paging uses {}kb, num pages {}/1gb, {}/2mb, {}/4kb",
        total_size_bytes / 1024,
        num_1gb_pages,
        num_2mb_pages,
        num_4kb_pages
    );
    sprintln!("--------------------");
}
