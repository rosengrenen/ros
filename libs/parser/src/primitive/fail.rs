use crate::{
    error::{ParseError, ParseErrorKind, ParseResult, ParserError},
    input::Input,
    parser::Parser,
};
use core::{alloc::Allocator, marker::PhantomData};

pub fn fail<I, E, A>() -> impl Parser<I, A, Output = (), Error = E>
where
    I: Input,
    E: ParseError<I, A>,
    A: Allocator + Clone,
{
    Fail { error: PhantomData }
}

pub struct Fail<E> {
    error: PhantomData<E>,
}

impl<I, E, A> Parser<I, A> for Fail<E>
where
    I: Input,
    E: ParseError<I, A>,
    A: Allocator,
{
    type Output = ();

    type Error = E;

    fn parse(&self, input: I, alloc: A) -> ParseResult<I, Self::Output, Self::Error> {
        match input.input_len() {
            0 => Ok((input, ())),
            _ => Err(ParserError::Failure(E::from_error_kind(
                input,
                ParseErrorKind::Fail,
                alloc,
            ))),
        }
    }
}
