use crate::aml::{ops::NoopOp, Context};
use core::alloc::Allocator;
use parser::{
    error::{ParseError, ParseResult},
    input::Input,
    parser::Parser,
};

#[derive(Debug)]
pub struct Noop;

impl Noop {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>, A: Allocator + Clone>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        NoopOp::p
            .map(|_| Self)
            .add_context("Noop")
            .parse(input, context, alloc)
    }
}
