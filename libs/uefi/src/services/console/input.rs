use crate::Status;
use core::ffi::c_void;

impl ConsoleInput {
    pub fn reset(&self, extended_verification: bool) -> Result<(), usize> {
        let status = (self.reset)(self, extended_verification);
        if status != 0 {
            return Err(status);
        }

        Ok(())
    }

    pub fn read_key(&self) -> Result<Key, usize> {
        let mut key = Key::default();
        let status = (self.read_key_stroke)(self, &mut key);
        if status != 0 {
            return Err(status);
        }

        Ok(key)
    }
}

/// UEFI Spec 2.10 section 12.3.3
#[derive(Clone, Copy, Default)]
#[repr(C)]
pub struct Key {
    pub scan_code: u16,
    pub unicode_char: u16,
}

/// UEFI Spec 2.10 section 12.3.1
#[repr(C)]
pub struct ConsoleInput {
    /// UEFI Spec 2.10 section 12.3.2
    reset: extern "efiapi" fn(&Self, extended_verification: bool) -> Status,
    /// UEFI Spec 2.10 section 12.3.3
    read_key_stroke: extern "efiapi" fn(&Self, key: &mut Key) -> Status,
    /// UEFI Spec 2.10 section 12.3.1
    wait_for_key: *mut c_void,
}
