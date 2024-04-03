use alloc::iter::IteratorCollectIn;
use alloc::vec::Vec;
use core::alloc::AllocError;
use core::alloc::Allocator;

pub type RawString16 = *const u16;

#[repr(C)]
pub struct String16<A: Allocator> {
    buf: Vec<u16, A>,
}

impl<A: Allocator> String16<A> {
    pub fn from_str(s: &str, alloc: A) -> Result<Self, AllocError> {
        let mut buf: Vec<_, _> = s.encode_utf16().collect_in(alloc)?;
        // Is this string or cstring??
        buf.push(0).unwrap();
        Ok(Self { buf })
    }

    pub fn as_raw(&self) -> RawString16 {
        self.buf.as_ptr()
    }
}
