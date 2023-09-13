use crate::{
    alloc::parser::ParserAlloc,
    combinator::or::Or,
    error::{ParseError, ParseResult},
};
use core::alloc::Allocator;

impl<'alloc, I, O, P1, P2, E, A> ParserAlloc<'alloc, I, A> for Or<P1, P2>
where
    I: Clone,
    P1: ParserAlloc<'alloc, I, A, Output = O, Error = E>,
    P2: ParserAlloc<'alloc, I, A, Output = O, Error = E>,
    E: ParseError<I>,
    A: Allocator,
{
    type Output = O;

    type Error = E;

    fn parse_alloc(&self, input: I, alloc: &'alloc A) -> ParseResult<I, Self::Output, Self::Error> {
        let Self { first, second } = self;
        let input1 = input.clone();
        Self::inner(
            input.clone(),
            || first.parse_alloc(input1, alloc),
            || second.parse_alloc(input, alloc),
        )
    }
}
