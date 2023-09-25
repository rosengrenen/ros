use crate::aml::{ops::BreakOp, Context};
use core::alloc::Allocator;
use parser::{
    error::{ParseError, ParseResult},
    input::Input,
    parser::Parser,
};

#[derive(Debug)]
pub struct Break;

impl Break {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>, A: Allocator + Clone>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        BreakOp::p
            .map(|_| Self)
            .add_context("Break")
            .parse(input, context, alloc)
    }
}
