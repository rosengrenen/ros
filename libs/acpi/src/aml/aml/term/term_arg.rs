use super::expr::Expr;
use crate::aml::{
    aml::{
        data::DataObj,
        misc::{ArgObj, LocalObj},
    },
    Context,
};
use alloc::boxed::Box;
use core::alloc::Allocator;
use parser::{
    error::{ParseError, ParseResult},
    input::Input,
    parser::Parser,
};

#[derive(Debug)]
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
        (
            ArgObj::p.map(Self::ArgObj),
            LocalObj::p.map(Self::LocalObj),
            DataObj::p.boxed().map(Self::DataObj),
            Expr::p.boxed().map(Self::Expr),
        )
            .alt()
            .add_context("TermArg")
            .parse(input, context, alloc)
    }
}
