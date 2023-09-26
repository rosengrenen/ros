use crate::aml::{
    aml::term::{statement::EventObj, term_arg::TermArg},
    ops::WaitOp,
    prefixed, Context,
};
use core::alloc::Allocator;
use parser::{
    error::{ParseError, ParseResult},
    input::Input,
    parser::Parser,
};

#[derive(Debug)]
pub struct Wait<A: Allocator> {
    pub event: EventObj<A>,
    pub operand: TermArg<A>,
}

impl<A: Allocator + Clone> Wait<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        prefixed(WaitOp::p, (EventObj::p, TermArg::p))
            .map(|(event, operand)| Self { event, operand })
            .add_context("Wait")
            .parse(input, context, alloc)
    }
}
