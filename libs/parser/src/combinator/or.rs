use crate::{
    error::{ParseError, ParseErrorKind, ParseResult, ParserError},
    parser::Parser,
};

pub struct Or<P1, P2> {
    pub(crate) first: P1,
    pub(crate) second: P2,
}

impl<P1, P2> Or<P1, P2> {
    pub(crate) fn inner<I, O, E, F1, F2>(input: I, first: F1, second: F2) -> ParseResult<I, O, E>
    where
        E: ParseError<I>,
        F1: FnOnce() -> ParseResult<I, O, E>,
        F2: FnOnce() -> ParseResult<I, O, E>,
    {
        match first() {
            Ok(result) => Ok(result),
            Err(ParserError::Error(_)) => match second() {
                Ok(result) => Ok(result),
                Err(ParserError::Error(_)) => Err(ParserError::Error(E::from_error_kind(
                    input,
                    ParseErrorKind::None,
                ))),
                Err(ParserError::Failure(error)) => Err(ParserError::Failure(error)),
            },
            Err(ParserError::Failure(error)) => Err(ParserError::Failure(error)),
        }
    }
}

impl<I, O, P1, P2, E> Parser<I> for Or<P1, P2>
where
    I: Clone,
    P1: Parser<I, Output = O, Error = E>,
    P2: Parser<I, Output = O, Error = E>,
    E: ParseError<I>,
{
    type Output = O;

    type Error = E;

    fn parse(&self, input: I) -> ParseResult<I, Self::Output, Self::Error> {
        let Self { first, second } = self;
        let input1 = input.clone();
        Self::inner(
            input.clone(),
            || first.parse(input1),
            || second.parse(input),
        )
    }
}
