use super::TermObj;
use crate::aml::{
    data::DataRefObj,
    name::NameString,
    ops::{AliasOp, NameOp, ScopeOp},
    pkg_len::{pkg, pkg_length_left},
    prefixed::prefixed,
    Context,
};
use alloc::vec::Vec;
use core::alloc::Allocator;
use parser::{
    error::{ParseError, ParseResult},
    input::Input,
    multi::many::many,
    parser::Parser,
    primitive::{fail::fail, take::take},
    sequence::preceded,
};

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
    preceded(
        ScopeOp::p,
        pkg((
            NameString::p.map_context(|name, context| { context.push_scope(name) }),
            many(TermObj::p)
        ))
    )
    .map_context(|_, context| context.pop_scope())
);
