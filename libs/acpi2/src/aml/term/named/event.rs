use core::alloc::Allocator;

use crate::aml::{
    name::NameString,
    ops::EventOp,
    parser::{fail, Input, ParseResult},
};

pub struct Event<A: Allocator> {
    name: NameString<A>,
}

impl<A: Allocator + Clone> Event<A> {
    pub fn parse<'a>(input: Input<'a>, alloc: A) -> ParseResult<'a, Self> {
        let (_, input) = EventOp::parse(input)?;
        fail(Self::parse_inner(input, alloc))
    }

    fn parse_inner<'a>(input: Input<'a>, alloc: A) -> ParseResult<'a, Self> {
        let (name, input) = NameString::parse(input, alloc)?;
        Ok((Self { name }, input))
    }
}
