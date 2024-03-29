use core::alloc::Allocator;

use super::field_element::FieldElement;
use crate::aml::{
    data::byte_data,
    name::NameString,
    ops::IndexFieldOp,
    parser::{fail, fail_if_not_empty, Input, ParseResult},
    pkg_len::pkg,
};
use alloc::vec::Vec;

pub struct IndexField<A: Allocator> {
    name1: NameString<A>,
    name2: NameString<A>,
    flags: u8,
    fields: Vec<FieldElement<A>, A>,
}

impl<A: Allocator + Clone> IndexField<A> {
    pub fn parse<'a>(input: Input<'a>, alloc: A) -> ParseResult<'a, Self> {
        let (_, input) = IndexFieldOp::parse(input)?;
        let (pkg_input, input) = pkg(input)?;
        let (field, pkg_input) = fail(Self::parse_inner(pkg_input, alloc))?;
        fail_if_not_empty(pkg_input)?;
        Ok((field, input))
    }

    fn parse_inner<'a>(input: Input<'a>, alloc: A) -> ParseResult<'a, Self> {
        let (name1, input) = NameString::parse(input, alloc.clone())?;
        let (name2, input) = NameString::parse(input, alloc.clone())?;
        let (flags, mut input) = byte_data(input)?;
        let mut fields = Vec::new(alloc.clone());
        while let Ok((field, i)) = FieldElement::parse(input, alloc.clone()) {
            fields.push(field).unwrap();
            input = i;
        }

        let this = Self {
            name1,
            name2,
            flags,
            fields,
        };
        Ok((this, input))
    }
}
