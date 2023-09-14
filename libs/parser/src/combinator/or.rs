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
        self.first
            .parse(input.clone(), alloc)
            .map_or_else(
                |error| match error {
                    ParserError::Error(_) => self.second.parse(input.clone(), alloc),
                    ParserError::Failure(error) => Err(ParserError::Failure(error)),
                },
                Ok,
            )
            .map_err(|error| error.append(input, ParseErrorKind::Or))
    }
}
