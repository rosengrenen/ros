use crate::aml::{ops::TimerOp, Context};
use core::alloc::Allocator;
use parser::{
    error::{ParseError, ParseResult},
    input::Input,
    parser::Parser,
};

#[derive(Debug)]
pub struct Timer;

impl Timer {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>, A: Allocator + Clone>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        TimerOp::p
            .map(|_| Self)
            .add_context("Timer")
            .parse(input, context, alloc)
    }
}
