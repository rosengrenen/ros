use super::boot::Guid;
use super::boot::UefiProtocol;
use crate::Status;

impl Graphics {
    pub fn query_mode(&self, mode_number: u32) -> Result<&'static ModeInfo, usize> {
        let mut info = core::ptr::null();
        let status = (self.query_mode)(self, mode_number, &mut 0, &mut info as *mut _);
        if status != 0 {
            return Err(status);
        }

        Ok(unsafe { &*info })
    }

    pub fn set_mode(&self, mode_number: u32) -> Result<(), usize> {
        let status = (self.set_mode)(self, mode_number);
        if status != 0 {
            return Err(status);
        }

        Ok(())
    }

    pub fn blt(
        &self,
        mut fill: BltPixel,
        x: usize,
        y: usize,
        w: usize,
        h: usize,
    ) -> Result<(), usize> {
        let status = (self.blt)(
            self,
            &mut fill as *mut _,
            BltOperation::VideoFill,
            0,
            0,
            x,
            y,
            w,
            h,
            0,
        );
        if status != 0 {
            return Err(status);
        }

        Ok(())
    }
}

impl UefiProtocol for Graphics {
    const GUID: Guid = Guid(
        0x9042a9de,
        0x23dc,
        0x4a38,
        [0x96, 0xfb, 0x7a, 0xde, 0xd0, 0x80, 0x51, 0x6a],
    );
}

#[repr(C)]
pub struct Graphics {
    query_mode: extern "efiapi" fn(
        &Self,
        mode_number: u32,
        size_of_info: &mut usize,
        info: *mut *const ModeInfo,
    ) -> Status,
    set_mode: extern "efiapi" fn(&Self, mode_number: u32) -> Status,
    blt: extern "efiapi" fn(
        &Self,
        blt_buffer: *mut BltPixel,
        blt_operation: BltOperation,
        source_x: usize,
        source_y: usize,
        destination_x: usize,
        destination_y: usize,
        width: usize,
        height: usize,
        delta: usize,
    ) -> Status,
    pub mode: &'static Mode,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct Mode {
    pub max_mode: u32,
    pub mode: u32,
    pub info: &'static ModeInfo,
    pub size_of_info: usize,
    pub frame_buffer_base: u64,
    pub frame_buffer_size: usize,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct ModeInfo {
    pub version: u32,
    pub horizontal_resolution: u32,
    pub vertical_resolution: u32,
    pub pixel_format: PixelFormat,
    pub pixel_information: PixelBitmask,
    pub pixels_per_scan_line: u32,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub enum PixelFormat {
    RedGreenBlueReserved8BitPerColor,
    BlueGreenRedReserved8BitPerColor,
    BitMask,
    BltOnly,
    FormatMax,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct PixelBitmask {
    pub red_mask: u32,
    pub green_mask: u32,
    pub blue_mask: u32,
    pub reserved_mask: u32,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct BltPixel {
    pub blue: u8,
    pub green: u8,
    pub red: u8,
    pub reserved: u8,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub enum BltOperation {
    VideoFill,
    VideoToBltBuffer,
    BufferToVideo,
    VideoToVideo,
    BltOperationMax,
}
