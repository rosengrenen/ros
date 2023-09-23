use crate::{
    error::{ParseError, ParseErrorKind, ParseResult, ParserError},
    input::Input,
    parser::Parser,
};
use core::{alloc::Allocator, marker::PhantomData};

pub fn fail<I, E, C, A>() -> impl Parser<I, C, A, Output = (), Error = E>
where
    I: Input,
    E: ParseError<I, A>,
    A: Allocator + Clone,
{
    Fail { error: PhantomData }
}

#[derive(Clone)]
pub struct Fail<E> {
    error: PhantomData<E>,
}

impl<I, E, C, A> Parser<I, C, A> for Fail<E>
where
    I: Input,
    E: ParseError<I, A>,
    A: Allocator,
{
    type Output = ();

    type Error = E;

    fn parse(
        &self,
        input: I,
        _context: &mut C,
        alloc: A,
    ) -> ParseResult<I, Self::Output, Self::Error> {
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
