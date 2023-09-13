use crate::{error::ParseResult, parser::Parser};
use core::alloc::Allocator;

pub struct Cut<P> {
    pub(crate) parser: P,
}

impl<'alloc, I, P, A> Parser<'alloc, I, A> for Cut<P>
where
    P: Parser<'alloc, I, A>,
    A: Allocator,
{
    type Output = P::Output;

    type Error = P::Error;

    fn parse(
        &self,
        input: I,
        alloc: &'alloc A,
    ) -> ParseResult<'alloc, I, Self::Output, Self::Error> {
        self.parser
            .parse(input, alloc)
            .map_err(|error| error.fail())
    }
}
