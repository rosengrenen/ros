use crate::{
    error::{ParseResult, ParserError},
    parser::Parser,
};
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
        match self.parser.parse(input, alloc) {
            Ok(result) => Ok(result),
            Err(ParserError::Error(error)) => Err(ParserError::Failure(error)),
            Err(ParserError::Failure(error)) => Err(ParserError::Failure(error)),
        }
    }
}
