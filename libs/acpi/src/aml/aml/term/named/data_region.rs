use crate::aml::{
    aml::{name::NameString, term::term_arg::TermArg},
    ops::DataRegionOp,
    prefixed, Context,
};
use core::alloc::Allocator;
use parser::{
    error::{ParseError, ParseResult},
    input::Input,
    parser::Parser,
};

#[derive(Debug)]
pub struct DataRegion<A: Allocator> {
    pub name: NameString<A>,
    pub term1: TermArg<A>,
    pub term2: TermArg<A>,
    pub term3: TermArg<A>,
}

impl<A: Allocator + Clone> DataRegion<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        prefixed(
            DataRegionOp::p,
            (NameString::p, TermArg::p, TermArg::p, TermArg::p),
        )
        .map(|(name, term1, term2, term3)| Self {
            name,
            term1,
            term2,
            term3,
        })
        .add_context("DataRegion")
        .parse(input, context, alloc)
    }
}
