use crate::{
    error::{ParseError, ParseErrorKind, ParseResult, ParserError},
    input::Input,
    parser::Parser,
};
use core::alloc::Allocator;

#[derive(Clone)]
pub struct Or<P1, P2> {
    pub(crate) first: P1,
    pub(crate) second: P2,
}

impl<I, O, E, C, P1, P2, A> Parser<I, C, A> for Or<P1, P2>
where
    I: Input,
    P1: Parser<I, C, A, Output = O, Error = E>,
    P2: Parser<I, C, A, Output = O, Error = E>,
    E: ParseError<I, A>,
    A: Allocator + Clone,
{
    type Output = O;

    type Error = E;

    fn parse(
        self,
        input: I,
        context: &mut C,
        alloc: A,
    ) -> ParseResult<I, Self::Output, Self::Error> {
        self.first
            .parse(input.clone(), context, alloc.clone())
            .map_or_else(
                |error| match error {
                    ParserError::Error(_) => self.second.parse(input.clone(), context, alloc),
                    ParserError::Failure(error) => Err(ParserError::Failure(error)),
                },
                Ok,
            )
            .map_err(|error| error.append(input, ParseErrorKind::Or))
    }
}
