use crate::aml::{
    data::{byte_data, word_data},
    name::NameString,
    ops::PowerResOp,
    pkg_len::pkg,
    prefixed::prefixed,
    term::TermObj,
};
use alloc::vec::Vec;
use parser::multi::many::many;

parser_struct_alloc!(
    struct PowerRes {
        name: NameString<A>,
        system_level: u8,
        resource_order: u16,
        terms: Vec<TermObj<A>, A>,
    },
    prefixed(
        PowerResOp::p,
        pkg((NameString::p, byte_data, word_data, many(TermObj::p),)),
    )
);
