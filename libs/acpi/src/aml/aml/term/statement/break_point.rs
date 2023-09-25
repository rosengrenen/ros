use crate::aml::{ops::BreakPointOp, Context};
use core::alloc::Allocator;
use parser::{
    error::{ParseError, ParseResult},
    input::Input,
    parser::Parser,
};

pub struct BreakPoint;

impl BreakPoint {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>, A: Allocator + Clone>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        BreakPointOp::p
            .map(|_| Self)
            .add_context("BreakPoint")
            .parse(input, context, alloc)
    }
}
