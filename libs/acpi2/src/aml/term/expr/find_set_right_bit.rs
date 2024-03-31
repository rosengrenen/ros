use core::alloc::Allocator;

use crate::aml::{
    context::Context,
    name::Target,
    ops::FindSetRightBitOp,
    parser::{fail, Input, ParseResult},
    term::TermArg,
};

pub struct FindSetRightBit<A: Allocator> {
    pub operand: TermArg<A>,
    pub target: Target<A>,
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

impl<A: Allocator> core::fmt::Debug for FindSetRightBit<A> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("FindSetRightBit")
            .field("operand", &self.operand)
            .field("target", &self.target)
            .finish()
    }
}
