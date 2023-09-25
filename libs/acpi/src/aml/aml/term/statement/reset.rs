use crate::aml::{ops::ResetOp, prefixed, Context};
use core::alloc::Allocator;
use parser::{
    error::{ParseError, ParseResult},
    input::Input,
    parser::Parser,
};

use super::EventObj;

pub struct Reset<A: Allocator> {
    pub event: EventObj<A>,
}

impl<A: Allocator + Clone> Reset<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        prefixed(ResetOp::p, EventObj::p)
            .map(|event| Self { event })
            .add_context("Reset")
            .parse(input, context, alloc)
    }
}
