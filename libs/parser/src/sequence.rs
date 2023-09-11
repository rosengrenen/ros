use crate::{
    error::{ParseError, ParserError},
    parser::Parser,
};

pub fn preceded<I, O1, O2, E, P1, P2>(
    first: P1,
    second: P2,
) -> impl Parser<I, Output = O2, Error = E>
where
    E: ParseError<I>,
    P1: Parser<I, Output = O1, Error = E>,
    P2: Parser<I, Output = O2, Error = E>,
{
    Preceded { first, second }
}

pub struct Preceded<P1, P2> {
    first: P1,
    second: P2,
}

impl<I, O1, O2, E, P1, P2> Parser<I> for Preceded<P1, P2>
where
    E: ParseError<I>,
    P1: Parser<I, Output = O1, Error = E>,
    P2: Parser<I, Output = O2, Error = E>,
{
    type Output = P2::Output;

    type Error = E;

    fn parse(&self, input0: I) -> crate::error::ParseResult<I, Self::Output, Self::Error> {
        match self.first.parse(input0) {
            Ok((input1, _)) => match self.second.parse(input1) {
                Ok(result) => Ok(result),
                Err(ParserError::Error(error)) => Err(ParserError::Error(error)),
                Err(ParserError::Failure(error)) => Err(ParserError::Failure(error)),
            },
            Err(ParserError::Error(error)) => Err(ParserError::Error(error)),
            Err(ParserError::Failure(error)) => Err(ParserError::Failure(error)),
        }
    }
}
