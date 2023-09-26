use super::PkgElement;
use crate::aml::{ops::VarPkgOp, pkg_len::pkg, prefixed::prefixed, term::TermArg};
use alloc::vec::Vec;
use parser::multi::many::many;

parser_struct_alloc!(
    struct VarPkg {
        len: TermArg<A>,
        elements: Vec<PkgElement<A>, A>,
    },
    prefixed(VarPkgOp::p, pkg((TermArg::p, many(PkgElement::p))))
);
