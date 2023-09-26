use super::field_element::FieldElement;
use crate::aml::{
    data::byte_data, name::NameString, ops::FieldOp, pkg_len::pkg, prefixed::prefixed,
};
use alloc::vec::Vec;
use parser::multi::many::many;

parser_struct_alloc!(
    struct Field {
        name: NameString<A>,
        flags: u8,
        fields: Vec<FieldElement<A>, A>,
    },
    prefixed(
        FieldOp::p,
        pkg((NameString::p, byte_data, many(FieldElement::p))),
    )
);
