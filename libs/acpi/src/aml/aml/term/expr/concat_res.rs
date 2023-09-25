use crate::aml::{
    aml::{name::Target, term::TermArg},
    ops::ConcatResOp,
    prefixed, Context,
};
use core::alloc::Allocator;
use parser::{
    error::{ParseError, ParseResult},
    input::Input,
    parser::Parser,
};

#[derive(Debug)]
pub struct ConcatRes<A: Allocator> {
    pub left: TermArg<A>,
    pub right: TermArg<A>,
    pub target: Target<A>,
}

impl<A: Allocator + Clone> ConcatRes<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        let buf_data = TermArg::p; // => Buffer
        prefixed(ConcatResOp::p, (buf_data, buf_data, Target::p))
            .map(|(left, right, target)| Self {
                left,
                right,
                target,
            })
            .add_context("ConcatRes")
            .parse(input, context, alloc)
    }
}
