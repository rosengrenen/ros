use alloc::vec::Vec;
use core::alloc::Allocator;

use super::field_element::FieldElement;
use crate::aml::context::Context;
use crate::aml::data::byte_data;
use crate::aml::name::NameString;
use crate::aml::ops::BankFieldOp;
use crate::aml::parser::fail;
use crate::aml::parser::fail_if_not_empty;
use crate::aml::parser::Input;
use crate::aml::parser::ParseResult;
use crate::aml::pkg_len::pkg;
use crate::aml::term::TermArg;

pub struct BankField<A: Allocator> {
    pub name1: NameString<A>,
    pub name2: NameString<A>,
    pub bank_value: TermArg<A>,
    pub field_flags: u8,
    pub field_list: Vec<FieldElement<A>, A>,
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

impl<A: Allocator> core::fmt::Debug for BankField<A> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("BankField")
            .field("name1", &self.name1)
            .field("name2", &self.name2)
            .field("bank_value", &self.bank_value)
            .field("field_flags", &self.field_flags)
            .field("field_list", &self.field_list)
            .finish()
    }
}
