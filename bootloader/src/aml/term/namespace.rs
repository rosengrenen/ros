use super::TermList;
use crate::aml::{data::DataRefObj, name::NameString, pkg};
use core::alloc::Allocator;
use parser::{
    error::{ParseError, ParseResult},
    input::Input,
    parser::Parser,
    primitive::item::item,
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
            DefAlias::p.map(Self::DefAlias),
            DefName::p.map(Self::DefName),
            DefScope::p.map(Self::DefScope),
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
        let alias_op = item(0x06);
        preceded(alias_op, (NameString::p, NameString::p))
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
        let name_op = item(0x08);
        preceded(name_op, (NameString::p, DataRefObj::p))
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
        let scope_op = item(0x10);
        preceded(scope_op, pkg((NameString::p, TermList::p)))
            .map(|(name, terms)| Self { name, terms })
            .add_context("DefScope")
            .parse(input, alloc)
    }
}
