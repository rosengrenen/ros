use crate::{
    error::{ParseError, ParseErrorKind, ParseResult, ParserError},
    input::Input,
    parser::Parser,
};
use core::alloc::Allocator;

pub struct Or<P1, P2> {
    pub(crate) first: P1,
    pub(crate) second: P2,
}

impl<I, O, P1, P2, E, A> Parser<I, A> for Or<P1, P2>
where
    I: Input,
    P1: Parser<I, A, Output = O, Error = E>,
    P2: Parser<I, A, Output = O, Error = E>,
    E: ParseError<I, A>,
    A: Allocator + Clone,
{
    type Output = O;

    type Error = E;

    fn parse(&self, input: I, alloc: A) -> ParseResult<I, Self::Output, Self::Error> {
        self.first
            .parse(input.clone(), alloc.clone())
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
