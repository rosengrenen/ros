use crate::aml::{
    aml::term::{TermArg, TermObj},
    ops::WhileOp,
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

#[derive(Debug)]
pub struct While<A: Allocator> {
    pub predicate: TermArg<A>,
    pub terms: Vec<TermObj<A>, A>,
}

impl<A: Allocator + Clone> While<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        let predicate = TermArg::p; // => Integer
        prefixed(WhileOp::p, pkg((predicate, many(TermObj::p))))
            .map(|(predicate, terms)| Self { predicate, terms })
            .add_context("While")
            .parse(input, context, alloc)
    }
}
