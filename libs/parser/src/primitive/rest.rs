use core::alloc::Allocator;
use core::marker::PhantomData;

use crate::error::ParseError;
use crate::error::ParseResult;
use crate::input::Input;
use crate::parser::Parser;

pub fn rest<I, E, C, A>() -> impl Parser<I, C, A, Output = I, Error = E>
where
    I: Input,
    E: ParseError<I, A>,
    A: Allocator,
{
    Rest { error: PhantomData }
}

#[derive(Clone)]
pub struct Rest<E> {
    error: PhantomData<E>,
}

impl<I, E, C, A> Parser<I, C, A> for Rest<E>
where
    I: Input,
    E: ParseError<I, A>,
    A: Allocator,
{
    type Output = I;

    type Error = E;

    fn parse(
        self,
        input: I,
        _context: &mut C,
        _alloc: A,
    ) -> ParseResult<I, Self::Output, Self::Error> {
        let (output, input) = input.split_at_index_unchecked(input.input_len());
        Ok((input, output))
    }
}
