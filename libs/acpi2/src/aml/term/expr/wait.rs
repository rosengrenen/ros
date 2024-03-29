use core::alloc::Allocator;

use crate::aml::{
    context::Context,
    ops::WaitOp,
    parser::{fail, Input, ParseResult},
    term::{statement::EventObj, TermArg},
};

pub struct Wait<A: Allocator> {
    event: EventObj<A>,
    operand: TermArg<A>,
}

impl<A: Allocator + Clone> Wait<A> {
    pub fn parse<'a>(
        input: Input<'a>,
        context: &mut Context<A>,
        alloc: A,
    ) -> ParseResult<'a, Self> {
        let (_, input) = WaitOp::parse(input)?;
        fail(Self::parse_inner(input, context, alloc))
    }

    fn parse_inner<'a>(
        input: Input<'a>,
        context: &mut Context<A>,
        alloc: A,
    ) -> ParseResult<'a, Self> {
        let (event, input) = EventObj::parse(input, context, alloc.clone())?;
        let (operand, input) = TermArg::parse(input, context, alloc)?;
        Ok((Self { event, operand }, input))
    }
}
