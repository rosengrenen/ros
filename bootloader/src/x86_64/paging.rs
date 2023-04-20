// https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html volume 3A contains paging information

#[derive(Debug)]
#[repr(C)]
pub struct PageMapLevel4TableEntry(pub u64);

#[derive(Debug)]
#[repr(C)]
pub struct PageDirPointerTableEntry(pub u64);

#[derive(Debug)]
#[repr(C)]
pub struct PageDirTableEntry(pub u64);

#[derive(Debug)]
#[repr(C)]
pub struct PageTableEntry(pub u64);

#[derive(Debug)]
#[repr(C, align(4096))]
pub struct PageMapLevel4Table {
    pub entries: [PageMapLevel4TableEntry; 512],
}

#[derive(Debug)]
#[repr(C, align(4096))]
pub struct PageDirPointerTable {
    pub entries: [PageDirPointerTableEntry; 512],
}

#[derive(Debug)]
#[repr(C, align(4096))]
pub struct PageDirTable {
    pub entries: [PageDirTableEntry; 512],
}

#[derive(Debug)]
#[repr(C, align(4096))]
pub struct PageTable {
    pub entries: [PageTableEntry; 512],
}
