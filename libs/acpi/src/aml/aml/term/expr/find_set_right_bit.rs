use crate::aml::{
    aml::{name::Target, term::TermArg},
    ops::FindSetRightBitOp,
    prefixed, Context,
};
use core::alloc::Allocator;
use parser::{
    error::{ParseError, ParseResult},
    input::Input,
    parser::Parser,
};

#[derive(Debug)]
pub struct FindSetRightBit<A: Allocator> {
    pub operand: TermArg<A>,
    pub target: Target<A>,
}

impl<A: Allocator + Clone> FindSetRightBit<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        prefixed(FindSetRightBitOp::p, (TermArg::p, Target::p))
            .map(|(operand, target)| Self { operand, target })
            .add_context("FindSetRightBit")
            .parse(input, context, alloc)
    }
}
