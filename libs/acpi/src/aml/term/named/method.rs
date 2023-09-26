use crate::aml::{
    data::byte_data, name::NameString, ops::MethodOp, pkg_len::pkg, prefixed::prefixed,
    term::TermObj,
};
use alloc::vec::Vec;
use parser::multi::many::many;

parser_struct_alloc!(
    struct Method {
        name: NameString<A>,
        flags: u8,
        terms: Vec<TermObj<A>, A>,
    },
    {
        prefixed(
            MethodOp::p,
            pkg((NameString::p, byte_data, many(TermObj::p))),
        )
    }
);
