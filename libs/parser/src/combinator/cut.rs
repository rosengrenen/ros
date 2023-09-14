use crate::{
    error::{ParseError, ParseErrorKind, ParseResult, ParserError},
    parser::Parser,
};
use core::alloc::Allocator;

pub struct Cut<P> {
    pub(crate) parser: P,
}

impl<'alloc, I, P, A> Parser<'alloc, I, A> for Cut<P>
where
    I: Clone,
    P: Parser<'alloc, I, A>,
    A: Allocator,
{
    type Output = P::Output;

    type Error = P::Error;

    fn parse(
        &self,
        input: I,
        alloc: &'alloc A,
    ) -> ParseResult<'alloc, I, Self::Output, Self::Error> {
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
