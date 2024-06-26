use crate::string::RawString16;
use crate::Status;

impl ConsoleOutput {
    pub fn reset(&self, extended_verification: bool) -> Result<(), usize> {
        let status = (self.reset)(self, extended_verification);
        if status != 0 {
            return Err(status);
        }

        Ok(())
    }

    pub fn output_string(&self, string: RawString16) -> Result<(), usize> {
        let status = (self.output_string)(self, string);
        if status != 0 {
            return Err(status);
        }

        Ok(())
    }

    pub fn query_mode(&self, mode_number: usize) -> Result<(usize, usize), usize> {
        let mut columns = 0;
        let mut rows = 0;
        let status = (self.query_mode)(self, mode_number, &mut columns, &mut rows);
        if status != 0 {
            return Err(status);
        }

        Ok((columns, rows))
    }

    pub fn set_mode(&self, mode_number: usize) -> Result<(), usize> {
        let status = (self.set_mode)(self, mode_number);
        if status != 0 {
            return Err(status);
        }

        Ok(())
    }

    pub fn set_cursor_position(&self, column: usize, row: usize) -> Result<(), usize> {
        let status = (self.set_cursor_position)(self, column, row);
        if status != 0 {
            return Err(status);
        }

        Ok(())
    }
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct Mode {
    pub max_mode: i32,
    pub mode: i32,
    pub attribute: i32,
    pub cursor_column: i32,
    pub cursor_row: i32,
    pub cursor_visible: bool,
}

/// UEFI Spec 2.10 section 12.4.1
#[repr(C)]
pub struct ConsoleOutput {
    /// UEFI Spec 2.10 section 12.4.2
    reset: extern "efiapi" fn(&Self, extended_verification: bool) -> Status,
    /// UEFI Spec 2.10 section 12.4.3
    output_string: extern "efiapi" fn(&Self, string: *const u16) -> Status,
    /// UEFI Spec 2.10 section 12.4.4
    test_string: extern "efiapi" fn(&Self, string: *const u16) -> Status,
    /// UEFI Spec 2.10 section 12.4.5
    query_mode: extern "efiapi" fn(
        &Self,
        mode_number: usize,
        columns: &mut usize,
        rows: &mut usize,
    ) -> Status,
    /// UEFI Spec 2.10 section 12.4.6
    set_mode: extern "efiapi" fn(&Self, mode_number: usize) -> Status,
    /// UEFI Spec 2.10 section 12.4.7
    set_attribute: extern "efiapi" fn(&Self, attribute: usize) -> Status,
    /// UEFI Spec 2.10 section 12.4.8
    clear_screen: extern "efiapi" fn(&Self) -> Status,
    /// UEFI Spec 2.10 section 12.4.9
    set_cursor_position: extern "efiapi" fn(&Self, column: usize, row: usize) -> Status,
    /// UEFI Spec 2.10 section 12.4.10
    enable_cursor: extern "efiapi" fn(&Self, visible: bool) -> Status,
    /// UEFI Spec 2.10 section 12.4.1
    pub mode: &'static Mode,
}
