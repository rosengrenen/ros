use core::{alloc::Allocator, fmt::Debug, ops::Deref, str::FromStr};

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

// impl FromStr for RawString16 {
//     type Err = ();

//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         let mut buffer: Vec<_, A> = s.encode_utf16().collect();
//         buffer.push(0);
//         let ptr = buffer.as_ptr();
//         core::mem::forget(buffer);
//         Ok(Self(ptr as _))
//     }
// }

#[repr(C)]
pub struct String16<A: Allocator> {
    buf: Vec<u16, A>,
}

impl<A: Allocator> String16<A> {
    fn from_str(s: &str) -> Result<Self, ()> {
        let mut buf: Vec<_, A> = s.encode_utf16().collect();
        buf.push(0);
        Ok(Self { buf })
    }

    pub fn as_ptr(&self) -> *const Char16 {
        self.0.as_ptr()
    }
}

impl<A: Allocator> FromStr for String16<A> {
    type Err = ();
}

impl Deref for String16 {
    type Target = RawString16;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
