use core::alloc::Allocator;

use crate::aml::context::Context;
use crate::aml::data::byte_data;
use crate::aml::name::NameString;
use crate::aml::ops::OpRegionOp;
use crate::aml::parser::fail;
use crate::aml::parser::Input;
use crate::aml::parser::ParseResult;
use crate::aml::term::TermArg;

pub struct OpRegion<A: Allocator> {
    pub name: NameString<A>,
    pub space: u8,
    pub offset: TermArg<A>,
    pub len: TermArg<A>,
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

impl<A: Allocator> core::fmt::Debug for OpRegion<A> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("OpRegion")
            .field("name", &self.name)
            .field("space", &self.space)
            .field("offset", &self.offset)
            .field("len", &self.len)
            .finish()
    }
}
