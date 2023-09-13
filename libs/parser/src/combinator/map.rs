use crate::{error::ParseResult, parser::Parser};
use core::alloc::Allocator;

pub struct Map<P, F> {
    pub(crate) parser: P,
    pub(crate) f: F,
}

impl<P, F> Map<P, F> {
    pub(crate) fn inner<'alloc, I, O1, O2, E>(
        &self,
        result: ParseResult<'alloc, I, O1, E>,
    ) -> ParseResult<'alloc, I, O2, E>
    where
        F: Fn(O1) -> O2,
    {
        result.map(|(input, output)| (input, (self.f)(output)))
    }
}

impl<'alloc, I, O, P, F, A> Parser<'alloc, I, A> for Map<P, F>
where
    P: Parser<'alloc, I, A>,
    F: Fn(P::Output) -> O,
    A: Allocator,
{
    type Output = O;

    type Error = P::Error;

    fn parse(
        &self,
        input: I,
        alloc: &'alloc A,
    ) -> ParseResult<'alloc, I, Self::Output, Self::Error> {
        self.inner(self.parser.parse(input, alloc))
    }
}
