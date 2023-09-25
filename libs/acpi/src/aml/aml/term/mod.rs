pub mod expr;
pub mod named;
pub mod namespace;
pub mod statement;

use crate::aml::Context;

use self::{expr::Expr, named::NamedObj, namespace::NameSpaceModObj, statement::Statement};
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
    primitive::fail::fail,
};

pub enum Obj<A: Allocator> {
    NameSpaceModObj(NameSpaceModObj<A>),
    NamedObj(NamedObj<A>),
}

impl<A: Allocator + Clone> Obj<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        (
            NameSpaceModObj::p.map(Self::NameSpaceModObj),
            NamedObj::p.map(Self::NamedObj),
        )
            .alt()
            .add_context("Obj")
            .parse(input, context, alloc)
    }
}

pub enum TermObj<A: Allocator> {
    Obj(Obj<A>),
    Statement(Statement<A>),
    Expr(Expr<A>),
}

impl<A: Allocator + Clone> TermObj<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        (
            Obj::p.map(Self::Obj),
            Statement::p.map(Self::Statement),
            Expr::p.map(Self::Expr),
        )
            .alt()
            .map(|a| {
                println!("we got one!!!");
                a
            })
            .add_context("TermObj")
            .parse(input, context, alloc)
    }
}

pub enum TermArg<A: Allocator> {
    Expr(Box<Expr<A>, A>),
    DataObj(Box<DataObj<A>, A>),
    ArgObj(ArgObj),
    LocalObj(LocalObj),
}

impl<A: Allocator + Clone> TermArg<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        let box_alloc1 = alloc.clone();
        let box_alloc2 = alloc.clone();
        (
            ArgObj::p.map(Self::ArgObj),
            LocalObj::p.map(Self::LocalObj),
            DataObj::p.map(|d| Self::DataObj(Box::new(d, box_alloc2.clone()).unwrap())),
            Expr::p.map(|e| Self::Expr(Box::new(e, box_alloc1.clone()).unwrap())),
        )
            .alt()
            .add_context("TermArg")
            .parse(input, context, alloc)
    }
}

pub struct MethodInvocation<A: Allocator> {
    pub name: NameString<A>,
    pub args: Vec<TermArg<A>, A>,
}

impl<A: Allocator + Clone> MethodInvocation<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        fail()
            .add_context("MethodInvocation")
            .parse(input, context, alloc)?;
        panic!();
        // (NameString::p, many(TermArg::p))
        //     .map(|(name, args)| Self { name, args })
        //     .add_context("MethodInvocation")
        //     .parse(input, context, alloc)
    }
}
