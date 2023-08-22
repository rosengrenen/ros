use core::alloc::{AllocError, Allocator};

use alloc::{iter::IteratorCollectIn, vec::Vec};

pub type RawString16 = *const u16;

#[repr(C)]
pub struct String16<'alloc, A: Allocator> {
    buf: Vec<'alloc, u16, A>,
}

impl<'alloc, A: Allocator> String16<'alloc, A> {
    pub fn from_str(s: &str, alloc: &'alloc A) -> Result<Self, AllocError> {
        let mut buf: Vec<_, _> = s.encode_utf16().collect_in(alloc)?;
        // Is this string or cstring??
        buf.push(0).unwrap();
        Ok(Self { buf })
    }

    pub fn as_raw(&self) -> RawString16 {
        self.buf.as_ptr()
    }
}
