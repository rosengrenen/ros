use crate::{
    error::{ParseError, ParseResult, ParserError},
    parser::Parser,
};

pub struct Opt<P> {
    pub(crate) parser: P,
}

impl<P> Opt<P> {
    pub(crate) fn inner<I, O, E>(
        result: ParseResult<I, O, E>,
        input: I,
    ) -> ParseResult<I, Option<O>, E> {
        match result {
            Ok((input, output)) => Ok((input, Some(output))),
            Err(ParserError::Error(_)) => Ok((input, None)),
            Err(ParserError::Failure(error)) => Err(ParserError::Failure(error)),
        }
    }
}

impl<I, O, E, P> Parser<I> for Opt<P>
where
    I: Clone,
    E: ParseError<I>,
    P: Parser<I, Output = O, Error = E>,
{
    type Output = Option<P::Output>;

    type Error = P::Error;

    fn parse(&self, input: I) -> ParseResult<I, Self::Output, Self::Error> {
        Self::inner(self.parser.parse(input.clone()), input)
    }
}
