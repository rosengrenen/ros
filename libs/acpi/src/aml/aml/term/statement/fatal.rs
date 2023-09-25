use crate::aml::{
    aml::{
        data::{byte_data, dword_data},
        term::TermArg,
    },
    ops::FatalOp,
    prefixed, Context,
};
use core::alloc::Allocator;
use parser::{
    error::{ParseError, ParseResult},
    input::Input,
    parser::Parser,
};

#[derive(Debug)]
pub struct Fatal<A: Allocator> {
    pub ty: u8,
    pub code: u32,
    pub arg: TermArg<A>,
}

impl<A: Allocator + Clone> Fatal<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        let fatal_arg = TermArg::p;
        prefixed(FatalOp::p, (byte_data, dword_data, fatal_arg))
            .map(|(ty, code, arg)| Self { ty, code, arg })
            .add_context("Fatal")
            .parse(input, context, alloc)
    }
}
