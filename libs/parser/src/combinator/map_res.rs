use crate::{
    error::{FromExternalError, ParseError, ParseErrorKind, ParseResult, ParserError},
    parser::Parser,
};
use core::{alloc::Allocator, marker::PhantomData};

pub struct MapRes<P, F, E> {
    pub(crate) parser: P,
    pub(crate) f: F,
    pub(crate) error: PhantomData<E>,
}

impl<'alloc, I, O1, O2, E1, E2, P, F, A> Parser<'alloc, I, A> for MapRes<P, F, E2>
where
    I: Clone,
    E1: ParseError<'alloc, I, A> + FromExternalError<'alloc, I, E2, A>,
    P: Parser<'alloc, I, A, Output = O1, Error = E1>,
    F: Fn(O1) -> Result<O2, E2>,
    A: Allocator,
{
    type Output = O2;

    type Error = P::Error;

    fn parse(
        &self,
        input: I,
        alloc: &'alloc A,
    ) -> ParseResult<'alloc, I, Self::Output, Self::Error> {
        self.parser.parse(input, alloc).and_then(|(input, output)| {
            let output = {
                let input = input.clone();
                (self.f)(output).map_err(move |error| {
                    ParserError::Error(E1::from_external_error(
                        input,
                        ParseErrorKind::None,
                        error,
                        alloc,
                    ))
                })?
            };
            Ok((input, output))
        })
    }
}

pub struct MapRes1<P, F, E> {
    pub(crate) parser: P,
    pub(crate) f: F,
    pub(crate) error: PhantomData<E>,
}

impl<'alloc, I, O1, O2, E1, E2, P, F, A> Parser<'alloc, I, A> for MapRes1<P, F, E2>
where
    I: Clone,
    E1: ParseError<'alloc, I, A>,
    P: Parser<'alloc, I, A, Output = O1, Error = E1>,
    F: Fn(O1) -> Result<O2, E2>,
    A: Allocator,
{
    type Output = O2;

    type Error = P::Error;

    fn parse(
        &self,
        input: I,
        alloc: &'alloc A,
    ) -> ParseResult<'alloc, I, Self::Output, Self::Error> {
        self.parser.parse(input, alloc).and_then(|(input, output)| {
            let output = {
                let input = input.clone();
                (self.f)(output).map_err(|_| {
                    ParserError::Error(E1::from_error_kind(input, ParseErrorKind::None, alloc))
                })?
            };
            Ok((input, output))
        })
    }
}
