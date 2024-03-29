use core::alloc::Allocator;

use crate::aml::{
    data::byte_data,
    name::NameString,
    ops::ExternalOp,
    parser::{fail, Input, ParseResult},
};

pub struct External<A: Allocator> {
    name: NameString<A>,
    obj_type: u8,
    argument_count: u8,
}

impl<A: Allocator + Clone> External<A> {
    pub fn parse<'a>(input: Input<'a>, alloc: A) -> ParseResult<'a, Self> {
        let (_, input) = ExternalOp::parse(input)?;
        fail(Self::parse_inner(input, alloc))
    }

    fn parse_inner<'a>(input: Input<'a>, alloc: A) -> ParseResult<'a, Self> {
        let (name, input) = NameString::parse(input, alloc)?;
        let (obj_type, input) = byte_data(input)?;
        let (argument_count, input) = byte_data(input)?;
        let this = Self {
            name,
            obj_type,
            argument_count,
        };
        Ok((this, input))
    }
}
