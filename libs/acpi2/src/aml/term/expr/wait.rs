use core::alloc::Allocator;

use crate::aml::context::Context;
use crate::aml::ops::WaitOp;
use crate::aml::parser::fail;
use crate::aml::parser::Input;
use crate::aml::parser::ParseResult;
use crate::aml::term::statement::EventObj;
use crate::aml::term::TermArg;

pub struct Wait<A: Allocator> {
    pub event: EventObj<A>,
    pub operand: TermArg<A>,
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

impl<A: Allocator> core::fmt::Debug for Wait<A> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Wait")
            .field("event", &self.event)
            .field("operand", &self.operand)
            .finish()
    }
}
