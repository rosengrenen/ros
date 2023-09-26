use crate::aml::{aml::name::NameString, ops::EventOp, prefixed, Context};
use core::alloc::Allocator;
use parser::{
    error::{ParseError, ParseResult},
    input::Input,
    parser::Parser,
};

#[derive(Debug)]
pub struct Event<A: Allocator> {
    pub name: NameString<A>,
}

impl<A: Allocator + Clone> Event<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        prefixed(EventOp::p, NameString::p)
            .map(|name| Self { name })
            .add_context("Event")
            .parse(input, context, alloc)
    }
}
