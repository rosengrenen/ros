use crate::aml::{ops::ContinueOp, Context};
use core::alloc::Allocator;
use parser::{
    error::{ParseError, ParseResult},
    input::Input,
    parser::Parser,
};

pub struct Continue;

impl Continue {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>, A: Allocator + Clone>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        ContinueOp::p
            .map(|_| Self)
            .add_context("Continue")
            .parse(input, context, alloc)
    }
}
