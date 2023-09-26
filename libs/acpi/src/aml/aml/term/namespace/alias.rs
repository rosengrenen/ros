use crate::aml::{aml::name::NameString, ops::AliasOp, prefixed, Context};
use core::alloc::Allocator;
use parser::{
    error::{ParseError, ParseResult},
    input::Input,
    parser::Parser,
};

#[derive(Debug)]
pub struct Alias<A: Allocator> {
    pub source: NameString<A>,
    pub alias: NameString<A>,
}

impl<A: Allocator + Clone> Alias<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        prefixed(AliasOp::p, (NameString::p, NameString::p))
            .map(|(source, alias)| Self { source, alias })
            .add_context("Alias")
            .parse(input, context, alloc)
    }
}
