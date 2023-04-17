use core::ffi::c_void;

use crate::{
    string::{Char16, RawString16},
    Status,
};

use super::boot::Guid;

impl File {
    pub fn open(
        &self,
        name: &RawString16,
        open_mode: u64,
        attributes: u64,
    ) -> Result<*const File, usize> {
        let mut file = core::ptr::null();
        let status = (self.open)(
            self,
            &mut file as *mut _,
            name.as_ptr(),
            open_mode,
            attributes,
        );
        if status != 0 {
            return Err(status);
        }

        Ok(file)
    }

    pub fn close(self) -> Result<(), usize> {
        let status = (self.close)(&self);
        if status != 0 {
            return Err(status);
        }

        Ok(())
    }

    pub fn read(&self, buffer: &mut [u8]) -> Result<usize, usize> {
        let mut bytes_read = buffer.len();
        let status = (self.read)(self, &mut bytes_read, buffer.as_mut_ptr() as _);
        if status != 0 {
            return Err(status);
        }

        Ok(bytes_read)
    }

    pub fn get_info(&self) -> Result<&'static FileInfo, usize> {
        let mut buffer = vec![0u8; core::mem::size_of::<FileInfo>()];
        loop {
            let mut buffer_size = buffer.len();
            let status = (self.get_info)(
                self,
                &FILE_INFO_GUID,
                &mut buffer_size,
                buffer.as_mut_ptr() as _,
            );
            if status == 0 {
                break;
            }

            if (status & 0xFFFFFFFF) == 5 {
                buffer = vec![0; buffer_size];
                continue;
            }

            return Err(status);
        }

        // TODO: this is a memory leak, since the buffer is never released
        let file_info = unsafe {
            let buffer_ptr = buffer.as_ptr() as *const FileInfo;
            core::mem::forget(buffer);
            &*buffer_ptr
        };
        Ok(file_info)
    }
}

#[repr(C)]
pub struct File {
    revision: u64,
    open: extern "efiapi" fn(
        &Self,
        file: *mut *const Self,
        file_name: *const Char16,
        open_mode: u64,
        attributes: u64,
    ) -> Status,
    close: extern "efiapi" fn(&Self) -> Status,
    delete: extern "efiapi" fn(&Self) -> Status,
    read: extern "efiapi" fn(&Self, buffer_size: &mut usize, buffer: *mut c_void) -> Status,
    write: extern "efiapi" fn(&Self) -> Status,
    get_position: extern "efiapi" fn(&Self) -> Status,
    set_position: extern "efiapi" fn(&Self) -> Status,
    get_info: extern "efiapi" fn(
        &Self,
        information_type: &Guid,
        buffer_size: &mut usize,
        buffer: *mut c_void,
    ) -> Status,
    set_info: extern "efiapi" fn(&Self) -> Status,
    flush: extern "efiapi" fn(&Self) -> Status,
    open_ex: extern "efiapi" fn(&Self) -> Status, // Added for revision 2
    read_ex: extern "efiapi" fn(&Self) -> Status, // Added for revision 2
    write_ex: extern "efiapi" fn(&Self) -> Status, // Added for revision 2
    flush_ex: extern "efiapi" fn(&Self) -> Status, // Added for revision 2
}

const FILE_INFO_GUID: Guid = Guid(
    0x09576e92,
    0x6d3f,
    0x11d2,
    [0x8e, 0x39, 0x00, 0xa0, 0xc9, 0x69, 0x72, 0x3b],
);

/// UEFI Spec 2.10 section 13.5.16
#[derive(Debug)]
#[repr(C)]
pub struct FileInfo {
    pub size: u64,
    pub file_size: u64,
    pub physical_size: u64,
    pub create_time: Time,
    pub last_access_time: Time,
    pub modification_time: Time,
    pub attribute: u64,
    pub file_name: RawString16,
}

/// UEFI Spec 2.10 section 8.3.1
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct Time {
    pub year: u16,  // 1900 - 9999
    pub month: u8,  // 1 - 12
    pub day: u8,    // 1 - 31
    pub hour: u8,   // 0 - 23
    pub minute: u8, // 0 - 59
    pub second: u8, // 0 - 59
    pub pad1: u8,
    pub nanosecond: u32, // 0 - 999,999,999
    pub time_zone: i16,  // --1440 to 1440 or 2047
    pub daylight: u8,
    pub pad2: u8,
}
