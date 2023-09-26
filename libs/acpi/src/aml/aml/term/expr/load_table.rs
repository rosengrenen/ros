use crate::aml::{aml::term::term_arg::TermArg, ops::LoadTableOp, prefixed, Context};
use core::alloc::Allocator;
use parser::{
    error::{ParseError, ParseResult},
    input::Input,
    parser::Parser,
};

#[derive(Debug)]
pub struct LoadTable<A: Allocator> {
    pub arg1: TermArg<A>,
    pub arg2: TermArg<A>,
    pub arg3: TermArg<A>,
    pub arg4: TermArg<A>,
    pub arg5: TermArg<A>,
    pub arg6: TermArg<A>,
}

impl<A: Allocator + Clone> LoadTable<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        prefixed(
            LoadTableOp::p,
            (
                TermArg::p,
                TermArg::p,
                TermArg::p,
                TermArg::p,
                TermArg::p,
                TermArg::p,
            ),
        )
        .map(|(arg1, arg2, arg3, arg4, arg5, arg6)| Self {
            arg1,
            arg2,
            arg3,
            arg4,
            arg5,
            arg6,
        })
        .add_context("LoadTable")
        .parse(input, context, alloc)
    }
}
