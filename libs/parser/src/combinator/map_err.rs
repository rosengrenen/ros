use crate::{
    error::{ParseResult, ParserError},
    parser::Parser,
};
use core::alloc::Allocator;

pub struct MapErr<P, F> {
    pub(crate) parser: P,
    pub(crate) f: F,
}

impl<'alloc, I, P, F, A> Parser<'alloc, I, A> for MapErr<P, F>
where
    I: Clone,
    P: Parser<'alloc, I, A>,
    F: Fn(P::Error) -> P::Error,
    A: Allocator,
{
    type Output = P::Output;

    type Error = P::Error;

    fn parse(
        &self,
        input: I,
        alloc: &'alloc A,
    ) -> ParseResult<'alloc, I, Self::Output, Self::Error> {
        self.parser.parse(input, alloc).map_err(|e| match e {
            ParserError::Error(error) => ParserError::Error((self.f)(error)),
            ParserError::Failure(error) => ParserError::Failure(error),
        })
    }
}
