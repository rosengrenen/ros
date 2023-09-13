use crate::{alloc::parser::ParserAlloc, combinator::map_err::MapErr, error::ParseResult};
use core::alloc::Allocator;

impl<'alloc, I, P, F, A> ParserAlloc<'alloc, I, A> for MapErr<P, F>
where
    I: Clone,
    P: ParserAlloc<'alloc, I, A>,
    F: Fn(P::Error) -> P::Error,
    A: Allocator,
{
    type Output = P::Output;

    type Error = P::Error;

    fn parse_alloc(&self, input: I, alloc: &'alloc A) -> ParseResult<I, Self::Output, Self::Error> {
        self.inner(self.parser.parse_alloc(input, alloc))
    }
}
