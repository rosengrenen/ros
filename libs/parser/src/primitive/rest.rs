use crate::{
    error::{ParseError, ParseResult},
    input::Input,
    parser::Parser,
};
use core::{alloc::Allocator, marker::PhantomData};

pub fn rest<I, E, A>() -> impl Parser<I, A, Output = I, Error = E>
where
    I: Input,
    E: ParseError<I, A>,
    A: Allocator,
{
    Rest { error: PhantomData }
}

pub struct Rest<E> {
    error: PhantomData<E>,
}

impl<I, E, A> Parser<I, A> for Rest<E>
where
    I: Input,
    E: ParseError<I, A>,
    A: Allocator,
{
    type Output = I;

    type Error = E;

    fn parse(&self, input: I, _alloc: A) -> ParseResult<I, Self::Output, Self::Error> {
        let (output, input) = input.split_at_index_unchecked(input.input_len());
        Ok((input, output))
    }
}
