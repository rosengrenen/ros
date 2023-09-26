use super::{expr::Expr, obj::Obj, statement::Statement};
use crate::aml::Context;
use core::alloc::Allocator;
use parser::{
    error::{ParseError, ParseResult},
    input::Input, parser::Parser,
};

#[derive(Debug)]
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
            .add_context("TermObj")
            .parse(input, context, alloc)
    }
}
