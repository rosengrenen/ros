use core::alloc::Allocator;

use crate::error::ParseError;
use crate::error::ParseErrorKind;
use crate::error::ParseResult;
use crate::input::Input;
use crate::parser::Parser;

pub fn preceded<I, O1, O2, E, C, P1, P2, A>(first: P1, second: P2) -> Preceded<P1, P2>
where
    I: Input,
    E: ParseError<I, A>,
    P1: Parser<I, C, A, Output = O1, Error = E>,
    P2: Parser<I, C, A, Output = O2, Error = E>,
    A: Allocator + Clone,
{
    Preceded { first, second }
}

#[derive(Clone)]
pub struct Preceded<P1, P2> {
    first: P1,
    second: P2,
}

impl<I, O1, O2, E, C, P1, P2, A> Parser<I, C, A> for Preceded<P1, P2>
where
    I: Input,
    E: ParseError<I, A>,
    P1: Parser<I, C, A, Output = O1, Error = E>,
    P2: Parser<I, C, A, Output = O2, Error = E>,
    A: Allocator + Clone,
{
    type Output = P2::Output;

    type Error = E;

    fn parse(
        self,
        input: I,
        context: &mut C,
        alloc: A,
    ) -> ParseResult<I, Self::Output, Self::Error> {
        self.first
            .parse(input.clone(), context, alloc.clone())
            .and_then(|(input, _)| self.second.parse(input, context, alloc))
            .map_err(|error| error.append(input, ParseErrorKind::Preceded))
    }
}
