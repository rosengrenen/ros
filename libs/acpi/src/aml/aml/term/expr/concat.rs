use crate::aml::{
    aml::{name::Target, term::TermArg},
    ops::ConcatOp,
    prefixed, Context,
};
use core::alloc::Allocator;
use parser::{
    error::{ParseError, ParseResult},
    input::Input,
    parser::Parser,
};

pub struct Concat<A: Allocator> {
    pub left: TermArg<A>,
    pub right: TermArg<A>,
    pub target: Target<A>,
}

impl<A: Allocator + Clone> Concat<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        let data = TermArg::p; // => ComputationalData
        prefixed(ConcatOp::p, (data, data, Target::p))
            .map(|(left, right, target)| Self {
                left,
                right,
                target,
            })
            .add_context("Concat")
            .parse(input, context, alloc)
    }
}
