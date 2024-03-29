use self::{expr::Expr, named::NamedObj, namespace::NameSpaceModObj, statement::Statement};

use super::{
    context::Context,
    data::DataObj,
    misc::{ArgObj, LocalObj},
    name::NameString,
    parser::{fail, Input, ParseResult},
};
use alloc::{boxed::Box, vec::Vec};
use core::alloc::Allocator;

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

// impl<A: Allocator> core::fmt::Debug for SymbolAccess<A> {
//     fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
//         match self {
//             SymbolAccess::Variable(name) => f.debug_tuple("Variable").field(name).finish(),
//             SymbolAccess::Method { name, args } => f
//                 .debug_struct("Method")
//                 .field("name", name)
//                 .field("args", args)
//                 .finish(),
//         }
//     }
// }

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
        if let Ok((value, input)) = NameSpaceModObj::parse(input, context, alloc.clone()) {
            return Ok((Self::NameSpaceModObj(value), input));
        }

        let (value, input) = NamedObj::parse(input, context, alloc)?;
        Ok((Self::NamedObj(value), input))
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
        if let Ok((value, input)) = ArgObj::parse(input) {
            return Ok((Self::ArgObj(value), input));
        }

        if let Ok((value, input)) = LocalObj::parse(input) {
            return Ok((Self::LocalObj(value), input));
        }

        if let Ok((value, input)) = DataObj::parse(input, context, alloc.clone()) {
            return Ok((
                Self::DataObj(Box::new(value, alloc.clone()).unwrap()),
                input,
            ));
        }

        let (value, input) = Expr::parse(input, context, alloc.clone())?;
        Ok((Self::Expr(Box::new(value, alloc).unwrap()), input))
    }
}

// impl<A: Allocator> core::fmt::Debug for TermArg<A> {
//     fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
//         match self {
//             Self::ArgObj(inner) => f.debug_tuple("ArgObj").field(inner).finish(),
//             Self::LocalObj(inner) => f.debug_tuple("LocalObj").field(inner).finish(),
//             Self::DataObj(inner) => f.debug_tuple("DataObj").field(inner).finish(),
//             Self::Expr(inner) => f.debug_tuple("Expr").field(inner).finish(),
//         }
//     }
// }

pub enum TermObj<A: Allocator> {
    Obj(Obj<A>),
    Statement(Statement<A>),
    Expr(Expr<A>),
}

impl<A: Allocator + Clone> TermObj<A> {
    pub fn parse<'a>(
        input: Input<'a>,
        context: &mut Context<A>,
        alloc: A,
    ) -> ParseResult<'a, Self> {
        if let Ok((value, input)) = Obj::parse(input, context, alloc.clone()) {
            return Ok((Self::Obj(value), input));
        }

        if let Ok((value, input)) = Statement::parse(input, context, alloc.clone()) {
            return Ok((Self::Statement(value), input));
        }

        let (value, input) = Expr::parse(input, context, alloc)?;
        Ok((Self::Expr(value), input))
    }
}
