use crate::aml::{
    aml::{name::NameString, term::term_obj::TermObj},
    ops::ScopeOp,
    pkg, prefixed, Context,
};
use alloc::vec::Vec;
use core::alloc::Allocator;
use parser::{
    error::{ParseError, ParseResult},
    input::Input,
    multi::many::many_n,
    parser::Parser,
    primitive::rest::rest,
};

#[derive(Debug)]
pub struct Scope<A: Allocator> {
    pub name: NameString<A>,
    pub terms: Vec<TermObj<A>, A>,
}

impl<A: Allocator + Clone> Scope<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        prefixed(
            ScopeOp::p,
            pkg((NameString::p, many_n(3, TermObj::p), rest())),
        )
        .map(|(name, terms, _)| Self { name, terms })
        .add_context("Scope")
        .parse(input, context, alloc)
    }
}
