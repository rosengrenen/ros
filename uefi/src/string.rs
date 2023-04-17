use core::{fmt::Debug, ops::Deref, str::FromStr};

use alloc::vec::Vec;

#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct Char16(pub u16);

#[derive(Debug)]
#[repr(C)]
pub struct RawString16(pub *const Char16);

impl RawString16 {
    pub fn as_ptr(&self) -> *const Char16 {
        self.0
    }
}

impl FromStr for RawString16 {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut buffer: Vec<_> = s.encode_utf16().collect();
        buffer.push(0);
        let ptr = buffer.as_ptr();
        core::mem::forget(buffer);
        Ok(Self(ptr as _))
    }
}

#[repr(C)]
pub struct String16(RawString16);

impl String16 {
    pub fn as_ptr(&self) -> *const Char16 {
        self.0.as_ptr()
    }
}

impl Drop for String16 {
    fn drop(&mut self) {
        let raw = &self.0;
        let ptr = raw.0;
        drop(ptr);
    }
}

impl FromStr for String16 {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.parse()?))
    }
}

impl Deref for String16 {
    type Target = RawString16;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
