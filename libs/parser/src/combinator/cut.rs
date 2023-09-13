use crate::{
    error::{ParseResult, ParserError},
    parser::Parser,
};

pub struct Cut<P> {
    pub(crate) parser: P,
}

impl<P> Cut<P> {
    pub(crate) fn inner<I, O, E>(result: ParseResult<I, O, E>) -> ParseResult<I, O, E> {
        match result {
            Ok(result) => Ok(result),
            Err(ParserError::Error(error)) => Err(ParserError::Failure(error)),
            Err(ParserError::Failure(error)) => Err(ParserError::Failure(error)),
        }
    }
}

impl<I, P> Parser<I> for Cut<P>
where
    P: Parser<I>,
{
    type Output = P::Output;

    type Error = P::Error;

    fn parse(&self, input: I) -> ParseResult<I, Self::Output, Self::Error> {
        Self::inner(self.parser.parse(input))
    }
}
