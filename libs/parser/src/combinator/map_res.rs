use crate::{
    error::{FromExternalError, ParseError, ParseErrorKind, ParseResult, ParserError},
    input::Input,
    parser::Parser,
};
use core::{alloc::Allocator, marker::PhantomData};

#[derive(Clone)]
pub struct MapRes<P, F, E> {
    pub(crate) parser: P,
    pub(crate) f: F,
    pub(crate) error: PhantomData<E>,
}

impl<I, O1, O2, E1, E2, C, P, F, A> Parser<I, C, A> for MapRes<P, F, E2>
where
    I: Input,
    E1: ParseError<I, A> + FromExternalError<I, E2, A>,
    E2: Clone,
    P: Parser<I, C, A, Output = O1, Error = E1>,
    F: Fn(O1) -> Result<O2, E2> + Clone,
    A: Allocator + Clone,
{
    type Output = O2;

    type Error = P::Error;

    fn parse(
        &self,
        input: I,
        context: &mut C,
        alloc: A,
    ) -> ParseResult<I, Self::Output, Self::Error> {
        self.parser
            .parse(input.clone(), context, alloc.clone())
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

#[derive(Clone)]
pub struct MapRes1<P, F, E> {
    pub(crate) parser: P,
    pub(crate) f: F,
    pub(crate) error: PhantomData<E>,
}

impl<I, O1, O2, E1, E2, C, P, F, A> Parser<I, C, A> for MapRes1<P, F, E1>
where
    I: Input,
    E1: ParseError<I, A>,
    P: Parser<I, C, A, Output = O1, Error = E1>,
    F: Fn(O1) -> Result<O2, E2> + Clone,
    A: Allocator + Clone,
{
    type Output = O2;

    type Error = P::Error;

    fn parse(
        &self,
        input: I,
        context: &mut C,
        alloc: A,
    ) -> ParseResult<I, Self::Output, Self::Error> {
        self.parser
            .parse(input.clone(), context, alloc.clone())
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
