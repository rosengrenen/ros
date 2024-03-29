use core::alloc::Allocator;

use super::field_element::FieldElement;
use crate::aml::{
    data::byte_data,
    name::NameString,
    ops::FieldOp,
    parser::{fail, fail_if_not_empty, Input, ParseResult},
    pkg_len::pkg,
};
use alloc::vec::Vec;

pub struct Field<A: Allocator> {
    name: NameString<A>,
    flags: u8,
    fields: Vec<FieldElement<A>, A>,
}

impl<A: Allocator + Clone> Field<A> {
    pub fn parse<'a>(input: Input<'a>, alloc: A) -> ParseResult<'a, Self> {
        let (_, input) = FieldOp::parse(input)?;
        let (pkg_input, input) = pkg(input)?;
        let (field, pkg_input) = fail(Self::parse_inner(pkg_input, alloc))?;
        fail_if_not_empty(pkg_input)?;
        Ok((field, input))
    }

    fn parse_inner<'a>(input: Input<'a>, alloc: A) -> ParseResult<'a, Self> {
        let (name, input) = NameString::parse(input, alloc.clone())?;
        let (flags, mut input) = byte_data(input)?;
        let mut fields = Vec::new(alloc.clone());
        while let Ok((field, i)) = FieldElement::parse(input, alloc.clone()) {
            fields.push(field).unwrap();
            input = i;
        }

        let this = Self {
            name,
            flags,
            fields,
        };
        Ok((this, input))
    }
}
