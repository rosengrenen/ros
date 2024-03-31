use core::alloc::Allocator;

use crate::aml::{
    context::Context,
    ops::DerefOfOp,
    parser::{fail, Input, ParseResult},
    term::TermArg,
};

pub struct DerefOf<A: Allocator> {
    pub obj_ref: TermArg<A>,
}

impl<A: Allocator + Clone> DerefOf<A> {
    pub fn parse<'a>(
        input: Input<'a>,
        context: &mut Context<A>,
        alloc: A,
    ) -> ParseResult<'a, Self> {
        let (_, input) = DerefOfOp::parse(input)?;
        fail(Self::parse_inner(input, context, alloc))
    }

    fn parse_inner<'a>(
        input: Input<'a>,
        context: &mut Context<A>,
        alloc: A,
    ) -> ParseResult<'a, Self> {
        let (obj_ref, input) = TermArg::parse(input, context, alloc)?;
        Ok((Self { obj_ref }, input))
    }
}

impl<A: Allocator> core::fmt::Debug for DerefOf<A> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("DerefOf")
            .field("obj_ref", &self.obj_ref)
            .finish()
    }
}
