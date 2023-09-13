use crate::{alloc::parser::ParserAlloc, combinator::map::Map, error::ParseResult};
use core::alloc::Allocator;

impl<'alloc, I, O, P, F, A> ParserAlloc<'alloc, I, A> for Map<P, F>
where
    P: ParserAlloc<'alloc, I, A>,
    F: Fn(P::Output) -> O,
    A: Allocator,
{
    type Output = O;

    type Error = P::Error;

    fn parse_alloc(&self, input: I, alloc: &'alloc A) -> ParseResult<I, Self::Output, Self::Error> {
        self.inner(self.parser.parse_alloc(input, alloc))
    }
}
