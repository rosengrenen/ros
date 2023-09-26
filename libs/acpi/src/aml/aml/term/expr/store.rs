use crate::aml::{
    aml::{name::SuperName, term::term_arg::TermArg},
    ops::StoreOp,
    prefixed, Context,
};
use core::alloc::Allocator;
use parser::{
    error::{ParseError, ParseResult},
    input::Input,
    parser::Parser,
};

#[derive(Debug)]
pub struct Store<A: Allocator> {
    pub term: TermArg<A>,
    pub name: SuperName<A>,
}

impl<A: Allocator + Clone> Store<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        prefixed(StoreOp::p, (TermArg::p, SuperName::p))
            .map(|(term, name)| Self { term, name })
            .add_context("Store")
            .parse(input, context, alloc)
    }
}
