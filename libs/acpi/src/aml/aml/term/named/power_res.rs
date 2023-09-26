use crate::aml::{
    aml::{
        data::integer::{byte_data, word_data},
        name::NameString,
        term::term_obj::TermObj,
    },
    ops::PowerResOp,
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
pub struct PowerRes<A: Allocator> {
    pub name: NameString<A>,
    pub system_level: u8,
    pub resource_order: u16,
    pub terms: Vec<TermObj<A>, A>,
}

impl<A: Allocator + Clone> PowerRes<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        prefixed(
            PowerResOp::p,
            pkg((
                NameString::p,
                system_level,
                resource_order,
                many(TermObj::p),
            )),
        )
        .map(|(name, system_level, resource_order, terms)| Self {
            name,
            system_level,
            resource_order,
            terms,
        })
        .add_context("PowerRes")
        .parse(input, context, alloc)
    }
}

fn system_level<I: Input<Item = u8>, E: ParseError<I, A>, A: Allocator + Clone>(
    input: I,
    context: &mut Context,
    alloc: A,
) -> ParseResult<I, u8, E> {
    byte_data
        .add_context("system_level")
        .parse(input, context, alloc)
}

fn resource_order<I: Input<Item = u8>, E: ParseError<I, A>, A: Allocator + Clone>(
    input: I,
    context: &mut Context,
    alloc: A,
) -> ParseResult<I, u16, E> {
    word_data
        .add_context("resource_order")
        .parse(input, context, alloc)
}
