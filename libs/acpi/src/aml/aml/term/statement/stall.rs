use crate::aml::{aml::term::term_arg::TermArg, ops::StallOp, prefixed, Context};
use core::alloc::Allocator;
use parser::{
    error::{ParseError, ParseResult},
    input::Input,
    parser::Parser,
};

#[derive(Debug)]
pub struct Stall<A: Allocator> {
    pub us: TermArg<A>,
}

impl<A: Allocator + Clone> Stall<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        let usec_time = TermArg::p; // => Integer
        prefixed(StallOp::p, usec_time)
            .map(|us| Self { us })
            .add_context("Stall")
            .parse(input, context, alloc)
    }
}
