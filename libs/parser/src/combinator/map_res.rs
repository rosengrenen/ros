use crate::{
    error::{FromExternalError, ParseError, ParseErrorKind, ParseResult, ParserError},
    parser::Parser,
};
use core::marker::PhantomData;

pub struct MapRes<P, F, E> {
    pub(crate) parser: P,
    pub(crate) f: F,
    pub(crate) error: PhantomData<E>,
}

impl<P, F, E2> MapRes<P, F, E2> {
    pub(crate) fn inner<I, O1, O2, E1>(
        &self,
        result: ParseResult<I, O1, E1>,
    ) -> ParseResult<I, O2, E1>
    where
        I: Clone,
        E1: ParseError<I> + FromExternalError<I, E2>,
        F: Fn(O1) -> Result<O2, E2>,
    {
        result.and_then(|(input, output)| {
            let output = {
                let input = input.clone();
                (self.f)(output).map_err(move |error| {
                    ParserError::Error(E1::from_external_error(input, ParseErrorKind::None, error))
                })?
            };
            Ok((input, output))
        })
    }
}

impl<I, O1, O2, E1, E2, P, F> Parser<I> for MapRes<P, F, E2>
where
    I: Clone,
    E1: ParseError<I> + FromExternalError<I, E2>,
    P: Parser<I, Output = O1, Error = E1>,
    F: Fn(O1) -> Result<O2, E2>,
{
    type Output = O2;

    type Error = P::Error;

    fn parse(&self, input: I) -> ParseResult<I, Self::Output, Self::Error> {
        self.inner(self.parser.parse(input))
    }
}

pub struct MapRes1<P, F, E> {
    pub(crate) parser: P,
    pub(crate) f: F,
    pub(crate) error: PhantomData<E>,
}

impl<P, F, E2> MapRes1<P, F, E2> {
    pub(crate) fn inner<I, O1, O2, E1>(
        &self,
        result: ParseResult<I, O1, E1>,
    ) -> ParseResult<I, O2, E1>
    where
        I: Clone,
        E1: ParseError<I>,
        F: Fn(O1) -> Result<O2, E2>,
    {
        result.and_then(|(input, output)| {
            let output = {
                let input = input.clone();
                (self.f)(output).map_err(|_| {
                    ParserError::Error(E1::from_error_kind(input, ParseErrorKind::None))
                })?
            };
            Ok((input, output))
        })
    }
}

impl<I, O1, O2, E1, E2, P, F> Parser<I> for MapRes1<P, F, E2>
where
    I: Clone,
    E1: ParseError<I>,
    P: Parser<I, Output = O1, Error = E1>,
    F: Fn(O1) -> Result<O2, E2>,
{
    type Output = O2;

    type Error = P::Error;

    fn parse(&self, input: I) -> ParseResult<I, Self::Output, Self::Error> {
        self.inner(self.parser.parse(input))
    }
}
