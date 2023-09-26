use crate::aml::{
    name::NameString, ops::DeviceOp, pkg_len::pkg, prefixed::prefixed, term::TermObj,
};
use alloc::vec::Vec;
use parser::multi::many::many;

parser_struct_alloc!(
    struct Device {
        name: NameString<A>,
        terms: Vec<TermObj<A>, A>,
    },
    prefixed(DeviceOp::p, pkg((NameString::p, many(TermObj::p))))
);
