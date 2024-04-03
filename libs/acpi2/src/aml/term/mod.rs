use alloc::boxed::Box;
use alloc::vec::Vec;
use core::alloc::Allocator;

use self::expr::Expr;
use self::named::NamedObj;
use self::namespace::NameSpaceModObj;
use self::statement::Statement;
use super::context::Context;
use super::data::DataObj;
use super::misc::ArgObj;
use super::misc::LocalObj;
use super::name::NameString;
use super::parser::fail;
use super::parser::Input;
use super::parser::ParseResult;
use super::parser::ParserError;

pub mod expr;
pub mod named;
pub mod namespace;
pub mod statement;

pub enum SymbolAccess<A: Allocator> {
    Variable(NameString<A>),
    Method {
        name: NameString<A>,
        args: Vec<TermArg<A>, A>,
    },
}

impl<A: Allocator + Clone> SymbolAccess<A> {
    pub fn parse<'a>(
        input: Input<'a>,
        context: &mut Context<A>,
        alloc: A,
    ) -> ParseResult<'a, Self> {
        let (name, input) = NameString::parse(input, alloc.clone())?;

        if let Some(arg_count) = context.method_args(&name) {
            let (args, input) = fail(Self::parse_args(input, context, alloc, arg_count))?;
            return Ok((Self::Method { name, args }, input));
        }

        Ok((Self::Variable(name), input))
    }

    fn parse_args<'a>(
        mut input: Input<'a>,
        context: &mut Context<A>,
        alloc: A,
        arg_count: usize,
    ) -> ParseResult<'a, Vec<TermArg<A>, A>> {
        let mut args = Vec::new(alloc.clone());
        for _ in 0..arg_count {
            let (arg, i) = TermArg::parse(input, context, alloc.clone())?;
            args.push(arg).unwrap();
            input = i;
        }

        Ok((args, input))
    }
}

impl<A: Allocator> core::fmt::Debug for SymbolAccess<A> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Variable(arg0) => f.debug_tuple("Variable").field(arg0).finish(),
            Self::Method { name, args } => f
                .debug_struct("Method")
                .field("name", name)
                .field("args", args)
                .finish(),
        }
    }
}

pub enum Obj<A: Allocator> {
    NameSpaceModObj(NameSpaceModObj<A>),
    NamedObj(NamedObj<A>),
}

impl<A: Allocator + Clone> Obj<A> {
    pub fn parse<'a>(
        input: Input<'a>,
        context: &mut Context<A>,
        alloc: A,
    ) -> ParseResult<'a, Self> {
        match NameSpaceModObj::parse(input, context, alloc.clone()) {
            Ok((value, input)) => return Ok((Self::NameSpaceModObj(value), input)),
            Err(ParserError::Failure) => return Err(ParserError::Failure),
            Err(_) => (),
        }

        let (value, input) = NamedObj::parse(input, context, alloc)?;
        Ok((Self::NamedObj(value), input))
    }
}

impl<A: Allocator> core::fmt::Debug for Obj<A> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::NameSpaceModObj(arg0) => f.debug_tuple("NameSpaceModObj").field(arg0).finish(),
            Self::NamedObj(arg0) => f.debug_tuple("NamedObj").field(arg0).finish(),
        }
    }
}

pub enum TermArg<A: Allocator> {
    ArgObj(ArgObj),
    LocalObj(LocalObj),
    // In ASL, but not AML... moved to MethodInvocation
    // NameString(NameString<A>),
    DataObj(Box<DataObj<A>, A>),
    Expr(Box<Expr<A>, A>),
}

impl<A: Allocator + Clone> TermArg<A> {
    pub fn parse<'a>(
        input: Input<'a>,
        context: &mut Context<A>,
        alloc: A,
    ) -> ParseResult<'a, Self> {
        match ArgObj::parse(input) {
            Ok((value, input)) => return Ok((Self::ArgObj(value), input)),
            Err(ParserError::Failure) => return Err(ParserError::Failure),
            Err(_) => (),
        }

        match LocalObj::parse(input) {
            Ok((value, input)) => return Ok((Self::LocalObj(value), input)),
            Err(ParserError::Failure) => return Err(ParserError::Failure),
            Err(_) => (),
        }

        match DataObj::parse(input, context, alloc.clone()) {
            Ok((value, input)) => {
                return Ok((
                    Self::DataObj(Box::new(value, alloc.clone()).unwrap()),
                    input,
                ))
            }
            Err(ParserError::Failure) => return Err(ParserError::Failure),
            Err(_) => (),
        }

        let (value, input) = Expr::parse(input, context, alloc.clone())?;
        Ok((Self::Expr(Box::new(value, alloc).unwrap()), input))
    }
}

impl<A: Allocator> core::fmt::Debug for TermArg<A> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::ArgObj(arg0) => f.debug_tuple("ArgObj").field(arg0).finish(),
            Self::LocalObj(arg0) => f.debug_tuple("LocalObj").field(arg0).finish(),
            Self::DataObj(arg0) => f.debug_tuple("DataObj").field(arg0).finish(),
            Self::Expr(arg0) => f.debug_tuple("Expr").field(arg0).finish(),
        }
    }
}

pub enum TermObj<A: Allocator> {
    Obj(Box<Obj<A>, A>),
    Statement(Box<Statement<A>, A>),
    Expr(Box<Expr<A>, A>),
}

impl<A: Allocator + Clone> TermObj<A> {
    pub fn parse<'a>(
        input: Input<'a>,
        context: &mut Context<A>,
        alloc: A,
    ) -> ParseResult<'a, Self> {
        match Obj::parse(input, context, alloc.clone()) {
            Ok((value, input)) => return Ok((Self::Obj(Box::new(value, alloc).unwrap()), input)),
            Err(ParserError::Failure) => return Err(ParserError::Failure),
            Err(_) => (),
        }

        match Statement::parse(input, context, alloc.clone()) {
            Ok((value, input)) => {
                return Ok((Self::Statement(Box::new(value, alloc).unwrap()), input))
            }
            Err(ParserError::Failure) => return Err(ParserError::Failure),
            Err(_) => (),
        }

        let (value, input) = Expr::parse(input, context, alloc.clone())?;
        Ok((Self::Expr(Box::new(value, alloc).unwrap()), input))
    }
}

impl<A: Allocator> core::fmt::Debug for TermObj<A> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Obj(arg0) => f.debug_tuple("Obj").field(arg0).finish(),
            Self::Statement(arg0) => f.debug_tuple("Statement").field(arg0).finish(),
            Self::Expr(arg0) => f.debug_tuple("Expr").field(arg0).finish(),
        }
    }
}
