use crate::{
    error::{ParseError, ParseErrorKind, ParseResult},
    input::Input,
    parser::Parser,
};
use core::alloc::Allocator;

#[derive(Clone)]
pub struct AndThen<P1, P2> {
    pub(crate) first: P1,
    pub(crate) second: P2,
}

impl<I, O1, O2, E, C, P1, P2, A> Parser<I, C, A> for AndThen<P1, P2>
where
    I: Input,
    E: ParseError<I, A>,
    P1: Parser<I, C, A, Output = O1, Error = E>,
    P2: Parser<O1, C, A, Output = O2, Error = E>,
    A: Allocator + Clone,
{
    type Output = O2;

    type Error = E;

    fn parse(
        &self,
        input: I,
        context: &mut C,
        alloc: A,
    ) -> ParseResult<I, Self::Output, Self::Error> {
        self.first
            .parse(input.clone(), context, alloc.clone())
            .and_then(|(_, output)| self.second.parse(output, context, alloc))
            .map(|(_, output)| (input.clone(), output))
            .map_err(|error| error.append(input, ParseErrorKind::AndThen))
    }
}
