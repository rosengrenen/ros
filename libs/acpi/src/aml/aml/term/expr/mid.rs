use crate::aml::{
    aml::{name::Target, term::TermArg},
    ops::MidOp,
    prefixed, Context,
};
use core::alloc::Allocator;
use parser::{
    error::{ParseError, ParseResult},
    input::Input,
    parser::Parser,
};

#[derive(Debug)]
pub struct Mid<A: Allocator> {
    pub mid_obj: TermArg<A>,
    pub term1: TermArg<A>,
    pub term2: TermArg<A>,
    pub target: Target<A>,
}

impl<A: Allocator + Clone> Mid<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        let mid_obj = TermArg::p; // => Buffer | String
        prefixed(MidOp::p, (mid_obj, TermArg::p, TermArg::p, Target::p))
            .map(|(mid_obj, term1, term2, target)| Self {
                mid_obj,
                term1,
                term2,
                target,
            })
            .add_context("Mid")
            .parse(input, context, alloc)
    }
}
