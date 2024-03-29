use core::alloc::Allocator;

use crate::aml::{
    context::Context,
    name::Target,
    ops::FindSetRightBitOp,
    parser::{fail, Input, ParseResult},
    term::TermArg,
};

pub struct FindSetRightBit<A: Allocator> {
    operand: TermArg<A>,
    target: Target<A>,
}

impl<A: Allocator + Clone> FindSetRightBit<A> {
    pub fn parse<'a>(
        input: Input<'a>,
        context: &mut Context<A>,
        alloc: A,
    ) -> ParseResult<'a, Self> {
        let (_, input) = FindSetRightBitOp::parse(input)?;
        fail(Self::parse_inner(input, context, alloc))
    }

    fn parse_inner<'a>(
        input: Input<'a>,
        context: &mut Context<A>,
        alloc: A,
    ) -> ParseResult<'a, Self> {
        let (operand, input) = TermArg::parse(input, context, alloc.clone())?;
        let (target, input) = Target::parse(input, context, alloc)?;
        Ok((Self { operand, target }, input))
    }
}
