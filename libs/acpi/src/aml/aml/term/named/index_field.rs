use super::field_element::FieldElement;
use crate::aml::{
    aml::{data::integer::byte_data, name::NameString},
    ops::IndexFieldOp,
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
pub struct IndexField<A: Allocator> {
    pub name1: NameString<A>,
    pub name2: NameString<A>,
    pub flags: u8,
    pub fields: Vec<FieldElement<A>, A>,
}

impl<A: Allocator + Clone> IndexField<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        prefixed(
            IndexFieldOp::p,
            pkg((
                NameString::p,
                NameString::p,
                byte_data,
                many(FieldElement::p),
            )),
        )
        .map(|(name1, name2, flags, fields)| Self {
            name1,
            name2,
            flags,
            fields,
        })
        .add_context("IndexField")
        .parse(input, context, alloc)
    }
}
