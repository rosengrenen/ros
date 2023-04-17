use crate::Status;

use super::{boot::Guid, file::File};

pub const PROTOCOL_GUID: Guid = Guid(
    0x0964e5b22,
    0x6459,
    0x11d2,
    [0x8e, 0x39, 0x00, 0xa0, 0xc9, 0x69, 0x72, 0x3b],
);

impl FileSystem {
    pub fn open_volume(&self) -> Result<*const File, usize> {
        let mut file = core::ptr::null();
        let status = (self.open_volume)(self, &mut file as *mut _);
        if status != 0 {
            return Err(status);
        }

        Ok(file)
    }
}

#[repr(C)]
pub struct FileSystem {
    revision: u64,
    open_volume: extern "efiapi" fn(&Self, *mut *const File) -> Status,
}
