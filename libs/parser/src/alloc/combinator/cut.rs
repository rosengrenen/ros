use crate::{alloc::parser::ParserAlloc, combinator::cut::Cut, error::ParseResult};
use core::alloc::Allocator;

impl<'alloc, I, P, A> ParserAlloc<'alloc, I, A> for Cut<P>
where
    P: ParserAlloc<'alloc, I, A>,
    A: Allocator,
{
    type Output = P::Output;

    type Error = P::Error;

    fn parse_alloc(&self, input: I, alloc: &'alloc A) -> ParseResult<I, Self::Output, Self::Error> {
        Self::inner(self.parser.parse_alloc(input, alloc))
    }
}
