use super::{TermList, TermObj};
use crate::aml::{
    data::DataRefObj,
    name::NameString,
    ops::{AliasOp, NameOp, ScopeOp},
    pkg,
};
use core::alloc::Allocator;
use parser::{
    error::{ParseError, ParseResult},
    input::Input,
    multi::many::many_n,
    parser::Parser,
    sequence::preceded,
};

pub enum NameSpaceModObj<A: Allocator> {
    DefAlias(DefAlias<A>),
    DefName(DefName<A>),
    DefScope(DefScope<A>),
}

impl<A: Allocator + Clone> NameSpaceModObj<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        (
            &DefAlias::p.map(Self::DefAlias),
            &DefName::p.map(Self::DefName),
            &DefScope::p.map(Self::DefScope),
        )
            .alt()
            .add_context("NameSpaceModObj")
            .parse(input, alloc)
    }
}

pub struct DefAlias<A: Allocator> {
    source: NameString<A>,
    alias: NameString<A>,
}

impl<A: Allocator + Clone> DefAlias<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        preceded(&AliasOp::p, &(&NameString::p, &NameString::p).cut())
            .map(|(source, alias)| Self { source, alias })
            .add_context("DefAlias")
            .parse(input, alloc)
    }
}

pub struct DefName<A: Allocator> {
    name: NameString<A>,
    data: DataRefObj<A>,
}

impl<A: Allocator + Clone> DefName<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        preceded(&NameOp::p, &(&NameString::p, &DataRefObj::p).cut())
            .map(|(name, data)| Self { name, data })
            .add_context("DefName")
            .parse(input, alloc)
    }
}

pub struct DefScope<A: Allocator> {
    name: NameString<A>,
    terms: TermList<A>,
}

impl<A: Allocator + Clone> DefScope<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        // preceded(scope_op, pkg((NameString::p, TermList::p)))
        preceded(
            &ScopeOp::p,
            &pkg(&(&NameString::p, &many_n(1, &TermObj::p).map(TermList))),
        )
        .map(|(name, terms)| Self { name, terms })
        .add_context("DefScope")
        .parse(input, alloc)
    }
}
