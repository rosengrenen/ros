use crate::aml::{aml::name::SuperName, ops::RefOfOp, prefixed, Context};
use core::alloc::Allocator;
use parser::{
    error::{ParseError, ParseResult},
    input::Input,
    parser::Parser,
};

#[derive(Debug)]
pub struct RefOf<A: Allocator> {
    pub name: SuperName<A>,
}

impl<A: Allocator + Clone> RefOf<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        prefixed(RefOfOp::p, SuperName::p)
            .map(|name| Self { name })
            .add_context("RefOf")
            .parse(input, context, alloc)
    }
}
