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
use parser::{
    error::{ParseError, ParseResult},
    input::Input,
    multi::many::many,
    parser::Parser,
    primitive::{fail::fail, take::take},
};
use std::alloc::Allocator;

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

pub struct Name<A: Allocator> {
    pub name: NameString<A>,
    pub data: DataRefObj<A>,
}

impl<A: Allocator> core::fmt::Debug for Name<A> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Name")
            .field("name", &self.name)
            .field("data", &self.data)
            .finish()
    }
}

impl<A: Allocator + Clone> Name<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        context: &mut Context<A>,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        let (input, name) = (prefixed(NameOp::p, (NameString::p, DataRefObj::p)))
            .map(|(name, data)| Self { name, data })
            .map(|a| {
                let name = stringify!(Name);
                println!(
                    "{:width$} matched {:x?}, {:x?}",
                    name,
                    a,
                    input.clone(),
                    width = 20
                );
                a
            })
            .add_context("Name")
            .parse(input.clone(), context, alloc)?;
        context.add_field(&name.name);
        Ok((input, name))
    }
}

pub struct Scope<A: Allocator> {
    pub name: NameString<A>,
    pub terms: Vec<TermObj<A>, A>,
}

impl<A: Allocator> core::fmt::Debug for Scope<A> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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
