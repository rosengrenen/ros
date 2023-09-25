use super::TermObj;
use crate::aml::{
    aml::{data::DataRefObj, name::NameString},
    ops::{AliasOp, NameOp, ScopeOp},
    pkg, prefixed, Context,
};
use alloc::vec::Vec;
use core::alloc::Allocator;
use parser::{
    error::{ParseError, ParseResult},
    input::Input,
    multi::many::many_n,
    parser::Parser,
    primitive::rest::rest,
};
use std::fmt::Debug;

#[derive(Debug)]
pub enum NameSpaceModObj<A: Allocator> {
    DefAlias(DefAlias<A>),
    DefName(DefName<A>),
    DefScope(DefScope<A>),
}

impl<A: Allocator + Clone> NameSpaceModObj<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        (
            DefAlias::p.map(Self::DefAlias),
            DefName::p.map(Self::DefName),
            DefScope::p.map(Self::DefScope),
        )
            .alt()
            .add_context("NameSpaceModObj")
            .parse(input, context, alloc)
    }
}

#[derive(Debug)]
pub struct DefAlias<A: Allocator> {
    pub source: NameString<A>,
    pub alias: NameString<A>,
}

impl<A: Allocator + Clone> DefAlias<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        prefixed(AliasOp::p, (NameString::p, NameString::p))
            .map(|(source, alias)| Self { source, alias })
            .add_context("DefAlias")
            .parse(input, context, alloc)
    }
}

#[derive(Debug)]
pub struct DefName<A: Allocator> {
    pub name: NameString<A>,
    pub data: DataRefObj<A>,
}

impl<A: Allocator + Clone> DefName<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        prefixed(NameOp::p, (NameString::p, DataRefObj::p))
            .map(|(name, data)| Self { name, data })
            .add_context("DefName")
            .parse(input, context, alloc)
    }
}

#[derive(Debug)]
pub struct DefScope<A: Allocator> {
    pub name: NameString<A>,
    pub terms: Vec<TermObj<A>, A>,
}

impl<A: Allocator + Clone> DefScope<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        // prefixed(scope_op, pkg((NameString::p, TermList::p)))
        prefixed(
            ScopeOp::p,
            pkg((NameString::p, many_n(3, TermObj::p), rest())),
        )
        .map(|(name, terms, _)| Self { name, terms })
        .add_context("DefScope")
        .parse(input, context, alloc)
    }
}
