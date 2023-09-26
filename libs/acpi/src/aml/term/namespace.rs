use super::TermObj;
use crate::aml::{
    data::DataRefObj,
    name::NameString,
    ops::{AliasOp, NameOp, ScopeOp},
    pkg_len::pkg,
    prefixed::prefixed,
};
use alloc::vec::Vec;
use parser::multi::many::many;

parser_enum_alloc!(
    enum NameSpaceModObj {
        Alias(Alias<A>),
        Name(Name<A>),
        Scope(Scope<A>),
    }
);

parser_struct_alloc!(
    struct Alias {
        source: NameString<A>,
        alias: NameString<A>,
    },
    prefixed(AliasOp::p, (NameString::p, NameString::p))
);

parser_struct_alloc!(
    struct Name {
        name: NameString<A>,
        data: DataRefObj<A>,
    },
    prefixed(NameOp::p, (NameString::p, DataRefObj::p))
);

parser_struct_alloc!(
    struct Scope {
        name: NameString<A>,
        terms: Vec<TermObj<A>, A>,
    },
    prefixed(ScopeOp::p, pkg((NameString::p, many(TermObj::p))))
);
