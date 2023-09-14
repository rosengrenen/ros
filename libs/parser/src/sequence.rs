use crate::{
    error::{ParseError, ParseErrorKind},
    parser::Parser,
};
use core::alloc::Allocator;

pub fn preceded<'alloc, I, O1, O2, E, P1, P2, A>(
    first: P1,
    second: P2,
) -> impl Parser<'alloc, I, A, Output = O2, Error = E>
where
    I: Clone,
    E: ParseError<'alloc, I, A>,
    P1: Parser<'alloc, I, A, Output = O1, Error = E>,
    P2: Parser<'alloc, I, A, Output = O2, Error = E>,
    A: Allocator,
{
    Preceded { first, second }
}

pub struct Preceded<P1, P2> {
    first: P1,
    second: P2,
}

impl<'alloc, I, O1, O2, E, P1, P2, A> Parser<'alloc, I, A> for Preceded<P1, P2>
where
    I: Clone,
    E: ParseError<'alloc, I, A>,
    P1: Parser<'alloc, I, A, Output = O1, Error = E>,
    P2: Parser<'alloc, I, A, Output = O2, Error = E>,
    A: Allocator,
{
    type Output = P2::Output;

    type Error = E;

    fn parse(
        &self,
        input: I,
        alloc: &'alloc A,
    ) -> crate::error::ParseResult<'alloc, I, Self::Output, Self::Error> {
        self.first
            .parse(input.clone(), alloc)
            .and_then(|(input, _)| self.second.parse(input, alloc))
            .map_err(|error| error.append(input, ParseErrorKind::Preceded))
    }
}
