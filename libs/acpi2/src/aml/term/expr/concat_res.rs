use core::alloc::Allocator;

use crate::aml::{
    context::Context,
    name::Target,
    ops::ConcatResOp,
    parser::{fail, Input, ParseResult},
    term::TermArg,
};

pub struct ConcatRes<A: Allocator> {
    pub left: TermArg<A>,
    pub right: TermArg<A>,
    pub target: Target<A>,
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

impl<A: Allocator> core::fmt::Debug for ConcatRes<A> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("ConcatRes")
            .field("left", &self.left)
            .field("right", &self.right)
            .field("target", &self.target)
            .finish()
    }
}
