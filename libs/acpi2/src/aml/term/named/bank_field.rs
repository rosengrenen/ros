use core::alloc::Allocator;

use crate::aml::{
    context::Context,
    data::byte_data,
    name::NameString,
    ops::BankFieldOp,
    parser::{fail, fail_if_not_empty, Input, ParseResult},
    pkg_len::pkg,
    term::TermArg,
};
use alloc::vec::Vec;

use super::field_element::FieldElement;

pub struct BankField<A: Allocator> {
    name1: NameString<A>,
    name2: NameString<A>,
    bank_value: TermArg<A>,
    field_flags: u8,
    field_list: Vec<FieldElement<A>, A>,
}

impl<A: Allocator + Clone> BankField<A> {
    pub fn parse<'a>(
        input: Input<'a>,
        context: &mut Context<A>,
        alloc: A,
    ) -> ParseResult<'a, Self> {
        let (_, input) = BankFieldOp::parse(input)?;
        let (pkg_input, input) = pkg(input)?;
        let (bank_field, pkg_input) = fail(Self::parse_inner(pkg_input, context, alloc))?;
        fail_if_not_empty(pkg_input)?;
        Ok((bank_field, input))
    }

    pub fn parse_inner<'a>(
        input: Input<'a>,
        context: &mut Context<A>,
        alloc: A,
    ) -> ParseResult<'a, Self> {
        let (name1, input) = NameString::parse(input, alloc.clone())?;
        let (name2, input) = NameString::parse(input, alloc.clone())?;
        let (bank_value, input) = TermArg::parse(input, context, alloc.clone())?;
        let (field_flags, mut input) = byte_data(input)?;
        let mut field_list = Vec::new(alloc.clone());
        while let Ok((field, i)) = FieldElement::parse(input, alloc.clone()) {
            field_list.push(field).unwrap();
            input = i;
        }

        let this = Self {
            name1,
            name2,
            bank_value,
            field_flags,
            field_list,
        };
        Ok((this, input))
    }
}
