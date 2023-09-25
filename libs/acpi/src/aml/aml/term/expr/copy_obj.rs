use crate::aml::{
    aml::{name::SimpleName, term::TermArg},
    ops::CopyObjOp,
    prefixed, Context,
};
use core::alloc::Allocator;
use parser::{
    error::{ParseError, ParseResult},
    input::Input,
    parser::Parser,
};

#[derive(Debug)]
pub struct CopyObj<A: Allocator> {
    pub arg: TermArg<A>,
    pub name: SimpleName<A>,
}

impl<A: Allocator + Clone> CopyObj<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        prefixed(CopyObjOp::p, (TermArg::p, SimpleName::p))
            .map(|(arg, name)| Self { arg, name })
            .add_context("CopyObj")
            .parse(input, context, alloc)
    }
}
