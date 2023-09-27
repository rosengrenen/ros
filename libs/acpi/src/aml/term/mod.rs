use self::{expr::Expr, named::NamedObj, namespace::NameSpaceModObj, statement::Statement};
use super::{
    data::DataObj,
    misc::{ArgObj, LocalObj},
    name::NameString,
};
use crate::aml::Context;
use alloc::{boxed::Box, vec::Vec};
use parser::{
    error::{ParseError, ParseErrorKind, ParseResult, ParserError},
    input::Input,
    multi::many::{many, many_n},
    parser::Parser,
    primitive::fail::fail,
};
use std::alloc::Allocator;

pub mod expr;
pub mod named;
pub mod namespace;
pub mod statement;

// TODO: naming
pub enum MethodInvocation<A: Allocator> {
    Variable(NameString<A>),
    Method {
        name: NameString<A>,
        args: Vec<TermArg<A>, A>,
    },
}

impl<A: Allocator> core::fmt::Debug for MethodInvocation<A> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MethodInvocation::Variable(name) => f.debug_tuple("Variable").field(name).finish(),
            MethodInvocation::Method { name, args } => f
                .debug_struct("Method")
                .field("name", name)
                .field("args", args)
                .finish(),
        }
    }
}

impl<A: Allocator + Clone> MethodInvocation<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        context: &mut Context<A>,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        let (input, name) =
            NameString::p
                .add_context("MethodInvocation")
                .parse(input, context, alloc.clone())?;
        println!("{:?}, {:?}", name, context);

        if let Some(args) = context.method_args(&name) {
            let (input, args) = many_n(args, TermArg::p)
                .cut()
                .add_context("MethodInvocation method")
                .parse(input, context, alloc)?;
            return Ok((input, Self::Method { name, args }));
        }

        Ok((input, Self::Variable(name)))
        //
        // panic!("{:?} {:?}", name, context);
        // Err(ParserError::Failure(E::from_error_kind(
        //     input,
        //     ParseErrorKind::Unknown,
        //     alloc,
        // )))
    }
}

parser_enum_alloc!(
    enum Obj {
        NameSpaceModObj(NameSpaceModObj<A>),
        NamedObj(NamedObj<A>),
    }
);

pub enum TermArg<A: Allocator> {
    ArgObj(ArgObj),
    LocalObj(LocalObj),
    // In ASL, but not AML... moved to MethodInvocation
    // NameString(NameString<A>),
    DataObj(Box<DataObj<A>, A>),
    Expr(Box<Expr<A>, A>),
}

impl<A: Allocator> core::fmt::Debug for TermArg<A> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ArgObj(inner) => f.debug_tuple("ArgObj").field(inner).finish(),
            Self::LocalObj(inner) => f.debug_tuple("LocalObj").field(inner).finish(),
            Self::DataObj(inner) => f.debug_tuple("DataObj").field(inner).finish(),
            Self::Expr(inner) => f.debug_tuple("Expr").field(inner).finish(),
        }
    }
}

impl<A: Allocator + Clone> TermArg<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        context: &mut Context<A>,
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
