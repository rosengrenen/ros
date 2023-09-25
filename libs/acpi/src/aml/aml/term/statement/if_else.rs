use crate::aml::{
    aml::term::{TermArg, TermObj},
    ops::{ElseOp, IfOp},
    pkg, prefixed, Context,
};
use alloc::vec::Vec;
use core::alloc::Allocator;
use parser::{
    error::{ParseError, ParseResult},
    input::Input,
    multi::many::many,
    parser::Parser,
};

pub struct IfElse<A: Allocator> {
    pub predicate: TermArg<A>,
    pub terms: Vec<TermObj<A>, A>,
    pub else_statement: Option<Else<A>>,
}

impl<A: Allocator + Clone> IfElse<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        let predicate = TermArg::p; // => Integer
        prefixed(IfOp::p, pkg((predicate, many(TermObj::p), Else::p)))
            .map(|(predicate, terms, else_statement)| Self {
                predicate,
                terms,
                else_statement,
            })
            .add_context("IfElse")
            .parse(input, context, alloc)
    }
}

pub struct Else<A: Allocator> {
    pub terms: Vec<TermObj<A>, A>,
}

impl<A: Allocator + Clone> Else<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Option<Self>, E> {
        prefixed(ElseOp::p, pkg(many(TermObj::p)))
            .map(|terms| Self { terms })
            .opt()
            .add_context("Else")
            .parse(input, context, alloc)
    }
}
