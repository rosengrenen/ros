use crate::aml::{aml::name::SuperName, ops::SizeOfOp, prefixed, Context};
use core::alloc::Allocator;
use parser::{
    error::{ParseError, ParseResult},
    input::Input,
    parser::Parser,
};

#[derive(Debug)]
pub struct SizeOf<A: Allocator> {
    pub name: SuperName<A>,
}

impl<A: Allocator + Clone> SizeOf<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        prefixed(SizeOfOp::p, SuperName::p)
            .map(|name| Self { name })
            .add_context("SizeOf")
            .parse(input, context, alloc)
    }
}
