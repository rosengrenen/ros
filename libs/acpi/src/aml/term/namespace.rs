use super::TermObj;
use crate::aml::{
    data::DataRefObj,
    name::NameString,
    ops::{AliasOp, NameOp, ScopeOp},
    pkg_len::pkg_length_left,
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

pub struct Scope<A: Allocator> {
    pub name: NameString<A>,
    pub terms: Vec<TermObj<A>, A>,
}

impl<A: Allocator> core::fmt::Debug for Scope<A> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Scope")
            .field("name", &self.name)
            .field("terms", &self.terms)
            .finish()
    }
}

impl<A: Allocator + Clone> Scope<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        context: &mut Context<A>,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        let (input, _) = ScopeOp::p.parse(input, context, alloc.clone())?;
        let (input, pkg_len) =
            pkg_length_left
                .cut()
                .add_context("Scope")
                .parse(input, context, alloc.clone())?;
        let (rest, input) =
            take(pkg_len)
                .cut()
                .add_context("Scope")
                .parse(input, context, alloc.clone())?;
        let (input, name) =
            NameString::p
                .cut()
                .add_context("Scope")
                .parse(input, context, alloc.clone())?;
        context.push_scope(&name);
        let (_, (terms, _)) =
            (many(TermObj::p), fail())
                .add_context("Scope")
                .parse(input, context, alloc.clone())?;
        context.pop_scope();
        Ok((rest, Self { name, terms }))
    }
}
