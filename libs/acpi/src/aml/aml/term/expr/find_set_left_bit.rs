use crate::aml::{
    aml::{name::Target, term::TermArg},
    ops::FindSetLeftBitOp,
    prefixed, Context,
};
use core::alloc::Allocator;
use parser::{
    error::{ParseError, ParseResult},
    input::Input,
    parser::Parser,
};

#[derive(Debug)]
pub struct FindSetLeftBit<A: Allocator> {
    pub operand: TermArg<A>,
    pub target: Target<A>,
}

impl<A: Allocator + Clone> FindSetLeftBit<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        prefixed(FindSetLeftBitOp::p, (TermArg::p, Target::p))
            .map(|(operand, target)| Self { operand, target })
            .add_context("FindSetLeftBit")
            .parse(input, context, alloc)
    }
}
