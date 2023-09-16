use crate::{
    error::{ParseError, ParseResult, ParserError},
    input::Input,
    parser::Parser,
};
use core::alloc::Allocator;

pub struct AddContext<'p, P> {
    pub(crate) parser: &'p P,
    pub(crate) context: &'static str,
}

impl<'p, I, O, E, P, A> Parser<I, A> for AddContext<'p, P>
where
    I: Input,
    E: ParseError<I, A>,
    P: Parser<I, A, Output = O, Error = E>,
    A: Allocator + Clone,
{
    type Output = O;

    type Error = E;

    fn parse(&self, input: I, alloc: A) -> ParseResult<I, Self::Output, Self::Error> {
        match self.parser.parse(input.clone(), alloc.clone()) {
            Ok(result) => Ok(result),
            Err(error) => match error {
                ParserError::Error(error) => Err(ParserError::Error(error)),
                ParserError::Failure(error) => {
                    Err(ParserError::Failure(error.add_context(input, self.context)))
                }
            },
        }
    }
}
