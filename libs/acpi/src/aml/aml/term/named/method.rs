use crate::aml::{
    aml::{data::integer::byte_data, name::NameString, term::term_obj::TermObj},
    ops::MethodOp,
    pkg, prefixed, Context,
};
use alloc::vec::Vec;
use core::alloc::Allocator;
use parser::{
    error::{ParseError, ParseResult},
    input::Input,
    multi::many::many,
    parser::Parser,
};

#[derive(Debug)]
pub struct Method<A: Allocator> {
    pub name: NameString<A>,
    pub flags: MethodFlags,
    pub terms: Vec<TermObj<A>, A>,
}

impl<A: Allocator + Clone> Method<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        prefixed(
            MethodOp::p,
            pkg((NameString::p, MethodFlags::p, many(TermObj::p))),
        )
        .map(|(name, flags, terms)| Self { name, flags, terms })
        .add_context("Method")
        .parse(input, context, alloc)
    }
}

#[derive(Debug)]
pub struct MethodFlags(u8);

impl MethodFlags {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>, A: Allocator + Clone>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        byte_data
            .map(Self)
            .add_context("MethodFlags")
            .parse(input, context, alloc)
    }
}
