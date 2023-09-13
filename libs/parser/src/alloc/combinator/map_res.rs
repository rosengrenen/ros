use crate::{
    alloc::parser::ParserAlloc,
    combinator::map_res::MapRes,
    error::{FromExternalError, ParseError, ParseResult},
};
use core::alloc::Allocator;

impl<'alloc, I, O1, O2, E1, E2, P, F, A> ParserAlloc<'alloc, I, A> for MapRes<P, F, E2>
where
    I: Clone,
    E1: ParseError<I> + FromExternalError<I, E2>,
    P: ParserAlloc<'alloc, I, A, Output = O1, Error = E1>,
    F: Fn(O1) -> Result<O2, E2>,
    A: Allocator,
{
    type Output = O2;

    type Error = P::Error;

    fn parse_alloc(&self, input: I, alloc: &'alloc A) -> ParseResult<I, Self::Output, Self::Error> {
        self.inner(self.parser.parse_alloc(input, alloc))
    }
}
