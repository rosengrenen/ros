use self::{expr::Expr, named::NamedObj, namespace::NameSpaceModObj, statement::Statement};
use super::{
    data::DataObj,
    misc::{ArgObj, LocalObj},
    name::NameString,
};
use crate::aml::Context;
use alloc::{boxed::Box, vec::Vec};
use parser::{
    error::{ParseError, ParseResult},
    input::Input,
    multi::many::many,
    parser::Parser,
    primitive::fail::fail,
};
use std::alloc::Allocator;

pub mod expr;
pub mod named;
pub mod namespace;
pub mod statement;

parser_struct_alloc!(
    struct MethodInvocation {
        name: NameString<A>,
        args: Vec<TermArg<A>, A>,
    },
    (fail(), NameString::p, many(TermArg::p)).map(|(_, name, args)| (name, args))
);

parser_enum_alloc!(
    enum Obj {
        NameSpaceModObj(NameSpaceModObj<A>),
        NamedObj(NamedObj<A>),
    }
);

pub enum TermArg<A: Allocator> {
    Expr(Box<Expr<A>, A>),
    DataObj(Box<DataObj<A>, A>),
    ArgObj(ArgObj),
    LocalObj(LocalObj),
}

impl<A: Allocator> core::fmt::Debug for TermArg<A> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Expr(inner) => f.debug_tuple("Expr").field(inner).finish(),
            Self::DataObj(inner) => f.debug_tuple("DataObj").field(inner).finish(),
            Self::ArgObj(inner) => f.debug_tuple("ArgObj").field(inner).finish(),
            Self::LocalObj(inner) => f.debug_tuple("LocalObj").field(inner).finish(),
        }
    }
}

impl<A: Allocator + Clone> TermArg<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        (
            ArgObj::p.map(Self::ArgObj),
            LocalObj::p.map(Self::LocalObj),
            DataObj::p.boxed().map(Self::DataObj),
            Expr::p.boxed().map(Self::Expr),
        )
            .alt()
            .map(|a| {
                println!(
                    "{:width$} matched {:x?}",
                    "TermArg",
                    input.clone(),
                    width = 20
                );
                a
            })
            .add_context("TermArg")
            .parse(input.clone(), context, alloc)
    }
}

parser_enum_alloc!(
    enum TermObj {
        Obj(Obj<A>),
        Statement(Statement<A>),
        Expr(Expr<A>),
    }
);
