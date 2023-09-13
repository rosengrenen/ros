use crate::{
    error::{ParseError, ParseErrorKind, ParseResult, ParserError},
    parser::Parser,
};
use core::alloc::Allocator;

pub struct Or<P1, P2> {
    pub(crate) first: P1,
    pub(crate) second: P2,
}

impl<'alloc, I, O, P1, P2, E, A> Parser<'alloc, I, A> for Or<P1, P2>
where
    I: Clone,
    P1: Parser<'alloc, I, A, Output = O, Error = E>,
    P2: Parser<'alloc, I, A, Output = O, Error = E>,
    E: ParseError<'alloc, I, A>,
    A: Allocator,
{
    type Output = O;

    type Error = E;

    fn parse(
        &self,
        input: I,
        alloc: &'alloc A,
    ) -> ParseResult<'alloc, I, Self::Output, Self::Error> {
        match self.first.parse(input.clone(), alloc) {
            Ok(result) => Ok(result),
            Err(ParserError::Error(_)) => {
                self.second.parse(input.clone(), alloc).map_err(|error| {
                    error.map(|_| E::from_error_kind(input, ParseErrorKind::None, alloc))
                })
            }
            Err(ParserError::Failure(error)) => Err(ParserError::Failure(error)),
        }
    }
}
