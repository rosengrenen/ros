use alloc::vec::Vec;
use core::alloc::Allocator;

use super::field_element::FieldElement;
use crate::aml::data::byte_data;
use crate::aml::name::NameString;
use crate::aml::ops::IndexFieldOp;
use crate::aml::parser::fail;
use crate::aml::parser::fail_if_not_empty;
use crate::aml::parser::Input;
use crate::aml::parser::ParseResult;
use crate::aml::pkg_len::pkg;

pub struct IndexField<A: Allocator> {
    pub name1: NameString<A>,
    pub name2: NameString<A>,
    pub flags: u8,
    pub fields: Vec<FieldElement<A>, A>,
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

impl<A: Allocator> core::fmt::Debug for IndexField<A> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("IndexField")
            .field("name1", &self.name1)
            .field("name2", &self.name2)
            .field("flags", &self.flags)
            .field("fields", &self.fields)
            .finish()
    }
}
