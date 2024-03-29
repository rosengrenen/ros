use core::alloc::Allocator;

use crate::aml::{
    context::Context,
    ops::DerefOfOp,
    parser::{fail, Input, ParseResult},
    term::TermArg,
};

pub struct DerefOf<A: Allocator> {
    obj_ref: TermArg<A>,
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
