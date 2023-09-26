use crate::aml::{
    name::NameString, ops::ThermalZoneOp, pkg_len::pkg, prefixed::prefixed, term::TermObj,
};
use alloc::vec::Vec;
use parser::multi::many::many;

parser_struct_alloc!(
    struct ThermalZone {
        name: NameString<A>,
        terms: Vec<TermObj<A>, A>,
    },
    prefixed(ThermalZoneOp::p, pkg((NameString::p, many(TermObj::p))))
);
