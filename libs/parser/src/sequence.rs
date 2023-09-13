use crate::{
    error::{ParseError, ParserError},
    parser::Parser,
};
use core::alloc::Allocator;

pub fn preceded<'alloc, I, O1, O2, E, P1, P2, A>(
    first: P1,
    second: P2,
) -> impl Parser<'alloc, I, A, Output = O2, Error = E>
where
    E: ParseError<'alloc, I, A>,
    P1: Parser<'alloc, I, A, Output = O1, Error = E>,
    P2: Parser<'alloc, I, A, Output = O2, Error = E>,
    A: Allocator,
{
    Preceded { first, second }
}

pub struct Preceded<P1, P2> {
    first: P1,
    second: P2,
}

impl<'alloc, I, O1, O2, E, P1, P2, A> Parser<'alloc, I, A> for Preceded<P1, P2>
where
    E: ParseError<'alloc, I, A>,
    P1: Parser<'alloc, I, A, Output = O1, Error = E>,
    P2: Parser<'alloc, I, A, Output = O2, Error = E>,
    A: Allocator,
{
    type Output = P2::Output;

    type Error = E;

    fn parse(
        &self,
        input0: I,
        alloc: &'alloc A,
    ) -> crate::error::ParseResult<'alloc, I, Self::Output, Self::Error> {
        match self.first.parse(input0, alloc) {
            Ok((input1, _)) => match self.second.parse(input1, alloc) {
                Ok(result) => Ok(result),
                Err(ParserError::Error(error)) => Err(ParserError::Error(error)),
                Err(ParserError::Failure(error)) => Err(ParserError::Failure(error)),
            },
            Err(ParserError::Error(error)) => Err(ParserError::Error(error)),
            Err(ParserError::Failure(error)) => Err(ParserError::Failure(error)),
        }
    }
}
