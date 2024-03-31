use core::alloc::Allocator;

use crate::aml::{
    data::byte_data,
    name::NameString,
    ops::MutexOp,
    parser::{fail, Input, ParseResult},
};

pub struct Mutex<A: Allocator> {
    pub name: NameString<A>,
    pub flags: u8,
}

impl<A: Allocator + Clone> Mutex<A> {
    pub fn parse<'a>(input: Input<'a>, alloc: A) -> ParseResult<'a, Self> {
        let (_, input) = MutexOp::parse(input)?;
        fail(Self::parse_inner(input, alloc))
    }

    fn parse_inner<'a>(input: Input<'a>, alloc: A) -> ParseResult<'a, Self> {
        let (name, input) = NameString::parse(input, alloc)?;
        let (flags, input) = byte_data(input)?;
        Ok((Self { name, flags }, input))
    }
}

impl<A: Allocator> core::fmt::Debug for Mutex<A> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Mutex")
            .field("name", &self.name)
            .field("flags", &self.flags)
            .finish()
    }
}
