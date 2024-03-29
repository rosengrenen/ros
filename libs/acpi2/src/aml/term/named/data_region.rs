use core::alloc::Allocator;

use crate::aml::{
    context::Context,
    name::NameString,
    ops::DataRegionOp,
    parser::{fail, Input, ParseResult},
    term::TermArg,
};

pub struct DataRegion<A: Allocator> {
    name: NameString<A>,
    term1: TermArg<A>,
    term2: TermArg<A>,
    term3: TermArg<A>,
}

impl<A: Allocator + Clone> DataRegion<A> {
    pub fn parse<'a>(
        input: Input<'a>,
        context: &mut Context<A>,
        alloc: A,
    ) -> ParseResult<'a, Self> {
        let (_, input) = DataRegionOp::parse(input)?;
        fail(Self::parse_inner(input, context, alloc))
    }

    fn parse_inner<'a>(
        input: Input<'a>,
        context: &mut Context<A>,
        alloc: A,
    ) -> ParseResult<'a, Self> {
        let (name, input) = NameString::parse(input, alloc.clone())?;
        let (term1, input) = TermArg::parse(input, context, alloc.clone())?;
        let (term2, input) = TermArg::parse(input, context, alloc.clone())?;
        let (term3, input) = TermArg::parse(input, context, alloc)?;
        let this = Self {
            name,
            term1,
            term2,
            term3,
        };
        Ok((this, input))
    }
}
