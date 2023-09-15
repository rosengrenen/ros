use crate::{
    error::{ParseError, ParseErrorKind, ParseResult, ParserError},
    input::Input,
    parser::Parser,
};
use core::alloc::Allocator;

pub struct Cut<P> {
    pub(crate) parser: P,
}

impl<I, P, A> Parser<I, A> for Cut<P>
where
    I: Input,
    P: Parser<I, A>,
    A: Allocator,
{
    type Output = P::Output;

    type Error = P::Error;

    fn parse(&self, input: I, alloc: A) -> ParseResult<I, Self::Output, Self::Error> {
        self.parser
            .parse(input.clone(), alloc)
            .map_err(|error| match error {
                ParserError::Error(error) => {
                    ParserError::Failure(error.append(input, ParseErrorKind::Cut))
                }
                ParserError::Failure(error) => {
                    ParserError::Failure(error.append(input, ParseErrorKind::Cut))
                }
            })
    }
}
