use crate::aml::{aml::term::term_arg::TermArg, ops::SleepOp, prefixed, Context};
use core::alloc::Allocator;
use parser::{
    error::{ParseError, ParseResult},
    input::Input,
    parser::Parser,
};

#[derive(Debug)]
pub struct Sleep<A: Allocator> {
    pub ms: TermArg<A>,
}

impl<A: Allocator + Clone> Sleep<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        let msec_time = TermArg::p; // => Integer
        prefixed(SleepOp::p, msec_time)
            .map(|ms| Self { ms })
            .add_context("Sleep")
            .parse(input, context, alloc)
    }
}
