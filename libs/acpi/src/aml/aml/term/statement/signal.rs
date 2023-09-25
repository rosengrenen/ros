use crate::aml::{ops::SignalOp, prefixed, Context};
use core::alloc::Allocator;
use parser::{
    error::{ParseError, ParseResult},
    input::Input,
    parser::Parser,
};

use super::EventObj;

#[derive(Debug)]
pub struct Signal<A: Allocator> {
    pub event: EventObj<A>,
}

impl<A: Allocator + Clone> Signal<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        prefixed(SignalOp::p, EventObj::p)
            .map(|event| Self { event })
            .add_context("Signal")
            .parse(input, context, alloc)
    }
}
