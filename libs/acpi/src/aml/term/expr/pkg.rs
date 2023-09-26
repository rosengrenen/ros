use super::PkgElement;
use crate::aml::{data::byte_data, ops::PkgOp, pkg_len::pkg, prefixed::prefixed};
use alloc::vec::Vec;
use parser::multi::many::many;

parser_struct_alloc!(
    struct Pkg {
        len: usize,
        elements: Vec<PkgElement<A>, A>,
    },
    prefixed(
        PkgOp::p,
        pkg((byte_data.map(|len| len as usize), many(PkgElement::p)))
    )
);
