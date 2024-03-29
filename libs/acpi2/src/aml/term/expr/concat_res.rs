use core::alloc::Allocator;

use crate::aml::{
    context::Context,
    name::Target,
    ops::ConcatResOp,
    parser::{fail, Input, ParseResult},
    term::TermArg,
};

pub struct ConcatRes<A: Allocator> {
    left: TermArg<A>,
    right: TermArg<A>,
    target: Target<A>,
}

impl<A: Allocator + Clone> ConcatRes<A> {
    pub fn parse<'a>(
        input: Input<'a>,
        context: &mut Context<A>,
        alloc: A,
    ) -> ParseResult<'a, Self> {
        let (_, input) = ConcatResOp::parse(input)?;
        fail(Self::parse_inner(input, context, alloc))
    }

    fn parse_inner<'a>(
        input: Input<'a>,
        context: &mut Context<A>,
        alloc: A,
    ) -> ParseResult<'a, Self> {
        let (left, input) = TermArg::parse(input, context, alloc.clone())?;
        let (right, input) = TermArg::parse(input, context, alloc.clone())?;
        let (target, input) = Target::parse(input, context, alloc)?;
        let this = Self {
            left,
            right,
            target,
        };
        Ok((this, input))
    }
}
