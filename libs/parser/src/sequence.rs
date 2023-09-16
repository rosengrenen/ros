use crate::{
    error::{ParseError, ParseErrorKind},
    input::Input,
    parser::Parser,
};
use core::alloc::Allocator;

pub fn preceded<'p, I, O1, O2, E, P1, P2, A>(first: &'p P1, second: &'p P2) -> Preceded<'p, P1, P2>
where
    I: Input,
    E: ParseError<I, A>,
    P1: Parser<I, A, Output = O1, Error = E>,
    P2: Parser<I, A, Output = O2, Error = E>,
    A: Allocator + Clone,
{
    Preceded { first, second }
}

pub struct Preceded<'p, P1, P2> {
    first: &'p P1,
    second: &'p P2,
}

impl<'p, I, O1, O2, E, P1, P2, A> Parser<I, A> for Preceded<'p, P1, P2>
where
    I: Input,
    E: ParseError<I, A>,
    P1: Parser<I, A, Output = O1, Error = E>,
    P2: Parser<I, A, Output = O2, Error = E>,
    A: Allocator + Clone,
{
    type Output = P2::Output;

    type Error = E;

    fn parse(&self, input: I, alloc: A) -> crate::error::ParseResult<I, Self::Output, Self::Error> {
        self.first
            .parse(input.clone(), alloc.clone())
            .and_then(|(input, _)| self.second.parse(input, alloc))
            .map_err(|error| error.append(input, ParseErrorKind::Preceded))
    }
}
