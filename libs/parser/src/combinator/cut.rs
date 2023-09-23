use crate::{
    error::{ParseError, ParseErrorKind, ParseResult, ParserError},
    input::Input,
    parser::Parser,
};
use core::alloc::Allocator;

#[derive(Clone)]
pub struct Cut<P> {
    pub(crate) parser: P,
}

impl<I, C, P, A> Parser<I, C, A> for Cut<P>
where
    I: Input,
    P: Parser<I, C, A>,
    A: Allocator,
{
    type Output = P::Output;

    type Error = P::Error;

    fn parse(
        &self,
        input: I,
        context: &mut C,
        alloc: A,
    ) -> ParseResult<I, Self::Output, Self::Error> {
        self.parser
            .parse(input.clone(), context, alloc)
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
