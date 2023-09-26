use super::field_element::FieldElement;
use crate::aml::{
    aml::{data::integer::byte_data, name::NameString},
    ops::FieldOp,
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
pub struct Field<A: Allocator> {
    pub name: NameString<A>,
    pub flags: u8,
    pub fields: Vec<FieldElement<A>, A>,
}

impl<A: Allocator + Clone> Field<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        // fail().add_context("heh").parse(input, context, alloc)?;
        // panic!("heh");
        prefixed(
            FieldOp::p,
            pkg((NameString::p, byte_data, many(FieldElement::p))),
        )
        .map(|(name, flags, fields)| Self {
            name,
            flags,
            fields,
        })
        .add_context("Field")
        .parse(input, context, alloc)
    }
}
