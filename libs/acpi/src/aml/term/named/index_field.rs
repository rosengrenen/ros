use super::field_element::FieldElement;
use crate::aml::{
    data::byte_data, name::NameString, ops::IndexFieldOp, pkg_len::pkg, prefixed::prefixed,
};
use alloc::vec::Vec;
use parser::multi::many::many;

parser_struct_alloc!(
    struct IndexField {
        name1: NameString<A>,
        name2: NameString<A>,
        flags: u8,
        fields: Vec<FieldElement<A>, A>,
    },
    prefixed(
        IndexFieldOp::p,
        pkg((
            NameString::p,
            NameString::p,
            byte_data,
            many(FieldElement::p),
        )),
    )
);
