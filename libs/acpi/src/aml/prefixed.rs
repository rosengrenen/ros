use super::Context;
use core::{alloc::Allocator, marker::PhantomData};
use parser::{
    error::{ParseError, ParseResult},
    input::Input,
    parser::Parser,
    sequence::preceded,
};

pub fn prefixed<I, O1, O2, E, P1, P2, A>(first: P1, second: P2) -> Prefixed<P1, P2, E>
where
    I: Input<Item = u8>,
    E: ParseError<I, A>,
    P1: Parser<I, Context, A, Output = O1, Error = E>,
    P2: Parser<I, Context, A, Output = O2, Error = E>,
    A: Allocator + Clone,
{
    Prefixed {
        first,
        second,
        error: PhantomData,
    }
}

#[derive(Clone)]
pub struct Prefixed<P1, P2, E> {
    first: P1,
    second: P2,
    error: PhantomData<E>,
}

impl<I, O1, O2, E: ParseError<I, A>, P1, P2, A: Allocator + Clone> Parser<I, Context, A>
    for Prefixed<P1, P2, E>
where
    I: Input<Item = u8>,
    E: ParseError<I, A>,
    P1: Parser<I, Context, A, Output = O1, Error = E>,
    P2: Parser<I, Context, A, Output = O2, Error = E>,
    A: Allocator + Clone,
{
    type Output = O2;

    type Error = E;

    fn parse(
        self,
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self::Output, Self::Error> {
        preceded(self.first, self.second.cut())
            .add_context("Prefixed")
            .parse(input, context, alloc)
    }
}
