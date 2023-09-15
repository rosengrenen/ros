pub mod named;
pub mod namespace;
pub mod opcodes;

use self::{
    named::NamedObj,
    namespace::NameSpaceModObj,
    opcodes::{expr::ExprOpcode, statement::StatementOpcode},
};
use super::{
    data::DataObj,
    misc::{ArgObj, LocalObj},
    name::NameString,
};
use alloc::{boxed::Box, vec::Vec};
use core::alloc::Allocator;
use parser::{
    error::{ParseError, ParseResult},
    input::Input,
    multi::many::many,
    parser::Parser,
};

pub enum Obj<A: Allocator> {
    NameSpaceModObj(NameSpaceModObj<A>),
    NamedObj(NamedObj<A>),
}

impl<A: Allocator + Clone> Obj<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        (
            NameSpaceModObj::p.map(Self::NameSpaceModObj),
            NamedObj::p.map(Self::NamedObj),
        )
            .alt()
            .add_context("Obj")
            .parse(input, alloc)
    }
}

pub enum TermObj<A: Allocator> {
    Obj(Obj<A>),
    StatementOpcode(StatementOpcode<A>),
    ExprOpcode(ExprOpcode<A>),
}

impl<A: Allocator + Clone> TermObj<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        (
            Obj::p.map(Self::Obj),
            StatementOpcode::p.map(Self::StatementOpcode),
            ExprOpcode::p.map(Self::ExprOpcode),
        )
            .alt()
            .add_context("TermObj")
            .parse(input, alloc)
    }
}

pub struct TermList<A: Allocator>(Vec<TermObj<A>, A>);

impl<A: Allocator + Clone> TermList<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        many(TermObj::p)
            .map(Self)
            .add_context("TermList")
            .parse(input, alloc)
    }
}

pub enum TermArg<A: Allocator> {
    ExprOpcode(Box<ExprOpcode<A>, A>),
    DataObj(Box<DataObj<A>, A>),
    ArgObj(ArgObj),
    LocalObj(LocalObj),
}

impl<A: Allocator + Clone> TermArg<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        let box_alloc1 = alloc.clone();
        let box_alloc2 = alloc.clone();
        (
            ExprOpcode::p.map(|e| Self::ExprOpcode(Box::new(e, box_alloc1.clone()).unwrap())),
            DataObj::p.map(|d| Self::DataObj(Box::new(d, box_alloc2.clone()).unwrap())),
            ArgObj::p.map(Self::ArgObj),
            LocalObj::p.map(Self::LocalObj),
        )
            .alt()
            .add_context("TermArg")
            .parse(input, alloc)
    }
}

pub struct MethodInvocation<A: Allocator> {
    name: NameString<A>,
    args: TermArgList<A>,
}

impl<A: Allocator + Clone> MethodInvocation<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        (NameString::p, TermArgList::p)
            .map(|(name, args)| Self { name, args })
            .add_context("MethodInvocation")
            .parse(input, alloc)
    }
}

pub struct TermArgList<A: Allocator>(Vec<TermArg<A>, A>);

impl<A: Allocator + Clone> TermArgList<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        many(TermArg::p)
            .map(Self)
            .add_context("TermArgList")
            .parse(input, alloc)
    }
}
