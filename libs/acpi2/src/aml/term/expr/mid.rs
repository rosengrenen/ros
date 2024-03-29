use core::alloc::Allocator;

use crate::aml::{
    context::Context,
    name::Target,
    ops::MidOp,
    parser::{fail, Input, ParseResult},
    term::TermArg,
};

pub struct Mid<A: Allocator> {
    mid_obj: TermArg<A>,
    term1: TermArg<A>,
    term2: TermArg<A>,
    target: Target<A>,
}

impl<A: Allocator + Clone> Mid<A> {
    pub fn parse<'a>(
        input: Input<'a>,
        context: &mut Context<A>,
        alloc: A,
    ) -> ParseResult<'a, Self> {
        let (_, input) = MidOp::parse(input)?;
        fail(Self::parse_inner(input, context, alloc))
    }

    fn parse_inner<'a>(
        input: Input<'a>,
        context: &mut Context<A>,
        alloc: A,
    ) -> ParseResult<'a, Self> {
        let (mid_obj, input) = TermArg::parse(input, context, alloc.clone())?;
        let (term1, input) = TermArg::parse(input, context, alloc.clone())?;
        let (term2, input) = TermArg::parse(input, context, alloc.clone())?;
        let (target, input) = Target::parse(input, context, alloc)?;
        let this = Self {
            mid_obj,
            term1,
            term2,
            target,
        };
        Ok((this, input))
    }
}
