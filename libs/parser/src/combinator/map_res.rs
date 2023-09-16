use crate::{
    error::{FromExternalError, ParseError, ParseErrorKind, ParseResult, ParserError},
    input::Input,
    parser::Parser,
};
use core::{alloc::Allocator, marker::PhantomData};

pub struct MapRes<'p, P, F, E> {
    pub(crate) parser: &'p P,
    pub(crate) f: F,
    pub(crate) error: PhantomData<E>,
}

impl<'p, I, O1, O2, E1, E2, P, F, A> Parser<I, A> for MapRes<'p, P, F, E2>
where
    I: Input,
    E1: ParseError<I, A> + FromExternalError<I, E2, A>,
    P: Parser<I, A, Output = O1, Error = E1>,
    F: Fn(O1) -> Result<O2, E2>,
    A: Allocator + Clone,
{
    type Output = O2;

    type Error = P::Error;

    fn parse(&self, input: I, alloc: A) -> ParseResult<I, Self::Output, Self::Error> {
        self.parser
            .parse(input.clone(), alloc.clone())
            .and_then(|(input, output)| {
                let output = {
                    let input = input.clone();
                    (self.f)(output).map_err(move |error| {
                        ParserError::Error(E1::from_external_error(
                            input,
                            ParseErrorKind::Unknown,
                            error,
                            alloc,
                        ))
                    })?
                };
                Ok((input, output))
            })
            .map_err(|error| error.append(input, ParseErrorKind::MapRes))
    }
}

pub struct MapRes1<'p, P, F, E> {
    pub(crate) parser: &'p P,
    pub(crate) f: F,
    pub(crate) error: PhantomData<E>,
}

impl<'p, I, O1, O2, E1, E2, P, F, A> Parser<I, A> for MapRes1<'p, P, F, E2>
where
    I: Input,
    E1: ParseError<I, A>,
    P: Parser<I, A, Output = O1, Error = E1>,
    F: Fn(O1) -> Result<O2, E2>,
    A: Allocator + Clone,
{
    type Output = O2;

    type Error = P::Error;

    fn parse(&self, input: I, alloc: A) -> ParseResult<I, Self::Output, Self::Error> {
        self.parser
            .parse(input.clone(), alloc.clone())
            .and_then(|(input, output)| {
                let output = {
                    let input = input.clone();
                    (self.f)(output).map_err(|_| {
                        ParserError::Error(E1::from_error_kind(
                            input,
                            ParseErrorKind::Unknown,
                            alloc,
                        ))
                    })?
                };
                Ok((input, output))
            })
            .map_err(|error| error.append(input, ParseErrorKind::MapRes1))
    }
}
