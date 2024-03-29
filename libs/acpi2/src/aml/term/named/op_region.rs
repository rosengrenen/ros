use core::alloc::Allocator;

use crate::aml::{
    context::Context,
    data::byte_data,
    name::NameString,
    ops::OpRegionOp,
    parser::{fail, Input, ParseResult},
    term::TermArg,
};

pub struct OpRegion<A: Allocator> {
    name: NameString<A>,
    space: u8,
    offset: TermArg<A>,
    len: TermArg<A>,
}

impl<A: Allocator + Clone> OpRegion<A> {
    pub fn parse<'a>(
        input: Input<'a>,
        context: &mut Context<A>,
        alloc: A,
    ) -> ParseResult<'a, Self> {
        let (_, input) = OpRegionOp::parse(input)?;
        fail(Self::parse_inner(input, context, alloc))
    }

    fn parse_inner<'a>(
        input: Input<'a>,
        context: &mut Context<A>,
        alloc: A,
    ) -> ParseResult<'a, Self> {
        let (name, input) = NameString::parse(input, alloc.clone())?;
        let (space, input) = byte_data(input)?;
        let (offset, input) = TermArg::parse(input, context, alloc.clone())?;
        let (len, input) = TermArg::parse(input, context, alloc)?;
        let this = Self {
            name,
            space,
            offset,
            len,
        };
        Ok((this, input))
    }
}
