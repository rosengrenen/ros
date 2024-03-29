use core::alloc::Allocator;

use crate::aml::{
    context::Context,
    name::Target,
    ops::ModOp,
    parser::{fail, Input, ParseResult},
    term::TermArg,
};

pub struct Mod<A: Allocator> {
    dividend: TermArg<A>,
    divisor: TermArg<A>,
    target: Target<A>,
}

impl<A: Allocator + Clone> Mod<A> {
    pub fn parse<'a>(
        input: Input<'a>,
        context: &mut Context<A>,
        alloc: A,
    ) -> ParseResult<'a, Self> {
        let (_, input) = ModOp::parse(input)?;
        fail(Self::parse_inner(input, context, alloc))
    }

    fn parse_inner<'a>(
        input: Input<'a>,
        context: &mut Context<A>,
        alloc: A,
    ) -> ParseResult<'a, Self> {
        let (dividend, input) = TermArg::parse(input, context, alloc.clone())?;
        let (divisor, input) = TermArg::parse(input, context, alloc.clone())?;
        let (target, input) = Target::parse(input, context, alloc)?;
        let this = Self {
            dividend,
            divisor,
            target,
        };
        Ok((this, input))
    }
}
