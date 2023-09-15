use crate::{
    error::{ParseError, ParseErrorKind, ParseResult},
    input::Input,
    parser::Parser,
};
use core::alloc::Allocator;

pub struct AndThen<P1, P2> {
    pub(crate) first: P1,
    pub(crate) second: P2,
}

impl<I, O1, O2, E, P1, P2, A> Parser<I, A> for AndThen<P1, P2>
where
    I: Input,
    E: ParseError<I, A>,
    P1: Parser<I, A, Output = O1, Error = E>,
    P2: Parser<O1, A, Output = O2, Error = E>,
    A: Allocator + Clone,
{
    type Output = O2;

    type Error = E;

    fn parse(&self, input: I, alloc: A) -> ParseResult<I, Self::Output, Self::Error> {
        self.first
            .parse(input.clone(), alloc.clone())
            .and_then(|(_, output)| self.second.parse(output, alloc))
            .map(|(_, output)| (input.clone(), output))
            .map_err(|error| error.append(input, ParseErrorKind::AndThen))
    }
}
