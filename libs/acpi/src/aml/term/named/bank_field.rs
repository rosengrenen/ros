use super::field_element::FieldElement;
use crate::aml::{
    data::byte_data, name::NameString, ops::BankFieldOp, pkg_len::pkg, prefixed::prefixed,
    term::TermArg,
};
use alloc::vec::Vec;
use parser::multi::many::many;

parser_struct_alloc!(
    struct BankField {
        name1: NameString<A>,
        name2: NameString<A>,
        bank_value: TermArg<A>,
        field_flags: u8,
        field_list: Vec<FieldElement<A>, A>,
    },
    prefixed(
        BankFieldOp::p,
        pkg((
            NameString::p,
            NameString::p,
            TermArg::p,
            byte_data,
            many(FieldElement::p),
        )),
    )
);
