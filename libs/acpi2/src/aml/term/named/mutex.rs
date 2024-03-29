use core::alloc::Allocator;

use crate::aml::{
    data::byte_data,
    name::NameString,
    ops::MutexOp,
    parser::{fail, Input, ParseResult},
};

pub struct Mutex<A: Allocator> {
    name: NameString<A>,
    flags: u8,
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
