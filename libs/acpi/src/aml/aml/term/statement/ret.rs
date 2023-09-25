use crate::aml::{ops::ReturnOp, prefixed, Context};
use core::alloc::Allocator;
use parser::{
    error::{ParseError, ParseResult},
    input::Input,
    parser::Parser,
};

use super::ArgObj;

pub struct Return<A: Allocator> {
    pub arg: ArgObj<A>,
}

impl<A: Allocator + Clone> Return<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        prefixed(ReturnOp::p, ArgObj::p)
            .map(|arg| Self { arg })
            .add_context("Return")
            .parse(input, context, alloc)
    }
}
