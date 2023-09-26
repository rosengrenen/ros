use super::field_element::FieldElement;
use crate::aml::{
    aml::{data::integer::byte_data, name::NameString, term::term_arg::TermArg},
    ops::BankFieldOp,
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
pub struct BankField<A: Allocator> {
    pub name1: NameString<A>,
    pub name2: NameString<A>,
    pub bank_value: TermArg<A>,
    pub field_flags: u8,
    pub field_list: Vec<FieldElement<A>, A>,
}

impl<A: Allocator + Clone> BankField<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        let bank_value = TermArg::p; // => Integer
        prefixed(
            BankFieldOp::p,
            pkg((
                NameString::p,
                NameString::p,
                bank_value,
                byte_data,
                many(FieldElement::p),
            )),
        )
        .map(|(name1, name2, bank_value, field_flags, field_list)| Self {
            name1,
            name2,
            bank_value,
            field_flags,
            field_list,
        })
        .add_context("BankField")
        .parse(input, context, alloc)
    }
}
