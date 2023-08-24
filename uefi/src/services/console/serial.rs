use crate::{
    services::boot::{Guid, UefiProtocol},
    Status,
};
use core::ffi::c_void;

impl Serial {
    pub fn reset(&mut self) -> Result<(), usize> {
        let status = (self.reset)(self);
        if status != 0 {
            return Err(status);
        }

        Ok(())
    }

    pub fn set_attributes(
        &mut self,
        baud_rate: u64,
        receive_fifo_depth: u32,
        timeout: u32,
        parity: ParityType,
        data_bits: u8,
        stop_bits: StopBitsType,
    ) -> Result<(), usize> {
        let status = (self.set_attributes)(
            self,
            baud_rate,
            receive_fifo_depth,
            timeout,
            parity,
            data_bits,
            stop_bits,
        );
        if status != 0 {
            return Err(status);
        }

        Ok(())
    }

    pub fn write(&mut self, value: &str) -> Result<(), usize> {
        let mut buffer_size = value.as_bytes().len();
        let status = (self.write)(self, &mut buffer_size, value.as_bytes().as_ptr().cast());
        if status != 0 {
            return Err(status);
        }

        Ok(())
    }
}

impl core::fmt::Write for Serial {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.write(s).unwrap();
        Ok(())
    }
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct Mode {
    control_mask: u32,
    timeout: u32,
    baud_rate: u64,
    receive_fifo_depth: u32,
    data_bits: u32,
    parity: u32,
    stop_bits: u32,
}

#[repr(C)]
pub enum ParityType {
    DefaultParity,
    NoParity,
    EvenParity,
    OddParity,
    MarkParity,
    SpaceParity,
}

#[repr(C)]
pub enum StopBitsType {
    DefaultStopBits,
    OneStopBit,
    OneFiveStopBits,
    TwoStopBits,
}

impl UefiProtocol for Serial {
    const GUID: Guid = Guid(
        0xbb25cf6f,
        0xf1d4,
        0x11d2,
        [0x9a, 0x0c, 0x00, 0x90, 0x27, 0x3f, 0xc1, 0xfd],
    );
}

/// UEFI Spec 2.10 section 12.8.1
#[repr(C)]
pub struct Serial {
    revision: u32,
    reset: extern "efiapi" fn(&Self) -> Status,
    set_attributes: extern "efiapi" fn(
        &Self,
        baud_rate: u64,
        receive_fifo_depth: u32,
        timeout: u32,
        parity: ParityType,
        data_bits: u8,
        stop_bits: StopBitsType,
    ) -> Status,
    set_control: extern "efiapi" fn(&Self, control: u32) -> Status,
    get_control: extern "efiapi" fn(&Self, control: &mut u32) -> Status,
    write: extern "efiapi" fn(&Self, buffer_size: &mut usize, buffer: *const c_void) -> Status,
    read: extern "efiapi" fn(&Self, buffer_size: &mut usize, *mut c_void) -> Status,
    mode: &'static Mode,
    // Revision 1.1
    // CONST EFI_GUID *DeviceTypeGuid;
}
