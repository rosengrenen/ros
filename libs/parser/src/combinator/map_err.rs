use crate::{
    error::{ParseResult, ParserError},
    parser::Parser,
};

pub struct MapErr<P, F> {
    pub(crate) parser: P,
    pub(crate) f: F,
}

impl<P, F> MapErr<P, F> {
    pub(crate) fn inner<I, O, E>(&self, result: ParseResult<I, O, E>) -> ParseResult<I, O, E>
    where
        F: Fn(E) -> E,
    {
        result.map_err(|e| match e {
            ParserError::Error(error) => ParserError::Error((self.f)(error)),
            ParserError::Failure(error) => ParserError::Failure(error),
        })
    }
}

impl<I, P, F> Parser<I> for MapErr<P, F>
where
    I: Clone,
    P: Parser<I>,
    F: Fn(P::Error) -> P::Error,
{
    type Output = P::Output;

    type Error = P::Error;

    fn parse(&self, input: I) -> ParseResult<I, Self::Output, Self::Error> {
        self.inner(self.parser.parse(input))
    }
}
