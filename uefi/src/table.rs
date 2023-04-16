#[repr(C)]
pub struct TableHeader {
    pub(crate) signature: u64,
    pub(crate) revision: u32,
    pub(crate) header_size: u32,
    pub(crate) crc32: u32,
    _reserved: u32,
}
